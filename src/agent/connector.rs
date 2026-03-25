// src/agent/connector.rs
use crate::agent::{load_agent_config, save_agent_config, ConnectorConfig, McpTool};
use crate::fs::agent_dir;
use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::{Command, Stdio};

pub fn run_new(
    name: &str,
    description: &str,
    command: &str,
    args: &[String],
    env_vars: &[(String, String)],
    working_dir: Option<&str>,
    timeout: Option<u64>,
) -> Result<String> {
    // Validate name
    if !name
        .chars()
        .all(|c| c.is_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(anyhow::anyhow!(
            "Connector name must be lowercase with underscores only"
        ));
    }

    let mut config = load_agent_config()?;

    // Check for duplicate
    if config.connectors.iter().any(|c| c.name == name) {
        return Err(anyhow::anyhow!("Connector '{}' already exists", name));
    }

    let mut env = std::collections::HashMap::new();
    for (key, value) in env_vars {
        env.insert(key.clone(), value.clone());
    }

    let connector = ConnectorConfig {
        name: name.to_string(),
        description: description.to_string(),
        command: command.to_string(),
        args: args.to_vec(),
        env,
        working_dir: working_dir.map(|s| s.to_string()),
        timeout,
    };

    config.connectors.push(connector);
    save_agent_config(&config)?;

    // Generate MCP server config
    save_mcp_config_to_file(&config)?;

    Ok(format!("Connector '{}' created successfully", name))
}

pub fn run_list() -> Result<()> {
    let config = load_agent_config()?;
    if config.connectors.is_empty() {
        println!("No connectors defined.");
        println!("Use 'unispec connector new' to create one.");
        return Ok(());
    }

    println!("Available Connectors:\n");
    for (i, connector) in config.connectors.iter().enumerate() {
        println!("{}. {} - {}", i + 1, connector.name, connector.description);
        println!(
            "   Command: {} {}",
            connector.command,
            connector.args.join(" ")
        );
        if !connector.env.is_empty() {
            let env_str: Vec<String> = connector
                .env
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            println!("   Env: {}", env_str.join(", "));
        }
        println!();
    }
    Ok(())
}

pub fn run_delete(name: &str) -> Result<String> {
    let mut config = load_agent_config()?;
    let original_len = config.connectors.len();
    config.connectors.retain(|c| c.name != name);

    if config.connectors.len() == original_len {
        return Err(anyhow::anyhow!("Connector '{}' not found", name));
    }

    save_agent_config(&config)?;
    save_mcp_config_to_file(&config)?;

    Ok(format!("Connector '{}' deleted", name))
}

pub fn run_run(name: &str, extra_args: &[String]) -> Result<String> {
    let config = load_agent_config()?;
    let connector = config
        .connectors
        .iter()
        .find(|c| c.name == name)
        .ok_or_else(|| anyhow::anyhow!("Connector '{}' not found", name))?;

    let mut cmd = Command::new(&connector.command);
    cmd.args(&connector.args);
    cmd.args(extra_args);

    for (key, value) in &connector.env {
        cmd.env(key, value);
    }

    if let Some(ref wd) = connector.working_dir {
        cmd.current_dir(wd);
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let timeout_secs = connector.timeout.unwrap_or(60);
    let output = cmd
        .output()
        .context(format!("Failed to execute connector '{}'", name))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("Connector '{}' failed:", name);
        if !stderr.is_empty() {
            eprintln!("STDERR:\n{}", stderr);
        }
        return Err(anyhow::anyhow!(
            "Connector '{}' exited with code {:?}",
            name,
            output.status.code()
        ));
    }

    if !stderr.is_empty() {
        eprintln!("STDOUT:\n{}", stderr);
    }

    Ok(stdout.to_string())
}

pub fn run_edit(name: &str, description: Option<&str>) -> Result<String> {
    let mut config = load_agent_config()?;
    let connector = config
        .connectors
        .iter_mut()
        .find(|c| c.name == name)
        .ok_or_else(|| anyhow::anyhow!("Connector '{}' not found", name))?;

    if let Some(desc) = description {
        connector.description = desc.to_string();
    }

    save_agent_config(&config)?;
    save_mcp_config_to_file(&config)?;

    Ok(format!("Connector '{}' updated", name))
}

/// Generate MCP server configuration and save to file.
fn save_mcp_config_to_file(config: &crate::agent::AgentConfig) -> Result<()> {
    let mcp_path = agent_dir().join("mcp.json");
    let mcp_config = serde_json::json!({
        "mcpServers": config
            .connectors
            .iter()
            .map(|c| {
                let mut hasher = DefaultHasher::new();
                c.name.hash(&mut hasher);
                let hash = hasher.finish();
                (
                    format!("unispec_{}", c.name),
                    serde_json::json!({
                        "command": "unispec",
                        "args": ["connector", "run", &c.name],
                        "env": c.env.is_empty().then_some(serde_json::Value::Null).unwrap_or_else(|| {
                            serde_json::Value::Object(
                                c.env
                                    .iter()
                                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                                    .collect()
                            )
                        })
                    }),
                )
            })
            .collect::<serde_json::Map<String, serde_json::Value>>()
    });

    let content = serde_json::to_string_pretty(&mcp_config)?;
    std::fs::write(&mcp_path, content)?;
    Ok(())
}

pub fn generate_mcp_config() -> Result<String> {
    let config = load_agent_config()?;
    let mcp_config = serde_json::json!({
        "mcpServers": config
            .connectors
            .iter()
            .map(|c| {
                (
                    format!("unispec_{}", c.name),
                    serde_json::json!({
                        "command": "unispec",
                        "args": ["connector", "run", &c.name],
                        "env": c.env.is_empty().then_some(serde_json::Value::Null).unwrap_or_else(|| {
                            serde_json::Value::Object(
                                c.env
                                    .iter()
                                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                                    .collect()
                            )
                        })
                    }),
                )
            })
            .collect::<serde_json::Map<String, serde_json::Value>>()
    });

    Ok(serde_json::to_string_pretty(&mcp_config)?)
}

pub fn get_mcp_tools() -> Vec<McpTool> {
    let config = match load_agent_config() {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    config
        .connectors
        .iter()
        .map(|c| McpTool {
            name: format!("unispec_{}", c.name),
            description: c.description.clone(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "args": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Additional arguments to pass to the command"
                    }
                }
            }),
        })
        .collect()
}
