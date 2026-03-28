// src/fs/mod.rs
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn project_root() -> Result<PathBuf> {
    Ok(std::env::current_dir()?)
}

pub fn spec_dir() -> PathBuf {
    project_root().unwrap_or_default().join("spec")
}

pub fn topic_path(topic: &str, area: &str) -> PathBuf {
    spec_dir().join(area).join(topic.replace('/', "/"))
}

pub fn config_path() -> PathBuf {
    agent_dir().join("config.toml")
}

pub fn agent_dir() -> PathBuf {
    project_root().unwrap_or_default().join(".agent")
}

fn get_global_base_dir() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        let unispec_config = config_dir.join("unispec");
        if unispec_config.exists() || std::fs::create_dir_all(&unispec_config).is_ok() {
            return unispec_config;
        }
    }
    #[cfg(unix)]
    {
        PathBuf::from("/usr/share/unispec")
    }
    #[cfg(windows)]
    {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("unispec")
    }
    #[cfg(not(any(unix, windows)))]
    {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/usr/share/unispec"))
            .join("unispec")
    }
}

pub fn global_modes_dir() -> PathBuf {
    get_global_base_dir().join(".agent").join("modes")
}

pub fn global_config_dir() -> PathBuf {
    get_global_base_dir()
}

pub fn system_modes_dir() -> PathBuf {
    system_install_dir().join(".agent").join("modes")
}

pub fn system_areas_dir() -> PathBuf {
    system_install_dir().join(".agent").join("areas")
}

#[cfg(unix)]
pub fn system_install_dir() -> PathBuf {
    PathBuf::from("/usr/share/unispec")
}

#[cfg(windows)]
pub fn system_install_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("unispec")
}

#[cfg(not(any(unix, windows)))]
pub fn system_install_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/usr/share/unispec"))
        .join("unispec")
}

pub fn current_mode_templates_dir() -> Option<PathBuf> {
    let mode_name = crate::agent::current_mode().ok()?;
    let agent_dir = agent_dir();

    // Check local mode first
    let local_templates = agent_dir.join("modes").join(&mode_name).join("templates");
    if local_templates.exists() {
        return Some(local_templates);
    }

    // Check global mode
    let global_templates = global_modes_dir().join(&mode_name).join("templates");
    if global_templates.exists() {
        return Some(global_templates);
    }

    None
}

/// Get the templates directory for a specific area in the current mode
pub fn current_mode_area_templates_dir(area: &str) -> Option<PathBuf> {
    let mode_name = crate::agent::current_mode().ok()?;
    let agent_dir = agent_dir();

    // Check local mode areas first: .agent/modes/<mode>/areas/<area>/
    let local_areas = agent_dir
        .join("modes")
        .join(&mode_name)
        .join("areas")
        .join(area.to_lowercase());
    if local_areas.exists() {
        return Some(local_areas);
    }

    // Check global mode areas: ~/.config/unispec/modes/<mode>/areas/<area>/
    let global_areas = global_modes_dir()
        .join(&mode_name)
        .join("areas")
        .join(area.to_lowercase());
    if global_areas.exists() {
        return Some(global_areas);
    }

    None
}

pub fn read_template(name: &str) -> Option<String> {
    if let Some(templates_dir) = current_mode_templates_dir() {
        let path = templates_dir.join(name);
        if path.exists() {
            return fs::read_to_string(&path).ok();
        }
    }

    // Fallback to global templates dir
    let global_path = global_config_dir().join("templates").join(name);
    if global_path.exists() {
        return fs::read_to_string(&global_path).ok();
    }

    None
}

/// Read a template from the current mode's area-specific templates
/// Fallback to global templates if area-specific doesn't exist
pub fn read_area_template(area: &str, name: &str) -> Option<String> {
    // First try area-specific templates
    if let Some(areas_dir) = current_mode_area_templates_dir(area) {
        let path = areas_dir.join(name);
        if path.exists() {
            return fs::read_to_string(&path).ok();
        }
    }

    // Fallback to global mode templates
    read_template(name)
}

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

// Re-export modules
pub mod config;
pub mod index;
pub mod spec;

// Re-export Area from cli::model
pub use crate::cli::model::Area;

// Helper functions for dynamic areas
pub fn list_areas() -> Result<Vec<String>> {
    let dir = spec_dir();
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut areas = vec![];
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.join("area.md").exists() {
            areas.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    areas.sort();
    Ok(areas)
}

pub fn area_exists(area: &str) -> bool {
    spec_dir().join(area).join("area.md").exists()
}

pub fn rename_area(old: &str, new: &str) -> Result<()> {
    let old_path = spec_dir().join(old);
    let new_path = spec_dir().join(new);
    if old_path.exists() && !new_path.exists() {
        fs::rename(&old_path, &new_path)?;
    }
    Ok(())
}
