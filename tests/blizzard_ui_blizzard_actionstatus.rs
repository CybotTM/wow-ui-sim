//! Wrapper binary for Blizzard_ActionStatus tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_actionstatus/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_actionstatus/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_actionstatus/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_actionstatus/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_actionstatus/surface_events.rs"]
mod surface_events;
