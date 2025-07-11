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

/// AppWorkflowTransitionRule : A workflow transition rule.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppWorkflowTransitionRule {
    #[serde(rename = "configuration")]
    pub configuration: Box<models::RuleConfiguration>,
    /// The ID of the transition rule.
    #[serde(rename = "id")]
    pub id: String,
    /// The key of the rule, as defined in the Connect or the Forge app descriptor.
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "transition", skip_serializing_if = "Option::is_none")]
    pub transition: Option<Box<models::WorkflowTransition>>,
}

impl AppWorkflowTransitionRule {
    /// A workflow transition rule.
    pub fn new(configuration: models::RuleConfiguration, id: String, key: String) -> AppWorkflowTransitionRule {
        AppWorkflowTransitionRule {
            configuration: Box::new(configuration),
            id,
            key,
            transition: None,
        }
    }
}

