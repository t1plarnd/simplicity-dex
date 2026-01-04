//! Options lifecycle commands: create, exercise, expire, cancel

use crate::cli::{Cli, OptionCommand};
use crate::config::Config;
use crate::error::Error;

impl Cli {
    pub(crate) async fn run_option(&self, config: Config, command: &OptionCommand) -> Result<(), Error> {
        let _wallet = self.get_wallet(&config).await?;

        match command {
            OptionCommand::Create {
                collateral_asset,
                collateral_amount,
                settlement_asset,
                settlement_amount,
                expiry,
                fee,
                broadcast,
            } => {
                println!("Creating option contract...");
                println!("  Collateral: {} of {}", collateral_amount, collateral_asset);
                println!("  Settlement: {} of {}", settlement_amount, settlement_asset);
                println!("  Expiry: {}", expiry);
                println!("  Fee: {} sats", fee);
                println!("  Broadcast: {}", broadcast);

                todo!("Option create not yet implemented")
            }
            OptionCommand::Exercise {
                grantor_token,
                fee,
                broadcast,
            } => {
                println!("Exercising option...");
                if let Some(token) = grantor_token {
                    println!("  Grantor token: {}", token);
                } else {
                    println!("  Interactive selection mode");
                }
                println!("  Fee: {} sats", fee);
                println!("  Broadcast: {}", broadcast);

                todo!("Option exercise not yet implemented")
            }
            OptionCommand::Expire {
                grantor_token,
                fee,
                broadcast,
            } => {
                println!("Expiring option...");
                if let Some(token) = grantor_token {
                    println!("  Grantor token: {}", token);
                } else {
                    println!("  Interactive selection mode");
                }
                println!("  Fee: {} sats", fee);
                println!("  Broadcast: {}", broadcast);

                todo!("Option expire not yet implemented")
            }
            OptionCommand::Cancel {
                option_token,
                fee,
                broadcast,
            } => {
                println!("Cancelling option...");
                if let Some(token) = option_token {
                    println!("  Option token: {}", token);
                } else {
                    println!("  Interactive selection mode");
                }
                println!("  Fee: {} sats", fee);
                println!("  Broadcast: {}", broadcast);

                todo!("Option cancel not yet implemented")
            }
        }
    }
}

