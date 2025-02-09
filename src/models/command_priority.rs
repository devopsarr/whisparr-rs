/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: b08981dee068e1ed23e4f45a0d8fe70ef7bf7703
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommandPriority {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,

}

impl std::fmt::Display for CommandPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::High => write!(f, "high"),
            Self::Low => write!(f, "low"),
        }
    }
}

impl Default for CommandPriority {
    fn default() -> CommandPriority {
        Self::Normal
    }
}

