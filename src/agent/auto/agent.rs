// src/agent/auto/agent.rs
// Agent process runner - runs individual build agents (MCP + CLI)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use super::lock::create_lock;
use super::minigit::{add_diff, create_commit};
use crate::fs::{agent_dir, spec_dir};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContext {
    pub session_id: String,
    pub topic: String,
    pub parent_topic: Option<String>,
    pub area: String,
    pub description: String,
    pub system_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub session_id: String,
    pub topic: String,
    pub success: bool,
    pub commits: Vec<String>,
    pub output: String,
    pub locked: bool,
    pub lock_message: Option<String>,
}

impl AgentContext {
    pub fn new(
        session_id: &str,
        topic: &str,
        parent_topic: Option<&str>,
        area: &str,
    ) -> Result<Self> {
        let description = Self::get_topic_description(topic, area)?;
        let system_prompt = Self::load_workflow_prompt("build")?;

        Ok(Self {
            session_id: session_id.to_string(),
            topic: topic.to_string(),
            parent_topic: parent_topic.map(|s| s.to_string()),
            area: area.to_string(),
            description,
            system_prompt,
        })
    }

    fn get_topic_description(topic: &str, area: &str) -> Result<String> {
        let topic_path = spec_dir().join(area).join(topic).join("topic.md");

        if topic_path.exists() {
            let content = fs::read_to_string(&topic_path)?;
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Short:") {
                    return Ok(trimmed.split(':').nth(1).unwrap_or("").trim().to_string());
                }
            }
        }

        Ok(topic.to_string())
    }

    fn load_workflow_prompt(workflow: &str) -> Result<String> {
        let workflow_path = agent_dir()
            .join("workflows")
            .join(format!("{}.md", workflow));

        if workflow_path.exists() {
            let content = fs::read_to_string(&workflow_path)?;
            let mut prompt = content.clone();

            prompt = prompt.replace("{topic}", "current_topic");
            prompt = prompt.replace("{area}", "Working");
            prompt = prompt.replace("{parent_topic}", "none");
            prompt = prompt.replace("{code_path}", "current_directory");
            prompt = prompt.replace("{master_spec}", "master.md");

            return Ok(prompt);
        }

        Ok(format!(
            "You are working on topic '{}'. Read specs.md for requirements and implement the code.",
            workflow
        ))
    }

    pub fn to_env(&self) -> Vec<(String, String)> {
        vec![
            ("UNISPEC_SESSION_ID".to_string(), self.session_id.clone()),
            ("UNISPEC_TOPIC".to_string(), self.topic.clone()),
            (
                "UNISPEC_PARENT_TOPIC".to_string(),
                self.parent_topic.clone().unwrap_or_default(),
            ),
            ("UNISPEC_AREA".to_string(), self.area.clone()),
        ]
    }
}

pub fn run_agent(
    topic: &str,
    session_id: Option<&str>,
    parent_topic: Option<&str>,
    area: Option<&str>,
    workflow: Option<&str>,
) -> Result<AgentResult> {
    let area = area.unwrap_or("Working");
    let workflow = workflow.unwrap_or("build");
    let session_id_str: String = match session_id {
        Some(s) => s.to_string(),
        None => {
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            format!("agent_{}", timestamp)
        }
    };

    let topic_path = spec_dir().join(area).join(topic);
    if !topic_path.exists() {
        return Err(anyhow::anyhow!(
            "Topic '{}' does not exist in area '{}'",
            topic,
            area
        ));
    }

    let context = AgentContext::new(&session_id_str, topic, parent_topic, area)?;

    println!("Starting agent session: {}", session_id_str);
    println!("Topic: {}", context.topic);
    println!("Description: {}", context.description);
    println!("Parent: {:?}", context.parent_topic);

    let system_prompt = AgentContext::load_workflow_prompt(workflow)?;

    let output = format!(
        "Agent context prepared for {}\nSystem prompt: {}",
        context.topic,
        system_prompt.lines().take(5).collect::<Vec<_>>().join("\n")
    );

    Ok(AgentResult {
        session_id: session_id_str,
        topic: topic.to_string(),
        success: true,
        commits: vec![],
        output,
        locked: false,
        lock_message: None,
    })
}

pub fn run_agent_with_lock(topic: &str, error_message: &str) -> Result<AgentResult> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let session_id = format!("agent_{}", timestamp);

    let lock = create_lock(
        &session_id,
        topic,
        None,
        "Agent encountered error",
        error_message,
    )?;

    Ok(AgentResult {
        session_id,
        topic: topic.to_string(),
        success: false,
        commits: vec![],
        output: error_message.to_string(),
        locked: true,
        lock_message: Some(lock.error_message),
    })
}

pub fn get_agent_tools() -> Vec<String> {
    vec![
        "topics_list".to_string(),
        "topics_show".to_string(),
        "index_add".to_string(),
        "index_find".to_string(),
        "index_exports".to_string(),
        "code_parse".to_string(),
        "code_analysis".to_string(),
        "connector_run".to_string(),
        "commit".to_string(),
        "get_diff".to_string(),
        "get_commits".to_string(),
    ]
}

pub fn get_workflow_template(workflow: &str) -> Result<String> {
    AgentContext::load_workflow_prompt(workflow)
}

pub fn validate_file_access(topic: &str, file_path: &str, area: &str) -> Result<bool> {
    let project_root = crate::fs::project_root()?;
    let topic_dir = spec_dir().join(area).join(topic);

    let full_path = project_root.join(file_path);

    let is_in_topic_dir = full_path.starts_with(&topic_dir);
    let is_in_src = full_path.starts_with(&project_root.join("src"));
    let is_in_tests = full_path.starts_with(&project_root.join("tests"));

    Ok(is_in_topic_dir || is_in_src || is_in_tests)
}

pub fn write_to_topic_file(
    session_id: &str,
    topic: &str,
    file_path: &str,
    content: &str,
    area: &str,
) -> Result<()> {
    if !validate_file_access(topic, file_path, area)? {
        add_diff(session_id, topic, file_path, None, Some(content))?;
        println!(
            "File {} outside topic dir - creating diff instead",
            file_path
        );
        return Ok(());
    }

    let project_root = crate::fs::project_root()?;
    let full_path = project_root.join(file_path);

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let old_content = if full_path.exists() {
        Some(fs::read_to_string(&full_path)?)
    } else {
        None
    };

    fs::write(&full_path, content)?;

    add_diff(
        session_id,
        topic,
        file_path,
        old_content.as_deref(),
        Some(content),
    )?;

    println!("Wrote file: {}", file_path);

    Ok(())
}

pub fn commit_agent_changes(
    session_id: &str,
    topic: &str,
    parent_topic: Option<&str>,
    description: &str,
    files: Vec<String>,
) -> Result<String> {
    let commit = create_commit(session_id, topic, parent_topic, description, files)?;
    Ok(commit.id)
}
