//! Tests for global WoW API functions and pre-created global frames.
//!
//! Split into focused submodules:
//! - `global_functions` — pure utility globals (locale, build info, string
//!   helpers, hooksecurefunc, Mixin, presence checks).
//! - `frames_and_attributes` — UIParent, CreateFrame, CreateTexture, attribute
//!   propagation, table.unpack.
//! - `startup_globals` — value/return shape of legacy global functions
//!   (UnitPower, GetTime, ACTIONBAR_HOTKEY_FONT_COLOR, etc.).
//! - `startup_namespaces` — `C_*` namespace surfaces created during bootstrap.
//! - `runtime_subsystems` — text helpers, scroll scripts, C_Texture/atlas,
//!   animation runtime, gamepad cursor, unit state, popup/tooltip frames.

mod account_state_flags;
mod caa_constants;
mod combat_log_object;
mod frames_and_attributes;
mod global_functions;
mod housing_result;
mod item_collection_secret_aspects;
mod patch_12_0_0_audit_enums;
mod patch_12_0_0_chat_combat_audio_enums;
mod patch_12_0_0_combat_audio_party_percent_enums;
mod patch_12_0_0_combat_audio_percent_enums;
mod patch_12_0_0_combat_audio_player_cast_format_enums;
mod patch_12_0_0_combat_audio_player_health_format_enums;
mod patch_12_0_0_combat_audio_player_resource_format_enums;
mod patch_12_0_0_combat_audio_say_if_targeted_enums;
mod patch_12_0_0_combat_audio_spec_setting_enums;
mod patch_12_0_0_combat_audio_target_cast_format_enums;
mod patch_12_0_0_combat_audio_target_death_behavior_enums;
mod patch_12_0_0_combat_audio_target_health_format_enums;
mod patch_12_0_0_combat_audio_throttle_enums;
mod patch_12_0_0_combat_audio_type_enums;
mod patch_12_0_0_combat_audio_unit_enums;
mod patch_12_0_0_combat_log_message_order_enums;
mod patch_12_0_0_constants;
mod patch_12_0_0_cooldown_housing_enums;
mod patch_12_0_0_cooldown_viewer_alert_type_enums;
mod patch_12_0_0_crafting_order_item_flags_enums;
mod patch_12_0_0_crafting_order_item_type_enums;
mod patch_12_0_0_crafting_order_result_enums;
mod patch_12_0_0_damage_meter_numbers_enums;
mod patch_12_0_0_damage_meter_override_type_enums;
mod patch_12_0_0_damage_meter_session_type_enums;
mod patch_12_0_0_damage_meter_spell_details_display_type_enums;
mod patch_12_0_0_damage_meter_storage_type_enums;
mod patch_12_0_0_damage_meter_type_enums;
mod patch_12_0_0_damage_meter_visibility_enums;
mod patch_12_0_0_dungeon_encounter_creature_flags_enums;
mod patch_12_0_0_dungeon_encounter_flags_enums;
mod patch_12_0_0_dungeon_encounter_trigger_type_enums;
mod patch_12_0_0_duration_time_modifier_enums;
mod patch_12_0_0_edit_mode_account_aura_settings_enums;
mod patch_12_0_0_edit_mode_damage_meter_settings_enums;
mod patch_12_0_0_edit_mode_encounter_events_settings_enums;
mod patch_12_0_0_edit_mode_personal_resource_display_settings_enums;
mod patch_12_0_0_edit_mode_system_enums;
mod patch_12_0_0_edit_mode_unit_frame_settings_enums;
mod patch_12_0_0_encounter_event_cast_state_enums;
mod patch_12_0_0_encounter_event_iconmask_enums;
mod patch_12_0_0_encounter_event_severity_enums;
mod patch_12_0_0_encounter_events_icon_direction_enums;
mod patch_12_0_0_encounter_events_orientation_enums;
mod patch_12_0_0_encounter_events_visibility_enums;
mod patch_12_0_0_encounter_timeline_event_sort_direction_enums;
mod patch_12_0_0_encounter_timeline_event_source_enums;
mod patch_12_0_0_encounter_timeline_event_state_enums;
mod patch_12_0_0_encounter_timeline_icon_set_enums;
mod patch_12_0_0_encounter_timeline_track_enums;
mod patch_12_0_0_encounter_timeline_track_type_enums;
mod patch_12_0_0_expansion_landing_page_type_removals;
mod patch_12_0_0_expansion_level_enums;
mod patch_12_0_0_item_creation_context_removals;
mod patch_12_0_0_nameplate_cast_bar_display_enums;
mod patch_12_0_0_nameplate_enemy_npc_aura_display_enums;
mod patch_12_0_0_nameplate_enemy_player_aura_display_enums;
mod patch_12_0_0_nameplate_stack_type_enums;
mod patch_12_0_0_nameplate_style_enums;
mod patch_12_0_0_small_enums;
mod patch_12_0_0_ui_enum_metadata;
mod patch_12_0_0_unit_power_spell_ids;
mod patch_12_1_service_payloads;
mod runtime_subsystems;
mod startup_globals;
mod startup_namespaces;
mod transmog_outfit_enums;
mod transmog_situation;
