//! Global WoW API modules that still exist in the current tree.

pub mod action_bar_api;
pub mod create_frame;
pub mod environment_restore;
pub mod global_frames;
pub mod hero_talents;
pub mod lua_duration_object;
pub mod rilua_admin;
pub mod rilua_create_frame;
pub mod rilua_font_strings_collection;
pub mod rilua_security;
pub mod rilua_stubs;
pub mod rilua_utility_system_spell;
pub mod spell_api;
pub mod spellbook_data;
pub mod strings;
pub mod template;
pub mod unit_api;

pub use super::globals_legacy::register_globals;
pub use strings::register_all_ui_strings;
