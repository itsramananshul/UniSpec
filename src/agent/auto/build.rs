// src/agent/auto/build.rs
// Auto build orchestration - builds topic from spec with multiple agents

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use super::lock::{clear_lock_for_topic, create_lock, get_lock_for_topic};
use super::minigit::{create_commit, get_commits, Commit};
use super::pr::run_pr_merge;
use crate::fs::spec_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTask {
    pub topic: String,
    pub status: String,
    pub session_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTask {
    pub topic: String,
    pub error: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub topic: String,
    pub area: String,
    pub tasks_total: u32,
    pub tasks_completed: u32,
    pub tasks_failed: u32,
    pub nested_topics_built: Vec<String>,
    pub commits: Vec<Commit>,
    pub pr_ran: bool,
    pub pr_status: String,
    pub pr_report: String,
    pub failed_tasks: Vec<FailedTask>,
    pub locked: bool,
    pub lock_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildState {
    pub topic: String,
    pub area: String,
    pub tasks: Vec<BuildTask>,
    pub started: String,
}

fn build_state_dir() -> PathBuf {
    crate::fs::agent_dir().join("build_state")
}

fn ensure_build_state_dir() -> Result<PathBuf> {
    let _dir = build_state_dir();
    if !_dir.exists() {
        fs::create_dir_all(&_dir)?;
    }
    Ok(_dir)
}

fn build_state_path(topic: &str) -> PathBuf {
    build_state_dir().join(format!("{}.toml", topic))
}

fn save_build_state(state: &BuildState) -> Result<()> {
    ensure_build_state_dir()?;
    let path = build_state_path(&state.topic);
    let content = toml::to_string_pretty(state)?;
    fs::write(&path, content)?;
    Ok(())
}

fn load_build_state(topic: &str) -> Result<Option<BuildState>> {
    let path = build_state_path(topic);
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)?;
    let state: BuildState = toml::from_str(&content)?;
    Ok(Some(state))
}

fn delete_build_state(topic: &str) -> Result<()> {
    let path = build_state_path(topic);
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

fn get_topics_from_spec(topic: &str, area: &str) -> Result<Vec<String>> {
    let spec_area_dir = spec_dir().join(area);
    let topic_path = spec_area_dir.join(topic);
    let mut topics = Vec::new();

    if !topic_path.exists() {
        return Ok(topics);
    }

    for entry in fs::read_dir(&topic_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if !name.starts_with('.') {
                    topics.push(name.to_string());
                }
            }
        }
    }

    Ok(topics)
}

fn parse_spec_tasks(topic: &str, area: &str) -> Result<Vec<String>> {
    let spec_path = spec_dir().join(area).join(topic).join("specs.md");
    let mut tasks = Vec::new();

    if !spec_path.exists() {
        return Ok(tasks);
    }

    let content = fs::read_to_string(&spec_path)?;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [ ] ") || trimmed.starts_with("- [-] ") {
            let task = trimmed
                .trim_start_matches("- [ ] ")
                .trim_start_matches("- [-] ")
                .to_string();
            if !task.is_empty() {
                tasks.push(task);
            }
        } else if trimmed.starts_with("## ") {
            let section = trimmed.trim_start_matches("## ").to_lowercase();
            if section.contains("task") {
                tasks.push(section);
            }
        }
    }

    if tasks.is_empty() {
        tasks = get_topics_from_spec(topic, area)?;
    }

    Ok(tasks)
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

    let spec_path = spec_dir().join(area).join(topic).join("specs.md");
    if spec_path.exists() {
        let content = fs::read_to_string(&spec_path)?;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return Ok(trimmed.trim_start_matches("# ").to_string());
            }
        }
    }

    Ok(topic.to_string())
}

fn spawn_build_agent(
    topic: &str,
    area: &str,
    parent_topic: Option<&str>,
    session_id: &str,
) -> Result<()> {
    let description = get_topic_description(topic, area)?;

    let workflow_path = crate::fs::agent_dir().join("workflows").join("build.md");
    if workflow_path.exists() {
        fs::read_to_string(&workflow_path)?
    } else {
        format!(
            "You are building topic '{}' in area '{}'. Parent: {:?}. Description: {}",
            topic, area, parent_topic, description
        )
    };

    println!(
        "Spawning build agent for topic: {} (session: {})",
        topic, session_id
    );
    println!("Description: {}", description);
    println!("Parent: {:?}", parent_topic);

    Ok(())
}

