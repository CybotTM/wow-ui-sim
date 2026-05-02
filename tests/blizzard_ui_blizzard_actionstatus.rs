//! Wrapper binary for Blizzard_ActionStatus tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_actionstatus/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_actionstatus/load.rs"]
mod load;
