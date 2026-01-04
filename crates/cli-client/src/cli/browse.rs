//! Browse command: fetch options/swaps from NOSTR, sync to coin-store, display

use crate::cli::Cli;
use crate::config::Config;
use crate::error::Error;

impl Cli {
    pub(crate) async fn run_browse(&self, config: Config) -> Result<(), Error> {
        let _wallet = self.get_wallet(&config).await?;

        println!("Browsing available options and swaps from NOSTR...");
        println!();
        println!("Available Options:");
        println!("------------------");
        println!("  (No options found - NOSTR sync not yet implemented)");
        println!();
        println!("Available Swaps:");
        println!("----------------");
        println!("  (No swaps found - NOSTR sync not yet implemented)");

        todo!("Browse not yet implemented")
    }
}

