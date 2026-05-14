//! C_* namespace implementations.
//!
//! Real/state-backed surfaces live at the root of this module. Intentionally
//! unsupported compatibility gaps stay isolated under `permanent_shims`.
//! Stopgap empty-result stubs with a path to real impl live under `temporary_shims`.

pub mod c_addon_profiler;
pub mod c_addons;
pub mod c_allied_races;
pub mod c_ardenweald_gardening;
pub mod c_arrow_callout_manager;
pub mod c_artifact_relic_forge_ui;
pub mod c_artifact_ui;
pub mod c_azerite_empowered_item;
pub mod c_azerite_essence;
pub mod c_azerite_item;
pub mod c_barber_shop;
pub mod c_behavioral_messaging;
pub mod c_configuration_warnings;
pub mod c_cursor;
pub mod c_fog_of_war;
pub mod c_major_factions;
pub mod c_map;
pub mod c_map_exploration_info;
pub mod c_paper_doll_info;
pub mod c_player_interaction_manager;
pub mod c_spec;
pub mod c_spell;
pub mod c_spell_book;
pub mod c_spell_diminish;
pub mod c_texture;
pub mod c_widget;
pub mod c_wow_token_public;
pub mod c_wowtoken_secure;
pub mod c_xml_util;
pub mod item_spell;
pub mod permanent_shims;
pub mod temporary_shims;

mod helpers;

pub(crate) use helpers::{ensure_global_table, ensure_namespace, global_val, set_global_val};
pub use permanent_shims::c_map_api;
