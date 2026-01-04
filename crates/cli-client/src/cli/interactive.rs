//! Interactive selection UI helpers for CLI commands.
//!
//! Provides table display and prompt functions for selecting tokens,
//! options, and swaps interactively.

use std::io::{self, Write};

/// Display information about a token for selection tables.
#[derive(Debug, Clone)]
pub struct TokenDisplay {
    /// Index for selection (1-based for user display)
    pub index: usize,
    /// The outpoint string (txid:vout)
    pub outpoint: String,
    /// Collateral description (e.g., "115,958 USDt")
    pub collateral: String,
    /// Settlement description (e.g., "1.0083 LBTC")
    pub settlement: String,
    /// Expiry description (e.g., "in 28 days" or "[SOON] in 6 hours")
    pub expires: String,
    /// Current status
    pub status: String,
}

/// Display information about a swap for selection tables.
#[derive(Debug, Clone)]
pub struct SwapDisplay {
    /// Index for selection (1-based for user display)
    pub index: usize,
    /// The NOSTR event ID
    pub event_id: String,
    /// What the swap is offering (e.g., "Grantor (115k USDt)")
    pub offering: String,
    /// What the swap wants (e.g., "0.05 LBTC")
    pub wants: String,
    /// Expiry description
    pub expires: String,
    /// Seller's NOSTR public key (truncated)
    pub seller: String,
}

/// Format a duration relative to now with urgency indicators.
///
/// Returns strings like:
/// - `[EXPIRED]` - past expiry
/// - `[URGENT] in 45 minutes` - less than 1 hour
/// - `[SOON] in 6 hours` - less than 24 hours
/// - `in 3 days 12 hours` - normal
#[must_use]
pub fn format_relative_time(expiry_timestamp: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let diff_secs = expiry_timestamp - now;

    if diff_secs <= 0 {
        return "[EXPIRED]".to_string();
    }

    let hours = diff_secs / 3600;
    let days = hours / 24;
    let remaining_hours = hours % 24;

    let time_str = if days > 0 {
        if remaining_hours > 0 {
            format!("in {} days {} hours", days, remaining_hours)
        } else {
            format!("in {} days", days)
        }
    } else if hours > 0 {
        format!("in {} hours", hours)
    } else {
        let minutes = diff_secs / 60;
        if minutes > 0 {
            format!("in {} minutes", minutes)
        } else {
            format!("in {} seconds", diff_secs)
        }
    };

    // Add urgency prefix
    if hours < 1 {
        format!("[URGENT] {}", time_str)
    } else if hours < 24 {
        format!("[SOON] {}", time_str)
    } else {
        time_str
    }
}

/// Display a table of tokens for selection.
pub fn display_token_table(tokens: &[TokenDisplay]) {
    if tokens.is_empty() {
        println!("  (No tokens found)");
        return;
    }

    println!(
        "  {:<3} | {:<18} | {:<14} | {:<18} | {}",
        "#", "Collateral", "Settlement", "Expires", "Status"
    );
    println!("{}", "-".repeat(80));

    for token in tokens {
        println!(
            "  {:<3} | {:<18} | {:<14} | {:<18} | {}",
            token.index, token.collateral, token.settlement, token.expires, token.status
        );
    }
}

/// Display a table of swaps for selection.
pub fn display_swap_table(swaps: &[SwapDisplay]) {
    if swaps.is_empty() {
        println!("  (No swaps found)");
        return;
    }

    println!(
        "  {:<3} | {:<20} | {:<14} | {:<15} | {}",
        "#", "Offering", "Wants", "Expires", "Seller"
    );
    println!("{}", "-".repeat(80));

    for swap in swaps {
        println!(
            "  {:<3} | {:<20} | {:<14} | {:<15} | {}",
            swap.index, swap.offering, swap.wants, swap.expires, swap.seller
        );
    }
}

/// Prompt user to select from a numbered list.
///
/// Returns the 0-based index of the selected item, or None if user quits.
pub fn prompt_selection(prompt: &str, max: usize) -> io::Result<Option<usize>> {
    print!("{} (1-{}, or 'q' to quit): ", prompt, max);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.eq_ignore_ascii_case("q") {
        return Ok(None);
    }

    match input.parse::<usize>() {
        Ok(n) if n >= 1 && n <= max => Ok(Some(n - 1)), // Convert to 0-based
        _ => {
            println!("Invalid selection. Please enter a number between 1 and {}.", max);
            prompt_selection(prompt, max) // Retry
        }
    }
}

/// Prompt user for an amount value.
pub fn prompt_amount(prompt: &str) -> io::Result<u64> {
    print!("{}: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    match input.parse::<u64>() {
        Ok(amount) => Ok(amount),
        Err(_) => {
            println!("Invalid amount. Please enter a positive number.");
            prompt_amount(prompt) // Retry
        }
    }
}

/// Prompt user for a yes/no confirmation.
pub fn prompt_confirm(prompt: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    Ok(input.eq_ignore_ascii_case("y") || input.eq_ignore_ascii_case("yes"))
}

/// Truncate a string with ellipsis if too long.
#[must_use]
pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s[..max_len].to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Format a satoshi amount as a human-readable string.
#[must_use]
pub fn format_sats(sats: u64) -> String {
    if sats >= 100_000_000 {
        format!("{:.4} BTC", sats as f64 / 100_000_000.0)
    } else if sats >= 1_000_000 {
        format!("{:.2}M sats", sats as f64 / 1_000_000.0)
    } else if sats >= 1_000 {
        format!("{:.1}k sats", sats as f64 / 1_000.0)
    } else {
        format!("{} sats", sats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_relative_time() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Expired
        assert_eq!(format_relative_time(now - 100), "[EXPIRED]");

        // Urgent (< 1 hour)
        let urgent = format_relative_time(now + 1800); // 30 minutes
        assert!(urgent.starts_with("[URGENT]"));

        // Soon (< 24 hours)
        let soon = format_relative_time(now + 6 * 3600); // 6 hours
        assert!(soon.starts_with("[SOON]"));

        // Normal
        let normal = format_relative_time(now + 3 * 24 * 3600); // 3 days
        assert!(normal.starts_with("in 3 days"));
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello", 10), "hello");
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello...");
        assert_eq!(truncate_with_ellipsis("abc", 3), "abc");
    }

    #[test]
    fn test_format_sats() {
        assert_eq!(format_sats(500), "500 sats");
        assert_eq!(format_sats(5000), "5.0k sats");
        assert_eq!(format_sats(5_000_000), "5.00M sats");
        assert_eq!(format_sats(100_000_000), "1.0000 BTC");
    }
}

