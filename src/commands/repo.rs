// src/commands/repo.rs
// UniSpec Repository - Install modes, connectors, and workflows from the community

use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_REPO_URL: &str = "https://github.com/uwzis/unispec-modes";

#[derive(Debug, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub description: String,
    #[serde(rename = "repo_url")]
    pub repo_url: String,
    pub license: String,
    pub category: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RepoConfig {
    #[serde(rename = "registry_url")]
    pub registry_url: String,
    #[serde(rename = "package")]
    pub packages: std::collections::HashMap<String, Package>,
}

pub fn get_modes_dir(global: bool) -> PathBuf {
    if global {
        crate::fs::system_install_dir().join(".agent").join("modes")
    } else {
        crate::fs::agent_dir().join("modes")
    }
}

pub fn get_global_modes_dir() -> PathBuf {
    crate::fs::global_modes_dir()
}

fn get_repo_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("unispec")
        .join("repo")
}

fn fetch_repo(repo_url: &str) -> Result<PathBuf> {
    let cache_dir = get_repo_cache_dir();
    fs::create_dir_all(&cache_dir)?;

    let repo_name = repo_url
        .trim_end_matches(".git")
        .split('/')
        .last()
        .unwrap_or("repo");

    let repo_path = cache_dir.join(repo_name);

    if repo_path.exists() {
        println!("Updating repository...");
        Command::new("git")
            .args(["pull", "origin", "main"])
            .current_dir(&repo_path)
            .output()?;
    } else {
        println!("Cloning repository...");
        let output = Command::new("git")
            .args(["clone", repo_url, repo_path.to_str().unwrap()])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to clone repository"));
        }
    }

    Ok(repo_path)
}

fn load_repo_config(repo_path: &Path) -> Result<RepoConfig> {
    let config_path = repo_path.join("unispec-repo.toml");
    let content = fs::read_to_string(&config_path)?;
    let config: RepoConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn list_packages(repo_url: Option<&str>) -> Result<()> {
    let url = repo_url.unwrap_or(DEFAULT_REPO_URL);
    let repo_path = fetch_repo(url)?;
    let config = load_repo_config(&repo_path)?;

    println!("\n📦 Available packages from {}\n", config.registry_url);
    println!("{:<25} {:<50} {:<10}", "NAME", "DESCRIPTION", "CATEGORY");
    println!("{}", "-".repeat(90));

    for (key, pkg) in &config.packages {
        println!(
            "{:<25} {:<50} {:<10}",
            key,
            &pkg.description[..pkg.description.len().min(48)],
            pkg.category
        );
    }

    println!();
    Ok(())
}

pub fn search_packages(query: &str, repo_url: Option<&str>) -> Result<()> {
    let url = repo_url.unwrap_or(DEFAULT_REPO_URL);
    let repo_path = fetch_repo(url)?;
    let config = load_repo_config(&repo_path)?;

    let query_lower = query.to_lowercase();
    let mut found = false;

    for (key, pkg) in &config.packages {
        if key.to_lowercase().contains(&query_lower)
            || pkg.name.to_lowercase().contains(&query_lower)
            || pkg.description.to_lowercase().contains(&query_lower)
        {
            found = true;
            println!("\n📦 {}", pkg.name);
            println!("   Key: {}", key);
            println!("   Description: {}", pkg.description);
            println!("   Category: {}", pkg.category);
            println!("   License: {}", pkg.license);
            println!("   URL: {}", pkg.repo_url);
            if let Some(tags) = &pkg.tags {
                println!("   Tags: {}", tags.join(", "));
            }
        }
    }

    if !found {
        println!("No packages found matching '{}'", query);
    }

    Ok(())
}

fn check_license(license: &str) -> bool {
    let allowed_licenses = [
        "mit",
        "apache-2.0",
        "apache2",
        "bsd-3-clause",
        "bsd3",
        "gpl-3.0",
        "gpl3",
        "lgpl-3.0",
        "lgpl3",
        "unlicense",
        "mpl-2.0",
    ];
    allowed_licenses.contains(
        &license
            .to_lowercase()
            .replace('-', "")
            .replace(' ', "")
            .as_str(),
    )
}

pub fn install_package(package_name: &str, global: bool, repo_url: Option<&str>) -> Result<()> {
    let url = repo_url.unwrap_or(DEFAULT_REPO_URL);
    let repo_path = fetch_repo(url)?;
    let config = load_repo_config(&repo_path)?;

    let pkg = config.packages.get(package_name)
        .ok_or_else(|| anyhow!("Package '{}' not found in repository. Run 'unispec repo list' to see available packages.", package_name))?;

    println!("\n📦 Installing {}...", pkg.name);
    println!("   Description: {}", pkg.description);
    println!("   License: {}", pkg.license);

    if !check_license(&pkg.license) {
        println!("\n⚠️  WARNING: This package has license '{}'.", pkg.license);
        println!("   Make sure you understand and agree to the license before using.");
        println!("   Continue? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Aborted.");
            return Ok(());
        }
    }

    println!("\n📥 Cloning from {}...", pkg.repo_url);
    let pkg_repo_path = fetch_repo(&pkg.repo_url)?;

    let modes_dir = if global {
        get_modes_dir(true)
    } else {
        get_modes_dir(false)
    };

    fs::create_dir_all(&modes_dir)?;

    let source_modes = pkg_repo_path.join("modes");
    if source_modes.exists() && source_modes.is_dir() {
        for entry in fs::read_dir(&source_modes)? {
            let entry = entry?;
            let src = entry.path();
            let dst = modes_dir.join(entry.file_name());

            if dst.exists() {
                println!(
                    "   ⚠️  Mode '{}' already exists, skipping.",
                    entry.file_name().to_string_lossy()
                );
                continue;
            }

            println!("   📁 Copying {}...", entry.file_name().to_string_lossy());
            copy_dir_recursive(&src, &dst)?;
        }
    }

    let source_connectors = pkg_repo_path.join("connectors");
    if source_connectors.exists() && source_connectors.is_dir() {
        let connectors_dir = if global {
            crate::fs::system_install_dir()
                .join(".agent")
                .join("connectors")
        } else {
            crate::fs::agent_dir().join("connectors")
        };
        fs::create_dir_all(&connectors_dir)?;

        for entry in fs::read_dir(&source_connectors)? {
            let entry = entry?;
            let src = entry.path();
            let dst = connectors_dir.join(entry.file_name());

            if dst.exists() {
                println!(
                    "   ⚠️  Connector '{}' already exists, skipping.",
                    entry.file_name().to_string_lossy()
                );
                continue;
            }

            println!(
                "   🔌 Copying connector {}...",
                entry.file_name().to_string_lossy()
            );
            copy_dir_recursive(&src, &dst)?;
        }
    }

    println!("\n✅ Successfully installed {}!", pkg.name);
    if global {
        println!("   Installed to: {}", modes_dir.display());
    } else {
        println!(
            "   Installed to: {}/.agent/modes/",
            std::env::current_dir()?.display()
        );
    }

    Ok(())
}

