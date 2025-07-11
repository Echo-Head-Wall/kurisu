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

/// MappingsByWorkflow : The status mappings by workflows. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MappingsByWorkflow {
    /// The ID of the new workflow.
    #[serde(rename = "newWorkflowId")]
    pub new_workflow_id: String,
    /// The ID of the old workflow.
    #[serde(rename = "oldWorkflowId")]
    pub old_workflow_id: String,
    /// The list of status mappings.
    #[serde(rename = "statusMappings")]
    pub status_mappings: Vec<models::WorkflowAssociationStatusMapping>,
}

impl MappingsByWorkflow {
    /// The status mappings by workflows. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`.
    pub fn new(new_workflow_id: String, old_workflow_id: String, status_mappings: Vec<models::WorkflowAssociationStatusMapping>) -> MappingsByWorkflow {
        MappingsByWorkflow {
            new_workflow_id,
            old_workflow_id,
            status_mappings,
        }
    }
}

