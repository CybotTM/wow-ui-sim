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

#[path = "blizzard_ui/blizzard_adventuremap/behavior_onload_registers_inset_update_event.rs"]
mod behavior_onload_registers_inset_update_event;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_onload_adds_three_data_providers.rs"]
mod behavior_onload_adds_three_data_providers;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_onshow_clears_area_ids_and_sets_map_id.rs"]
mod behavior_onshow_clears_area_ids_and_sets_map_id;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_onhide_calls_dialog_parent_hide_and_close.rs"]
mod behavior_onhide_calls_dialog_parent_hide_and_close;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_refresh_insets_skips_when_no_count.rs"]
mod behavior_refresh_insets_skips_when_no_count;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_refresh_insets_filters_by_area_table_id.rs"]
mod behavior_refresh_insets_filters_by_area_table_id;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_inset_initialize_sizes_and_positions.rs"]
mod behavior_inset_initialize_sizes_and_positions;

#[path = "blizzard_ui/blizzard_adventuremap/behavior_inset_collapse_releases_area_trigger.rs"]
mod behavior_inset_collapse_releases_area_trigger;
