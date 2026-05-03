//! Wrapper binary for Blizzard_ArtifactUI tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_artifact_ui/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_artifact_ui/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_artifact_ui/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_artifact_ui/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_artifact_ui/surface_events.rs"]
mod surface_events;

#[path = "blizzard_ui/blizzard_artifact_ui/behavior_show_panel_with_no_artifact_redirects.rs"]
mod behavior_show_panel_with_no_artifact_redirects;
