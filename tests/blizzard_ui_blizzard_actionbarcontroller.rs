//! Wrapper binary for Blizzard_ActionBarController tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_actionbarcontroller/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_actionbarcontroller/load.rs"]
mod load;
