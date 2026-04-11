//! Global WoW API functions.
//!
//! This module contains the split WoW API implementations:
//! - `addon_api` - C_AddOns namespace and legacy addon functions
//! - `locale_api` - Locale, region, and build info functions
//! - `create_frame` - CreateFrame function implementation
//! - `unit_api` - Unit information functions (UnitName, UnitClass, etc.)
//! - `timer_api` - C_Timer namespace for timer management
//! - `enum_api` - Enum table with game enumerations
//! - `c_map_api` - C_Map and map/location related namespaces
//! - `c_quest_api` - C_QuestLog, C_TaskQuest, and quest related namespaces
//! - `c_collection_api` - C_MountJournal, C_PetJournal, C_ToyBox, C_Transmog, etc.
//! - `c_misc_api` - Miscellaneous C_* namespaces (C_ScenarioInfo, C_TooltipInfo, etc.)
//! - `c_system_api` - System C_* namespaces (C_XMLUtil, C_Console, C_VoiceChat, C_TTSSettings, etc.)
//! - `dropdown_api` - UIDropDownMenu system
//! - `strings` - UI string constants (ERR_*, localization, font codes, etc.)
//! - `utility_api` - Table manipulation (wipe, tinsert, tContains), string utilities, secure functions
//! - `font_api` - Font object creation (CreateFont, CreateFontFamily, standard fonts)
//! - `settings_api` - Settings namespace for addon configuration UI
//! - `mixin_api` - UI mixins (POIButtonMixin, MapCanvasPinMixin, Menu, MenuUtil)
//! - `player_api` - Player related functions (BattleNet, specialization, action bars)
//! - `cvar_api` - CVar and key binding functions
//! - `global_frames` - Global frame objects (UIParent, WorldFrame, PlayerFrame, etc.)
//!
//! The main `register_globals` function is still in `globals_legacy.rs`
//! but calls into these split modules.

pub mod abbreviate_config;
pub mod action_bar_api;
mod action_bar_api_namespace;
pub mod addon_api;
mod addon_api_runtime;
pub mod admin_api;
mod admin_api_mail_premade;
mod admin_api_world;
pub mod admin_combat;
pub mod admin_encounter;
pub mod aura_api;
pub mod bit_api;
pub mod c_collection_api;
mod c_collection_transmog;
pub mod c_container_api;
pub mod c_editmode_api;
pub mod c_event_utils_api;
pub mod c_item_api;
mod c_item_api_globals;
mod c_item_location_api;
pub mod c_mail_api;
pub mod c_map_api;
pub mod c_misc_api;
mod c_misc_api_core;
mod c_misc_api_core_progression;
mod c_misc_api_core_social;
mod c_misc_api_core_tooltip;
mod c_misc_api_game;
mod c_misc_api_game_systems;
mod c_misc_api_ui;
mod c_misc_api_ui_housing;
mod c_misc_api_ui_player;
pub mod c_quest_api;
mod c_quest_api_tasks;
pub mod c_stubs_achievement;
pub mod c_stubs_api;
pub mod c_stubs_api_chat_quest;
pub mod c_stubs_api_combat;
mod c_stubs_api_combat_curve;
mod c_stubs_api_combat_log;
mod c_stubs_api_encounter;
pub mod c_stubs_api_extra;
pub mod c_stubs_api_glue;
pub mod c_stubs_api_lfg;
mod c_stubs_api_missing;
mod c_stubs_api_missing_player_location;
mod c_stubs_api_namespaces;
pub mod c_stubs_api_professions;
pub mod c_stubs_api_secure;
mod c_stubs_api_guild_delves;
mod c_stubs_api_shop;
mod c_stubs_api_social;
pub mod c_stubs_api_store;
pub mod c_stubs_api_unit_frame;
pub mod c_system_api;
pub mod c_unit_auras_api;
pub mod constants_api;
pub mod create_frame;
mod create_frame_util;
pub mod currency_data;
pub mod cursor_api;
pub mod cvar_api;
pub mod dropdown_api;
pub mod early_globals;
pub mod enum_api;
pub mod enum_data;
pub mod environment_restore;
pub mod event_query_api;
pub mod fading_frame_api;
pub mod font_api;
pub mod frame_enumerate;
pub mod frame_level_api;
pub mod function_container;
pub mod generated_stubs;
pub mod global_frames;
pub mod hero_talents;
pub mod item_api;
pub mod locale_api;
pub mod lua_duration_object;
pub mod mixin_api;
pub mod nil_symbol_audit;
pub mod player_api;
pub(crate) mod player_api_helpers;
pub mod profession_data;
pub mod protected_call;
pub mod quest_frames;
pub mod reputation_data;
pub mod security_api;
pub mod settings_api;
pub mod sound_api;
pub mod spell_api;
pub mod spellbook_data;
pub mod strings;
pub mod system_api;
mod system_api_runtime;
pub mod targeting_api;
pub mod template;
pub mod timer_api;
pub mod tooltip_api;
pub mod traits_api;
pub mod traits_api_node;
pub mod unit_api;
mod unit_api_extra;
pub mod unit_combat_api;
pub mod unit_heal_prediction;
pub mod unit_health_power_api;
pub mod utility_api;
pub mod utility_stubs;

// Re-export for backwards compatibility
pub use strings::register_all_ui_strings;

pub use super::globals_legacy::register_globals;
