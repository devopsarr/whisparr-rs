/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: b08981dee068e1ed23e4f45a0d8fe70ef7bf7703
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_import_list_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportListConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_import_list_config_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetImportListConfigByIdError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_import_list_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateImportListConfigError {
    UnknownValue(serde_json::Value),
}


pub async fn get_import_list_config(configuration: &configuration::Configuration, ) -> Result<models::ImportListConfigResource, Error<GetImportListConfigError>> {

    let uri_str = format!("{}/api/v3/config/importlist", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ImportListConfigResource`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ImportListConfigResource`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetImportListConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_import_list_config_by_id(configuration: &configuration::Configuration, id: i32) -> Result<models::ImportListConfigResource, Error<GetImportListConfigByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/api/v3/config/importlist/{id}", configuration.base_path, id=p_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ImportListConfigResource`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ImportListConfigResource`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetImportListConfigByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_import_list_config(configuration: &configuration::Configuration, id: &str, import_list_config_resource: Option<models::ImportListConfigResource>) -> Result<models::ImportListConfigResource, Error<UpdateImportListConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_import_list_config_resource = import_list_config_resource;

    let uri_str = format!("{}/api/v3/config/importlist/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };
    req_builder = req_builder.json(&p_import_list_config_resource);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ImportListConfigResource`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ImportListConfigResource`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateImportListConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

