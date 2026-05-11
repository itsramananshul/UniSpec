// src/commands/spec.rs
//
// Shared logic for creating a spec + task file for a topic. Used by both the
// CLI (`unispec spec add ...`) and the MCP server (`spec_add` tool).
//
// Behaviour (preserved verbatim from the original MCP handler):
// - `topic` may contain `/` for nested topics. The parent must already exist.
// - `area` defaults to "Staging" when callers pass `None`. Case is preserved as
//   provided; the function will fall back to the lowercase variant if the
//   uppercased directory does not exist.
// - `short` is required and non-empty.
// - `spec_content` and `task_content` must each be at least 11 characters
//   (`>10`) after trimming.
// - Any leading `---` frontmatter block the caller includes in content is
//   stripped before the function prepends its own canonical frontmatter.
// - Files are written as `<topic-safe>_spec.md` and `<topic-safe>_task.md`
//   where `topic-safe = topic.replace('/', "-").replace(' ', "-")`.

use anyhow::Result;
use std::path::PathBuf;

pub struct SpecAddOutput {
    pub topic: String,
    pub area: String,
    pub spec_file: String,
    pub task_file: String,
    pub spec_path: PathBuf,
    pub task_path: PathBuf,
}

/// Strip a leading `---` YAML frontmatter block from `content` if one is
/// present. The first line must start with `---`; the block ends at the next
/// line beginning with `---`.
fn strip_frontmatter(content: &str) -> &str {
    if content.trim_start().starts_with("---") {
        if let Some(end) = content.find("\n---") {
            return &content[end + 5..];
        }
    }
    content
}

pub fn run_spec_add(
    topic: &str,
    area: Option<&str>,
    short: &str,
    spec_content: &str,
    task_content: &str,
) -> Result<SpecAddOutput> {
    let area = area.unwrap_or("Staging");

    let short = short.trim();
    if short.is_empty() {
        return Err(anyhow::anyhow!("'short' parameter is required and must be non-empty"));
    }

    let spec_content = spec_content.trim();
    if spec_content.len() <= 10 {
        return Err(anyhow::anyhow!(
            "'spec_content' must be at least 11 characters of actual text"
        ));
    }

    let task_content = task_content.trim();
    if task_content.len() <= 10 {
        return Err(anyhow::anyhow!(
            "'task_content' must be at least 11 characters of actual text"
        ));
    }

    // For nested topics ("auth/login"), verify the parent topic directory
    // exists before we create a child inside it.
    if topic.contains('/') {
        let parts: Vec<&str> = topic.split('/').collect();
        if parts.len() >= 2 {
            let parent_topic = parts[0];
            let parent_upper = crate::fs::spec_dir().join(area).join(parent_topic);
            let parent_lower = crate::fs::spec_dir()
                .join(area.to_lowercase())
                .join(parent_topic);
            if !parent_upper.exists() && !parent_lower.exists() {
                return Err(anyhow::anyhow!(
                    "Parent topic '{}' does not exist. Create it first with: \
                     unispec topic add {} --area {} --short \"…\" --content \"…\"",
                    parent_topic,
                    parent_topic,
                    area
                ));
            }
        }
    }

    // Topic-safe filename component (matches the original MCP handler).
    let topic_safe = topic.replace('/', "-").replace(' ', "-");
    let spec_filename = format!("{}_spec.md", topic_safe);
    let task_filename = format!("{}_task.md", topic_safe);

    // Resolve / create the topic directory, supporting nested paths.
    let spec_dir = {
        let upper = crate::fs::spec_dir().join(area).join(topic);
        if upper.exists() {
            upper
        } else {
            let lower = crate::fs::spec_dir().join(area.to_lowercase()).join(topic);
            if lower.exists() {
                lower
            } else {
                std::fs::create_dir_all(&upper)?;
                upper
            }
        }
    };

    let cleaned_spec = strip_frontmatter(spec_content).trim_start();
    let cleaned_task = strip_frontmatter(task_content).trim_start();

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let author = crate::commands::topic::get_agent_id();

    let spec_frontmatter = format!(
        "---\ntitle: {}\nshort: {}\ncreated: {}\nauthor: {}\nstatus: draft\n---\n\n",
        topic, short, now, author
    );
    let task_frontmatter = format!(
        "---\nspec: {}\nshort: {}\nstatus: pending\ndate: {}\n---\n\n",
        topic,
        short,
        now.split(' ').next().unwrap_or(&now)
    );

    let spec_full = format!("{}{}", spec_frontmatter, cleaned_spec);
    let task_full = format!("{}{}", task_frontmatter, cleaned_task);

    let spec_path = spec_dir.join(&spec_filename);
    let task_path = spec_dir.join(&task_filename);

    std::fs::write(&spec_path, spec_full)?;
    std::fs::write(&task_path, task_full)?;

    Ok(SpecAddOutput {
        topic: topic.to_string(),
        area: area.to_string(),
        spec_file: spec_filename,
        task_file: task_filename,
        spec_path,
        task_path,
    })
}
