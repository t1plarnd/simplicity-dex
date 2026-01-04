//! Swap lifecycle commands: create, take, cancel

use crate::cli::{Cli, SwapCommand};
use crate::config::Config;
use crate::error::Error;

impl Cli {
    pub(crate) async fn run_swap(&self, config: Config, command: &SwapCommand) -> Result<(), Error> {
        let _wallet = self.get_wallet(&config).await?;

        match command {
            SwapCommand::Create {
                grantor_token,
                premium_asset,
                premium_amount,
                expiry,
                fee,
                broadcast,
            } => {
                println!("Creating swap offer...");
                if let Some(token) = grantor_token {
                    println!("  Grantor token: {}", token);
                } else {
                    println!("  Interactive selection mode");
                }
                if let Some(asset) = premium_asset {
                    println!("  Premium asset: {}", asset);
                } else {
                    println!("  Premium asset: LBTC (default)");
                }
                println!("  Premium amount: {}", premium_amount);
                if let Some(exp) = expiry {
                    println!("  Expiry: {}", exp);
                }
                println!("  Fee: {} sats", fee);
                println!("  Broadcast: {}", broadcast);

                todo!("Swap create not yet implemented")
            }
            SwapCommand::Take {
                swap_event,
                fee,
                broadcast,
            } => {
                println!("Taking swap offer...");
                if let Some(event) = swap_event {
                    println!("  Swap event: {}", event);
                } else {
                    println!("  Interactive selection mode");
                }
                println!("  Fee: {} sats", fee);
                println!("  Broadcast: {}", broadcast);

                todo!("Swap take not yet implemented")
            }
            SwapCommand::Cancel {
                swap_event,
                fee,
                broadcast,
            } => {
                println!("Cancelling swap offer...");
                if let Some(event) = swap_event {
                    println!("  Swap event: {}", event);
                } else {
                    println!("  Interactive selection mode");
                }
                println!("  Fee: {} sats", fee);
                println!("  Broadcast: {}", broadcast);

                todo!("Swap cancel not yet implemented")
            }
        }
    }
}

