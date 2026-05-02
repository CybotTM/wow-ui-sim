//! Wrapper binary for Blizzard_AdventureMap tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_adventuremap/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_adventuremap/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_adventuremap/surface_globals.rs"]
mod surface_globals;
