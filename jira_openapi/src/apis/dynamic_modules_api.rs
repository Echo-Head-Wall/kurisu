/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-ccbf72d894d6b699175624f7a94244e68c9dbc6d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`dynamic_modules_resource_period_get_modules_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicModulesResourcePeriodGetModulesGetError {
    Status401(models::ErrorMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dynamic_modules_resource_period_register_modules_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicModulesResourcePeriodRegisterModulesPostError {
    Status400(models::ErrorMessage),
    Status401(models::ErrorMessage),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`dynamic_modules_resource_period_remove_modules_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicModulesResourcePeriodRemoveModulesDeleteError {
    Status401(models::ErrorMessage),
    UnknownValue(serde_json::Value),
}


/// Returns all modules registered dynamically by the calling app.  **[Permissions](#permissions) required:** Only Connect apps can make this request.
pub async fn dynamic_modules_resource_period_get_modules_get(configuration: &configuration::Configuration, ) -> Result<models::ConnectModules, Error<DynamicModulesResourcePeriodGetModulesGetError>> {

    let uri_str = format!("{}/rest/atlassian-connect/1/app/module/dynamic", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ConnectModules`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ConnectModules`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DynamicModulesResourcePeriodGetModulesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Registers a list of modules.  **[Permissions](#permissions) required:** Only Connect apps can make this request.
pub async fn dynamic_modules_resource_period_register_modules_post(configuration: &configuration::Configuration, connect_modules: models::ConnectModules) -> Result<(), Error<DynamicModulesResourcePeriodRegisterModulesPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_connect_modules = connect_modules;

    let uri_str = format!("{}/rest/atlassian-connect/1/app/module/dynamic", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_connect_modules);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DynamicModulesResourcePeriodRegisterModulesPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Remove all or a list of modules registered by the calling app.  **[Permissions](#permissions) required:** Only Connect apps can make this request.
pub async fn dynamic_modules_resource_period_remove_modules_delete(configuration: &configuration::Configuration, module_key: Option<Vec<String>>) -> Result<(), Error<DynamicModulesResourcePeriodRemoveModulesDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_module_key = module_key;

    let uri_str = format!("{}/rest/atlassian-connect/1/app/module/dynamic", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_module_key {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("moduleKey".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("moduleKey", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DynamicModulesResourcePeriodRemoveModulesDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

