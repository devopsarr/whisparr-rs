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
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_quality_definition_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetQualityDefinitionByIdError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_quality_definition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListQualityDefinitionError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_quality_definition_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutQualityDefinitionUpdateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_quality_definition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateQualityDefinitionError {
    UnknownValue(serde_json::Value),
}


pub async fn get_quality_definition_by_id(configuration: &configuration::Configuration, id: i32) -> Result<models::QualityDefinitionResource, Error<GetQualityDefinitionByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/api/v3/qualitydefinition/{id}", configuration.base_path, id=p_id);
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

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetQualityDefinitionByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_quality_definition(configuration: &configuration::Configuration, ) -> Result<Vec<models::QualityDefinitionResource>, Error<ListQualityDefinitionError>> {

    let uri_str = format!("{}/api/v3/qualitydefinition", configuration.base_path);
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

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ListQualityDefinitionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn put_quality_definition_update(configuration: &configuration::Configuration, quality_definition_resource: Option<Vec<models::QualityDefinitionResource>>) -> Result<(), Error<PutQualityDefinitionUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_quality_definition_resource = quality_definition_resource;

    let uri_str = format!("{}/api/v3/qualitydefinition/update", configuration.base_path);
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
    req_builder = req_builder.json(&p_quality_definition_resource);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PutQualityDefinitionUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_quality_definition(configuration: &configuration::Configuration, id: &str, quality_definition_resource: Option<models::QualityDefinitionResource>) -> Result<models::QualityDefinitionResource, Error<UpdateQualityDefinitionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_quality_definition_resource = quality_definition_resource;

    let uri_str = format!("{}/api/v3/qualitydefinition/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
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
    req_builder = req_builder.json(&p_quality_definition_resource);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateQualityDefinitionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

