// src/tui/state.rs
use crate::agent::mode::{DisplayType, ModeConfig};
use crate::fs::{self, spec::SpecMetadata};
use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct TopicNode {
    pub topic: String,
    pub path: PathBuf,
    pub area: String,
    pub area_type: DisplayType,
    pub status: String,
    pub tasks_total: usize,
    pub tasks_completed: usize,
    pub tasks_in_progress: usize,
    pub metadata: SpecMetadata,
    pub children: Vec<TopicNode>,
}

impl TopicNode {
    pub fn relative_path(&self) -> String {
        let area_path = fs::spec_dir().join(&self.area);
        if let Ok(rel) = self.path.strip_prefix(area_path) {
            rel.to_string_lossy().to_string()
        } else {
            self.topic.clone()
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum NavState {
    AreaSelection,
    TopicList(String),
    NestedSpecs(PathBuf),
    FindResults { topic: String, paths: Vec<String> },
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub nav_state: NavState,
    pub topics: Vec<TopicNode>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        Ok(AppState {
            nav_state: NavState::AreaSelection,
            topics: vec![],
        })
    }

    pub fn load_areas(&self) -> Result<Vec<String>> {
        let mut areas = vec![];
        let spec_dir = fs::spec_dir();
        if spec_dir.exists() {
            for entry in std::fs::read_dir(spec_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    let area_filename = crate::agent::mode::get_area_filename();
                    if entry.path().join(&area_filename).exists() {
                        areas.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
        }
        Ok(areas)
    }

    pub fn load_topics_for_area(&mut self, area: &str) -> Result<()> {
        let area_type = self.get_area_type(area);
        let spec_file = self.get_spec_filename_for_area(area);
        let mut topics = vec![];
        let area_spec_dir = fs::spec_dir().join(area);
        if area_spec_dir.exists() {
            for entry in std::fs::read_dir(&area_spec_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    topics.push(self.load_topic_recursive(
                        &entry.path(),
                        area.to_string(),
                        area_type.clone(),
                        &spec_file,
                    )?);
                }
            }
        }
        self.topics = topics;
        self.nav_state = NavState::TopicList(area.to_string());
        Ok(())
    }

    fn get_area_type(&self, area: &str) -> DisplayType {
        let area_lower = area.to_lowercase();

        // First, check area name directly - most reliable
        if area_lower.contains("roadmap") {
            return DisplayType::Roadmap;
        }
        if area_lower.contains("build") && area_lower != "working" {
            return DisplayType::Build;
        }
        if area_lower.contains("working") {
            return DisplayType::Working;
        }

        // Then check mode config
        if let Ok(mode_name) = crate::agent::current_mode() {
            if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                if let Some(ref roadmap_config) = config.area_types.roadmap {
                    if area_lower == "roadmap" || area_lower.contains("roadmap") {
                        return DisplayType::Roadmap;
                    }
                }
                if let Some(ref working_config) = config.area_types.working {
                    if area_lower == "working" || area_lower.contains("working") {
                        return DisplayType::Working;
                    }
                }
                if let Some(ref build_config) = config.area_types.build {
                    if area_lower == "build" || area_lower.contains("build") {
                        return DisplayType::Build;
                    }
                }
            }
        }
        DisplayType::Standard
    }

    fn get_spec_filename_for_area(&self, area: &str) -> String {
        let area_lower = area.to_lowercase();

        // Check area name directly first
        if area_lower.contains("roadmap") {
            // Try mode config first, then check what files exist
            if let Ok(mode_name) = crate::agent::current_mode() {
                if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                    if let Some(ref roadmap_config) = config.area_types.roadmap {
                        let filename = &roadmap_config.spec_file;
                        // Check if file exists, if not try alternatives
                        let spec_dir = fs::spec_dir().join(area);
                        if spec_dir.join(filename).exists() {
                            return filename.clone();
                        }
                    }
                }
            }
            // Fallback: return first existing common name
            return "spec.md".to_string();
        }

        if area_lower.contains("build") {
            if let Ok(mode_name) = crate::agent::current_mode() {
                if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                    if let Some(ref build_config) = config.area_types.build {
                        return build_config.spec_file.clone();
                    }
                }
            }
            return "specs.md".to_string();
        }

        if area_lower.contains("working") {
            if let Ok(mode_name) = crate::agent::current_mode() {
                if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                    if let Some(ref working_config) = config.area_types.working {
                        return working_config.spec_file.clone();
                    }
                }
            }
        }

