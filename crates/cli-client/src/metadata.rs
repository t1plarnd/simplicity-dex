//! Contract metadata for NOSTR event tracking.
//!
//! This module defines application-specific metadata that is stored
//! alongside contracts in coin-store. The metadata tracks NOSTR event
//! information for options and swaps.

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Metadata linking a contract to its NOSTR event origin.
///
/// This struct is serialized and stored in coin-store's generic
/// `app_metadata` column, keeping NOSTR-specific concerns in the CLI layer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContractMetadata {
    /// The NOSTR event ID that created this contract
    pub nostr_event_id: Option<String>,

    /// The NOSTR public key of the event author
    pub nostr_author: Option<String>,

    /// Unix timestamp when the event was created
    pub created_at: Option<i64>,

    /// For swaps: reference to the parent option's NOSTR event ID
    pub parent_event_id: Option<String>,
}

impl ContractMetadata {
    /// Create new metadata from a NOSTR event
    #[must_use]
    pub fn from_nostr(event_id: String, author: String, created_at: i64) -> Self {
        Self {
            nostr_event_id: Some(event_id),
            nostr_author: Some(author),
            created_at: Some(created_at),
            parent_event_id: None,
        }
    }

    /// Create new metadata for a swap linked to a parent option event
    #[must_use]
    pub fn from_nostr_with_parent(
        event_id: String,
        author: String,
        created_at: i64,
        parent_event_id: String,
    ) -> Self {
        Self {
            nostr_event_id: Some(event_id),
            nostr_author: Some(author),
            created_at: Some(created_at),
            parent_event_id: Some(parent_event_id),
        }
    }

    /// Serialize metadata to bytes for storage in coin-store
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).map_err(Error::MetadataEncode)
    }

    /// Deserialize metadata from bytes retrieved from coin-store
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let (metadata, _): (Self, usize) =
            bincode::serde::decode_from_slice(bytes, bincode::config::standard()).map_err(Error::MetadataDecode)?;
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_roundtrip() {
        let metadata = ContractMetadata::from_nostr(
            "event123".to_string(),
            "npub1abc".to_string(),
            1704067200,
        );

        let bytes = metadata.to_bytes().unwrap();
        let restored = ContractMetadata::from_bytes(&bytes).unwrap();

        assert_eq!(restored.nostr_event_id, Some("event123".to_string()));
        assert_eq!(restored.nostr_author, Some("npub1abc".to_string()));
        assert_eq!(restored.created_at, Some(1704067200));
        assert_eq!(restored.parent_event_id, None);
    }

    #[test]
    fn test_metadata_with_parent_roundtrip() {
        let metadata = ContractMetadata::from_nostr_with_parent(
            "swap456".to_string(),
            "npub1xyz".to_string(),
            1704153600,
            "option123".to_string(),
        );

        let bytes = metadata.to_bytes().unwrap();
        let restored = ContractMetadata::from_bytes(&bytes).unwrap();

        assert_eq!(restored.nostr_event_id, Some("swap456".to_string()));
        assert_eq!(restored.parent_event_id, Some("option123".to_string()));
    }

    #[test]
    fn test_default_metadata() {
        let metadata = ContractMetadata::default();

        assert!(metadata.nostr_event_id.is_none());
        assert!(metadata.nostr_author.is_none());
        assert!(metadata.created_at.is_none());
        assert!(metadata.parent_event_id.is_none());
    }
}

