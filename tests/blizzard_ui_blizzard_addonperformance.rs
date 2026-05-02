//! Wrapper binary for Blizzard_AddOnPerformance tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_addonperformance/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_init_arms_ten_second_ticker.rs"]
mod behavior_init_arms_ten_second_ticker;
