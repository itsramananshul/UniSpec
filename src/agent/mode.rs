// src/agent/mode.rs
use crate::agent::current_mode;
use crate::commands::area;
use crate::fs::{agent_dir, global_modes_dir};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AreaTypesConfig {
    #[serde(default)]
    pub roadmap: Option<AreaTypeConfig>,
    #[serde(default)]
    pub working: Option<AreaTypeConfig>,
    #[serde(default)]
    pub build: Option<AreaTypeConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaTypeConfig {
    pub display_type: DisplayType,
    #[serde(default = "default_area_spec_file")]
    pub spec_file: String,
    #[serde(default)]
    pub task_file: Option<String>,
    pub parser: ParserType,
    #[serde(default = "default_list_fields")]
    pub list_fields: Vec<String>,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default)]
    pub impact_labels: HashMap<String, String>,
    #[serde(default)]
    pub change_type_labels: HashMap<String, String>,
}

fn default_area_spec_file() -> String {
    "specs.md".to_string()
}

fn default_list_fields() -> Vec<String> {
    vec!["title".to_string()]
}

fn default_sort_by() -> String {
    "title".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisplayType {
    #[serde(rename = "roadmap")]
    Roadmap,
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "build")]
    Build,
    #[serde(rename = "standard")]
    Standard,
}

impl Default for DisplayType {
    fn default() -> Self {
        DisplayType::Standard
    }
}

impl DisplayType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "roadmap" => DisplayType::Roadmap,
            "working" => DisplayType::Working,
            "build" => DisplayType::Build,
            _ => DisplayType::Standard,
        }
    }
}

pub fn get_impact_labels(area_type: &str) -> HashMap<String, String> {
    let mut defaults = HashMap::new();
    defaults.insert("critical".to_string(), "CRITICAL".to_string());
    defaults.insert("high".to_string(), "HIGH".to_string());
    defaults.insert("medium".to_string(), "MEDIUM".to_string());
    defaults.insert("low".to_string(), "LOW".to_string());

    if let Ok(mode_name) = current_mode() {
        if let Ok(config) = get_mode_info(&mode_name) {
            let area_lower = area_type.to_lowercase();

            let config_labels = if area_lower.contains("roadmap") {
                config.area_types.roadmap.as_ref()
            } else if area_lower.contains("build") {
                config.area_types.build.as_ref()
            } else if area_lower.contains("working") {
                config.area_types.working.as_ref()
            } else {
                None
            };

            if let Some(area_config) = config_labels {
                let mut labels = defaults;
                for (key, value) in &area_config.impact_labels {
                    labels.insert(key.to_lowercase(), value.clone());
                }
                return labels;
            }
        }
    }
    defaults
}

