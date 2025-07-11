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

/// AttachmentArchiveMetadataReadable : Metadata for an archive (for example a zip) and its contents.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentArchiveMetadataReadable {
    /// The list of the items included in the archive.
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<models::AttachmentArchiveItemReadable>>,
    /// The ID of the attachment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The MIME type of the attachment.
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The name of the archive file.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of items included in the archive.
    #[serde(rename = "totalEntryCount", skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i64>,
}

impl AttachmentArchiveMetadataReadable {
    /// Metadata for an archive (for example a zip) and its contents.
    pub fn new() -> AttachmentArchiveMetadataReadable {
        AttachmentArchiveMetadataReadable {
            entries: None,
            id: None,
            media_type: None,
            name: None,
            total_entry_count: None,
        }
    }
}

