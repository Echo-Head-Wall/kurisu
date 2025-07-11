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

/// WorkflowTransitionRules : A workflow with transition rules.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionRules {
    /// The list of conditions within the workflow.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::AppWorkflowTransitionRule>>,
    /// The list of post functions within the workflow.
    #[serde(rename = "postFunctions", skip_serializing_if = "Option::is_none")]
    pub post_functions: Option<Vec<models::AppWorkflowTransitionRule>>,
    /// The list of validators within the workflow.
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<models::AppWorkflowTransitionRule>>,
    #[serde(rename = "workflowId")]
    pub workflow_id: Box<models::WorkflowId>,
}

impl WorkflowTransitionRules {
    /// A workflow with transition rules.
    pub fn new(workflow_id: models::WorkflowId) -> WorkflowTransitionRules {
        WorkflowTransitionRules {
            conditions: None,
            post_functions: None,
            validators: None,
            workflow_id: Box::new(workflow_id),
        }
    }
}

