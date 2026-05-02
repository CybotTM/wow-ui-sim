//! Wrapper binary for Blizzard_AddOnList tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_addonlist/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_addonlist/load.rs"]
mod load;
