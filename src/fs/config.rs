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
    #[serde(default)]
    pub ingest: IngestConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestConfig {
    #[serde(default = "default_auto_index")]
    pub auto_index: bool,
    #[serde(default)]
    pub index_on_complete: bool,
    #[serde(default = "default_capture_functions")]
    pub capture_functions: bool,
    #[serde(default = "default_capture_structs")]
    pub capture_structs: bool,
    #[serde(default = "default_capture_enums")]
    pub capture_enums: bool,
    #[serde(default = "default_capture_imports")]
    pub capture_imports: bool,
    #[serde(default = "default_output_format")]
    pub output_format: String,
    #[serde(default)]
    pub languages: Vec<String>,
}

fn default_area() -> String {
    "Working".to_string()
}

fn default_paddy_enabled() -> bool {
    true
}

fn default_auto_index() -> bool {
    false
}

fn default_index_on_complete() -> bool {
    false
}

fn default_capture_functions() -> bool {
    true
}

fn default_capture_structs() -> bool {
    true
}

fn default_capture_enums() -> bool {
    true
}

fn default_capture_imports() -> bool {
    true
}

fn default_output_format() -> String {
    "toml".to_string()
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
            ingest: IngestConfig::default(),
        }
    }
}

impl Default for IngestConfig {
    fn default() -> Self {
        Self {
            auto_index: false,
            index_on_complete: false,
            capture_functions: true,
            capture_structs: true,
            capture_enums: true,
            capture_imports: true,
            output_format: "toml".to_string(),
            languages: vec![],
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

pub fn get_ingest_config() -> Result<IngestConfig> {
    let config = load_extended_config()?;
    Ok(config.ingest)
}

pub fn set_ingest_config(ingest: IngestConfig) -> Result<()> {
    let mut config = load_extended_config()?;
    config.ingest = ingest;
    let content = toml::to_string_pretty(&config)?;
    fs::write(&config_path(), content)?;
    Ok(())
}
