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

/// WorkflowLayout : The starting point for the statuses in the workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowLayout {
    /// The x axis location.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// The y axis location.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

impl WorkflowLayout {
    /// The starting point for the statuses in the workflow.
    pub fn new() -> WorkflowLayout {
        WorkflowLayout {
            x: None,
            y: None,
        }
    }
}

