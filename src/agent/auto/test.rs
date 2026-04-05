// src/agent/auto/test.rs
// Test agent - builds project and runs scripts to verify it compiles/runs
// CLI only - not part of auto mode

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::agent::load_agent_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub pre_build_script: Option<String>,
    pub post_build_script: Option<String>,
    pub build_command: String,
    pub test_command: String,
    pub timeout: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            pre_build_script: None,
            post_build_script: None,
            build_command: "cargo build".to_string(),
            test_command: "cargo test".to_string(),
            timeout: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub topic: String,
    pub passed: bool,
    pub build_output: String,
    pub build_success: bool,
    pub test_output: String,
    pub test_success: bool,
    pub pre_script_output: Option<String>,
    pub post_script_output: Option<String>,
    pub duration_seconds: u64,
    pub errors: Vec<String>,
}

fn load_test_config() -> Result<TestConfig> {
    let config = load_agent_config()?;

    let test_config = TestConfig {
        pre_build_script: config.settings.custom.get("test_pre_script").cloned(),
        post_build_script: config.settings.custom.get("test_post_script").cloned(),
        build_command: config
            .settings
            .custom
            .get("test_build_command")
            .cloned()
            .unwrap_or_else(|| "cargo build".to_string()),
        test_command: config
            .settings
            .custom
            .get("test_command")
            .cloned()
            .unwrap_or_else(|| "cargo test".to_string()),
        timeout: config
            .settings
            .custom
            .get("test_timeout")
            .and_then(|s| s.parse().ok())
            .unwrap_or(300),
    };

    Ok(test_config)
}

fn run_script(script: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", script]).output()?
    } else {
        Command::new("sh").args(["-c", script]).output()?
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(anyhow::anyhow!("Script failed: {}\n{}", stdout, stderr));
    }

    Ok(format!("{}\n{}", stdout, stderr))
}

fn run_build(command: &str) -> Result<(bool, String)> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Ok((false, "Empty build command".to_string()));
    }

    let (program, args) = parts.split_at(1);
    let output = Command::new(program[0]).args(args).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}\n{}", stdout, stderr);

    Ok((output.status.success(), combined))
}

pub fn run_auto_test(
    topic: Option<&str>,
    pre_script: Option<&str>,
    post_script: Option<&str>,
) -> Result<TestResult> {
    let start = std::time::Instant::now();

    let config = load_test_config()?;

    let topic_name = topic.unwrap_or("unknown").to_string();

    println!("Running test for topic: {}", topic_name);
    println!("Build command: {}", config.build_command);
    println!("Test command: {}", config.test_command);

    let mut pre_output: Option<String> = None;
    let mut post_output: Option<String> = None;
    let mut errors = Vec::new();

    if let Some(ref script) = pre_script.or(config.pre_build_script.as_ref().map(|s| s.as_str())) {
        println!("Running pre-build script: {}", script);
        match run_script(script) {
            Ok(output) => {
                println!("Pre-script succeeded");
                pre_output = Some(output);
            }
            Err(e) => {
                let err_msg = format!("Pre-script failed: {}", e);
                println!("{}", err_msg);
                errors.push(err_msg);
            }
        }
    }

    println!("Running build: {}", config.build_command);
    let (build_success, build_output) = run_build(&config.build_command)?;
    println!(
        "Build: {}",
        if build_success { "SUCCESS" } else { "FAILED" }
    );

    if !build_success {
        errors.push(format!("Build failed: {}", build_output));
    }

    if build_success {
        println!("Running test: {}", config.test_command);
        let (test_success, test_output) = run_build(&config.test_command)?;
        println!("Test: {}", if test_success { "SUCCESS" } else { "FAILED" });

        if !test_success {
            errors.push(format!("Tests failed: {}", test_output));
        }

        if let Some(ref script) =
            post_script.or(config.post_build_script.as_ref().map(|s| s.as_str()))
        {
            println!("Running post-test script: {}", script);
            match run_script(script) {
                Ok(output) => {
                    println!("Post-script succeeded");
                    post_output = Some(output);
                }
                Err(e) => {
                    let err_msg = format!("Post-script failed: {}", e);
                    println!("{}", err_msg);
                    errors.push(err_msg);
                }
            }
        }

        Ok(TestResult {
            topic: topic_name,
            passed: build_success && errors.is_empty(),
            build_output,
            build_success,
            test_output: if build_success {
                test_output
            } else {
                String::new()
            },
            test_success: build_success,
            pre_script_output: pre_output,
            post_script_output: post_output,
            duration_seconds: start.elapsed().as_secs(),
            errors,
        })
    } else {
        Ok(TestResult {
            topic: topic_name,
            passed: false,
            build_output,
            build_success: false,
            test_output: String::new(),
            test_success: false,
            pre_script_output: pre_output,
            post_script_output: None,
            duration_seconds: start.elapsed().as_secs(),
            errors,
        })
    }
}

pub fn test_build_only() -> Result<TestResult> {
    let config = load_test_config()?;

    let start = std::time::Instant::now();

    println!("Running build only: {}", config.build_command);
    let (build_success, build_output) = run_build(&config.build_command)?;

    Ok(TestResult {
        topic: "build_only".to_string(),
        passed: build_success,
        build_output: build_output.clone(),
        build_success,
        test_output: String::new(),
        test_success: true,
        pre_script_output: None,
        post_script_output: None,
        duration_seconds: start.elapsed().as_secs(),
        errors: if !build_success {
            vec![format!("Build failed: {}", build_output)]
        } else {
            vec![]
        },
    })
}

pub fn test_run_connectors() -> Result<TestResult> {
    let start = std::time::Instant::now();
    let mut errors = Vec::new();
    let mut build_output = String::new();
    let mut test_output = String::new();
    let mut build_success = false;

    let connectors = crate::agent::connector::run_list();

    match run_build("cargo build") {
        Ok((success, output)) => {
            build_success = success;
            build_output = output;
        }
        Err(e) => {
            errors.push(format!("Build error: {}", e));
        }
    }

    if build_success {
        match run_build("cargo test") {
            Ok((success, output)) => {
                test_output = output;
                if !success {
                    errors.push("Tests failed".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("Test error: {}", e));
            }
        }
    }

    Ok(TestResult {
        topic: "connectors".to_string(),
        passed: build_success && errors.is_empty(),
        build_output,
        build_success,
        test_output,
        test_success: errors.is_empty(),
        pre_script_output: None,
        post_script_output: None,
        duration_seconds: start.elapsed().as_secs(),
        errors,
    })
}
