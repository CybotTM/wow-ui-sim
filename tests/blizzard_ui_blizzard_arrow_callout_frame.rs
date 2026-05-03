//! Wrapper binary for Blizzard_ArrowCalloutFrame tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_arrow_callout_frame/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_arrow_callout_frame/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_arrow_callout_frame/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_arrow_callout_frame/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_arrow_callout_frame/surface_events.rs"]
mod surface_events;
