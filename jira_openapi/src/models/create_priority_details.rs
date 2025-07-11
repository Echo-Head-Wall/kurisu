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

/// CreatePriorityDetails : Details of an issue priority.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePriorityDetails {
    /// The ID for the avatar for the priority. Either the iconUrl or avatarId must be defined, but not both. This parameter is nullable and will become mandatory once the iconUrl parameter is deprecated.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The description of the priority.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used. Either the iconUrl or avatarId must be defined, but not both.
    #[serde(rename = "iconUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<IconUrl>>,
    /// The name of the priority. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
    /// The status color of the priority in 3-digit or 6-digit hexadecimal format.
    #[serde(rename = "statusColor")]
    pub status_color: String,
}

impl CreatePriorityDetails {
    /// Details of an issue priority.
    pub fn new(name: String, status_color: String) -> CreatePriorityDetails {
        CreatePriorityDetails {
            avatar_id: None,
            description: None,
            icon_url: None,
            name,
            status_color,
        }
    }
}
/// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used. Either the iconUrl or avatarId must be defined, but not both.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IconUrl {
    #[serde(rename = "/images/icons/priorities/blocker.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashBlockerPeriodPng,
    #[serde(rename = "/images/icons/priorities/critical.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashCriticalPeriodPng,
    #[serde(rename = "/images/icons/priorities/high.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashHighPeriodPng,
    #[serde(rename = "/images/icons/priorities/highest.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashHighestPeriodPng,
    #[serde(rename = "/images/icons/priorities/low.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashLowPeriodPng,
    #[serde(rename = "/images/icons/priorities/lowest.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashLowestPeriodPng,
    #[serde(rename = "/images/icons/priorities/major.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashMajorPeriodPng,
    #[serde(rename = "/images/icons/priorities/medium.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashMediumPeriodPng,
    #[serde(rename = "/images/icons/priorities/minor.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashMinorPeriodPng,
    #[serde(rename = "/images/icons/priorities/trivial.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashTrivialPeriodPng,
    #[serde(rename = "/images/icons/priorities/blocker_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashBlockerNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/critical_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashCriticalNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/high_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashHighNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/highest_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashHighestNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/low_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashLowNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/lowest_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashLowestNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/major_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashMajorNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/medium_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashMediumNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/minor_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashMinorNewPeriodPng,
    #[serde(rename = "/images/icons/priorities/trivial_new.png")]
    SlashImagesSlashIconsSlashPrioritiesSlashTrivialNewPeriodPng,
}

impl Default for IconUrl {
    fn default() -> IconUrl {
        Self::SlashImagesSlashIconsSlashPrioritiesSlashBlockerPeriodPng
    }
}

