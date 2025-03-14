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
pub struct RemotePathMappingResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "host", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub host: Option<Option<String>>,
    #[serde(rename = "remotePath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remote_path: Option<Option<String>>,
    #[serde(rename = "localPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub local_path: Option<Option<String>>,
}

impl RemotePathMappingResource {
    pub fn new() -> RemotePathMappingResource {
        RemotePathMappingResource {
            id: None,
            host: None,
            remote_path: None,
            local_path: None,
        }
    }
}

