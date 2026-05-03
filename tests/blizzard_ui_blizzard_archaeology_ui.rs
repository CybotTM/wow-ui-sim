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
