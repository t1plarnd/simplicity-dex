#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod client;
pub mod config;
pub mod error;
pub mod events;

pub use client::{PublishingClient, ReadOnlyClient};
pub use config::NostrRelayConfig;
pub use error::{ParseError, RelayError};
pub use events::{
    ACTION_COMPLETED, ACTION_OPTION_CANCELLED, ACTION_OPTION_CREATED, ACTION_OPTION_EXERCISED, ACTION_OPTION_EXPIRED,
    ACTION_OPTION_FUNDED, ACTION_OPTION_OFFER_CANCELLED, ACTION_OPTION_OFFER_CREATED, ACTION_OPTION_OFFER_EXERCISED,
    ACTION_SETTLEMENT_CLAIMED, ActionCompletedEvent, ActionType, OPTION_CREATED, OPTION_OFFER_CREATED,
    OptionCreatedEvent, OptionOfferCreatedEvent,
};
