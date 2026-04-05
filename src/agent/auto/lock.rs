// src/agent/auto/lock.rs
// Lock management for auto-build agent resume functionality

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::fs::agent_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lock {
    pub session_id: String,
    pub topic: String,
    pub parent_topic: Option<String>,
    pub last_task: String,
    pub error_message: String,
    pub timestamp: String,
    pub status: String,
}

impl Lock {
    pub fn new(
        session_id: &str,
        topic: &str,
        parent_topic: Option<&str>,
        last_task: &str,
        error_message: &str,
    ) -> Self {
        Self {
            session_id: session_id.to_string(),
            topic: topic.to_string(),
            parent_topic: parent_topic.map(|s| s.to_string()),
            last_task: last_task.to_string(),
            error_message: error_message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            status: "blocked".to_string(),
        }
    }
}

fn locks_dir() -> PathBuf {
    agent_dir().join("locks")
}

fn ensure_locks_dir() -> Result<PathBuf> {
    let dir = locks_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}

pub fn create_lock(
    session_id: &str,
    topic: &str,
    parent_topic: Option<&str>,
    last_task: &str,
    error_message: &str,
) -> Result<Lock> {
    let dir = ensure_locks_dir()?;
    let lock = Lock::new(session_id, topic, parent_topic, last_task, error_message);
    let path = dir.join(format!("{}.lock", session_id));
    let content = toml::to_string_pretty(&lock)?;
    fs::write(&path, content)?;
    Ok(lock)
}

pub fn get_lock(session_id: &str) -> Result<Option<Lock>> {
    let dir = locks_dir();
    let path = dir.join(format!("{}.lock", session_id));
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path)?;
    let lock: Lock = toml::from_str(&content)?;
    Ok(Some(lock))
}

pub fn get_lock_for_topic(topic: &str) -> Result<Option<Lock>> {
    let dir = locks_dir();
    if !dir.exists() {
        return Ok(None);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "lock").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(lock) = toml::from_str::<Lock>(&content) {
                    if lock.topic == topic && lock.status == "blocked" {
                        return Ok(Some(lock));
                    }
                }
            }
        }
    }
    Ok(None)
}

pub fn clear_lock(session_id: &str) -> Result<()> {
    let dir = locks_dir();
    let path = dir.join(format!("{}.lock", session_id));
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn clear_lock_for_topic(topic: &str) -> Result<()> {
    if let Some(lock) = get_lock_for_topic(topic)? {
        clear_lock(&lock.session_id)?;
    }
    Ok(())
}

pub fn list_locks() -> Result<Vec<Lock>> {
    let dir = locks_dir();
    let mut locks = Vec::new();

    if !dir.exists() {
        return Ok(locks);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "lock").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(lock) = toml::from_str::<Lock>(&content) {
                    locks.push(lock);
                }
            }
        }
    }

    locks.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(locks)
}

pub fn update_lock_status(session_id: &str, status: &str) -> Result<()> {
    if let Some(mut lock) = get_lock(session_id)? {
        lock.status = status.to_string();
        let dir = ensure_locks_dir()?;
        let path = dir.join(format!("{}.lock", session_id));
        let content = toml::to_string_pretty(&lock)?;
        fs::write(&path, content)?;
    }
    Ok(())
}
