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

/// Scope : The projects the item is associated with. Indicated for items associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    /// The project the item has scope in.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<models::ProjectDetails>>,
    /// The type of scope.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl Scope {
    /// The projects the item is associated with. Indicated for items associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).
    pub fn new() -> Scope {
        Scope {
            project: None,
            r#type: None,
        }
    }
}
/// The type of scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "TEMPLATE")]
    Template,
}

impl Default for Type {
    fn default() -> Type {
        Self::Project
    }
}