        crate::agent::mode::get_spec_filename_for_area(area)
    }

    fn load_topic_recursive(
        &self,
        path: &Path,
        area: String,
        area_type: DisplayType,
        spec_file: &str,
    ) -> Result<TopicNode> {
        let topic = path.file_name().unwrap().to_string_lossy().to_string();

        let mut children = vec![];
        if path.exists() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    children.push(self.load_topic_recursive(
                        &entry.path(),
                        area.clone(),
                        area_type.clone(),
                        spec_file,
                    )?);
                }
            }
        }

        let metadata = self.load_spec_metadata(path, spec_file);

        let (total, completed, in_progress) = match area_type {
            DisplayType::Roadmap | DisplayType::Build => (0, 0, 0),
            DisplayType::Working | DisplayType::Standard => {
                self.calculate_total_tasks(path, &children, &area)
            }
        };

        let status = if total > 0 && completed == total {
            "complete".to_string()
        } else if in_progress > 0 || completed > 0 {
            "in-progress".to_string()
        } else if metadata.impact.is_some() || metadata.change_type.is_some() {
            "pending".to_string()
        } else {
            "waiting".to_string()
        };

        Ok(TopicNode {
            topic,
            path: path.to_path_buf(),
            area,
            area_type,
            status,
            tasks_total: total,
            tasks_completed: completed,
            tasks_in_progress: in_progress,
            metadata,
            children,
        })
    }

    fn load_spec_metadata(&self, path: &Path, spec_file: &str) -> SpecMetadata {
        // Try the configured spec file first
        let spec_path = path.join(spec_file);
        if spec_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&spec_path) {
                if let Some(metadata) = crate::fs::spec::parse_spec_metadata(&content) {
                    return metadata;
                }
            }
        }

        // Fallback: try common spec file names
        for filename in &["spec.md", "specs.md", "spec.MD", "SPEC.MD"] {
            let fallback_path = path.join(filename);
            if fallback_path.exists() && fallback_path != spec_path {
                if let Ok(content) = std::fs::read_to_string(&fallback_path) {
                    if let Some(metadata) = crate::fs::spec::parse_spec_metadata(&content) {
                        return metadata;
                    }
                }
            }
        }

        SpecMetadata::default()
    }

    fn calculate_total_tasks(
        &self,
        path: &Path,
        children: &[TopicNode],
        area: &str,
    ) -> (usize, usize, usize) {
        let (mut total, mut completed, mut in_progress) = Self::count_tasks_in_dir(path, area);
        for child in children {
            let (c_total, c_completed, c_in_progress) =
                self.calculate_total_tasks(&child.path, &child.children, area);
            total += c_total;
            completed += c_completed;
            in_progress += c_in_progress;
        }
        (total, completed, in_progress)
    }

    fn count_tasks_in_dir(path: &Path, area: &str) -> (usize, usize, usize) {
        let task_filename = crate::agent::mode::get_task_filename_for_area(area);
        let tasks_path = path.join(&task_filename);
        if !tasks_path.exists() {
            return (0, 0, 0);
        }

        let mut total = 0;
        let mut completed = 0;
        let mut in_progress = 0;
        if let Ok(content) = std::fs::read_to_string(&tasks_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("- [") {
                    total += 1;
                    if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
                        completed += 1;
                    } else if trimmed.starts_with("- [-]") {
                        in_progress += 1;
                    }
                }
            }
        }
        (total, completed, in_progress)
    }

    pub fn update_status(&mut self) {
        match self.nav_state.clone() {
            NavState::TopicList(area) => {
                let _ = self.load_topics_for_area(&area);
            }
            NavState::NestedSpecs(path) => {
                let spec_dir = fs::spec_dir();
                if let Ok(rel) = path.strip_prefix(&spec_dir) {
                    let parts: Vec<&str> = rel.iter().filter_map(|p| p.to_str()).collect();
                    let area = parts.first().unwrap_or(&"Working").to_string();
                    if path.exists() {
                        if let Ok(new_children) = self.load_nested_topics(&path, area) {
                            self.topics = new_children;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn load_nested_topics(&self, path: &Path, area: String) -> Result<Vec<TopicNode>> {
        let area_type = self.get_area_type(&area);
        let spec_file = self.get_spec_filename_for_area(&area);
        let mut topics = vec![];
        if path.exists() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    topics.push(self.load_topic_recursive(
                        &entry.path(),
                        area.clone(),
                        area_type.clone(),
                        &spec_file,
                    )?);
                }
            }
        }
        Ok(topics)
    }
}
