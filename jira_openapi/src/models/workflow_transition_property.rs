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

/// WorkflowTransitionProperty : Details about the server Jira is running on.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionProperty {
    /// The ID of the transition property.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the transition property. Also known as the name of the transition property.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value of the transition property.
    #[serde(rename = "value")]
    pub value: String,
}

impl WorkflowTransitionProperty {
    /// Details about the server Jira is running on.
    pub fn new(value: String) -> WorkflowTransitionProperty {
        WorkflowTransitionProperty {
            id: None,
            key: None,
            value,
        }
    }
}

