//! NOSTR event synchronization to coin-store.
//!
//! This module handles syncing NOSTR events (options and swaps) to the local
//! coin-store database, including contract metadata tracking.

use coin_store::{Store, UtxoStore};
use options_relay::{OptionCreatedEvent, SwapCreatedEvent};

use crate::error::Error;
use crate::metadata::ContractMetadata;

/// Sync an option created event to coin-store.
///
/// This stores the contract with its NOSTR metadata so we can track
/// which NOSTR event created this option.
///
/// # Arguments
/// * `store` - The coin-store database
/// * `event` - The parsed option created event from NOSTR
/// * `source` - The Simplicity source code for the options contract
/// * `arguments` - The compiled arguments for the contract
pub async fn sync_option_event(
    store: &Store,
    event: &OptionCreatedEvent,
    source: &str,
    arguments: simplicityhl::Arguments,
) -> Result<(), Error> {
    #[allow(clippy::cast_possible_wrap)]
    let created_at = event.created_at.as_secs() as i64;

    let metadata = ContractMetadata::from_nostr(
        event.event_id.to_hex(),
        event.pubkey.to_hex(),
        created_at,
    );

    let metadata_bytes = metadata.to_bytes()?;

    store
        .add_contract(
            source,
            arguments,
            event.taproot_pubkey_gen.clone(),
            Some(&metadata_bytes),
        )
        .await?;

    Ok(())
}

/// Sync a swap created event to coin-store.
///
/// This stores the swap contract with its NOSTR metadata, including
/// a reference to the parent option event if provided.
///
/// # Arguments
/// * `store` - The coin-store database
/// * `event` - The parsed swap created event from NOSTR
/// * `source` - The Simplicity source code for the swap contract
/// * `arguments` - The compiled arguments for the contract
/// * `parent_option_event_id` - Optional reference to the parent option's event ID
pub async fn sync_swap_event(
    store: &Store,
    event: &SwapCreatedEvent,
    source: &str,
    arguments: simplicityhl::Arguments,
    parent_option_event_id: Option<String>,
) -> Result<(), Error> {
    #[allow(clippy::cast_possible_wrap)]
    let created_at = event.created_at.as_secs() as i64;

    let metadata = if let Some(parent_id) = parent_option_event_id {
        ContractMetadata::from_nostr_with_parent(
            event.event_id.to_hex(),
            event.pubkey.to_hex(),
            created_at,
            parent_id,
        )
    } else {
        ContractMetadata::from_nostr(
            event.event_id.to_hex(),
            event.pubkey.to_hex(),
            created_at,
        )
    };

    let metadata_bytes = metadata.to_bytes()?;

    store
        .add_contract(
            source,
            arguments,
            event.taproot_pubkey_gen.clone(),
            Some(&metadata_bytes),
        )
        .await?;

    Ok(())
}

/// Retrieve contract metadata from coin-store.
///
/// Returns the parsed metadata if it exists for the given contract.
pub async fn get_contract_metadata(
    store: &Store,
    taproot_pubkey_gen: &contracts::sdk::taproot_pubkey_gen::TaprootPubkeyGen,
) -> Result<Option<ContractMetadata>, Error> {
    let metadata_bytes = store.get_contract_metadata(taproot_pubkey_gen).await?;

    match metadata_bytes {
        Some(bytes) => Ok(Some(ContractMetadata::from_bytes(&bytes)?)),
        None => Ok(None),
    }
}

/// Update contract metadata in coin-store.
///
/// Useful for adding parent event references after the fact.
pub async fn update_contract_metadata(
    store: &Store,
    taproot_pubkey_gen: &contracts::sdk::taproot_pubkey_gen::TaprootPubkeyGen,
    metadata: &ContractMetadata,
) -> Result<(), Error> {
    let metadata_bytes = metadata.to_bytes()?;
    store
        .update_contract_metadata(taproot_pubkey_gen, &metadata_bytes)
        .await?;
    Ok(())
}

