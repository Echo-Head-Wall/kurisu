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

/// ProjectIdentifiers : Identifiers for a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIdentifiers {
    /// The ID of the created project.
    #[serde(rename = "id")]
    pub id: i64,
    /// The key of the created project.
    #[serde(rename = "key")]
    pub key: String,
    /// The URL of the created project.
    #[serde(rename = "self")]
    pub param_self: String,
}

impl ProjectIdentifiers {
    /// Identifiers for a project.
    pub fn new(id: i64, key: String, param_self: String) -> ProjectIdentifiers {
        ProjectIdentifiers {
            id,
            key,
            param_self,
        }
    }
}

