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

/// PriorityWithSequence : An issue priority with sequence information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriorityWithSequence {
    /// The description of the issue priority.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the icon for the issue priority.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The ID of the issue priority.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this priority is the default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the issue priority.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the issue priority.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The sequence of the issue priority.
    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
    /// The color used to indicate the issue priority.
    #[serde(rename = "statusColor", skip_serializing_if = "Option::is_none")]
    pub status_color: Option<String>,
}

impl PriorityWithSequence {
    /// An issue priority with sequence information.
    pub fn new() -> PriorityWithSequence {
        PriorityWithSequence {
            description: None,
            icon_url: None,
            id: None,
            is_default: None,
            name: None,
            param_self: None,
            sequence: None,
            status_color: None,
        }
    }
}

