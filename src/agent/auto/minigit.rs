// src/agent/auto/minigit.rs
// Lightweight diff/commit tracking for auto-build agents

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::fs::agent_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub topic: String,
    pub parent_topic: Option<String>,
    pub description: String,
    pub files: Vec<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommitsStore {
    pub topic: String,
    pub parent_topic: Option<String>,
    pub description: Option<String>,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiffFile {
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub file_path: String,
}

fn diffs_dir() -> PathBuf {
    agent_dir().join("diffs")
}

fn ensure_topic_dir(topic: &str) -> Result<PathBuf> {
    let dir = diffs_dir().join(topic);
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

fn ensure_pending_dir(topic: &str) -> Result<PathBuf> {
    let dir = diffs_dir().join(topic).join("pending");
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

fn commits_path(topic: &str) -> PathBuf {
    diffs_dir().join(topic).join("commits.json")
}

pub fn create_commit(
    session_id: &str,
    topic: &str,
    parent_topic: Option<&str>,
    description: &str,
    files: Vec<String>,
) -> Result<Commit> {
    let dir = ensure_topic_dir(topic)?;

    let commit = Commit {
        id: session_id.to_string(),
        topic: topic.to_string(),
        parent_topic: parent_topic.map(|s| s.to_string()),
        description: description.to_string(),
        files: files.clone(),
        timestamp: Utc::now().to_rfc3339(),
    };

    let path = commits_path(topic);
    let mut store: CommitsStore = if path.exists() {
        let content = fs::read_to_string(&path)?;
        toml::from_str(&content).unwrap_or_default()
    } else {
        CommitsStore {
            topic: topic.to_string(),
            parent_topic: parent_topic.map(|s| s.to_string()),
            description: None,
            commits: vec![],
        }
    };

    store.commits.push(commit.clone());

    let content = toml::to_string_pretty(&store)?;
    fs::write(&path, content)?;

    Ok(commit)
}

pub fn get_commits(topic: &str) -> Result<Vec<Commit>> {
    let path = commits_path(topic);
    if !path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&path)?;
    let store: CommitsStore = toml::from_str(&content)?;
    Ok(store.commits)
}

pub fn get_topic_tree(topic: &str) -> Result<HashMap<String, Vec<Commit>>> {
    let mut tree = HashMap::new();

    fn collect_recursive(t: &str, tree: &mut HashMap<String, Vec<Commit>>) -> Result<()> {
        let commits = get_commits(t)?;
        if !commits.is_empty() {
            tree.insert(t.to_string(), commits);
        }

        let subtopics_dir = crate::fs::spec_dir()
            .join(crate::agent::current_mode().unwrap_or_default().as_str())
            .join(t);

        if subtopics_dir.exists() {
            for entry in fs::read_dir(&subtopics_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name != "specs.md"
                            && name != "tasks.md"
                            && name != "topic.md"
                            && name != "links.md"
                        {
                            collect_recursive(&format!("{}/{}", t, name), tree)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    collect_recursive(topic, &mut tree)?;
    Ok(tree)
}

pub fn add_diff(
    session_id: &str,
    topic: &str,
    file_path: &str,
    old_content: Option<&str>,
    new_content: Option<&str>,
) -> Result<()> {
    let dir = ensure_topic_dir(topic)?;
    let diff_path = dir.join(format!("{}.diff", session_id));

    let mut diffs: HashMap<String, DiffFile> = if diff_path.exists() {
        let content = fs::read_to_string(&diff_path)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        HashMap::new()
    };

    let diff = DiffFile {
        old_content: old_content.map(|s| s.to_string()),
        new_content: new_content.map(|s| s.to_string()),
        file_path: file_path.to_string(),
    };

    diffs.insert(file_path.to_string(), diff);

    let content = serde_json::to_string_pretty(&diffs)?;
    fs::write(&diff_path, content)?;

    Ok(())
}

pub fn get_diff(session_id: &str, topic: &str) -> Result<HashMap<String, DiffFile>> {
    let dir = diffs_dir().join(topic);
    let diff_path = dir.join(format!("{}.diff", session_id));

    if !diff_path.exists() {
        return Ok(HashMap::new());
    }

    let content = fs::read_to_string(&diff_path)?;
    let diffs: HashMap<String, DiffFile> = serde_json::from_str(&content)?;
    Ok(diffs)
}

pub fn add_pending_diff(
    session_id: &str,
    topic: &str,
    target_topic: &str,
    file_path: &str,
    old_content: Option<&str>,
    new_content: Option<&str>,
) -> Result<()> {
    let dir = ensure_pending_dir(target_topic)?;
    let diff_path = dir.join(format!(
        "{}_{}.diff",
        session_id,
        file_path.replace('/', "_")
    ));

    let diff = DiffFile {
        old_content: old_content.map(|s| s.to_string()),
        new_content: new_content.map(|s| s.to_string()),
        file_path: file_path.to_string(),
    };

    let content = serde_json::to_string_pretty(&diff)?;
    fs::write(&diff_path, content)?;

    Ok(())
}

pub fn get_pending_diffs(topic: &str) -> Result<Vec<(String, DiffFile)>> {
    let dir = diffs_dir().join(topic).join("pending");
    let mut diffs = Vec::new();

    if !dir.exists() {
        return Ok(diffs);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "diff").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(diff) = serde_json::from_str::<DiffFile>(&content) {
                    diffs.push((
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        diff,
                    ));
                }
            }
        }
    }

    Ok(diffs)
}

pub fn mark_commit_failed(session_id: &str, topic: &str) -> Result<()> {
    let path = commits_path(topic);
    if !path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&path)?;
    let mut store: CommitsStore = toml::from_str(&content)?;

    let new_content = toml::to_string_pretty(&store)?;
    fs::write(&path, new_content)?;

    Ok(())
}

pub fn get_all_topics() -> Result<Vec<String>> {
    let dir = diffs_dir();
    let mut topics = Vec::new();

    if !dir.exists() {
        return Ok(topics);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                topics.push(name.to_string());
            }
        }
    }

    Ok(topics)
}
