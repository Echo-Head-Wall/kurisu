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

/// FieldLayoutSchemePayload : Defines the payload for the field layout schemes. See \"Field Configuration Scheme\" - https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-field-configurations/\\#api-rest-api-3-fieldconfigurationscheme-post https://support.atlassian.com/jira-cloud-administration/docs/configure-a-field-configuration-scheme/
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldLayoutSchemePayload {
    #[serde(rename = "defaultFieldLayout", skip_serializing_if = "Option::is_none")]
    pub default_field_layout: Option<Box<models::ProjectCreateResourceIdentifier>>,
    /// The description of the field layout scheme
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// There is a default configuration \"fieldlayout\" that is applied to all issue types using this scheme that don't have an explicit mapping users can create (or re-use existing) configurations for other issue types and map them to this scheme
    #[serde(rename = "explicitMappings", skip_serializing_if = "Option::is_none")]
    pub explicit_mappings: Option<std::collections::HashMap<String, models::ProjectCreateResourceIdentifier>>,
    /// The name of the field layout scheme
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pcri", skip_serializing_if = "Option::is_none")]
    pub pcri: Option<Box<models::ProjectCreateResourceIdentifier>>,
}

impl FieldLayoutSchemePayload {
    /// Defines the payload for the field layout schemes. See \"Field Configuration Scheme\" - https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-field-configurations/\\#api-rest-api-3-fieldconfigurationscheme-post https://support.atlassian.com/jira-cloud-administration/docs/configure-a-field-configuration-scheme/
    pub fn new() -> FieldLayoutSchemePayload {
        FieldLayoutSchemePayload {
            default_field_layout: None,
            description: None,
            explicit_mappings: None,
            name: None,
            pcri: None,
        }
    }
}

