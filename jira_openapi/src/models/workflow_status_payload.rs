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

/// WorkflowStatusPayload : The statuses to be used in the workflow
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatusPayload {
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<Box<models::WorkflowStatusLayoutPayload>>,
    #[serde(rename = "pcri", skip_serializing_if = "Option::is_none")]
    pub pcri: Option<Box<models::ProjectCreateResourceIdentifier>>,
    /// The properties of the workflow status.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

impl WorkflowStatusPayload {
    /// The statuses to be used in the workflow
    pub fn new() -> WorkflowStatusPayload {
        WorkflowStatusPayload {
            layout: None,
            pcri: None,
            properties: None,
        }
    }
}

