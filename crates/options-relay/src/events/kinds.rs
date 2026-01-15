use nostr::Kind;

pub const OPTION_CREATED: Kind = Kind::Custom(9910);
pub const OPTION_OFFER_CREATED: Kind = Kind::Custom(9911);
pub const ACTION_COMPLETED: Kind = Kind::Custom(9912);

pub const TAG_OPTIONS_ARGS: &str = "options_args";
pub const TAG_OPTIONS_UTXO: &str = "options_utxo";
pub const TAG_OPTION_OFFER_ARGS: &str = "option_offer_args";
pub const TAG_OPTION_OFFER_UTXO: &str = "option_offer_utxo";
pub const TAG_TAPROOT_GEN: &str = "t";
pub const TAG_ACTION: &str = "action";
pub const TAG_OUTPOINT: &str = "outpoint";
pub const TAG_EXPIRY: &str = "expiry";

pub const ACTION_OPTION_CREATED: &str = "option_created";
pub const ACTION_OPTION_FUNDED: &str = "option_funded";
pub const ACTION_OPTION_OFFER_CREATED: &str = "option_offer_created";
pub const ACTION_OPTION_OFFER_EXERCISED: &str = "option_offer_exercised";
pub const ACTION_OPTION_OFFER_CANCELLED: &str = "option_offer_cancelled";
pub const ACTION_OPTION_EXERCISED: &str = "option_exercised";
pub const ACTION_OPTION_CANCELLED: &str = "option_cancelled";
pub const ACTION_SETTLEMENT_CLAIMED: &str = "settlement_claimed";
pub const ACTION_OPTION_EXPIRED: &str = "option_expired";
