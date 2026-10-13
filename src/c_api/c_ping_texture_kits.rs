//! Explicit host-supplied input only; no query implementation or registration.

use std::collections::HashMap;

/// Whole host input; empty by default, with no native mapping catalog.
#[derive(Default)]
pub struct PingTextureKits {
    pub by_type: HashMap<i32, String>,
}
