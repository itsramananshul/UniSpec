// src/commands/init.rs
use crate::fs::ensure_dir;
use anyhow::Result;
use std::fs;
use std::path::Path;

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
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

pub fn run_init(root: Option<&std::path::Path>) -> Result<()> {
    let root = root.unwrap_or_else(|| std::path::Path::new("."));
    let spec_root = root.join("spec");

    // Default areas for simple mode
    let default_areas = ["Staging", "Working", "Build"];

    // Copy area templates from global areas dir (or system install dir as fallback)
    let areas_dir = crate::fs::global_config_dir().join(".agent").join("areas");
    let system_areas_dir = crate::fs::system_areas_dir();
    for area in &default_areas {
        let area_spec_dir = spec_root.join(area);
        ensure_dir(&area_spec_dir)?;
        let area_path = area_spec_dir.join("area.md");
        if !area_path.exists() {
            let template_path = areas_dir.join(area.to_lowercase()).join("area.md");
            let system_template = system_areas_dir.join(area.to_lowercase()).join("area.md");
            if template_path.exists() {
                fs::copy(&template_path, &area_path)?;
            } else if system_template.exists() {
                fs::copy(&system_template, &area_path)?;
            }
        }
    }

    // Create .agent directory
    let agent_root = root.join(".agent");
    ensure_dir(&agent_root)?;

    let config_file = agent_root.join("config.toml");
    if !config_file.exists() {
        fs::write(&config_file, CONFIG_TEMPLATE)?;
    }

    // Copy simple mode from global (or system install dir as fallback)
    let simple_mode = agent_root.join("modes").join("simple");
    if !simple_mode.exists() {
        ensure_dir(&simple_mode)?;
        let global_simple = crate::fs::global_modes_dir().join("simple");
        let system_simple = crate::fs::system_modes_dir().join("simple");
        if global_simple.exists() {
            copy_dir_recursive(&global_simple, &simple_mode)?;
        } else if system_simple.exists() {
            copy_dir_recursive(&system_simple, &simple_mode)?;
        }
    }

    // Copy workflows to main .agent/workflows for active use
    let workflows_dir = agent_root.join("workflows");
    ensure_dir(&workflows_dir)?;
    for workflow_file in ["osdd:spec.md", "osdd:build.md", "osdd:verify.md"] {
        let src = simple_mode.join("workflows").join(workflow_file);
        let dst = workflows_dir.join(workflow_file);
        if src.exists() && !dst.exists() {
            fs::copy(&src, &dst)?;
        }
    }

    // Create default area.md files if they don't exist
    let default_area_content = r#"# Area: {area}

This is a {area} area for organizing topics.

## Purpose

Add purpose description here.

## Topics

List topics in this area:
"#;
    for area in &default_areas {
        let area_spec_dir = spec_root.join(area);
        ensure_dir(&area_spec_dir)?;
        let area_path = area_spec_dir.join("area.md");
        if !area_path.exists() {
            let content = default_area_content.replace("{area}", area);
            fs::write(&area_path, content)?;
        }
    }

    // Copy skill.md to .agent for active use
    let skill_src = simple_mode.join("skill.md");
    let skill_dst = agent_root.join("skill.md");
    if skill_src.exists() && !skill_dst.exists() {
        fs::copy(&skill_src, &skill_dst)?;
    }

    // Create modes README
    fs::write(agent_root.join("modes").join("README.md"), MODES_README)?;

    println!(
        "
        ██╗   ██╗███╗   ██╗██╗    ███████╗██████╗ ███████╗ ██████╗
        ██║   ██║████╗  ██║██║    ██╔════╝██╔══██╗██╔════╝██╔════╝
        ██║   ██║██╔██╗ ██║██║    ███████╗██████╔╝█████╗  ██║
        ██║   ██║██║╚██╗██║██║    ╚════██║██╔═══╝ ██╔══╝  ██║
        ╚██████╔╝██║ ╚████║██║    ███████║██║     ███████╗╚██████╗
         ╚═════╝ ╚═╝  ╚═══╝╚═╝    ╚══════╝╚═╝     ╚══════╝ ╚═════╝
         "
    );
    println!("UniSpec initialized with Simple Mode! -- Meet Paddy the Pladdy");
    println!();
    println!("Agent commands available:");
    println!("  unispec --help -    Available commands, see docs for more info");
    println!();
    println!("See .agent/modes/README.md for creating custom modes!");
    Ok(())
}

const CONFIG_TEMPLATE: &str = r#"# UniSpec Agent Configuration

# Current active mode
current_mode = "simple"

# Default area for topic operations
# default_area = "Working"

# Protected areas that cannot be deleted
# protected_areas = ["Staging", "Working", "Build"]

# Connectors - Custom commands that become MCP tools
# Example:
# [[connector]]
# name = "test"
# description = "Run the test suite"
# command = "pytest"
# args = ["tests/", "-v"]
"#;

const MODES_README: &str = r#"# UniSpec Modes

Modes define different agent configurations. Each mode has its own workflows and capabilities.

## Creating a Mode

1. Create directory: `.agent/modes/<mode_name>/`
2. Add `mode.toml` with metadata
3. Add `skill.md` with agent persona
4. Add `workflows/*.md` files
5. Add `areas/` with staging/, working/, build/ directories containing area.md

## Global Modes

Modes in `~/.config/unispec/modes/` are available to all projects.

## Area Templates

Place area templates in `.agent/areas/<area_name>/area.md`. The init command will copy these to spec/<area_name>/area.md.
"#;
