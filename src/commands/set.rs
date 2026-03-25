use anyhow::Result;

pub fn run_set(area_str: &str) -> Result<()> {
    let mut config = crate::agent::load_agent_config()?;
    config.default_area = Some(area_str.to_string());
    crate::agent::save_agent_config(&config)?;
    println!("✅ Default area set to: {}", area_str);
    Ok(())
}
