//! Small path helpers shared across the app.

use std::path::PathBuf;

/// The current user's home directory.
///
/// Uses `USERPROFILE` on Windows and `HOME` elsewhere, falling back to
/// the current directory if neither is set.
pub fn home() -> PathBuf {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}
