//! Wrapper binary for Blizzard_AdventureMap tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_adventuremap/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_adventuremap/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_adventuremap/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_adventuremap/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_adventuremap/surface_events.rs"]
mod surface_events;

#[path = "blizzard_ui/blizzard_adventuremap/surface_mixins.rs"]
mod surface_mixins;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_uipanel_window_entry_registered_at_load.rs"]
mod behavior_uipanel_window_entry_registered_at_load;
