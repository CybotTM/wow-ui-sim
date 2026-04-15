//! Bytecode caching toggle.
//!
//! The old on-disk cache implementation is gone from the live rilua path. The
//! loader still consults this module for the env-var gate, so keep the toggle
//! surface small and explicit instead of carrying the unused pack-file code.

use std::sync::OnceLock;

/// Check if bytecode caching is disabled.
/// Result is cached after first check.
pub fn is_disabled() -> bool {
    static DISABLED: OnceLock<bool> = OnceLock::new();
    *DISABLED.get_or_init(|| {
        if let Ok(enable) = std::env::var("WOW_SIM_ENABLE_BYTECODE_CACHE") {
            return !(enable == "1" || enable.eq_ignore_ascii_case("true"));
        }

        std::env::var("WOW_SIM_DISABLE_BYTECODE_CACHE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    })
}
