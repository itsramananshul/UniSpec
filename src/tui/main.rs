// src/tui/main.rs
use crate::fs::config;
use crate::tui::app::App;
use anyhow::Result;

pub fn main() -> Result<()> {
    let mut app = App::new()?;
    app.platypus_enabled = config::get_paddy_enabled().unwrap_or(true);
    app.run()?;
    Ok(())
}
