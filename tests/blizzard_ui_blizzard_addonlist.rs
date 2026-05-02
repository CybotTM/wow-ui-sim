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

#[path = "blizzard_ui/blizzard_addonlist/behavior_initial_show_populates_scroll.rs"]
mod behavior_initial_show_populates_scroll;

#[path = "blizzard_ui/blizzard_addonlist/behavior_search_box_filters_rows.rs"]
mod behavior_search_box_filters_rows;

#[path = "blizzard_ui/blizzard_addonlist/behavior_force_load_toggles_version_check.rs"]
mod behavior_force_load_toggles_version_check;

#[path = "blizzard_ui/blizzard_addonlist/behavior_enable_all_disables_disable_all.rs"]
mod behavior_enable_all_disables_disable_all;

#[path = "blizzard_ui/blizzard_addonlist/behavior_okay_button_saves_when_pending_changes.rs"]
mod behavior_okay_button_saves_when_pending_changes;

#[path = "blizzard_ui/blizzard_addonlist/behavior_cancel_button_resets_changes.rs"]
mod behavior_cancel_button_resets_changes;