pub fn get_change_type_labels(area_type: &str) -> HashMap<String, String> {
    let mut defaults = HashMap::new();
    defaults.insert("feature".to_string(), "feature".to_string());
    defaults.insert("bugfix".to_string(), "bugfix".to_string());
    defaults.insert("bug".to_string(), "bugfix".to_string());
    defaults.insert("refactor".to_string(), "refactor".to_string());
    defaults.insert("refactoring".to_string(), "refactor".to_string());
    defaults.insert("documentation".to_string(), "docs".to_string());
    defaults.insert("docs".to_string(), "docs".to_string());
    defaults.insert("security".to_string(), "security".to_string());

    if let Ok(mode_name) = current_mode() {
        if let Ok(config) = get_mode_info(&mode_name) {
            let area_lower = area_type.to_lowercase();

            let config_labels = if area_lower.contains("roadmap") {
                config.area_types.roadmap.as_ref()
            } else if area_lower.contains("build") {
                config.area_types.build.as_ref()
            } else if area_lower.contains("working") {
                config.area_types.working.as_ref()
            } else {
                None
            };

            if let Some(area_config) = config_labels {
                let mut labels = defaults;
                for (key, value) in &area_config.change_type_labels {
                    labels.insert(key.to_lowercase(), value.clone());
                }
                return labels;
            }
        }
    }
    defaults
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParserType {
    #[serde(rename = "frontmatter")]
    Frontmatter,
    #[serde(rename = "tasks")]
    Tasks,
    #[serde(rename = "dates")]
    Dates,
    #[serde(rename = "standard")]
    Standard,
}

impl Default for ParserType {
    fn default() -> Self {
        ParserType::Standard
    }
}

impl ParserType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "frontmatter" => ParserType::Frontmatter,
            "tasks" => ParserType::Tasks,
            "dates" => ParserType::Dates,
            _ => ParserType::Standard,
        }
    }
}

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
    #[serde(default)]
    pub templates: TemplatesConfig,
    #[serde(default)]
    pub area_types: AreaTypesConfig,
    #[serde(default)]
    pub topic_order: TopicOrderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TopicOrderConfig {
    #[serde(default)]
    pub areas: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    pub modes: Vec<String>,
    #[serde(default)]
    pub connectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplatesConfig {
    #[serde(default = "default_spec_file")]
    pub spec_file: String,
    #[serde(default = "default_task_file")]
    pub task_file: String,
    #[serde(default = "default_area_file")]
    pub area_file: String,
    #[serde(default)]
    pub use_area_templates: bool,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, String>,
}

impl TemplatesConfig {
    pub fn get_override(&self, key: &str) -> Option<String> {
        self.extra.get(key).cloned()
    }
}

fn default_spec_file() -> String {
    "specs.md".to_string()
}

fn default_task_file() -> String {
    "tasks.md".to_string()
}

fn default_area_file() -> String {
    "area.md".to_string()
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
    Err(anyhow::anyhow!("Mode '{}' not found", name))
}

/// Get the spec file name from current mode's config
pub fn get_spec_filename() -> String {
    get_spec_filename_for_area("Working")
}

/// Get the spec file name for a specific area
pub fn get_spec_filename_for_area(area: &str) -> String {
    if let Ok(mode_name) = current_mode() {
        if let Ok(config) = get_mode_info(&mode_name) {
            let extra = &config.templates.extra;
            let area_lower = area.to_lowercase();
            let spec_key = format!("{}-spec-file", area_lower);
            if let Some(f) = extra.get(&spec_key) {
                return f.clone();
            }
            return config.templates.spec_file;
        }
    }
    "specs.md".to_string()
}

/// Get the task file name from current mode's config
pub fn get_task_filename() -> String {
    get_task_filename_for_area("Working")
}

/// Get the task file name for a specific area
pub fn get_task_filename_for_area(area: &str) -> String {
    if let Ok(mode_name) = current_mode() {
        if let Ok(config) = get_mode_info(&mode_name) {
            let extra = &config.templates.extra;
            let area_lower = area.to_lowercase();
            let task_key = format!("{}-task-file", area_lower);
            if let Some(f) = extra.get(&task_key) {
                return f.clone();
            }
            return config.templates.task_file;
        }
    }
    "tasks.md".to_string()
}

/// Get the area file name from current mode's config
pub fn get_area_filename() -> String {
    get_area_filename_for_area("Working")
}

/// Get the area file name for a specific area
pub fn get_area_filename_for_area(area: &str) -> String {
    if let Ok(mode_name) = current_mode() {
        if let Ok(config) = get_mode_info(&mode_name) {
            let extra = &config.templates.extra;
            let area_lower = area.to_lowercase();
            let area_key = format!("{}-area-file", area_lower);
            if let Some(f) = extra.get(&area_key) {
                return f.clone();
            }
            return config.templates.area_file;
        }
    }
    "area.md".to_string()
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

pub fn get_topic_order(area: &str) -> Vec<String> {
    if let Ok(mode_name) = current_mode() {
        if let Ok(config) = get_mode_info(&mode_name) {
            if let Some(order) = config.topic_order.areas.get(area) {
                return order.clone();
            }
        }
    }
    vec![]
}

pub fn set_topic_order(area: &str, order: Vec<String>) -> Result<()> {
    let mode_name = current_mode()?;

    let local_path = agent_dir().join("modes").join(&mode_name).join("mode.toml");
    let global_path = global_modes_dir().join(&mode_name).join("mode.toml");

    let config_path = if local_path.exists() {
        local_path
    } else if global_path.exists() {
        global_path
    } else {
        return Err(anyhow::anyhow!("Mode '{}' not found", mode_name));
    };

    let content = fs::read_to_string(&config_path)?;
    let mut config: ModeConfig = toml::from_str(&content)?;

    config.topic_order.areas.insert(area.to_string(), order);

    let new_content = toml::to_string_pretty(&config)?;
    fs::write(&config_path, new_content)?;

    Ok(())
}

pub fn add_to_topic_order(area: &str, topics: Vec<String>, position: Option<usize>) -> Result<()> {
    let current_order = get_topic_order(area);

    let mut new_order = current_order;
    for topic in topics {
        if !new_order.contains(&topic) {
            if let Some(pos) = position {
                if pos <= new_order.len() {
                    new_order.insert(pos, topic);
                } else {
                    new_order.push(topic);
                }
            } else {
                new_order.push(topic);
            }
        }
    }

    set_topic_order(area, new_order)
}

pub fn remove_from_topic_order(area: &str, topics: Vec<String>) -> Result<()> {
    let current_order = get_topic_order(area);
    let new_order: Vec<String> = current_order
        .into_iter()
        .filter(|t| !topics.contains(t))
        .collect();
    set_topic_order(area, new_order)
}

pub fn reset_topic_order(area: &str) -> Result<()> {
    set_topic_order(area, vec![])
}

pub fn get_area_order() -> Vec<String> {
    if let Ok(mode_name) = current_mode() {
        if let Ok(config) = get_mode_info(&mode_name) {
            return config.areas.default.clone();
        }
    }
    vec![]
}

pub fn set_area_order(order: Vec<String>) -> Result<()> {
    let mode_name = current_mode()?;

    let local_path = agent_dir().join("modes").join(&mode_name).join("mode.toml");
    let global_path = global_modes_dir().join(&mode_name).join("mode.toml");

    let config_path = if local_path.exists() {
        local_path
    } else if global_path.exists() {
        global_path
    } else {
        return Err(anyhow::anyhow!("Mode '{}' not found", mode_name));
    };

    let content = fs::read_to_string(&config_path)?;
    let mut config: ModeConfig = toml::from_str(&content)?;

    config.areas.default = order;

    let new_content = toml::to_string_pretty(&config)?;
    fs::write(&config_path, new_content)?;

    Ok(())
}

pub fn add_to_area_order(areas: Vec<String>, position: Option<usize>) -> Result<()> {
    let current_order = get_area_order();

    let mut new_order = current_order;
    for area in areas {
        if !new_order.contains(&area) {
            if let Some(pos) = position {
                if pos <= new_order.len() {
                    new_order.insert(pos, area);
                } else {
                    new_order.push(area);
                }
            } else {
                new_order.push(area);
            }
        }
    }

    set_area_order(new_order)
}

pub fn remove_from_area_order(areas: Vec<String>) -> Result<()> {
    let current_order = get_area_order();
    let new_order: Vec<String> = current_order
        .into_iter()
        .filter(|a| !areas.contains(a))
        .collect();
    set_area_order(new_order)
}

pub fn reset_area_order() -> Result<()> {
    set_area_order(vec![])
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
