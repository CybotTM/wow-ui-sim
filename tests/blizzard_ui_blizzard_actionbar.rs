//! Wrapper binary that pulls every per-aspect Blizzard_ActionBar
//! test file under `tests/blizzard_ui/blizzard_actionbar/` into a
//! single `cargo test --test ...` target. Cargo only auto-discovers
//! `tests/*.rs`, so the nested `load.rs` / `surface_*.rs` / `behavior_*.rs`
//! files declared by the per-addon plan template need a flat re-export here
//! to be reachable.

mod common;

#[path = "blizzard_ui/blizzard_actionbar/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_actionbar/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_actionbar/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_actionbar/surface_mixins.rs"]
mod surface_mixins;

#[path = "blizzard_ui/blizzard_actionbar/surface_events.rs"]
mod surface_events;

#[path = "blizzard_ui/blizzard_actionbar/behavior_main_bar_buttons.rs"]
mod behavior_main_bar_buttons;

#[path = "blizzard_ui/blizzard_actionbar/behavior_page_change.rs"]
mod behavior_page_change;
