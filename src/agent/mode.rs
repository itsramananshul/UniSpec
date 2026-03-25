// src/agent/mode.rs
use crate::agent::current_mode;
use crate::commands::area;
use crate::fs::{agent_dir, global_modes_dir};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeConfig {
    pub mode: ModeMeta,
    #[serde(default)]
    pub author: Option<AuthorMeta>,
    #[serde(default)]
    pub requirements: Requirements,
    #[serde(default)]
    pub areas: AreasConfig,
    #[serde(default)]
    pub capabilities: Capabilities,
    #[serde(default)]
    pub dependencies: Dependencies,
    #[serde(default)]
    pub scripts: Scripts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeMeta {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorMeta {
    pub name: String,
    #[serde(default)]
    pub contact: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Requirements {
    #[serde(default = "default_min_version")]
    pub min_unispec_version: String,
}

fn default_min_version() -> String {
    "0.9.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AreasConfig {
    #[serde(default)]
    pub default: Vec<String>,
    #[serde(default)]
    pub protected: Vec<String>,
    #[serde(default)]
    pub default_area: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Capabilities {
    #[serde(default)]
    pub spec_writing: bool,
    #[serde(default)]
    pub building: bool,
    #[serde(default)]
    pub verification: bool,
    #[serde(default)]
    pub connectors: bool,
    #[serde(default)]
    pub custom_workflows: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dependencies {
    #[serde(default)]
    pub extends: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Scripts {
    #[serde(default)]
    pub pre_activate: Option<String>,
    #[serde(default)]
    pub post_activate: Option<String>,
}

pub fn list_modes() -> Result<Vec<ModeInfo>> {
    let mut modes = vec![];
    let current = current_mode()?;

    // List local modes
    let local_modes_dir = agent_dir().join("modes");
    if local_modes_dir.exists() {
        for entry in fs::read_dir(&local_modes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(mode_info) =
                    load_mode_info_from_path(&path, &ModeSource::Local, &current)?
                {
                    modes.push(mode_info);
                }
            }
        }
    }

    // List global modes
    let global_modes_dir = global_modes_dir();
    if global_modes_dir.exists() {
        for entry in fs::read_dir(&global_modes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(mode_info) =
                    load_mode_info_from_path(&path, &ModeSource::Global, &current)?
                {
                    // Skip if local mode with same name exists
                    if !modes.iter().any(|m: &ModeInfo| m.name == mode_info.name) {
                        modes.push(mode_info);
                    }
                }
            }
        }
    }

    modes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(modes)
}

fn load_mode_info_from_path(
    path: &PathBuf,
    source: &ModeSource,
    current: &str,
) -> Result<Option<ModeInfo>> {
    let mode_path = path.join("mode.toml");
    if mode_path.exists() {
        if let Ok(content) = fs::read_to_string(&mode_path) {
            if let Ok(config) = toml::from_str::<ModeConfig>(&content) {
                let mode_name = config.mode.name.clone();
                return Ok(Some(ModeInfo {
                    name: mode_name,
                    display_name: config.mode.display_name,
                    description: config.mode.description,
                    version: config.mode.version,
                    path: path.clone(),
                    is_active: config.mode.name == current,
                    source: source.clone(),
                }));
            }
        }
    }
    Ok(None)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub version: String,
    pub path: PathBuf,
    pub is_active: bool,
    pub source: ModeSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModeSource {
    Local,
    Global,
}

pub fn get_mode_info(name: &str) -> Result<ModeConfig> {
    // Check local modes first
    let local_path = agent_dir().join("modes").join(name).join("mode.toml");
    if local_path.exists() {
        let content = fs::read_to_string(&local_path)?;
        return Ok(toml::from_str(&content)?);
    }

    // Check global modes
    let global_path = global_modes_dir().join(name).join("mode.toml");
    if global_path.exists() {
        let content = fs::read_to_string(&global_path)?;
        return Ok(toml::from_str(&content)?);
    }

    Err(anyhow::anyhow!("Mode '{}' not found", name))
}

pub fn get_mode_path(name: &str) -> Result<PathBuf> {
    // Check local modes first
    let local_path = agent_dir().join("modes").join(name);
    if local_path.join("mode.toml").exists() {
        return Ok(local_path);
    }

    // Check global modes
    let global_path = global_modes_dir().join(name);
    if global_path.join("mode.toml").exists() {
        return Ok(global_path);
    }

    Err(anyhow::anyhow!("Mode '{}' not found", name))
}

pub fn run_activate(mode_name: &str) -> Result<String> {
    let current = current_mode()?;
    if current == mode_name {
        return Ok(format!("Already using mode '{}'", mode_name));
    }

    // Load mode config
    let mode_config = get_mode_info(mode_name)?;

    // Create required areas from mode
    for area_name in &mode_config.areas.default {
        if !crate::fs::area_exists(area_name) {
            area::run_add(area_name)?;
            println!("Created area: {}", area_name);
        }
    }

    // Run pre-activate script if exists
    if let Some(ref script) = mode_config.scripts.pre_activate {
        if !script.is_empty() {
            run_script(script)?;
        }
    }

    // Activate new mode - copy files from mode directory
    let agent_path = agent_dir();
    let mode_dir = agent_path.join("modes").join(mode_name);

    // Copy workflows
    let workflows_src = mode_dir.join("workflows");
    if workflows_src.exists() {
        let workflows_dst = agent_path.join("workflows");
        let _ = fs::remove_dir_all(&workflows_dst);
        fs::create_dir_all(&workflows_dst)?;
        copy_dir_recursive(&workflows_src, &workflows_dst)?;
    }

    // Copy skill.md
    copy_file_if_exists(&mode_dir.join("skill.md"), &agent_path.join("skill.md"))?;

    // Update config.toml with new mode
    let mut agent_config = crate::agent::load_agent_config()?;
    agent_config.current_mode = mode_name.to_string();

    // Set default area from mode config (if config doesn't have one set)
    if agent_config.default_area.is_none() {
        if let Some(ref default_area) = mode_config.areas.default_area {
            agent_config.default_area = Some(default_area.clone());
        }
    }

    // NOTE: protected_areas are NOT merged from mode - they are user-defined in config.toml
    // Modes can define protected areas for their own logic, but config.toml keeps its own

    crate::agent::save_agent_config(&agent_config)?;

    // Run post-activate script if exists
    if let Some(ref script) = mode_config.scripts.post_activate {
        if !script.is_empty() {
            run_script(script)?;
        }
    }

    Ok(format!(
        "Mode '{}' activated successfully",
        mode_config.mode.display_name
    ))
}

fn copy_file_if_exists(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    if src.exists() && src.is_file() {
        fs::copy(src, dst)?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let new_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &new_path)?;
        } else {
            fs::copy(&path, &new_path)?;
        }
    }
    Ok(())
}

fn run_script(script: &str) -> Result<()> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", script]).output()?
    } else {
        Command::new("sh").args(["-c", script]).output()?
    };

    if !output.status.success() {
        eprintln!("Script warning: exited with {:?}", output.status.code());
    }
    Ok(())
}

pub fn get_workflows(mode_name: &str) -> Result<Vec<WorkflowInfo>> {
    let workflows_dir = agent_dir().join("modes").join(mode_name).join("workflows");

    if !workflows_dir.exists() {
        return Ok(vec![]);
    }

    let mut workflows = vec![];
    for entry in fs::read_dir(&workflows_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "md").unwrap_or(false) {
            let filename = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            // Extract description from frontmatter
            let description = extract_description(&path).unwrap_or_default();

            workflows.push(WorkflowInfo {
                name: filename,
                path,
                description,
            });
        }
    }

    Ok(workflows)
}

