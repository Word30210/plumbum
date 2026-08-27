use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub scope: String,
    pub version: String,
    pub tool_manager: ToolManager,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolManager {
    Rokit,
    Mise,
    None,
}

impl std::fmt::Display for ToolManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Rokit => "rokit",
            Self::Mise => "mise",
            Self::None => "none (others)",
        })
    }
}
