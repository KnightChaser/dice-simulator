//! Simmple input helpers for the CLI.

use std::io::{self, Write};

/// Prompt for a `u32` with a default value.
/// Empty input returns `default`.
/// Non-numeric input, which is invalid, also returns `default`.
pub fn prompt_u32_default(msg: &str, default: u32) -> u32 {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut s = String::new();

    if io::stdin().read_line(&mut s).is_ok() {
        // Proceed only if read_line was successful
        let t = s.trim();
        if t.is_empty() {
            // If trimmed input is empty, return default
            return default;
        }
        if let Ok(v) = t.parse::<u32>() {
            // If parsing is successful, return the parsed value
            return v;
        }
    }
    default
}

/// Prompt for a `u64` with a default value.
/// Empty input returns `default`.
/// Non-numeric input, which is invalid, also returns `default`.
pub fn prompt_u64_default(msg: &str, default: u64) -> u64 {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut s = String::new();

    if io::stdin().read_line(&mut s).is_ok() {
        // Proceed only if read_line was successful
        let t = s.trim();
        if t.is_empty() {
            // If trimmed input is empty, return default
            return default;
        }
        if let Ok(v) = t.parse::<u64>() {
            // If parsing is successful, return the parsed value
            return v;
        }
    }
    default
}
