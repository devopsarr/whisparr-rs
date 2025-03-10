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
pub struct MetadataConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "certificationCountry", skip_serializing_if = "Option::is_none")]
    pub certification_country: Option<models::TmdbCountryCode>,
}

impl MetadataConfigResource {
    pub fn new() -> MetadataConfigResource {
        MetadataConfigResource {
            id: None,
            certification_country: None,
        }
    }
}

