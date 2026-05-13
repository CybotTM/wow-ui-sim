//! Wrapper binary for Blizzard_AddOnPerformance tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_addonperformance/` are re-exported here.

use crate::common;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_init_arms_ten_second_ticker.rs"]
mod behavior_init_arms_ten_second_ticker;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_combat_lockdown_short_circuits_check.rs"]
mod behavior_combat_lockdown_short_circuits_check;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_check_skips_when_no_message.rs"]
mod behavior_check_skips_when_no_message;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_check_marks_message_type_seen.rs"]
mod behavior_check_marks_message_type_seen;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_check_dedupes_repeat_message_type.rs"]
mod behavior_check_dedupes_repeat_message_type;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_check_records_per_addon_warning.rs"]
mod behavior_check_records_per_addon_warning;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_check_refreshes_visible_addon_list.rs"]
mod behavior_check_refreshes_visible_addon_list;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_check_skips_addon_list_refresh_after_first_warning.rs"]
mod behavior_check_skips_addon_list_refresh_after_first_warning;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_display_specific_chat_warning.rs"]
mod behavior_display_specific_chat_warning;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_display_specific_error_dialog.rs"]
mod behavior_display_specific_error_dialog;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_display_overall_error_dialog.rs"]
mod behavior_display_overall_error_dialog;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_display_unknown_type_assertsafe.rs"]
mod behavior_display_unknown_type_assertsafe;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_specific_error_popup_disable_path.rs"]
mod behavior_specific_error_popup_disable_path;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_overall_error_popup_opens_addon_list.rs"]
mod behavior_overall_error_popup_opens_addon_list;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_addon_warning_predicate_returns_nil_when_unflagged.rs"]
mod behavior_addon_warning_predicate_returns_nil_when_unflagged;

#[path = "blizzard_ui/blizzard_addonperformance/behavior_specific_message_without_addon_name_is_invalid.rs"]
mod behavior_specific_message_without_addon_name_is_invalid;
