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
pub mod c_cursor;
pub mod c_fog_of_war;
pub mod c_glue;
pub mod c_login;
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
pub mod c_ui;
pub mod c_widget;
pub mod c_wow_token_public;
pub mod c_wowtoken_secure;
pub mod c_xml_util;
pub mod item_spell;
pub mod permanent_shims;
pub mod temporary_shims;

mod helpers;
mod registration;

pub(crate) use helpers::{ensure_global_table, ensure_namespace, global_val, set_global_val};
pub use permanent_shims::c_map_api;
pub(crate) use registration::{
    register_character_progression_tables, register_character_services_tables,
    register_configuration_warning_tables, register_gossip_info_tables,
    register_interaction_tables, register_item_power_tables, register_map_environment_tables,
    register_map_prefix_tables, register_nameplate_tables, register_party_info_fallback_tables,
    register_pet_battle_fallback_tables, register_spell_and_widget_tables,
    register_world_activity_tables,
};

use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_utility_bootstrap_tables(state: &mut LuaState) -> LuaResult<()> {
    register_specialization_and_model_tables(state)?;
    register_glue_and_display_tables(state)?;
    register_auxiliary_utility_tables(state)
}

fn register_specialization_and_model_tables(state: &mut LuaState) -> LuaResult<()> {
    c_spec::register_c_specialization_info(state)?;
    temporary_shims::c_specialization_mastery::register_c_specialization_mastery_shim(state)?;
    temporary_shims::c_specialization_pvp_talents::register_c_specialization_pvp_talent_shims(
        state,
    )?;
    permanent_shims::c_model_info::register_c_model_info(state)?;
    Ok(())
}

fn register_glue_and_display_tables(state: &mut LuaState) -> LuaResult<()> {
    c_glue::register_c_glue(state)?;
    c_login::register_c_login(state)?;
    c_ui::register_c_ui(state)
}

fn register_auxiliary_utility_tables(state: &mut LuaState) -> LuaResult<()> {
    register_glue_auxiliary_shims(state)?;
    register_service_auxiliary_shims(state)?;
    register_token_texture_xml_tables(state)
}

fn register_glue_auxiliary_shims(state: &mut LuaState) -> LuaResult<()> {
    temporary_shims::c_chat_info::register_c_chat_info_shims(state)?;
    temporary_shims::c_campaign_covenant_defaults::register_campaign_covenant_default_shims(state)?;
    temporary_shims::c_contribution_collector::register_c_contribution_collector(state)?;
    temporary_shims::c_cooldown_viewer::register_c_cooldown_viewer_shims(state)?;
    temporary_shims::c_date_and_time::register_c_date_and_time_shims(state)?;
    temporary_shims::c_level_link_spell_lock::register_c_level_link_spell_lock_shim(state)?;
    temporary_shims::c_perks_program::register_c_perks_program(state)
}

fn register_service_auxiliary_shims(state: &mut LuaState) -> LuaResult<()> {
    temporary_shims::c_minimap::register_c_minimap_shims(state)?;
    temporary_shims::c_prototype_dialog::register_c_prototype_dialog_shims(state)?;
    temporary_shims::c_reincarnation::register_c_reincarnation_shims(state)?;
    temporary_shims::c_scenario::register_c_scenario_shims(state)?;
    temporary_shims::c_shared_character_services::register_c_shared_character_services_shims(
        state,
    )?;
    temporary_shims::c_super_track::register_c_super_track_shims(state)?;
    temporary_shims::c_trade_info::register_c_trade_info_shims(state)?;
    temporary_shims::c_transmog_outfit_slots::register_c_transmog_outfit_slot_shims(state)?;
    temporary_shims::c_transmog_sets::register_c_transmog_sets_shims(state)
}

fn register_token_texture_xml_tables(state: &mut LuaState) -> LuaResult<()> {
    c_wowtoken_secure::register_c_wowtoken_secure(state)?;
    c_wow_token_public::register_c_wow_token_public(state)?;
    c_texture::register_c_texture(state)?;
    c_xml_util::register_c_xml_util(state)
}
