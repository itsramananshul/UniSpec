use crate::fs::{ensure_dir, topic_path, Area};
use anyhow::Result;
use std::fs;

#[allow(dead_code)]
pub fn run_pull(topic: &str, source_area_str: &str) -> Result<String> {
    let current_config = crate::fs::config::load_config()?;
    let target_area = current_config.area;
    let source_area = Area::from_str(source_area_str)?;

    let src = topic_path(topic, source_area.as_str());
    let dst = topic_path(topic, target_area.as_str());

    if !src.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            source_area
        ));
    }

    if dst.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' already exists in {}.",
            topic,
            target_area
        ));
    }

    ensure_dir(&dst)?;
    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_file() {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(format!(
        "✅ Pulled topic '{}' from {} to {}",
        topic, source_area, target_area
    ))
}
