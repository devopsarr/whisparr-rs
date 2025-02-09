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
pub struct SystemResource {
    #[serde(rename = "appName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub app_name: Option<Option<String>>,
    #[serde(rename = "instanceName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<Option<String>>,
    #[serde(rename = "version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub version: Option<Option<String>>,
    #[serde(rename = "buildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
    #[serde(rename = "isDebug", skip_serializing_if = "Option::is_none")]
    pub is_debug: Option<bool>,
    #[serde(rename = "isProduction", skip_serializing_if = "Option::is_none")]
    pub is_production: Option<bool>,
    #[serde(rename = "isAdmin", skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(rename = "isUserInteractive", skip_serializing_if = "Option::is_none")]
    pub is_user_interactive: Option<bool>,
    #[serde(rename = "startupPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub startup_path: Option<Option<String>>,
    #[serde(rename = "appData", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub app_data: Option<Option<String>>,
    #[serde(rename = "osName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub os_name: Option<Option<String>>,
    #[serde(rename = "osVersion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<Option<String>>,
    #[serde(rename = "isNetCore", skip_serializing_if = "Option::is_none")]
    pub is_net_core: Option<bool>,
    #[serde(rename = "isLinux", skip_serializing_if = "Option::is_none")]
    pub is_linux: Option<bool>,
    #[serde(rename = "isOsx", skip_serializing_if = "Option::is_none")]
    pub is_osx: Option<bool>,
    #[serde(rename = "isWindows", skip_serializing_if = "Option::is_none")]
    pub is_windows: Option<bool>,
    #[serde(rename = "isDocker", skip_serializing_if = "Option::is_none")]
    pub is_docker: Option<bool>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::RuntimeMode>,
    #[serde(rename = "branch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub branch: Option<Option<String>>,
    #[serde(rename = "databaseType", skip_serializing_if = "Option::is_none")]
    pub database_type: Option<models::DatabaseType>,
    #[serde(rename = "databaseVersion", skip_serializing_if = "Option::is_none")]
    pub database_version: Option<String>,
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<models::AuthenticationType>,
    #[serde(rename = "migrationVersion", skip_serializing_if = "Option::is_none")]
    pub migration_version: Option<i32>,
    #[serde(rename = "urlBase", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url_base: Option<Option<String>>,
    #[serde(rename = "runtimeVersion", skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[serde(rename = "runtimeName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub runtime_name: Option<Option<String>>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "packageVersion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub package_version: Option<Option<String>>,
    #[serde(rename = "packageAuthor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub package_author: Option<Option<String>>,
    #[serde(rename = "packageUpdateMechanism", skip_serializing_if = "Option::is_none")]
    pub package_update_mechanism: Option<models::UpdateMechanism>,
    #[serde(rename = "packageUpdateMechanismMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub package_update_mechanism_message: Option<Option<String>>,
}

impl SystemResource {
    pub fn new() -> SystemResource {
        SystemResource {
            app_name: None,
            instance_name: None,
            version: None,
            build_time: None,
            is_debug: None,
            is_production: None,
            is_admin: None,
            is_user_interactive: None,
            startup_path: None,
            app_data: None,
            os_name: None,
            os_version: None,
            is_net_core: None,
            is_linux: None,
            is_osx: None,
            is_windows: None,
            is_docker: None,
            mode: None,
            branch: None,
            database_type: None,
            database_version: None,
            authentication: None,
            migration_version: None,
            url_base: None,
            runtime_version: None,
            runtime_name: None,
            start_time: None,
            package_version: None,
            package_author: None,
            package_update_mechanism: None,
            package_update_mechanism_message: None,
        }
    }
}

