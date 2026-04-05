// src/agent/auto/verify.rs
// Auto verify - bottom-up verification of spec/code alignment

use super::lock::create_lock;
use crate::fs::spec_dir;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyIssue {
    pub severity: String,
    pub description: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResult {
    pub topic: String,
    pub area: String,
    pub aligned: bool,
    pub issues: Vec<VerifyIssue>,
    pub summary_path: String,
    pub locked: bool,
    pub lock_message: Option<String>,
}

fn get_topic_description(topic: &str, area: &str) -> Result<String> {
    let topic_path = spec_dir().join(area).join(topic).join("topic.md");

    if topic_path.exists() {
        let content = fs::read_to_string(&topic_path)?;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Short:") || trimmed.starts_with("Description:") {
                return Ok(trimmed.split(':').nth(1).unwrap_or("").trim().to_string());
            }
        }
    }

    Ok(topic.to_string())
}

fn get_linked_files(topic: &str, area: &str) -> Result<Vec<String>> {
    let index_path = spec_dir().join("index.toml");
    let mut files = Vec::new();

    if !index_path.exists() {
        return Ok(files);
    }

    let content = fs::read_to_string(&index_path)?;

    for line in content.lines() {
        if line.contains(&format!("topic = \"{}\"", topic))
            || line.contains(&format!("topic = '{}'", topic))
        {
            continue;
        }

        if line.starts_with("path") || line.contains("path =") {
            if let Some(path) = line.split("path = ").nth(1) {
                let path = path.trim().trim_matches('"').trim_matches('\'');
                files.push(path.to_string());
            }
        }
    }

    if files.is_empty() {
        let topic_spec_dir = spec_dir().join(area).join(topic);
        if topic_spec_dir.exists() {
            for entry in walkdir::WalkDir::new(&topic_spec_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_string();
                        if ["rs", "js", "ts", "py", "go", "sh"].contains(&ext_str.as_str()) {
                            files.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}

fn parse_spec_requirements(topic: &str, area: &str) -> Result<Vec<String>> {
    let spec_path = spec_dir().join(area).join(topic).join("specs.md");
    let mut requirements = Vec::new();

    if !spec_path.exists() {
        return Ok(requirements);
    }

    let content = fs::read_to_string(&spec_path)?;
    let mut in_requirements = false;
    let mut in_acceptance = false;

    for line in content.lines() {
        let trimmed = line.trim();

        let lower = trimmed.to_lowercase();
        if lower.contains("requirement") {
            in_requirements = true;
            in_acceptance = false;
            continue;
        } else if lower.contains("acceptance") {
            in_requirements = false;
            in_acceptance = true;
            continue;
        }

        if trimmed.starts_with("- [ ] ")
            || trimmed.starts_with("- [x] ")
            || trimmed.starts_with("- [-] ")
        {
            let req = trimmed
                .trim_start_matches("- [ ] ")
                .trim_start_matches("- [x] ")
                .trim_start_matches("- [-] ")
                .to_string();
            if !req.is_empty() {
                requirements.push(req);
            }
        }
    }

    Ok(requirements)
}

fn verify_code_implements_spec(files: &[String], requirements: &[String]) -> Vec<VerifyIssue> {
    let mut issues = Vec::new();

    if files.is_empty() {
        issues.push(VerifyIssue {
            severity: "warning".to_string(),
            description: "No files linked to this topic".to_string(),
            file: None,
            line: None,
        });
        return issues;
    }

    if requirements.is_empty() {
        issues.push(VerifyIssue {
            severity: "info".to_string(),
            description: "No specific requirements found in specs".to_string(),
            file: None,
            line: None,
        });
        return issues;
    }

    for req in requirements {
        let req_lower = req.to_lowercase();
        let mut found = false;

        for file in files {
            if let Ok(content) = fs::read_to_string(file) {
                if content.to_lowercase().contains(&req_lower) {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            issues.push(VerifyIssue {
                severity: "warning".to_string(),
                description: format!("Requirement not found in code: {}", req),
                file: None,
                line: None,
            });
        }
    }

    issues
}

fn update_topic_summary(
    topic: &str,
    area: &str,
    aligned: bool,
    issues: &[VerifyIssue],
    file_count: usize,
    requirement_count: usize,
) -> Result<String> {
    let topic_path = spec_dir().join(area).join(topic).join("topic.md");

    let alignment_status = if aligned { "aligned" } else { "misaligned" };
    let status_line = format!("Status: {}\n", alignment_status);

    let mut content = format!("# {}\n\n", topic);

    if let Ok(desc) = get_topic_description(topic, area) {
        content.push_str(&format!("Short: {}\n\n", desc));
    }

    content.push_str("Parent: \nChildren: \n\n");
    content.push_str(&status_line);
    content.push_str("\n## Verification Summary\n\n");
    content.push_str(&format!("- Files checked: {}\n", file_count));
    content.push_str(&format!("- Requirements: {}\n", requirement_count));
    content.push_str(&format!("- Aligned: {}\n\n", aligned));

    if !issues.is_empty() {
        content.push_str("## Issues\n\n");
        for issue in issues {
            content.push_str(&format!(
                "- [{}] {}",
                issue.severity.to_uppercase(),
                issue.description
            ));
            if let Some(ref file) = issue.file {
                content.push_str(&format!(" (file: {})", file));
            }
            content.push_str("\n");
        }
        content.push_str("\n");
    }

    fs::write(&topic_path, content)?;

    Ok(topic_path.to_string_lossy().to_string())
}

pub fn run_auto_verify(topic: &str, area: Option<&str>) -> Result<VerifyResult> {
    let area = area.unwrap_or("Working");

    let topic_path = spec_dir().join(area).join(topic);
    if !topic_path.exists() {
        return Err(anyhow::anyhow!(
            "Topic '{}' does not exist in area '{}'",
            topic,
            area
        ));
    }

    let description = get_topic_description(topic, area)?;

    println!("Verifying topic: {} ({})", topic, description);
    println!("Area: {}", area);

    let files = get_linked_files(topic, area)?;
    println!("Found {} linked files", files.len());

    let requirements = parse_spec_requirements(topic, area)?;
    println!("Found {} requirements", requirements.len());

    let issues = verify_code_implements_spec(&files, &requirements);

    let aligned = issues.iter().all(|i| i.severity != "error");

    let summary_path = update_topic_summary(
        topic,
        area,
        aligned,
        &issues,
        files.len(),
        requirements.len(),
    )?;

    println!(
        "Alignment: {}",
        if aligned { "ALIGNED" } else { "MISALIGNED" }
    );

    if !aligned {
        let error_summary = issues
            .iter()
            .filter(|i| i.severity == "error")
            .map(|i| i.description.clone())
            .collect::<Vec<_>>()
            .join("; ");

        let lock = create_lock(
            &format!("verify_{}", topic),
            topic,
            None,
            "Verification failed",
            &error_summary,
        )?;

        return Ok(VerifyResult {
            topic: topic.to_string(),
            area: area.to_string(),
            aligned,
            issues,
            summary_path,
            locked: true,
            lock_message: Some(lock.error_message),
        });
    }

    Ok(VerifyResult {
        topic: topic.to_string(),
        area: area.to_string(),
        aligned,
        issues,
        summary_path,
        locked: false,
        lock_message: None,
    })
}

pub fn verify_topic_tree(topic: &str, area: Option<&str>) -> Result<VerifyResult> {
    let area = area.unwrap_or("Working");

    let result = run_auto_verify(topic, Some(area))?;

    let subtopics = {
        let topic_path = spec_dir().join(area).join(topic);
        let mut subtopics = Vec::new();

        if topic_path.exists() {
            for entry in fs::read_dir(&topic_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !name.starts_with('.') {
                            subtopics.push(format!("{}/{}", topic, name));
                        }
                    }
                }
            }
        }

        subtopics
    };

    for subtopic in subtopics {
        let _ = run_auto_verify(&subtopic, Some(area));
    }

    Ok(result)
}
