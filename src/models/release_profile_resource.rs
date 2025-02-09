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
pub struct ReleaseProfileResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "required", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub required: Option<Option<serde_json::Value>>,
    #[serde(rename = "ignored", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ignored: Option<Option<serde_json::Value>>,
    #[serde(rename = "indexerId", skip_serializing_if = "Option::is_none")]
    pub indexer_id: Option<i32>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
}

impl ReleaseProfileResource {
    pub fn new() -> ReleaseProfileResource {
        ReleaseProfileResource {
            id: None,
            name: None,
            enabled: None,
            required: None,
            ignored: None,
            indexer_id: None,
            tags: None,
        }
    }
}

