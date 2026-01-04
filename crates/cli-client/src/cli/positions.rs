//! Positions command: show holdings with expiration warnings

use crate::cli::Cli;
use crate::config::Config;
use crate::error::Error;

impl Cli {
    pub(crate) async fn run_positions(&self, config: Config) -> Result<(), Error> {
        let _wallet = self.get_wallet(&config).await?;

        println!("Your Positions:");
        println!("===============");
        println!();
        println!("Option Tokens:");
        println!("--------------");
        println!("  (No option tokens found)");
        println!();
        println!("Grantor Tokens:");
        println!("---------------");
        println!("  (No grantor tokens found)");
        println!();
        println!("Pending Swaps:");
        println!("--------------");
        println!("  (No pending swaps found)");

        todo!("Positions not yet implemented")
    }
}

