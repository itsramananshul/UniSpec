// src/fs/config.rs
use crate::fs::config_path;
use anyhow::Result;
use std::fs;

pub fn load_config() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config {
            area: "Working".to_string(),
        });
    }

    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    fs::write(&config_path(), content)?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub area: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedConfig {
    #[serde(default = "default_area")]
    pub area: String,
    #[serde(default)]
    pub protected_areas: Vec<String>,
    #[serde(default = "default_paddy_enabled")]
    pub paddy_enabled: bool,
}

fn default_area() -> String {
    "Working".to_string()
}

fn default_paddy_enabled() -> bool {
    true
}

impl Default for ExtendedConfig {
    fn default() -> Self {
        Self {
            area: "Working".to_string(),
            protected_areas: vec![
                "Staging".to_string(),
                "Working".to_string(),
                "Build".to_string(),
            ],
            paddy_enabled: true,
        }
    }
}

pub fn load_extended_config() -> Result<ExtendedConfig> {
    let path = config_path();
    if !path.exists() {
        return Ok(ExtendedConfig::default());
    }

    let content = fs::read_to_string(&path)?;
    let config: ExtendedConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn get_paddy_enabled() -> Result<bool> {
    let config = load_extended_config()?;
    Ok(config.paddy_enabled)
}

pub fn set_paddy_enabled(enabled: bool) -> Result<()> {
    let mut config = load_extended_config()?;
    config.paddy_enabled = enabled;
    let content = toml::to_string_pretty(&config)?;
    fs::write(&config_path(), content)?;
    Ok(())
}
