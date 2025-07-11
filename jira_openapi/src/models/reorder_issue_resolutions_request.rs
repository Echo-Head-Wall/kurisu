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

/// ReorderIssueResolutionsRequest : Change the order of issue resolutions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReorderIssueResolutionsRequest {
    /// The ID of the resolution. Required if `position` isn't provided.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The list of resolution IDs to be reordered. Cannot contain duplicates nor after ID.
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    /// The position for issue resolutions to be moved to. Required if `after` isn't provided.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

impl ReorderIssueResolutionsRequest {
    /// Change the order of issue resolutions.
    pub fn new(ids: Vec<String>) -> ReorderIssueResolutionsRequest {
        ReorderIssueResolutionsRequest {
            after: None,
            ids,
            position: None,
        }
    }
}