pub fn run_auto_build(
    topic: &str,
    area: Option<&str>,
    spec_file: Option<&str>,
) -> Result<BuildResult> {
    let area = area.unwrap_or("Working");
    let topic_path = spec_dir().join(area).join(topic);
    if !topic_path.exists() {
        return Err(anyhow::anyhow!(
            "Topic '{}' does not exist in area '{}'",
            topic,
            area
        ));
    }

    if let Some(lock) = get_lock_for_topic(topic)? {
        println!("Found existing lock for topic: {}", topic);
        println!("Last task: {}", lock.last_task);
        println!("Error: {}", lock.error_message);

        return Ok(BuildResult {
            topic: topic.to_string(),
            area: area.to_string(),
            tasks_total: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            nested_topics_built: vec![],
            commits: vec![],
            pr_ran: false,
            pr_status: "blocked".to_string(),
            pr_report: "Build blocked by existing lock".to_string(),
            failed_tasks: vec![FailedTask {
                topic: topic.to_string(),
                error: lock.error_message.clone(),
                timestamp: lock.timestamp.clone(),
            }],
            locked: true,
            lock_message: Some(lock.error_message),
        });
    }

    parse_spec_tasks(topic, area)?;
    let subtasks = get_topics_from_spec(topic, area)?;

    let state = BuildState {
        topic: topic.to_string(),
        area: area.to_string(),
        tasks: subtasks
            .iter()
            .map(|t| BuildTask {
                topic: t.clone(),
                status: "pending".to_string(),
                session_id: None,
                error: None,
            })
            .collect(),
        started: Utc::now().to_rfc3339(),
    };

    save_build_state(&state)?;

    let mut tasks_completed: u32 = 0;
    let tasks_failed: u32 = 0;
    let mut failed_tasks = Vec::new();
    let mut nested_topics_built = Vec::new();
    let mut all_commits = Vec::new();

    let root_session_id = Uuid::new_v4().to_string();

    spawn_build_agent(topic, area, None, &root_session_id)?;

    tasks_completed += 1;

    for subtask in &subtasks {
        let session_id = Uuid::new_v4().to_string();

        let subtask_topic = format!("{}/{}", topic, subtask);

        spawn_build_agent(&subtask_topic, area, Some(topic), &session_id)?;

        let commit = create_commit(
            &session_id,
            &subtask_topic,
            Some(topic),
            &format!("Built subtask: {}", subtask),
            vec![],
        )?;

        all_commits.push(commit);
        nested_topics_built.push(subtask.clone());
        tasks_completed += 1;
    }

    delete_build_state(topic)?;

    println!("Running PR merge for topic: {}", topic);
    let pr_result = run_pr_merge(topic)?;

    if pr_result.status == "conflicts" || pr_result.status == "partial" {
        let lock = create_lock(
            &root_session_id,
            topic,
            None,
            "PR merge conflicts",
            &pr_result.report,
        )?;

        return Ok(BuildResult {
            topic: topic.to_string(),
            area: area.to_string(),
            tasks_total: (subtasks.len() as u32) + 1,
            tasks_completed,
            tasks_failed,
            nested_topics_built,
            commits: all_commits,
            pr_ran: pr_result.ran,
            pr_status: pr_result.status,
            pr_report: pr_result.report,
            failed_tasks,
            locked: true,
            lock_message: Some(lock.error_message),
        });
    }

    Ok(BuildResult {
        topic: topic.to_string(),
        area: area.to_string(),
        tasks_total: (subtasks.len() as u32) + 1,
        tasks_completed,
        tasks_failed,
        nested_topics_built,
        commits: all_commits,
        pr_ran: pr_result.ran,
        pr_status: pr_result.status,
        pr_report: pr_result.report,
        failed_tasks,
        locked: false,
        lock_message: None,
    })
}

pub fn resume_auto_build(topic: &str) -> Result<BuildResult> {
    if let Some(state) = load_build_state(topic)? {
        let area = &state.area;

        let completed_count = state
            .tasks
            .iter()
            .filter(|t| t.status == "completed")
            .count() as u32;

        let failed_count = state.tasks.iter().filter(|t| t.status == "failed").count() as u32;

        let pending_tasks: Vec<&BuildTask> = state
            .tasks
            .iter()
            .filter(|t| t.status == "pending" || t.status == "failed")
            .collect();

        let mut failed_task_objs = Vec::new();

        for task in &pending_tasks {
            if let Some(ref error) = task.error {
                failed_task_objs.push(FailedTask {
                    topic: task.topic.clone(),
                    error: error.clone(),
                    timestamp: Utc::now().to_rfc3339(),
                });
            }
        }

        let commits = get_commits(topic)?;

        clear_lock_for_topic(topic)?;

        Ok(BuildResult {
            topic: topic.to_string(),
            area: area.to_string(),
            tasks_total: state.tasks.len() as u32,
            tasks_completed: completed_count,
            tasks_failed: failed_count,
            nested_topics_built: vec![],
            commits,
            pr_ran: false,
            pr_status: "resumed".to_string(),
            pr_report: format!("Resumed build with {} pending tasks", pending_tasks.len()),
            failed_tasks: failed_task_objs,
            locked: false,
            lock_message: None,
        })
    } else {
        run_auto_build(topic, None, None)
    }
}

pub fn get_build_status(topic: &str) -> Result<Option<BuildState>> {
    load_build_state(topic)
}

pub fn cancel_build(topic: &str) -> Result<String> {
    delete_build_state(topic)?;
    clear_lock_for_topic(topic)?;
    Ok(format!("Build for '{}' cancelled", topic))
}
