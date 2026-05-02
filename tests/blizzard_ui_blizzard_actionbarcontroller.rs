//! Wrapper binary for Blizzard_ActionBarController tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_actionbarcontroller/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_actionbarcontroller/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_actionbarcontroller/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_actionbarcontroller/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_actionbarcontroller/surface_events.rs"]
mod surface_events;

#[path = "blizzard_ui/blizzard_actionbarcontroller/behavior_initial_state.rs"]
mod behavior_initial_state;

#[path = "blizzard_ui/blizzard_actionbarcontroller/behavior_player_entering_world_runs_update_all.rs"]
mod behavior_player_entering_world_runs_update_all;

#[path = "blizzard_ui/blizzard_actionbarcontroller/behavior_page_changed_routes_to_update_all.rs"]
mod behavior_page_changed_routes_to_update_all;
