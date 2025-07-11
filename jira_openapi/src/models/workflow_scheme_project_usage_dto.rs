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

/// WorkflowSchemeProjectUsageDto : Projects using the workflow scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeProjectUsageDto {
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Box<models::ProjectUsagePage>>,
    /// The workflow scheme ID.
    #[serde(rename = "workflowSchemeId", skip_serializing_if = "Option::is_none")]
    pub workflow_scheme_id: Option<String>,
}

impl WorkflowSchemeProjectUsageDto {
    /// Projects using the workflow scheme.
    pub fn new() -> WorkflowSchemeProjectUsageDto {
        WorkflowSchemeProjectUsageDto {
            projects: None,
            workflow_scheme_id: None,
        }
    }
}

