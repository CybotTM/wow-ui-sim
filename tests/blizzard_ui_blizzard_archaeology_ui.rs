//! Wrapper binary for Blizzard_ArchaeologyUI tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_archaeology_ui/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_archaeology_ui/load.rs"]
mod load;
