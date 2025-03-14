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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileFormatItemResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

impl ProfileFormatItemResource {
    pub fn new() -> ProfileFormatItemResource {
        ProfileFormatItemResource {
            id: None,
            format: None,
            name: None,
            score: None,
        }
    }
}