pub fn install_from_url(url: &str, global: bool) -> Result<()> {
    println!("\n📥 Installing from URL: {}", url);

    let pkg_repo_path = fetch_repo(url)?;

    let modes_dir = if global {
        get_modes_dir(true)
    } else {
        get_modes_dir(false)
    };

    fs::create_dir_all(&modes_dir)?;

    let source_modes = pkg_repo_path.join("modes");
    if source_modes.exists() && source_modes.is_dir() {
        for entry in fs::read_dir(&source_modes)? {
            let entry = entry?;
            let src = entry.path();
            let dst = modes_dir.join(entry.file_name());

            if dst.exists() {
                println!(
                    "   ⚠️  Mode '{}' already exists, skipping.",
                    entry.file_name().to_string_lossy()
                );
                continue;
            }

            println!("   📁 Copying {}...", entry.file_name().to_string_lossy());
            copy_dir_recursive(&src, &dst)?;
        }
        println!("✅ Modes installed!");
    } else {
        return Err(anyhow!(
            "No 'modes' folder found in repository. Make sure this is a valid UniSpec package."
        ));
    }

    println!("\n✅ Successfully installed from {}!", url);
    Ok(())
}

pub fn remove_package(package_name: &str, global: bool) -> Result<()> {
    let modes_dir = if global {
        get_modes_dir(true)
    } else {
        get_modes_dir(false)
    };

    let package_path = modes_dir.join(package_name);

    if !package_path.exists() {
        return Err(anyhow!("Package '{}' is not installed.", package_name));
    }

    println!("🗑️  Removing {}...", package_name);
    fs::remove_dir_all(&package_path)?;

    println!("✅ Successfully removed {}.", package_name);
    Ok(())
}

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

pub fn list_installed(global: bool) -> Result<()> {
    let modes_dir = if global {
        get_modes_dir(true)
    } else {
        get_modes_dir(false)
    };

    println!("\n📦 Installed packages:\n");

    if !modes_dir.exists() || fs::read_dir(&modes_dir)?.count() == 0 {
        println!("   No packages installed.");
        if !global {
            println!("   Run 'unispec repo install <package>' to install packages.");
        }
        println!();
        return Ok(());
    }

    for entry in fs::read_dir(&modes_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let mode_file = path.join("mode.toml");
            if mode_file.exists() {
                if let Ok(content) = fs::read_to_string(&mode_file) {
                    if let Ok(toml) = content.parse::<toml::Value>() {
                        let name = toml
                            .get("mode")
                            .and_then(|m| m.get("display_name"))
                            .and_then(|n| n.as_str())
                            .unwrap_or(file_name.as_str());
                        println!("   - {}", name);
                        continue;
                    }
                }
            }
            println!("   - {}", file_name);
        }
    }

    println!();
    Ok(())
}
