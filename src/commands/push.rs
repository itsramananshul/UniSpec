use crate::fs::{ensure_dir, spec_dir, topic_path};
use anyhow::Result;
use std::fs;

pub fn run_push(topic: &str, target_area: &str, source_area: Option<&str>) -> Result<String> {
    let source_area = source_area.map(String::from).unwrap_or_else(|| {
        crate::fs::config::load_config()
            .map(|c| c.area)
            .unwrap_or_else(|_| "Working".to_string())
    });

    // Check if target area exists
    let target_area_path = spec_dir().join(target_area);
    if !target_area_path.exists() || !target_area_path.join("area.md").exists() {
        return Err(anyhow::anyhow!("❌ Area '{}' does not exist.", target_area));
    }

    if source_area == target_area {
        return Err(anyhow::anyhow!(
            "❌ Source and target areas are the same: {}",
            source_area
        ));
    }

    let src = topic_path(topic, &source_area);
    let dst = topic_path(topic, target_area);

    if !src.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            source_area
        ));
    }

    if dst.exists() {
        fs::remove_dir_all(&dst)?;
    }

    copy_dir_recursive(&src, &dst)?;
    fs::remove_dir_all(&src)?;

    Ok(format!(
        "✅ Moved topic '{}' from {} to {}",
        topic, source_area, target_area
    ))
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    ensure_dir(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_file() {
            fs::copy(&path, &dest_path)?;
        } else if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        }
    }
    Ok(())
}
