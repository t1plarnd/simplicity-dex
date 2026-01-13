use crate::cli::interactive::{SwapDisplay, TokenDisplay};
use crate::cli::positions::{CollateralDisplay, UserTokenDisplay};
use crate::cli::swap::{ActiveSwapDisplay, CancellableSwapDisplay, WithdrawableSwapDisplay};
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Table};

trait TableData {
    fn get_header() -> Vec<String>;
    fn to_row(&self) -> Vec<String>;
}

impl TableData for TokenDisplay {
    fn get_header() -> Vec<String> {
        vec!["#", "Collateral/Token", "Strike/Token", "Expires", "Contract"]
            .into_iter()
            .map(String::from)
            .collect()
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.index.to_string(),
            self.collateral.clone(),
            self.settlement.clone(),
            self.expires.clone(),
            self.status.clone(),
        ]
    }
}

impl TableData for SwapDisplay {
    fn get_header() -> Vec<String> {
        vec!["#", "Price", "Wants", "Expires", "Seller"]
            .into_iter()
            .map(String::from)
            .collect()
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.index.to_string(),
            self.offering.clone(),
            self.wants.clone(),
            self.expires.clone(),
            self.seller.clone(),
        ]
    }
}

impl TableData for CollateralDisplay {
    fn get_header() -> Vec<String> {
        vec!["#", "Locked Assets", "Settlement", "Expires", "Contract"]
            .into_iter()
            .map(String::from)
            .collect()
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.index.to_string(),
            self.collateral.clone(),
            self.settlement.clone(),
            self.expires.clone(),
            self.contract.clone(),
        ]
    }
}

impl TableData for UserTokenDisplay {
    fn get_header() -> Vec<String> {
        vec!["#", "Type", "Amount", "Strike/Token", "Expires", "Contract"]
            .into_iter()
            .map(String::from)
            .collect()
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.index.to_string(),
            self.token_type.clone(),
            self.amount.clone(),
            self.strike.clone(),
            self.expires.clone(),
            self.contract.clone(),
        ]
    }
}

impl TableData for ActiveSwapDisplay {
    fn get_header() -> Vec<String> {
        vec!["#", "Offering", "Price", "Wants", "Expires", "Seller"]
            .into_iter()
            .map(String::from)
            .collect()
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.index.to_string(),
            self.offering.clone(),
            self.price.clone(),
            self.wants.clone(),
            self.expires.clone(),
            self.seller.clone(),
        ]
    }
}

impl TableData for CancellableSwapDisplay {
    fn get_header() -> Vec<String> {
        vec!["#", "Collateral", "Asset", "Expired", "Contract"]
            .into_iter()
            .map(String::from)
            .collect()
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.index.to_string(),
            self.collateral.clone(),
            self.asset.clone(),
            self.expired.clone(),
            self.contract.clone(),
        ]
    }
}

impl TableData for WithdrawableSwapDisplay {
    fn get_header() -> Vec<String> {
        vec!["#", "Settlement Available", "Asset", "Contract"]
            .into_iter()
            .map(String::from)
            .collect()
    }
    fn to_row(&self) -> Vec<String> {
        vec![
            self.index.to_string(),
            self.settlement.clone(),
            self.asset.clone(),
            self.contract.clone(),
        ]
    }
}

fn render_table<T: TableData>(items: &[T], empty_msg: &str) {
    if items.is_empty() {
        println!("  ({empty_msg})");
        return;
    }

    let mut table = Table::new();

    table.load_preset(UTF8_FULL);

    let header_cells: Vec<Cell> = T::get_header()
        .into_iter()
        .map(|h| Cell::new(h).add_attribute(Attribute::Bold))
        .collect();
    table.set_header(header_cells);

    for item in items {
        table.add_row(item.to_row());
    }

    for line in table.to_string().lines() {
        println!("  {line}");
    }
}

pub fn display_token_table(tokens: &[TokenDisplay]) {
    render_table(tokens, "No tokens found");
}

pub fn display_swap_table(swaps: &[SwapDisplay]) {
    render_table(swaps, "No swaps found");
}

pub fn display_collateral_table(displays: &[CollateralDisplay]) {
    render_table(displays, "No locked assets found");
}

pub fn display_user_token_table(displays: &[UserTokenDisplay]) {
    render_table(displays, "No option/grantor tokens found");
}

pub fn display_active_swaps_table(active_swaps: &[ActiveSwapDisplay]) {
    render_table(active_swaps, "No swaps found");
}

pub fn display_cancellable_swaps_table(cancellable_swaps: &[CancellableSwapDisplay]) {
    render_table(cancellable_swaps, "No cancellable swaps found");
}

pub fn display_withdrawable_swaps_table(withdrawable_swaps: &[WithdrawableSwapDisplay]) {
    render_table(withdrawable_swaps, "No withdrawable swaps found");
}
