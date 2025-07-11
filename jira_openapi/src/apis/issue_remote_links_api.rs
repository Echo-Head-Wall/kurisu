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


/// struct for typed errors of method [`create_or_update_remote_issue_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrUpdateRemoteIssueLinkError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_remote_issue_link_by_global_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRemoteIssueLinkByGlobalIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_remote_issue_link_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRemoteIssueLinkByIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_remote_issue_link_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRemoteIssueLinkByIdError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_remote_issue_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRemoteIssueLinksError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    Status413(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_remote_issue_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRemoteIssueLinkError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Creates or updates a remote issue link for an issue.  If a `globalId` is provided and a remote issue link with that global ID is found it is updated. Any fields without values in the request are set to null. Otherwise, the remote issue link is created.  This operation requires [issue linking to be active](https://confluence.atlassian.com/x/yoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* and *Link issues* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn create_or_update_remote_issue_link(configuration: &configuration::Configuration, issue_id_or_key: &str, remote_issue_link_request: models::RemoteIssueLinkRequest) -> Result<models::RemoteIssueLinkIdentifies, Error<CreateOrUpdateRemoteIssueLinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_remote_issue_link_request = remote_issue_link_request;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/remotelink", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key));
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
    req_builder = req_builder.json(&p_remote_issue_link_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RemoteIssueLinkIdentifies`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RemoteIssueLinkIdentifies`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateOrUpdateRemoteIssueLinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes the remote issue link from the issue using the link's global ID. Where the global ID includes reserved URL characters these must be escaped in the request. For example, pass `system=http://www.mycompany.com/support&id=1` as `system%3Dhttp%3A%2F%2Fwww.mycompany.com%2Fsupport%26id%3D1`.  This operation requires [issue linking to be active](https://confluence.atlassian.com/x/yoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* and *Link issues* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is implemented, issue-level security permission to view the issue.
pub async fn delete_remote_issue_link_by_global_id(configuration: &configuration::Configuration, issue_id_or_key: &str, global_id: &str) -> Result<(), Error<DeleteRemoteIssueLinkByGlobalIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_global_id = global_id;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/remotelink", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[("globalId", &p_global_id.to_string())]);
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
        let entity: Option<DeleteRemoteIssueLinkByGlobalIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes a remote issue link from an issue.  This operation requires [issue linking to be active](https://confluence.atlassian.com/x/yoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects*, *Edit issues*, and *Link issues* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn delete_remote_issue_link_by_id(configuration: &configuration::Configuration, issue_id_or_key: &str, link_id: &str) -> Result<(), Error<DeleteRemoteIssueLinkByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_link_id = link_id;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/remotelink/{linkId}", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key), linkId=crate::apis::urlencode(p_link_id));
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
        let entity: Option<DeleteRemoteIssueLinkByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a remote issue link for an issue.  This operation requires [issue linking to be active](https://confluence.atlassian.com/x/yoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn get_remote_issue_link_by_id(configuration: &configuration::Configuration, issue_id_or_key: &str, link_id: &str) -> Result<models::RemoteIssueLink, Error<GetRemoteIssueLinkByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_link_id = link_id;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/remotelink/{linkId}", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key), linkId=crate::apis::urlencode(p_link_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RemoteIssueLink`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RemoteIssueLink`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRemoteIssueLinkByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the remote issue links for an issue. When a remote issue link global ID is provided the record with that global ID is returned, otherwise all remote issue links are returned. Where a global ID includes reserved URL characters these must be escaped in the request. For example, pass `system=http://www.mycompany.com/support&id=1` as `system%3Dhttp%3A%2F%2Fwww.mycompany.com%2Fsupport%26id%3D1`.  This operation requires [issue linking to be active](https://confluence.atlassian.com/x/yoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn get_remote_issue_links(configuration: &configuration::Configuration, issue_id_or_key: &str, global_id: Option<&str>) -> Result<models::RemoteIssueLink, Error<GetRemoteIssueLinksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_global_id = global_id;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/remotelink", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_global_id {
        req_builder = req_builder.query(&[("globalId", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RemoteIssueLink`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RemoteIssueLink`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetRemoteIssueLinksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Updates a remote issue link for an issue.  Note: Fields without values in the request are set to null.  This operation requires [issue linking to be active](https://confluence.atlassian.com/x/yoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* and *Link issues* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn update_remote_issue_link(configuration: &configuration::Configuration, issue_id_or_key: &str, link_id: &str, remote_issue_link_request: models::RemoteIssueLinkRequest) -> Result<serde_json::Value, Error<UpdateRemoteIssueLinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_link_id = link_id;
    let p_remote_issue_link_request = remote_issue_link_request;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/remotelink/{linkId}", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key), linkId=crate::apis::urlencode(p_link_id));
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
    req_builder = req_builder.json(&p_remote_issue_link_request);

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
        let entity: Option<UpdateRemoteIssueLinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

