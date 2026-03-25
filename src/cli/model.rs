// src/cli/model.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub enum Area {
    Staging,
    Working,
    Build,
}

impl Area {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Area::Staging => "Staging",
            Area::Working => "Working",
            Area::Build => "Build",
        }
    }

    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        match s.to_lowercase().as_str() {
            "staging" => Ok(Area::Staging),
            "working" => Ok(Area::Working),
            "build" => Ok(Area::Build),
            _ => Err(anyhow::anyhow!(
                "Invalid area: '{}'. Use: Staging, Working, Build",
                s
            )),
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
