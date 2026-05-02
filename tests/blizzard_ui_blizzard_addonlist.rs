//! Wrapper binary for Blizzard_AddOnList tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_addonlist/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_addonlist/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_addonlist/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_addonlist/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_addonlist/surface_mixins.rs"]
mod surface_mixins;

#[path = "blizzard_ui/blizzard_addonlist/surface_events.rs"]
mod surface_events;
