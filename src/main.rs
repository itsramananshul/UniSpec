// src/main.rs
mod agent;
mod cli;
mod commands;
mod fs;
mod mcp;
mod platypus;
mod tui;
mod version;

use anyhow::Result;
use clap::Parser;

use crate::agent::{connector as agent_connector, mode as agent_mode};
use crate::cli::{
    AreaCommands, ConnectorCommands, IndexCommands, ModeCommands, PattyCommands, TopicCommands,
};
use crate::cli::{Cli, Commands};
use crate::commands::{area, index, init, init_editor, set, topic};

fn get_show_platypus() -> bool {
    crate::fs::config::get_paddy_enabled().unwrap_or(true)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // Default: launch TUI
            if !crate::fs::spec_dir().exists() {
                eprintln!("No spec folder found. Run unispec init to initialize a project.");
                std::process::exit(1);
            }
            tui::main::main()?;
        }
        Some(Commands::Init {
            root,
            amazon_q,
            antigravity,
            auggie,
            claude_code,
            cline,
            codex,
            codebuddy,
            continue_,
            costrict,
            crush,
            cursor,
            factory,
            gemini_cli,
            github,
            iflow,
            kilo_code,
            kiro,
            opencode,
            pi,
            qoder,
            qwen_code,
            roo_code,
            windsurf,
            trae,
        }) => {
            let mut editors = Vec::new();

            if amazon_q {
                editors.push("amazon-q");
            }
            if antigravity {
                editors.push("antigravity");
            }
            if auggie {
                editors.push("auggie");
            }
            if claude_code {
                editors.push("claude-code");
            }
            if cline {
                editors.push("cline");
            }
            if codex {
                editors.push("codex");
            }
            if codebuddy {
                editors.push("codebuddy");
            }
            if continue_ {
                editors.push("continue");
            }
            if costrict {
                editors.push("costrict");
            }
            if crush {
                editors.push("crush");
            }
            if cursor {
                editors.push("cursor");
            }
            if factory {
                editors.push("factory");
            }
            if gemini_cli {
                editors.push("gemini-cli");
            }
            if github {
                editors.push("github");
            }
            if iflow {
                editors.push("iflow");
            }
            if kilo_code {
                editors.push("kilo-code");
            }
            if kiro {
                editors.push("kiro");
            }
            if opencode {
                editors.push("opencode");
            }
            if pi {
                editors.push("pi");
            }
            if qoder {
                editors.push("qoder");
            }
            if qwen_code {
                editors.push("qwen-code");
            }
            if roo_code {
                editors.push("roo-code");
            }
            if windsurf {
                editors.push("windsurf");
            }
            if trae {
                editors.push("trae");
            }

            let root_path = root.as_deref().unwrap_or_else(|| std::path::Path::new("."));

            if !editors.is_empty() {
                let results = init_editor::run_init_editors(root_path, &editors)?;
                init_editor::print_editor_results(&results);
            } else {
                init::run_init(root.as_deref())?;
            }

            if get_show_platypus() {
                platypus::happy();
            }
        }
        Some(Commands::Set { area }) => {
            set::run_set(&area)?;
            if get_show_platypus() {
                platypus::happy();
            }
        }
        Some(Commands::Area(area_cmd)) => match area_cmd {
            AreaCommands::Add { name } => {
                area::run_add(&name)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            AreaCommands::Remove { name } => {
                area::run_remove(&name)?;
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            AreaCommands::List => area::run_list()?,
            AreaCommands::Rename { old, new } => {
                area::run_rename(&old, &new)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            AreaCommands::Default { name } => {
                area::run_default(&name)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            AreaCommands::Health => area::run_health()?,
        },
        Some(Commands::Topic(topic_cmd)) => match topic_cmd {
            TopicCommands::Add { topic, area } => {
                topic::run_new(&topic, &area)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            TopicCommands::List { area, hierarchy: _ } => topic::run_list(&area, false)?,
            TopicCommands::Push { topic, area } => {
                topic::run_push(&topic, &area, None)?;
                if get_show_platypus() {
                    platypus::working();
                }
            }
            TopicCommands::Pull { topic, area } => {
                topic::run_pull(&topic, &area)?;
                if get_show_platypus() {
                    platypus::working();
                }
            }
            TopicCommands::Remove { topic, force } => {
                topic::run_delete(
                    &topic,
                    &crate::fs::config::load_config()?.area.as_str(),
                    force,
                )?;
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            TopicCommands::Show { topic } => topic::run_show(&topic)?,
            TopicCommands::Progress { area } => topic::run_progress(area.as_deref())?,
        },
        Some(Commands::Index(index_cmd)) => match index_cmd {
            IndexCommands::Add {
                topic,
                path,
                area,
                link_type,
            } => {
                let area = area.unwrap_or_else(|| "Working".to_string());
                let link_type = link_type.unwrap_or_else(|| index::detect_type(&path));
                index::run_add(&topic, &path, &area, &link_type)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            IndexCommands::Remove { topic, path } => {
                index::run_remove(&topic, &path)?;
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            IndexCommands::List { topic, path } => {
                index::run_list(topic.as_deref(), path.as_deref())?;
            }
            IndexCommands::Find { query, by } => {
                index::run_find(&query, &by)?;
            }
            IndexCommands::Full => index::run_full()?,
            IndexCommands::Watch => index::run_watch()?,
            IndexCommands::Cleanup => index::run_cleanup()?,
        },
        Some(Commands::Mcp { path }) => {
            let path_str = path.map(|p| p.to_string_lossy().to_string());
            mcp::run_mcp_server(path_str.as_deref())?;
        }
        Some(Commands::Mode(mode_cmd)) => match mode_cmd {
            ModeCommands::List => {
                let modes = agent_mode::list_modes()?;
                if modes.is_empty() {
                    println!("No modes found.");
                    println!("Modes should be placed in .agent/modes/ or ~/.config/unispec/modes/");
                } else {
                    println!("Available Modes:\n");
                    for mode in &modes {
                        let active = if mode.is_active { " [ACTIVE]" } else { "" };
                        let source = if mode.source == agent_mode::ModeSource::Global {
                            " (global)"
                        } else {
                            ""
                        };
                        println!(
                            "  {} - {}{}{}",
                            mode.name, mode.display_name, active, source
                        );
                        println!("    {}", mode.description);
                        println!();
                    }
                }
            }
            ModeCommands::Info { name } => {
                let config = agent_mode::get_mode_info(&name)?;
                let path = agent_mode::get_mode_path(&name)?;
                let is_global = path.starts_with(crate::fs::global_modes_dir());
                println!("Mode: {}", config.mode.name);
                println!("Display Name: {}", config.mode.display_name);
                println!("Version: {}", config.mode.version);
                println!("Location: {}", if is_global { "global" } else { "local" });
                if let Some(author) = config.author {
                    println!("Author: {}", author.name);
                }
                println!("\nDescription:");
                println!("{}", config.mode.description);
                println!("\nAreas:");
                println!("  Default: {}", config.areas.default.join(", "));
                println!("  Protected: {}", config.areas.protected.join(", "));
                println!("\nCapabilities:");
                println!("  Spec Writing: {}", config.capabilities.spec_writing);
                println!("  Building: {}", config.capabilities.building);
                println!("  Verification: {}", config.capabilities.verification);
                println!("  Connectors: {}", config.capabilities.connectors);
                println!(
                    "  Custom Workflows: {}",
                    config.capabilities.custom_workflows
                );
            }
            ModeCommands::Activate { name } => {
                let result = agent_mode::run_activate(&name)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::celebrating();
                }
            }
            ModeCommands::Add { path, global } => {
                let result = agent_mode::add_mode(&path, global)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            ModeCommands::Remove { name, global } => {
                let result = agent_mode::remove_mode(&name, global)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            ModeCommands::Current => {
                let current = crate::agent::current_mode()?;
                println!("Current mode: {}", current);
            }
        },
        Some(Commands::Connector(conn_cmd)) => match conn_cmd {
            ConnectorCommands::New {
                name,
                description,
                command,
                args,
                env,
                working_dir,
                timeout,
            } => {
                let args: Vec<String> = args
                    .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
                    .unwrap_or_default();
                let env: Vec<(String, String)> = env
                    .map(|s| {
                        s.split(',')
                            .filter_map(|pair| {
                                let mut parts = pair.split('=');
                                match (parts.next(), parts.next()) {
                                    (Some(k), Some(v)) => {
                                        Some((k.trim().to_string(), v.trim().to_string()))
                                    }
                                    _ => None,
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let result = agent_connector::run_new(
                    &name,
                    &description,
                    &command,
                    &args,
                    &env,
                    working_dir.as_deref(),
                    timeout,
                )?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            ConnectorCommands::List => {
                agent_connector::run_list()?;
            }
            ConnectorCommands::Delete { name } => {
                let result = agent_connector::run_delete(&name)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            ConnectorCommands::Run { name, args } => {
                let output = agent_connector::run_run(&name, &args)?;
                println!("{}", output);
            }
            ConnectorCommands::Edit { name, description } => {
                let result = agent_connector::run_edit(&name, description.as_deref())?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            ConnectorCommands::Mcp => {
                let config = agent_connector::generate_mcp_config()?;
                println!("{}", config);
            }
        },
        Some(Commands::Patty(paddy_cmd)) => match paddy_cmd {
            PattyCommands::Enable => {
                crate::fs::config::set_paddy_enabled(true)?;
                println!("Platypus enabled!");
                platypus::happy();
            }
            PattyCommands::Disable => {
                crate::fs::config::set_paddy_enabled(false)?;
                println!("Platypus disabled.");
                platypus::sad();
            }
            PattyCommands::Status => {
                let enabled = get_show_platypus();
                if enabled {
                    println!("Platypus is enabled");
                } else {
                    println!("Platypus is disabled");
                }
            }
        },
    }

    Ok(())
}
