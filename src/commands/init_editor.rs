// src/commands/init_editor.rs
// Editor-specific initialization for AI coding assistants

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Editor configuration
pub struct EditorConfig {
    pub cli_name: &'static str,
    pub display_name: &'static str,
    pub subdirectory: &'static str,
    pub extension: Option<&'static str>,
    pub strip_prefix: bool,
    pub is_home_based: bool,
}

pub const EDITORS: &[EditorConfig] = &[
    EditorConfig {
        cli_name: "amazon-q",
        display_name: "Amazon Q Developer",
        subdirectory: ".amazonq/prompts",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "antigravity",
        display_name: "Antigravity",
        subdirectory: ".agent/workflows",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "auggie",
        display_name: "Augment CLI",
        subdirectory: ".augment/commands",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "claude-code",
        display_name: "Claude Code",
        subdirectory: ".claude/commands/unispec",
        extension: None,
        strip_prefix: true,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "cline",
        display_name: "Cline",
        subdirectory: ".clinerules/workflows",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "codex",
        display_name: "Codex",
        subdirectory: ".codex/prompts",
        extension: None,
        strip_prefix: false,
        is_home_based: true,
    },
    EditorConfig {
        cli_name: "codebuddy",
        display_name: "CodeBuddy",
        subdirectory: ".codebuddy/commands/unispec",
        extension: None,
        strip_prefix: true,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "continue",
        display_name: "Continue",
        subdirectory: ".continue/prompts",
        extension: Some(".prompt"),
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "costrict",
        display_name: "CoStrict",
        subdirectory: ".cospec/unispec/commands",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "crush",
        display_name: "Crush",
        subdirectory: ".crush/commands/unispec",
        extension: None,
        strip_prefix: true,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "cursor",
        display_name: "Cursor",
        subdirectory: ".cursor/commands",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "factory",
        display_name: "Factory Droid",
        subdirectory: ".factory/commands",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "gemini-cli",
        display_name: "Gemini CLI",
        subdirectory: ".gemini/commands/unispec",
        extension: Some(".toml"),
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "github",
        display_name: "GitHub",
        subdirectory: ".github/prompts",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "iflow",
        display_name: "iFlow",
        subdirectory: ".iflow/commands",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "kilo-code",
        display_name: "Kilo Code",
        subdirectory: ".kilocode/workflows",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "kiro",
        display_name: "Kiro",
        subdirectory: ".kiro/prompts",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "opencode",
        display_name: "OpenCode",
        subdirectory: ".opencode/command",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "pi",
        display_name: "Pi",
        subdirectory: ".pi/prompts",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "qoder",
        display_name: "Qoder",
        subdirectory: ".qoder/commands/unispec",
        extension: None,
        strip_prefix: true,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "qwen-code",
        display_name: "Qwen Code",
        subdirectory: ".qwen/commands",
        extension: Some(".toml"),
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "roo-code",
        display_name: "RooCode",
        subdirectory: ".roo/commands",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "windsurf",
        display_name: "Windsurf",
        subdirectory: ".windsurf/workflows",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
    EditorConfig {
        cli_name: "trae",
        display_name: "TRAE",
        subdirectory: ".trae/rule",
        extension: None,
        strip_prefix: false,
        is_home_based: false,
    },
];

pub fn find_editor(cli_name: &str) -> Option<&EditorConfig> {
    EDITORS.iter().find(|e| e.cli_name == cli_name)
}

pub fn get_all_editor_names() -> Vec<&'static str> {
    EDITORS.iter().map(|e| e.cli_name).collect()
}

pub fn run_init_editors(project_root: &Path, editor_names: &[&str]) -> Result<Vec<EditorResult>> {
    let workflows_dir = project_root.join(".agent/workflows");

    if !workflows_dir.exists() {
        let system_workflows = crate::fs::system_install_dir()
            .join(".agent")
            .join("modes")
            .join("simple")
            .join("workflows");
        if !system_workflows.exists() {
            return Ok(Vec::new());
        }
        crate::commands::init::run_init(Some(project_root))?;
        let new_workflows_dir = project_root.join(".agent/workflows");
        return init_editors_from_dir(project_root, editor_names, &new_workflows_dir);
    }

    init_editors_from_dir(project_root, editor_names, &workflows_dir)
}

fn init_editors_from_dir(
    project_root: &Path,
    editor_names: &[&str],
    workflows_dir: &Path,
) -> Result<Vec<EditorResult>> {
    let mut results = Vec::new();

    for editor_name in editor_names {
        if let Some(editor) = find_editor(editor_name) {
            let result = init_editor(project_root, editor, &workflows_dir)?;
            results.push(result);
        } else {
            results.push(EditorResult {
                editor: editor_name.to_string(),
                display_name: editor_name.to_string(),
                success: false,
                message: format!("Unknown editor: {}", editor_name),
                path: None,
            });
        }
    }

    Ok(results)
}

pub struct EditorResult {
    pub editor: String,
    pub display_name: String,
    pub success: bool,
    pub message: String,
    pub path: Option<String>,
}

fn init_editor(
    project_root: &Path,
    editor: &EditorConfig,
    workflows_dir: &Path,
) -> Result<EditorResult> {
    let target_dir = if editor.is_home_based {
        dirs::home_dir()
            .map(|h| h.join(&editor.subdirectory))
            .unwrap_or_else(|| Path::new(&editor.subdirectory).to_path_buf())
    } else {
        project_root.join(&editor.subdirectory)
    };

    fs::create_dir_all(&target_dir)?;

    let mut files_copied = 0;

    for entry in fs::read_dir(workflows_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            let new_filename = transform_filename(filename, editor);
            let dest_path = target_dir.join(&new_filename);

            if dest_path.exists() {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&path) {
                fs::write(&dest_path, content)?;
                files_copied += 1;
            }
        }
    }

    let relative_path = if editor.is_home_based {
        format!("~/.{}", editor.subdirectory)
    } else {
        format!("./{}", editor.subdirectory)
    };

    let message = if files_copied > 0 {
        format!("{} workflow(s) exported", files_copied)
    } else {
        "No workflows to export (or already exists)".to_string()
    };

    if editor.is_home_based {
        let project_link = project_root.join(".codex");
        if !project_link.exists() {
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&target_dir, &project_link).ok();
            }
        }
    }

    Ok(EditorResult {
        editor: editor.cli_name.to_string(),
        display_name: editor.display_name.to_string(),
        success: files_copied > 0 || !workflows_dir.exists(),
        message,
        path: Some(relative_path),
    })
}

fn transform_filename(filename: &str, editor: &EditorConfig) -> String {
    let name_without_ext = filename.trim_end_matches(".md");

    let stripped_name = if editor.strip_prefix {
        name_without_ext
            .strip_prefix("unispec:")
            .or_else(|| name_without_ext.strip_prefix("unispec:"))
            .or_else(|| name_without_ext.strip_prefix("unispec_"))
            .unwrap_or(name_without_ext)
            .trim_start_matches("unispec_")
            .trim_start_matches("unispec_")
    } else {
        name_without_ext
    };

    let extension = editor.extension.unwrap_or(".md");

    format!("{}{}", stripped_name, extension)
}

pub fn print_editor_results(results: &[EditorResult]) {
    println!("\nEditor Integration:");
    for result in results {
        let status = if result.success { "✓" } else { "✗" };
        let path = result.path.as_deref().unwrap_or("N/A");
        println!(
            "  {} {}: {} [{}]",
            status, result.display_name, result.message, path
        );
    }
}
