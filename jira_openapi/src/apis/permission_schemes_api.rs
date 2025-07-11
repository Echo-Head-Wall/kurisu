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


/// struct for typed errors of method [`create_permission_grant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePermissionGrantError {
    Status400(),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_permission_scheme`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePermissionSchemeError {
    Status400(),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_permission_scheme`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePermissionSchemeError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_permission_scheme_entity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePermissionSchemeEntityError {
    Status400(),
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_permission_schemes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllPermissionSchemesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_permission_scheme`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionSchemeError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_permission_scheme_grant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionSchemeGrantError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_permission_scheme_grants`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionSchemeGrantsError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_permission_scheme`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePermissionSchemeError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Creates a permission grant in a permission scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn create_permission_grant(configuration: &configuration::Configuration, scheme_id: i64, permission_grant: models::PermissionGrant, expand: Option<&str>) -> Result<models::PermissionGrant, Error<CreatePermissionGrantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_scheme_id = scheme_id;
    let p_permission_grant = permission_grant;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/permissionscheme/{schemeId}/permission", configuration.base_path, schemeId=p_scheme_id);
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
    req_builder = req_builder.json(&p_permission_grant);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PermissionGrant`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PermissionGrant`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreatePermissionGrantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Creates a new permission scheme. You can create a permission scheme with or without defining a set of permission grants.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn create_permission_scheme(configuration: &configuration::Configuration, permission_scheme: models::PermissionScheme, expand: Option<&str>) -> Result<models::PermissionScheme, Error<CreatePermissionSchemeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_permission_scheme = permission_scheme;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/permissionscheme", configuration.base_path);
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
    req_builder = req_builder.json(&p_permission_scheme);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PermissionScheme`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PermissionScheme`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreatePermissionSchemeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes a permission scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_permission_scheme(configuration: &configuration::Configuration, scheme_id: i64) -> Result<(), Error<DeletePermissionSchemeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_scheme_id = scheme_id;

    let uri_str = format!("{}/rest/api/3/permissionscheme/{schemeId}", configuration.base_path, schemeId=p_scheme_id);
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
        let entity: Option<DeletePermissionSchemeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes a permission grant from a permission scheme. See [About permission schemes and grants](../api-group-permission-schemes/#about-permission-schemes-and-grants) for more details.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_permission_scheme_entity(configuration: &configuration::Configuration, scheme_id: i64, permission_id: i64) -> Result<(), Error<DeletePermissionSchemeEntityError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_scheme_id = scheme_id;
    let p_permission_id = permission_id;

    let uri_str = format!("{}/rest/api/3/permissionscheme/{schemeId}/permission/{permissionId}", configuration.base_path, schemeId=p_scheme_id, permissionId=p_permission_id);
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
        let entity: Option<DeletePermissionSchemeEntityError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all permission schemes.  ### About permission schemes and grants ###  A permission scheme is a collection of permission grants. A permission grant consists of a `holder` and a `permission`.  #### Holder object ####  The `holder` object contains information about the user or group being granted the permission. For example, the *Administer projects* permission is granted to a group named *Teams in space administrators*. In this case, the type is `\"type\": \"group\"`, and the parameter is the group name, `\"parameter\": \"Teams in space administrators\"` and the value is group ID, `\"value\": \"ca85fac0-d974-40ca-a615-7af99c48d24f\"`.  The `holder` object is defined by the following properties:   *  `type` Identifies the user or group (see the list of types below).  *  `parameter` As a group's name can change, use of `value` is recommended. The value of this property depends on the `type`. For example, if the `type` is a group, then you need to specify the group name.  *  `value` The value of this property depends on the `type`. If the `type` is a group, then you need to specify the group ID. For other `type` it has the same value as `parameter`  The following `types` are available. The expected values for `parameter` and `value` are given in parentheses (some types may not have a `parameter` or `value`):   *  `anyone` Grant for anonymous users.  *  `applicationRole` Grant for users with access to the specified application (application name, application name). See [Update product access settings](https://confluence.atlassian.com/x/3YxjL) for more information.  *  `assignee` Grant for the user currently assigned to an issue.  *  `group` Grant for the specified group (`parameter` : group name, `value` : group ID).  *  `groupCustomField` Grant for a user in the group selected in the specified custom field (`parameter` : custom field ID, `value` : custom field ID).  *  `projectLead` Grant for a project lead.  *  `projectRole` Grant for the specified project role (`parameter` :project role ID, `value` : project role ID).  *  `reporter` Grant for the user who reported the issue.  *  `sd.customer.portal.only` Jira Service Desk only. Grants customers permission to access the customer portal but not Jira. See [Customizing Jira Service Desk permissions](https://confluence.atlassian.com/x/24dKLg) for more information.  *  `user` Grant for the specified user (`parameter` : user ID - historically this was the userkey but that is deprecated and the account ID should be used, `value` : user ID).  *  `userCustomField` Grant for a user selected in the specified custom field (`parameter` : custom field ID, `value` : custom field ID).  #### Built-in permissions ####  The [built-in Jira permissions](https://confluence.atlassian.com/x/yodKLg) are listed below. Apps can also define custom permissions. See the [project permission](https://developer.atlassian.com/cloud/jira/platform/modules/project-permission/) and [global permission](https://developer.atlassian.com/cloud/jira/platform/modules/global-permission/) module documentation for more information.  **Administration permissions**   *  `ADMINISTER_PROJECTS`  *  `EDIT_WORKFLOW`  *  `EDIT_ISSUE_LAYOUT`  **Project permissions**   *  `BROWSE_PROJECTS`  *  `MANAGE_SPRINTS_PERMISSION` (Jira Software only)  *  `SERVICEDESK_AGENT` (Jira Service Desk only)  *  `VIEW_DEV_TOOLS` (Jira Software only)  *  `VIEW_READONLY_WORKFLOW`  **Issue permissions**   *  `ASSIGNABLE_USER`  *  `ASSIGN_ISSUES`  *  `CLOSE_ISSUES`  *  `CREATE_ISSUES`  *  `DELETE_ISSUES`  *  `EDIT_ISSUES`  *  `LINK_ISSUES`  *  `MODIFY_REPORTER`  *  `MOVE_ISSUES`  *  `RESOLVE_ISSUES`  *  `SCHEDULE_ISSUES`  *  `SET_ISSUE_SECURITY`  *  `TRANSITION_ISSUES`  **Voters and watchers permissions**   *  `MANAGE_WATCHERS`  *  `VIEW_VOTERS_AND_WATCHERS`  **Comments permissions**   *  `ADD_COMMENTS`  *  `DELETE_ALL_COMMENTS`  *  `DELETE_OWN_COMMENTS`  *  `EDIT_ALL_COMMENTS`  *  `EDIT_OWN_COMMENTS`  **Attachments permissions**   *  `CREATE_ATTACHMENTS`  *  `DELETE_ALL_ATTACHMENTS`  *  `DELETE_OWN_ATTACHMENTS`  **Time tracking permissions**   *  `DELETE_ALL_WORKLOGS`  *  `DELETE_OWN_WORKLOGS`  *  `EDIT_ALL_WORKLOGS`  *  `EDIT_OWN_WORKLOGS`  *  `WORK_ON_ISSUES`  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn get_all_permission_schemes(configuration: &configuration::Configuration, expand: Option<&str>) -> Result<models::PermissionSchemes, Error<GetAllPermissionSchemesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/permissionscheme", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PermissionSchemes`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PermissionSchemes`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAllPermissionSchemesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a permission scheme.  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn get_permission_scheme(configuration: &configuration::Configuration, scheme_id: i64, expand: Option<&str>) -> Result<models::PermissionScheme, Error<GetPermissionSchemeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_scheme_id = scheme_id;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/permissionscheme/{schemeId}", configuration.base_path, schemeId=p_scheme_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PermissionScheme`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PermissionScheme`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPermissionSchemeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a permission grant.  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn get_permission_scheme_grant(configuration: &configuration::Configuration, scheme_id: i64, permission_id: i64, expand: Option<&str>) -> Result<models::PermissionGrant, Error<GetPermissionSchemeGrantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_scheme_id = scheme_id;
    let p_permission_id = permission_id;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/permissionscheme/{schemeId}/permission/{permissionId}", configuration.base_path, schemeId=p_scheme_id, permissionId=p_permission_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PermissionGrant`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PermissionGrant`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPermissionSchemeGrantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all permission grants for a permission scheme.  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn get_permission_scheme_grants(configuration: &configuration::Configuration, scheme_id: i64, expand: Option<&str>) -> Result<models::PermissionGrants, Error<GetPermissionSchemeGrantsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_scheme_id = scheme_id;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/permissionscheme/{schemeId}/permission", configuration.base_path, schemeId=p_scheme_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PermissionGrants`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PermissionGrants`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPermissionSchemeGrantsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Updates a permission scheme. Below are some important things to note when using this resource:   *  If a permissions list is present in the request, then it is set in the permission scheme, overwriting *all existing* grants.  *  If you want to update only the name and description, then do not send a permissions list in the request.  *  Sending an empty list will remove all permission grants from the permission scheme.  If you want to add or delete a permission grant instead of updating the whole list, see [Create permission grant](#api-rest-api-3-permissionscheme-schemeId-permission-post) or [Delete permission scheme entity](#api-rest-api-3-permissionscheme-schemeId-permission-permissionId-delete).  See [About permission schemes and grants](../api-group-permission-schemes/#about-permission-schemes-and-grants) for more details.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn update_permission_scheme(configuration: &configuration::Configuration, scheme_id: i64, permission_scheme: models::PermissionScheme, expand: Option<&str>) -> Result<models::PermissionScheme, Error<UpdatePermissionSchemeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_scheme_id = scheme_id;
    let p_permission_scheme = permission_scheme;
    let p_expand = expand;

    let uri_str = format!("{}/rest/api/3/permissionscheme/{schemeId}", configuration.base_path, schemeId=p_scheme_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
    req_builder = req_builder.json(&p_permission_scheme);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PermissionScheme`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PermissionScheme`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdatePermissionSchemeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