fn extract_description(path: &PathBuf) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("description:") {
            return Some(trimmed["description:".len()..].trim().to_string());
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct WorkflowInfo {
    pub name: String,
    pub path: PathBuf,
    pub description: String,
}

pub fn get_protected_areas(mode_name: &str) -> Result<Vec<String>> {
    let config = get_mode_info(mode_name)?;
    Ok(config.areas.protected)
}

pub fn add_mode(source_path: &str, force_global: bool) -> Result<String> {
    let source = PathBuf::from(source_path);

    if !source.exists() {
        return Err(anyhow::anyhow!("Path '{}' does not exist", source_path));
    }

    let mode_toml = source.join("mode.toml");
    if !mode_toml.exists() {
        return Err(anyhow::anyhow!(
            "Path '{}' is not a valid mode (missing mode.toml)",
            source_path
        ));
    }

    let content = fs::read_to_string(&mode_toml)?;
    let config: ModeConfig = toml::from_str(&content)?;
    let mode_name = config.mode.name.clone();

    let local_dir = agent_dir().join("modes").join(&mode_name);
    let global_dir = global_modes_dir().join(&mode_name);

    let target_dir = if force_global {
        if local_dir.join("mode.toml").exists() {
            return Err(anyhow::anyhow!(
                "Mode '{}' already exists locally. Use 'unispec mode remove {}' first.",
                mode_name,
                mode_name
            ));
        }
        global_dir
    } else if local_dir.join("mode.toml").exists() {
        return Err(anyhow::anyhow!(
            "Mode '{}' already exists locally. Use 'unispec mode remove {}' first.",
            mode_name,
            mode_name
        ));
    } else if global_dir.join("mode.toml").exists() {
        return Err(anyhow::anyhow!(
            "Mode '{}' already exists globally. Use 'unispec mode remove {} --global' first.",
            mode_name,
            mode_name
        ));
    } else if agent_dir().join("modes").exists() {
        local_dir
    } else {
        global_dir
    };

    fs::create_dir_all(&target_dir)?;
    copy_dir_recursive(&source, &target_dir)?;

    let source_type = if target_dir.starts_with(&global_modes_dir()) {
        "global"
    } else {
        "local"
    };

    Ok(format!(
        "Mode '{}' added to {} modes at: {}",
        mode_name,
        source_type,
        target_dir.display()
    ))
}

pub fn remove_mode(name: &str, global: bool) -> Result<String> {
    let mode_path = if global {
        global_modes_dir().join(name)
    } else {
        agent_dir().join("modes").join(name)
    };

    let mode_toml = mode_path.join("mode.toml");
    if !mode_toml.exists() {
        if global {
            return Err(anyhow::anyhow!("Mode '{}' not found in global modes", name));
        } else {
            return Err(anyhow::anyhow!("Mode '{}' not found in local modes", name));
        }
    }

    // Check if this is the active mode
    let current = current_mode()?;
    if current == name {
        return Err(anyhow::anyhow!(
            "Cannot remove the active mode '{}'. Switch to another mode first.",
            name
        ));
    }

    fs::remove_dir_all(&mode_path)?;

    Ok(format!(
        "Mode '{}' removed from {} modes",
        name,
        if global { "global" } else { "local" }
    ))
}
