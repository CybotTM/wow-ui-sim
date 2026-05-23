use super::permanent_shims::c_nameplate;
use super::temporary_shims::{
    c_addons_beta_policy, c_behavioral_messaging, c_character_services, c_click_bindings,
    c_configuration_warnings, c_gossip_info, c_major_faction_display, c_map_groups,
    c_merchant_raid_defaults, c_mythic_plus, c_paper_doll_stagger, c_party_info_instance_abandon,
    c_party_info_static_fallbacks, c_pet_battles_static_fallbacks, c_spell_classification,
    c_spell_counts, c_spell_priority_aura, c_spell_static_fallbacks, c_spell_target,
    c_ui_widget_manager_power_bar,
};
use super::{
    c_allied_races, c_ardenweald_gardening, c_arrow_callout_manager, c_artifact_relic_forge_ui,
    c_artifact_ui, c_azerite_empowered_item, c_azerite_essence, c_azerite_item, c_barber_shop,
    c_cursor, c_fog_of_war, c_major_factions, c_map, c_map_exploration_info, c_paper_doll_info,
    c_player_interaction_manager, c_spell, c_spell_diminish, c_widget,
};

use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_addon_policy_tables(state: &mut LuaState) -> LuaResult<()> {
    c_addons_beta_policy::register_c_addons_beta_policy(state)
}

pub(crate) fn register_spell_and_widget_tables(state: &mut LuaState) -> LuaResult<()> {
    c_spell::register_c_spell_surface(state)?;
    c_spell_classification::register_c_spell_classification_shims(state)?;
    c_spell_counts::register_c_spell_count_shims(state)?;
    c_spell_priority_aura::register_c_spell_priority_aura(state)?;
    c_spell_static_fallbacks::register_c_spell_static_fallbacks(state)?;
    c_spell_target::register_c_spell_target_shims(state)?;
    c_spell_diminish::register_c_spell_diminish_surface(state)?;
    c_widget::register_c_widget_surface(state)
}

pub(crate) fn register_item_power_tables(state: &mut LuaState) -> LuaResult<()> {
    c_paper_doll_info::register_c_paper_doll_info_surface(state)?;
    c_paper_doll_stagger::register_c_paper_doll_stagger_shim(state)?;
    c_artifact_ui::register_c_artifact_ui_surface(state)?;
    c_artifact_relic_forge_ui::register_c_artifact_relic_forge_ui_surface(state)?;
    c_azerite_item::register_c_azerite_item_surface(state)?;
    c_azerite_essence::register_c_azerite_essence_surface(state)?;
    c_azerite_empowered_item::register_c_azerite_empowered_item_surface(state)
}

pub(crate) fn register_character_progression_tables(state: &mut LuaState) -> LuaResult<()> {
    c_barber_shop::register_c_barber_shop_surface(state)?;
    c_cursor::register_c_cursor_surface(state)?;
    c_major_factions::register_c_major_factions_surface(state)?;
    c_major_faction_display::register_c_major_faction_display_shims(state)?;
    c_allied_races::register_c_allied_races_surface(state)
}

pub(crate) fn register_interaction_tables(state: &mut LuaState) -> LuaResult<()> {
    c_ardenweald_gardening::register_c_ardenweald_gardening_surface(state)?;
    c_arrow_callout_manager::register_c_arrow_callout_manager_surface(state)?;
    c_behavioral_messaging::register_c_behavioral_messaging(state)?;
    c_click_bindings::register_c_click_bindings_fallback(state)?;
    c_player_interaction_manager::register_c_player_interaction_manager_surface(state)
}

pub(crate) fn register_map_prefix_tables(state: &mut LuaState) -> LuaResult<()> {
    c_map::register_c_map_surface(state)?;
    c_map_groups::register_c_map_group_shims(state)
}

pub(crate) fn register_map_environment_tables(state: &mut LuaState) -> LuaResult<()> {
    c_fog_of_war::register_fog_of_war_surface(state)?;
    c_map_exploration_info::register_c_map_exploration_info_surface(state)
}

pub(crate) fn register_gossip_info_tables(state: &mut LuaState) -> LuaResult<()> {
    c_gossip_info::register_c_gossip_info_shims(state)
}

pub(crate) fn register_world_activity_tables(state: &mut LuaState) -> LuaResult<()> {
    c_mythic_plus::register_c_mythic_plus_shims(state)?;
    c_merchant_raid_defaults::register_c_merchant_and_raid_defaults(state)
}

pub(crate) fn register_nameplate_tables(state: &mut LuaState) -> LuaResult<()> {
    c_nameplate::register_c_nameplate(state)
}

pub(crate) fn register_ui_widget_power_bar_tables(state: &mut LuaState) -> LuaResult<()> {
    c_ui_widget_manager_power_bar::register_c_ui_widget_manager_power_bar(state)
}

pub(crate) fn register_configuration_warning_tables(state: &mut LuaState) -> LuaResult<()> {
    c_configuration_warnings::register_c_configuration_warnings(state)
}

pub(crate) fn register_character_services_tables(state: &mut LuaState) -> LuaResult<()> {
    c_character_services::register_c_character_services_shims(state)
}

pub(crate) fn register_party_info_fallback_tables(state: &mut LuaState) -> LuaResult<()> {
    c_party_info_instance_abandon::register_c_party_info_instance_abandon(state)?;
    c_party_info_static_fallbacks::register_c_party_info_static_fallbacks(state)
}

pub(crate) fn register_pet_battle_fallback_tables(state: &mut LuaState) -> LuaResult<()> {
    c_pet_battles_static_fallbacks::register_c_pet_battles_static_fallbacks(state)
}
