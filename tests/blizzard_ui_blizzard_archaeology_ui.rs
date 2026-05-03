//! Wrapper binary for Blizzard_ArchaeologyUI tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_archaeology_ui/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_archaeology_ui/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_archaeology_ui/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_archaeology_ui/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_archaeology_ui/surface_events.rs"]
mod surface_events;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_show_opens_the_panel.rs"]
mod behavior_show_opens_the_panel;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_onload_seeds_title_and_pages.rs"]
mod behavior_onload_seeds_title_and_pages;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_summary_page_lists_races.rs"]
mod behavior_summary_page_lists_races;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_artifact_page_renders_selected.rs"]
mod behavior_artifact_page_renders_selected;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_solve_button_clicks_solve.rs"]
mod behavior_solve_button_clicks_solve;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_keystone_click_toggles_socket.rs"]
mod behavior_keystone_click_toggles_socket;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_completed_page_paginates.rs"]
mod behavior_completed_page_paginates;

#[path = "blizzard_ui/blizzard_archaeology_ui/behavior_race_filter_dropdown_filters_listing.rs"]
mod behavior_race_filter_dropdown_filters_listing;
