//! Wrapper binary for Blizzard_AddOnPerformance tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_addonperformance/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_init_arms_ten_second_ticker.rs"]
mod behavior_init_arms_ten_second_ticker;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_combat_lockdown_short_circuits_check.rs"]
mod behavior_combat_lockdown_short_circuits_check;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_check_skips_when_no_message.rs"]
mod behavior_check_skips_when_no_message;
