//! Wrapper binary that pulls every per-aspect Blizzard_AchievementUI
//! test file under `tests/blizzard_ui/blizzard_achievementui/` into a
//! single `cargo test --test ...` target. Cargo only auto-discovers
//! `tests/*.rs`, so the nested `load.rs` / `surface_*.rs` / `behavior_*.rs`
//! files declared by the per-addon plan template need a flat re-export here
//! to be reachable.

mod common;

#[path = "blizzard_ui/blizzard_achievementui/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_achievementui/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_achievementui/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_achievementui/surface_events.rs"]
mod surface_events;

#[path = "blizzard_ui/blizzard_achievementui/surface_mixins.rs"]
mod surface_mixins;

#[path = "blizzard_ui/blizzard_achievementui/behavior_toggle.rs"]
mod behavior_toggle;

#[path = "blizzard_ui/blizzard_achievementui/behavior_progress_bar.rs"]
mod behavior_progress_bar;
