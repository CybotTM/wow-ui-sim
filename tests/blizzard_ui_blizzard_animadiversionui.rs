//! Wrapper binary for Blizzard_AnimaDiversionUI tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_animadiversionui/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_animadiversionui/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_animadiversionui/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_animadiversionui/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_animadiversionui/surface_events.rs"]
mod surface_events;

#[path = "blizzard_ui/blizzard_animadiversionui/surface_mixins.rs"]
mod surface_mixins;

#[path = "blizzard_ui/blizzard_animadiversionui/behavior_onload_seeds_data_providers_and_pin_levels.rs"]
mod behavior_onload_seeds_data_providers_and_pin_levels;

#[path = "blizzard_ui/blizzard_animadiversionui/behavior_onload_styles_close_button.rs"]
mod behavior_onload_styles_close_button;

#[path = "blizzard_ui/blizzard_animadiversionui/behavior_onhide_unregisters_events_and_plays_close_sound.rs"]
mod behavior_onhide_unregisters_events_and_plays_close_sound;

#[path = "blizzard_ui/blizzard_animadiversionui/behavior_onevent_anima_close_hides_panel.rs"]
mod behavior_onevent_anima_close_hides_panel;

#[path = "blizzard_ui/blizzard_animadiversionui/behavior_onshow_registers_events_and_plays_open_sound.rs"]
mod behavior_onshow_registers_events_and_plays_open_sound;

#[path = "blizzard_ui/blizzard_animadiversionui/behavior_try_show_seeds_state_and_opens_panel.rs"]
mod behavior_try_show_seeds_state_and_opens_panel;
