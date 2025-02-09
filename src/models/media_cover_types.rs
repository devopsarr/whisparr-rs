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
pub enum MediaCoverTypes {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "poster")]
    Poster,
    #[serde(rename = "banner")]
    Banner,
    #[serde(rename = "fanart")]
    Fanart,
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "headshot")]
    Headshot,
    #[serde(rename = "clearlogo")]
    Clearlogo,

}

impl std::fmt::Display for MediaCoverTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Poster => write!(f, "poster"),
            Self::Banner => write!(f, "banner"),
            Self::Fanart => write!(f, "fanart"),
            Self::Screenshot => write!(f, "screenshot"),
            Self::Headshot => write!(f, "headshot"),
            Self::Clearlogo => write!(f, "clearlogo"),
        }
    }
}

impl Default for MediaCoverTypes {
    fn default() -> MediaCoverTypes {
        Self::Unknown
    }
}

