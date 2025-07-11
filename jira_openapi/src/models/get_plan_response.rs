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
pub struct GetPlanResponse {
    /// The cross-project releases included in the plan.
    #[serde(rename = "crossProjectReleases", skip_serializing_if = "Option::is_none")]
    pub cross_project_releases: Option<Vec<models::GetCrossProjectReleaseResponse>>,
    /// The custom fields for the plan.
    #[serde(rename = "customFields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<models::GetCustomFieldResponse>>,
    /// The exclusion rules for the plan.
    #[serde(rename = "exclusionRules", skip_serializing_if = "Option::is_none")]
    pub exclusion_rules: Option<Box<models::GetExclusionRulesResponse>>,
    /// The plan ID.
    #[serde(rename = "id")]
    pub id: i64,
    /// The issue sources included in the plan.
    #[serde(rename = "issueSources", skip_serializing_if = "Option::is_none")]
    pub issue_sources: Option<Vec<models::GetIssueSourceResponse>>,
    /// The date when the plan was last saved in UTC.
    #[serde(rename = "lastSaved", skip_serializing_if = "Option::is_none")]
    pub last_saved: Option<String>,
    /// The account ID of the plan lead.
    #[serde(rename = "leadAccountId", skip_serializing_if = "Option::is_none")]
    pub lead_account_id: Option<String>,
    /// The plan name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The permissions for the plan.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<models::GetPermissionResponse>>,
    /// The scheduling settings for the plan.
    #[serde(rename = "scheduling")]
    pub scheduling: Box<models::GetSchedulingResponse>,
    /// The plan status. This is \"Active\", \"Trashed\" or \"Archived\".
    #[serde(rename = "status")]
    pub status: Status,
}

impl GetPlanResponse {
    pub fn new(id: i64, scheduling: models::GetSchedulingResponse, status: Status) -> GetPlanResponse {
        GetPlanResponse {
            cross_project_releases: None,
            custom_fields: None,
            exclusion_rules: None,
            id,
            issue_sources: None,
            last_saved: None,
            lead_account_id: None,
            name: None,
            permissions: None,
            scheduling: Box::new(scheduling),
            status,
        }
    }
}
/// The plan status. This is \"Active\", \"Trashed\" or \"Archived\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Trashed")]
    Trashed,
    #[serde(rename = "Archived")]
    Archived,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

