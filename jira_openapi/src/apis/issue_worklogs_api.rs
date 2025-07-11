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


/// struct for typed errors of method [`add_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddWorklogError {
    Status400(),
    Status401(),
    Status404(),
    Status413(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_delete_worklogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkDeleteWorklogsError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bulk_move_worklogs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkMoveWorklogsError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorklogError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ids_of_worklogs_deleted_since`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIdsOfWorklogsDeletedSinceError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ids_of_worklogs_modified_since`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIdsOfWorklogsModifiedSinceError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_issue_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIssueWorklogError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorklogError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_worklogs_for_ids`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorklogsForIdsError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_worklog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorklogError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Adds a worklog to an issue.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* and *Work on issues* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub async fn add_worklog(configuration: &configuration::Configuration, issue_id_or_key: &str, worklog: models::Worklog, notify_users: Option<bool>, adjust_estimate: Option<&str>, new_estimate: Option<&str>, reduce_by: Option<&str>, expand: Option<&str>, override_editable_flag: Option<bool>) -> Result<models::Worklog, Error<AddWorklogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_worklog = worklog;
    let p_notify_users = notify_users;
    let p_adjust_estimate = adjust_estimate;
    let p_new_estimate = new_estimate;
    let p_reduce_by = reduce_by;
    let p_expand = expand;
    let p_override_editable_flag = override_editable_flag;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/worklog", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_notify_users {
        req_builder = req_builder.query(&[("notifyUsers", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_adjust_estimate {
        req_builder = req_builder.query(&[("adjustEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_new_estimate {
        req_builder = req_builder.query(&[("newEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_reduce_by {
        req_builder = req_builder.query(&[("reduceBy", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_expand {
        req_builder = req_builder.query(&[("expand", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_override_editable_flag {
        req_builder = req_builder.query(&[("overrideEditableFlag", &param_value.to_string())]);
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
    req_builder = req_builder.json(&p_worklog);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Worklog`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Worklog`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddWorklogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes a list of worklogs from an issue. This is an experimental API with limitations:   *  You can't delete more than 5000 worklogs at once.  *  No notifications will be sent for deleted worklogs.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any worklog.  *  If any worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn bulk_delete_worklogs(configuration: &configuration::Configuration, issue_id_or_key: &str, worklog_ids_request_bean: models::WorklogIdsRequestBean, adjust_estimate: Option<&str>, override_editable_flag: Option<bool>) -> Result<(), Error<BulkDeleteWorklogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_worklog_ids_request_bean = worklog_ids_request_bean;
    let p_adjust_estimate = adjust_estimate;
    let p_override_editable_flag = override_editable_flag;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/worklog", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_adjust_estimate {
        req_builder = req_builder.query(&[("adjustEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_override_editable_flag {
        req_builder = req_builder.query(&[("overrideEditableFlag", &param_value.to_string())]);
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
    req_builder = req_builder.json(&p_worklog_ids_request_bean);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkDeleteWorklogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Moves a list of worklogs from one issue to another. This is an experimental API with several limitations:   *  You can't move more than 5000 worklogs at once.  *  You can't move worklogs containing an attachment.  *  You can't move worklogs restricted by project roles.  *  No notifications will be sent for moved worklogs.  *  No webhooks or events will be sent for moved worklogs.  *  No issue history will be recorded for moved worklogs.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the projects containing the source and destination issues.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all worklogs*[ and *Edit all worklogs*](https://confluence.atlassian.com/x/yodKLg)[project permission](https://confluence.atlassian.com/x/yodKLg)  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn bulk_move_worklogs(configuration: &configuration::Configuration, issue_id_or_key: &str, worklogs_move_request_bean: models::WorklogsMoveRequestBean, adjust_estimate: Option<&str>, override_editable_flag: Option<bool>) -> Result<(), Error<BulkMoveWorklogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_worklogs_move_request_bean = worklogs_move_request_bean;
    let p_adjust_estimate = adjust_estimate;
    let p_override_editable_flag = override_editable_flag;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/worklog/move", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_adjust_estimate {
        req_builder = req_builder.query(&[("adjustEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_override_editable_flag {
        req_builder = req_builder.query(&[("overrideEditableFlag", &param_value.to_string())]);
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
    req_builder = req_builder.json(&p_worklogs_move_request_bean);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<BulkMoveWorklogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes a worklog from an issue.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Delete all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any worklog or *Delete own worklogs* to delete worklogs created by the user,  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn delete_worklog(configuration: &configuration::Configuration, issue_id_or_key: &str, id: &str, notify_users: Option<bool>, adjust_estimate: Option<&str>, new_estimate: Option<&str>, increase_by: Option<&str>, override_editable_flag: Option<bool>) -> Result<(), Error<DeleteWorklogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_id = id;
    let p_notify_users = notify_users;
    let p_adjust_estimate = adjust_estimate;
    let p_new_estimate = new_estimate;
    let p_increase_by = increase_by;
    let p_override_editable_flag = override_editable_flag;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/worklog/{id}", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key), id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_notify_users {
        req_builder = req_builder.query(&[("notifyUsers", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_adjust_estimate {
        req_builder = req_builder.query(&[("adjustEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_new_estimate {
        req_builder = req_builder.query(&[("newEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_increase_by {
        req_builder = req_builder.query(&[("increaseBy", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_override_editable_flag {
        req_builder = req_builder.query(&[("overrideEditableFlag", &param_value.to_string())]);
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
        let entity: Option<DeleteWorklogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of IDs and delete timestamps for worklogs deleted after a date and time.  This resource is paginated, with a limit of 1000 worklogs per page. Each page lists worklogs from oldest to youngest. If the number of items in the date range exceeds 1000, `until` indicates the timestamp of the youngest item on the page. Also, `nextPage` provides the URL for the next page of worklogs. The `lastPage` parameter is set to true on the last page of worklogs.  This resource does not return worklogs deleted during the minute preceding the request.  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn get_ids_of_worklogs_deleted_since(configuration: &configuration::Configuration, since: Option<i64>) -> Result<models::ChangedWorklogs, Error<GetIdsOfWorklogsDeletedSinceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_since = since;

    let uri_str = format!("{}/rest/api/3/worklog/deleted", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ChangedWorklogs`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ChangedWorklogs`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIdsOfWorklogsDeletedSinceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of IDs and update timestamps for worklogs updated after a date and time.  This resource is paginated, with a limit of 1000 worklogs per page. Each page lists worklogs from oldest to youngest. If the number of items in the date range exceeds 1000, `until` indicates the timestamp of the youngest item on the page. Also, `nextPage` provides the URL for the next page of worklogs. The `lastPage` parameter is set to true on the last page of worklogs.  This resource does not return worklogs updated during the minute preceding the request.  **[Permissions](#permissions) required:** Permission to access Jira, however, worklogs are only returned where either of the following is true:   *  the worklog is set as *Viewable by All Users*.  *  the user is a member of a project role or group with permission to view the worklog.
pub async fn get_ids_of_worklogs_modified_since(configuration: &configuration::Configuration, since: Option<i64>, expand: Option<&str>) -> Result<models::ChangedWorklogs, Error<GetIdsOfWorklogsModifiedSinceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_since = since;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/worklog/updated", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_expand {
        req_builder = req_builder.query(&[("expand", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ChangedWorklogs`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ChangedWorklogs`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIdsOfWorklogsModifiedSinceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns worklogs for an issue (ordered by created time), starting from the oldest worklog or from the worklog started on or after a date and time.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** Workloads are only returned where the user has:   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn get_issue_worklog(configuration: &configuration::Configuration, issue_id_or_key: &str, start_at: Option<i64>, max_results: Option<i32>, started_after: Option<i64>, started_before: Option<i64>, expand: Option<&str>) -> Result<models::PageOfWorklogs, Error<GetIssueWorklogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_start_at = start_at;
    let p_max_results = max_results;
    let p_started_after = started_after;
    let p_started_before = started_before;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/worklog", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_start_at {
        req_builder = req_builder.query(&[("startAt", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_max_results {
        req_builder = req_builder.query(&[("maxResults", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_started_after {
        req_builder = req_builder.query(&[("startedAfter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_started_before {
        req_builder = req_builder.query(&[("startedBefore", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_expand {
        req_builder = req_builder.query(&[("expand", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PageOfWorklogs`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PageOfWorklogs`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetIssueWorklogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a worklog.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn get_worklog(configuration: &configuration::Configuration, issue_id_or_key: &str, id: &str, expand: Option<&str>) -> Result<models::Worklog, Error<GetWorklogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_id = id;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/worklog/{id}", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key), id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_expand {
        req_builder = req_builder.query(&[("expand", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Worklog`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Worklog`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWorklogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns worklog details for a list of worklog IDs.  The returned list of worklogs is limited to 1000 items.  **[Permissions](#permissions) required:** Permission to access Jira, however, worklogs are only returned where either of the following is true:   *  the worklog is set as *Viewable by All Users*.  *  the user is a member of a project role or group with permission to view the worklog.
pub async fn get_worklogs_for_ids(configuration: &configuration::Configuration, worklog_ids_request_bean: models::WorklogIdsRequestBean, expand: Option<&str>) -> Result<Vec<models::Worklog>, Error<GetWorklogsForIdsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_worklog_ids_request_bean = worklog_ids_request_bean;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/worklog/list", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_expand {
        req_builder = req_builder.query(&[("expand", &param_value.to_string())]);
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
    req_builder = req_builder.json(&p_worklog_ids_request_bean);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::Worklog&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::Worklog&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWorklogsForIdsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Updates a worklog.  Time tracking must be enabled in Jira, otherwise this operation returns an error. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.  *  *Edit all worklogs*[ project permission](https://confluence.atlassian.com/x/yodKLg) to update any worklog or *Edit own worklogs* to update worklogs created by the user.  *  If the worklog has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub async fn update_worklog(configuration: &configuration::Configuration, issue_id_or_key: &str, id: &str, worklog: models::Worklog, notify_users: Option<bool>, adjust_estimate: Option<&str>, new_estimate: Option<&str>, expand: Option<&str>, override_editable_flag: Option<bool>) -> Result<models::Worklog, Error<UpdateWorklogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_issue_id_or_key = issue_id_or_key;
    let p_id = id;
    let p_worklog = worklog;
    let p_notify_users = notify_users;
    let p_adjust_estimate = adjust_estimate;
    let p_new_estimate = new_estimate;
    let p_expand = expand;
    let p_override_editable_flag = override_editable_flag;

    let uri_str = format!("{}/rest/api/3/issue/{issueIdOrKey}/worklog/{id}", configuration.base_path, issueIdOrKey=crate::apis::urlencode(p_issue_id_or_key), id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref param_value) = p_notify_users {
        req_builder = req_builder.query(&[("notifyUsers", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_adjust_estimate {
        req_builder = req_builder.query(&[("adjustEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_new_estimate {
        req_builder = req_builder.query(&[("newEstimate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_expand {
        req_builder = req_builder.query(&[("expand", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_override_editable_flag {
        req_builder = req_builder.query(&[("overrideEditableFlag", &param_value.to_string())]);
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
    req_builder = req_builder.json(&p_worklog);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Worklog`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Worklog`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateWorklogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

