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

#[path = "blizzard_ui/blizzard_actionbar/behavior_keybind_dispatch.rs"]
mod behavior_keybind_dispatch;

#[path = "blizzard_ui/blizzard_actionbar/behavior_show_grid.rs"]
mod behavior_show_grid;

#[path = "blizzard_ui/blizzard_actionbar/behavior_stance_select.rs"]
mod behavior_stance_select;

#[path = "blizzard_ui/blizzard_actionbar/behavior_pet_bar_update.rs"]
mod behavior_pet_bar_update;

#[path = "blizzard_ui/blizzard_actionbar/behavior_possess_bar_show.rs"]
mod behavior_possess_bar_show;

#[path = "blizzard_ui/blizzard_actionbar/behavior_extra_action_bar.rs"]
mod behavior_extra_action_bar;

#[path = "blizzard_ui/blizzard_actionbar/behavior_vehicle_leave_button.rs"]
mod behavior_vehicle_leave_button;

#[path = "blizzard_ui/blizzard_actionbar/behavior_xp_bar_update.rs"]
mod behavior_xp_bar_update;

#[path = "blizzard_ui/blizzard_actionbar/behavior_reputation_bar_update.rs"]
mod behavior_reputation_bar_update;

#[path = "blizzard_ui/blizzard_actionbar/behavior_honor_bar_update.rs"]
mod behavior_honor_bar_update;

#[path = "blizzard_ui/blizzard_actionbar/behavior_house_favor_bar.rs"]
mod behavior_house_favor_bar;
