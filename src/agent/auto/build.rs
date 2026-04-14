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
use crate::commands::topic as topic_cmd;
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

fn get_project_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn ensure_src_dir() -> Result<PathBuf> {
    let src_dir = get_project_root().join("src");
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir)?;
        println!("Created /src directory at project root for code files");
    } else {
        println!("Using existing /src directory at project root");
    }
    Ok(src_dir)
}

fn find_queue_topic(staging_area: &str) -> Option<String> {
    // queue.md is in the area root (e.g., Staging/queue.md), not in topic folders
    let queue_file = crate::agent::mode::get_readiness_queue_file();
    let queue_path = spec_dir().join(staging_area).join(&queue_file);

    if queue_path.exists() {
        // Read the first topic from the queue
        if let Ok(content) = fs::read_to_string(&queue_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("- ") {
                    let topic_name = trimmed.trim_start_matches("- ").trim().to_string();
                    if !topic_name.is_empty() {
                        return Some(topic_name);
                    }
                }
            }
        }
    }
    None
}

fn read_queue_items(area: &str) -> Vec<String> {
    // Read from area-level queue.md (e.g., Working/queue.md)
    let queue_file = crate::agent::mode::get_readiness_queue_file();
    let queue_path = spec_dir().join(area).join(&queue_file);

    if let Ok(content) = fs::read_to_string(&queue_path) {
        return content
            .lines()
            .filter(|l| l.trim().starts_with("- "))
            .map(|l| l.trim().trim_start_matches("- ").to_string())
            .collect();
    }
    vec![]
}

fn push_topic_to_working(topic: &str, source_area: &str) -> Result<String> {
    let result = topic_cmd::run_push(topic, "Working", Some(source_area))?;
    Ok(result)
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
    topic: Option<&str>,
    area: Option<&str>,
    spec_file: Option<&str>,
) -> Result<BuildResult> {
    let staging_area = "Staging";
    let working_area = "Working";

    // Step 1: Find topic with queue.md in Staging (the ROOT topic)
    let queue_topic = find_queue_topic(staging_area);

    let root_topic = match queue_topic {
        Some(qt) => qt,
        None => {
            // If no topic provided and no queue topic found, error
            if topic.is_none() {
                return Err(anyhow::anyhow!(
                    "No topic with queue.md found in Staging. Cannot auto-build.\nCreate a topic with queue.md inside it first."
                ));
            }
            // Fall back to provided topic
            topic.unwrap().to_string()
        }
    };

    // Step 1b: Ensure /src directory exists BEFORE pushing to Working
    println!("Ensuring /src directory exists at project root...");
    ensure_src_dir()?;

    // Step 2: Push the entire root topic from Staging to Working (includes all children)
    println!("Found root topic '{}' with queue.md in Staging", root_topic);
    println!(
        "Pushing '{}' (including all sub-topics) from Staging to Working",
        root_topic
    );
    push_topic_to_working(&root_topic, staging_area)?;

    // Work on the root topic in Working area
    let work_area = working_area;

    let topic_path = spec_dir().join(work_area).join(&root_topic);
    if !topic_path.exists() {
        return Err(anyhow::anyhow!(
            "Topic '{}' does not exist in area '{}'",
            root_topic,
            work_area
        ));
    }

    if let Some(lock) = get_lock_for_topic(&root_topic)? {
        println!("Found existing lock for topic: {}", root_topic);
        println!("Last task: {}", lock.last_task);
        println!("Error: {}", lock.error_message);

        return Ok(BuildResult {
            topic: root_topic.to_string(),
            area: work_area.to_string(),
            tasks_total: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            nested_topics_built: vec![],
            commits: vec![],
            pr_ran: false,
            pr_status: "blocked".to_string(),
            pr_report: "Build blocked by existing lock".to_string(),
            failed_tasks: vec![FailedTask {
                topic: root_topic.to_string(),
                error: lock.error_message.clone(),
                timestamp: lock.timestamp.clone(),
            }],
            locked: true,
            lock_message: Some(lock.error_message),
        });
    }

    // Step 3: Get tasks from queue.md (central to-do list) in Working
    let subtasks = read_queue_items(working_area);

    println!("Tasks from queue: {:?}", subtasks);

    let state = BuildState {
        topic: root_topic.to_string(),
        area: work_area.to_string(),
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

    // Ensure /src directory exists for source files
    ensure_src_dir()?;

    let mut tasks_completed: u32 = 0;
    let tasks_failed: u32 = 0;
    let mut failed_tasks = Vec::new();
    let mut nested_topics_built = Vec::new();
    let mut all_commits = Vec::new();

    let root_session_id = Uuid::new_v4().to_string();

    spawn_build_agent(&root_topic, work_area, None, &root_session_id)?;

    tasks_completed += 1;

    for subtask in &subtasks {
        let session_id = Uuid::new_v4().to_string();

        let subtask_topic = format!("{}/{}", root_topic, subtask);

        spawn_build_agent(&subtask_topic, work_area, Some(&root_topic), &session_id)?;

        let commit = create_commit(
            &session_id,
            &subtask_topic,
            Some(&root_topic),
            &format!("Built subtask: {}", subtask),
            vec![],
        )?;

        all_commits.push(commit);
        nested_topics_built.push(subtask.clone());
        tasks_completed += 1;
    }

    delete_build_state(&root_topic)?;

    println!("Running PR merge for topic: {}", root_topic);
    let pr_result = run_pr_merge(&root_topic)?;

    if pr_result.status == "conflicts" || pr_result.status == "partial" {
        let lock = create_lock(
            &root_session_id,
            &root_topic,
            None,
            "PR merge conflicts",
            &pr_result.report,
        )?;

        return Ok(BuildResult {
            topic: root_topic.to_string(),
            area: work_area.to_string(),
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
        topic: root_topic.to_string(),
        area: work_area.to_string(),
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
        // Resume mode - use provided topic as the root
        run_auto_build(Some(topic), None, None)
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
