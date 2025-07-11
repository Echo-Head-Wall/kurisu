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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIssueSourceResponse {
    /// The issue source type. This is \"Board\", \"Project\" or \"Filter\".
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The issue source value. This is a board ID if the type is \"Board\", a project ID if the type is \"Project\" or a filter ID if the type is \"Filter\".
    #[serde(rename = "value")]
    pub value: i64,
}

impl GetIssueSourceResponse {
    pub fn new(r#type: Type, value: i64) -> GetIssueSourceResponse {
        GetIssueSourceResponse {
            r#type,
            value,
        }
    }
}
/// The issue source type. This is \"Board\", \"Project\" or \"Filter\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Board")]
    Board,
    #[serde(rename = "Project")]
    Project,
    #[serde(rename = "Filter")]
    Filter,
    #[serde(rename = "Custom")]
    Custom,
}

impl Default for Type {
    fn default() -> Type {
        Self::Board
    }
}

