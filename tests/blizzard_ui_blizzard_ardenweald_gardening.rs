//! Wrapper binary for Blizzard_ArdenwealdGardening tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_ardenweald_gardening/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/behavior_create_attaches_panel.rs"]
mod behavior_create_attaches_panel;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/behavior_onenter_active_branch.rs"]
mod behavior_onenter_active_branch;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/behavior_onenter_ready_branch.rs"]
mod behavior_onenter_ready_branch;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/behavior_onenter_active_and_ready_branch.rs"]
mod behavior_onenter_active_and_ready_branch;

#[path = "blizzard_ui/blizzard_ardenweald_gardening/behavior_onenter_dormant_branch.rs"]
mod behavior_onenter_dormant_branch;
