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

/// NotificationSchemeEventTypeId : The ID of an event that is being mapped to notifications.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationSchemeEventTypeId {
    /// The ID of the notification scheme event.
    #[serde(rename = "id")]
    pub id: String,
}

impl NotificationSchemeEventTypeId {
    /// The ID of an event that is being mapped to notifications.
    pub fn new(id: String) -> NotificationSchemeEventTypeId {
        NotificationSchemeEventTypeId {
            id,
        }
    }
}

