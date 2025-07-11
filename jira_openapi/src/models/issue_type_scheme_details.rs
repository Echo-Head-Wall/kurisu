/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-ccbf72d894d6b699175624f7a94244e68c9dbc6d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueTypeSchemeDetails : Details of an issue type scheme and its associated issue types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeSchemeDetails {
    /// The ID of the default issue type of the issue type scheme. This ID must be included in `issueTypeIds`.
    #[serde(rename = "defaultIssueTypeId", skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<String>,
    /// The description of the issue type scheme. The maximum length is 4000 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The list of issue types IDs of the issue type scheme. At least one standard issue type ID is required.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    /// The name of the issue type scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
}

impl IssueTypeSchemeDetails {
    /// Details of an issue type scheme and its associated issue types.
    pub fn new(issue_type_ids: Vec<String>, name: String) -> IssueTypeSchemeDetails {
        IssueTypeSchemeDetails {
            default_issue_type_id: None,
            description: None,
            issue_type_ids,
            name,
        }
    }
}

