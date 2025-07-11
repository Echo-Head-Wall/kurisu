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


/// struct for typed errors of method [`add_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddScreenTabError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScreenTabError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_screen_tabs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllScreenTabsError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_bulk_screen_tabs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBulkScreenTabsError {
    Status400(),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`move_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoveScreenTabError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rename_screen_tab`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameScreenTabError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Creates a tab for a screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn add_screen_tab(configuration: &configuration::Configuration, screen_id: i64, screenable_tab: models::ScreenableTab) -> Result<models::ScreenableTab, Error<AddScreenTabError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_screen_id = screen_id;
    let p_screenable_tab = screenable_tab;

    let uri_str = format!("{}/rest/api/3/screens/{screenId}/tabs", configuration.base_path, screenId=p_screen_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_screenable_tab);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScreenableTab`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScreenableTab`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddScreenTabError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_screen_tab(configuration: &configuration::Configuration, screen_id: i64, tab_id: i64) -> Result<(), Error<DeleteScreenTabError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_screen_id = screen_id;
    let p_tab_id = tab_id;

    let uri_str = format!("{}/rest/api/3/screens/{screenId}/tabs/{tabId}", configuration.base_path, screenId=p_screen_id, tabId=p_tab_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteScreenTabError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the list of tabs for a screen.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.
pub async fn get_all_screen_tabs(configuration: &configuration::Configuration, screen_id: i64, project_key: Option<&str>) -> Result<Vec<models::ScreenableTab>, Error<GetAllScreenTabsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_screen_id = screen_id;
    let p_project_key = project_key;

    let uri_str = format!("{}/rest/api/3/screens/{screenId}/tabs", configuration.base_path, screenId=p_screen_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_project_key {
        req_builder = req_builder.query(&[("projectKey", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::ScreenableTab&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::ScreenableTab&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAllScreenTabsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the list of tabs for a bulk of screens.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn get_bulk_screen_tabs(configuration: &configuration::Configuration, screen_id: Option<Vec<i64>>, tab_id: Option<Vec<i64>>, start_at: Option<i64>, max_result: Option<i32>) -> Result<(), Error<GetBulkScreenTabsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_screen_id = screen_id;
    let p_tab_id = tab_id;
    let p_start_at = start_at;
    let p_max_result = max_result;

    let uri_str = format!("{}/rest/api/3/screens/tabs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_screen_id {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("screenId".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("screenId", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_tab_id {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("tabId".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("tabId", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_start_at {
        req_builder = req_builder.query(&[("startAt", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_max_result {
        req_builder = req_builder.query(&[("maxResult", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBulkScreenTabsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Moves a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn move_screen_tab(configuration: &configuration::Configuration, screen_id: i64, tab_id: i64, pos: i32) -> Result<serde_json::Value, Error<MoveScreenTabError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_screen_id = screen_id;
    let p_tab_id = tab_id;
    let p_pos = pos;

    let uri_str = format!("{}/rest/api/3/screens/{screenId}/tabs/{tabId}/move/{pos}", configuration.base_path, screenId=p_screen_id, tabId=p_tab_id, pos=p_pos);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MoveScreenTabError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Updates the name of a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn rename_screen_tab(configuration: &configuration::Configuration, screen_id: i64, tab_id: i64, screenable_tab: models::ScreenableTab) -> Result<models::ScreenableTab, Error<RenameScreenTabError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_screen_id = screen_id;
    let p_tab_id = tab_id;
    let p_screenable_tab = screenable_tab;

    let uri_str = format!("{}/rest/api/3/screens/{screenId}/tabs/{tabId}", configuration.base_path, screenId=p_screen_id, tabId=p_tab_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_screenable_tab);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScreenableTab`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScreenableTab`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RenameScreenTabError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

