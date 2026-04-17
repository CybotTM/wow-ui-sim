use super::{ensure_namespace, set_table_array};
use crate::lua_api::globals::hero_talents;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_table, frame_ref,
    table_set,
};
use crate::lua_api::script_helpers::{get_event_listeners, get_script};
use crate::lua_api::talent_state;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use crate::specializations;
use crate::spell_descriptions;
use crate::traits::{
    TRAIT_COND_DB, TRAIT_CURRENCY_DB, TRAIT_DEFINITION_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB,
    TRAIT_SUBTREE_DB, TRAIT_TREE_DB,
};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_trait_surfaces(state: &mut LuaState) -> LuaResult<()> {
    register_c_traits(state)?;
    register_c_class_talents(state)?;
    Ok(())
}

fn fire_named_event_with_arg(state: &mut LuaState, event_name: &str, arg: Val) {
    for widget_id in get_event_listeners(state, event_name) {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };
        let event_name_val = create_string(state, event_name);
        let _ = call_function_state(state, handler, &[frame, event_name_val, arg]);
    }
}

fn current_spec_id(state: &LuaState) -> Option<u32> {
    let sim = borrow_state(state).ok()?;
    let class_id = sim.player.class_index as u32;
    let spec_index = sim.player.active_spec_index.max(1) as usize - 1;
    specializations::specs_for_class(class_id)
        .nth(spec_index)
        .map(|spec| spec.id)
}

fn current_spec_set_id(state: &LuaState) -> u32 {
    match current_spec_id(state) {
        Some(65) => 27,
        Some(66) => 28,
        Some(70) => 29,
        _ => 0,
    }
}

fn config_name(config_id: i32) -> &'static str {
    match config_id {
        101 => "Holy Mythic+",
        102 => "Holy Raid",
        201 => "Protection Raid",
        202 => "Protection Mythic+",
        301 => "Retribution Raid",
        302 => "Retribution Mythic+",
        _ => "Default Loadout",
    }
}

fn config_spec_id(config_id: i32) -> Option<u32> {
    match config_id {
        101 | 102 => Some(65),
        201 | 202 => Some(66),
        301 | 302 => Some(70),
        _ => None,
    }
}

fn trait_node_spec_set(cond_ids: &[u32]) -> u32 {
    cond_ids
        .iter()
        .filter_map(|cond_id| TRAIT_COND_DB.get(cond_id))
        .find(|cond| cond.cond_type == 1)
        .map(|cond| cond.spec_set_id)
        .unwrap_or(0)
}

fn trait_node_is_visible(state: &LuaState, node_id: u32) -> bool {
    let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
        return true;
    };
    let required_spec_set = trait_node_spec_set(node.cond_ids);
    required_spec_set == 0 || required_spec_set == current_spec_set_id(state)
}

fn push_u32_array(state: &mut LuaState, values: impl IntoIterator<Item = u32>) -> Val {
    let table = create_table(state);
    for (index, value) in values.into_iter().enumerate() {
        set_table_array(state, table, index as i64 + 1, Val::Num(value as f64));
    }
    table
}

fn push_i32_array(state: &mut LuaState, values: impl IntoIterator<Item = i32>) -> Val {
    let table = create_table(state);
    for (index, value) in values.into_iter().enumerate() {
        set_table_array(state, table, index as i64 + 1, Val::Num(value as f64));
    }
    table
}

fn node_max_ranks(node: &crate::traits::TraitNodeInfo) -> u32 {
    node.entry_ids
        .first()
        .and_then(|entry_id| TRAIT_ENTRY_DB.get(entry_id))
        .map(|entry| entry.max_ranks)
        .unwrap_or(1)
}

fn total_node_max_ranks(node: &crate::traits::TraitNodeInfo) -> u32 {
    let total: u32 = node
        .entry_ids
        .iter()
        .filter_map(|entry_id| TRAIT_ENTRY_DB.get(entry_id))
        .map(|entry| entry.max_ranks)
        .sum();
    if total > 0 {
        total
    } else {
        node_max_ranks(node)
    }
}

fn spec_set_contains_active_spec(spec_set_id: u32, state: &crate::lua_api::SimState) -> bool {
    if spec_set_id == 0 {
        return true;
    }
    let active_spec_id = specializations::specs_for_class(state.player.class_index as u32)
        .nth((state.player.active_spec_index - 1).max(0) as usize)
        .map(|spec| spec.id)
        .unwrap_or(66);
    match spec_set_id {
        27 => active_spec_id == 65,
        28 => active_spec_id == 66,
        29 => active_spec_id == 70,
        _ => true,
    }
}

fn check_spec_conditions_met(
    node: &crate::traits::TraitNodeInfo,
    state: &crate::lua_api::SimState,
) -> bool {
    for &cond_id in node.cond_ids.iter().chain(node.group_cond_ids.iter()) {
        let Some(cond) = TRAIT_COND_DB.get(&cond_id) else {
            continue;
        };
        if cond.cond_type == 1 && !spec_set_contains_active_spec(cond.spec_set_id, state) {
            return false;
        }
    }
    true
}

fn check_node_available(
    node: &crate::traits::TraitNodeInfo,
    state: &crate::lua_api::SimState,
) -> bool {
    for &cond_id in node.cond_ids.iter() {
        let Some(cond) = TRAIT_COND_DB.get(&cond_id) else {
            continue;
        };
        if cond.cond_type != 0 || cond.currency_id == 0 {
            continue;
        }
        if state.talents.spent_for_currency(cond.currency_id) < cond.spent_amount {
            return false;
        }
    }
    true
}

fn check_edge_requirements(
    node: &crate::traits::TraitNodeInfo,
    state: &crate::lua_api::SimState,
) -> bool {
    let mut has_sufficient = false;
    let mut any_sufficient_met = false;
    for edge in node.edges {
        let purchased = state
            .talents
            .node_ranks
            .get(&edge.source_node_id)
            .copied()
            .unwrap_or(0)
            > 0;
        match edge.edge_type {
            2 => {
                has_sufficient = true;
                if purchased {
                    any_sufficient_met = true;
                }
            }
            3 => {
                if !purchased {
                    return false;
                }
            }
            _ => {}
        }
    }
    !has_sufficient || any_sufficient_met
}

fn max_points_for_currency(currency_id: u32) -> u32 {
    let Some(currency) = TRAIT_CURRENCY_DB.get(&currency_id) else {
        return 0;
    };
    match currency.flags {
        4 => 31,
        8 => 30,
        _ => 0,
    }
}

fn has_unspent_talent_points(state: &crate::lua_api::SimState) -> bool {
    let Some(tree) = TRAIT_TREE_DB.get(&790) else {
        return false;
    };
    tree.currency_ids.iter().any(|&currency_id| {
        let max_points = max_points_for_currency(currency_id);
        max_points > 0 && state.talents.spent_for_currency(currency_id) < max_points
    })
}

fn check_has_currency(node_id: u32, state: &crate::lua_api::SimState) -> bool {
    let Some(&currency_id) = state.talents.node_currency_map.get(&node_id) else {
        return true;
    };
    let max_points = max_points_for_currency(currency_id);
    max_points == 0 || state.talents.spent_for_currency(currency_id) < max_points
}

fn push_node_active_entry(state: &mut LuaState, info: Val, lookup_node_id: Option<u32>) {
    let active_entry = create_table(state);
    let entry_id = borrow_state(state)
        .ok()
        .and_then(|sim| lookup_node_id.and_then(|id| sim.talents.node_selections.get(&id).copied()))
        .or_else(|| {
            lookup_node_id
                .and_then(|id| TRAIT_NODE_DB.get(&id))
                .and_then(|node| node.entry_ids.first().copied())
        })
        .unwrap_or(0);
    let rank = borrow_state(state)
        .ok()
        .and_then(|sim| lookup_node_id.and_then(|id| sim.talents.node_ranks.get(&id).copied()))
        .unwrap_or(0);
    table_set(state, active_entry, "entryID", Val::Num(entry_id as f64));
    table_set(state, active_entry, "rank", Val::Num(rank as f64));
    table_set(state, info, "activeEntry", active_entry);
}

fn push_node_rank_fields(state: &mut LuaState, info: Val, lookup_node_id: Option<u32>) {
    let ranks_purchased = borrow_state(state)
        .ok()
        .and_then(|sim| lookup_node_id.and_then(|id| sim.talents.node_ranks.get(&id).copied()))
        .unwrap_or(0);
    let rank_val = Val::Num(ranks_purchased as f64);
    table_set(state, info, "ranksPurchased", rank_val);
    table_set(state, info, "currentRank", rank_val);
    table_set(state, info, "activeRank", rank_val);
    table_set(state, info, "ranksIncreased", rank_val);
    let entry_ranks_increased = create_table(state);
    table_set(
        state,
        info,
        "entryIDToRanksIncreased",
        entry_ranks_increased,
    );
}

fn push_node_array_fields(state: &mut LuaState, info: Val, node: &crate::traits::TraitNodeInfo) {
    let entry_ids = push_u32_array(state, node.entry_ids.iter().copied());
    table_set(state, info, "entryIDs", entry_ids);

    let condition_ids = push_u32_array(state, node.cond_ids.iter().copied());
    table_set(state, info, "conditionIDs", condition_ids);

    let group_ids = push_u32_array(state, node.group_ids.iter().copied());
    table_set(state, info, "groupIDs", group_ids);
}

fn push_node_visible_edges(
    state: &mut LuaState,
    info: Val,
    node: &crate::traits::TraitNodeInfo,
    active_source_nodes: &[u32],
) {
    let visible_edges = create_table(state);
    for (index, edge) in node.edges.iter().enumerate() {
        let edge_info = create_table(state);
        table_set(
            state,
            edge_info,
            "targetNode",
            Val::Num(edge.source_node_id as f64),
        );
        table_set(state, edge_info, "type", Val::Num(edge.edge_type as f64));
        table_set(
            state,
            edge_info,
            "visualStyle",
            Val::Num(edge.visual_style as f64),
        );
        let is_active = active_source_nodes.contains(&edge.source_node_id);
        table_set(state, edge_info, "isActive", Val::Bool(is_active));
        set_table_array(state, visible_edges, index as i64 + 1, edge_info);
    }
    table_set(state, info, "visibleEdges", visible_edges);
}

fn push_node_db_fields(state: &mut LuaState, info: Val, lookup_node_id: Option<u32>) {
    if let Some(node) = lookup_node_id.and_then(|id| TRAIT_NODE_DB.get(&id)) {
        let (
            ranks_purchased,
            active_entry_id,
            active_hero_subtree,
            is_spec_visible,
            is_available,
            meets_edge_requirements,
            has_currency,
            active_source_nodes,
        ) = match borrow_state(state).ok() {
            Some(sim) => (
                lookup_node_id
                    .and_then(|id| sim.talents.node_ranks.get(&id).copied())
                    .unwrap_or(0),
                lookup_node_id.and_then(|id| sim.talents.node_selections.get(&id).copied()),
                sim.talents.active_hero_subtree(),
                check_spec_conditions_met(node, &sim),
                check_node_available(node, &sim),
                check_edge_requirements(node, &sim),
                lookup_node_id.is_none_or(|node_id| check_has_currency(node_id, &sim)),
                sim.talents
                    .node_ranks
                    .iter()
                    .filter_map(|(&node_id, &rank)| (rank > 0).then_some(node_id))
                    .collect::<Vec<_>>(),
            ),
            None => (0, None, None, true, false, false, true, Vec::new()),
        };
        let total_max_ranks = total_node_max_ranks(node);
        let is_visible =
            lookup_node_id.is_some_and(|id| trait_node_is_visible(state, id)) && is_spec_visible;
        let can_purchase_rank = ranks_purchased < total_max_ranks
            && is_available
            && meets_edge_requirements
            && has_currency;

        table_set(state, info, "posX", Val::Num(node.pos_x as f64));
        table_set(state, info, "posY", Val::Num(node.pos_y as f64));
        table_set(state, info, "type", Val::Num(node.node_type as f64));
        table_set(state, info, "flags", Val::Num(node.flags as f64));
        push_node_array_fields(state, info, node);

        let entry_ids_with_committed_ranks = create_table(state);
        if let Some(active_entry_id) = active_entry_id.filter(|_| ranks_purchased > 0) {
            set_table_array(
                state,
                entry_ids_with_committed_ranks,
                1,
                Val::Num(active_entry_id as f64),
            );
        }
        table_set(
            state,
            info,
            "entryIDsWithCommittedRanks",
            entry_ids_with_committed_ranks,
        );

        table_set(
            state,
            info,
            "maxRanks",
            Val::Num(node_max_ranks(node) as f64),
        );
        table_set(
            state,
            info,
            "totalMaxRanks",
            Val::Num(total_max_ranks as f64),
        );
        table_set(state, info, "isVisible", Val::Bool(is_visible));
        table_set(state, info, "isAvailable", Val::Bool(is_available));
        table_set(state, info, "isDisplayError", Val::Bool(false));
        table_set(state, info, "canPurchaseRank", Val::Bool(can_purchase_rank));
        table_set(state, info, "canRefundRank", Val::Bool(ranks_purchased > 0));
        table_set(
            state,
            info,
            "meetsEdgeRequirements",
            Val::Bool(meets_edge_requirements),
        );
        table_set(state, info, "isCascadeRepurchasable", Val::Bool(false));
        table_set(state, info, "cascadeRepurchaseEntryID", Val::Nil);
        if node.sub_tree_id != 0 {
            table_set(state, info, "subTreeID", Val::Num(node.sub_tree_id as f64));
            let sub_tree_active = active_hero_subtree == Some(node.sub_tree_id);
            table_set(state, info, "subTreeActive", Val::Bool(sub_tree_active));
        } else {
            table_set(state, info, "subTreeID", Val::Nil);
            table_set(state, info, "subTreeActive", Val::Nil);
        }
        push_node_visible_edges(state, info, node, &active_source_nodes);
    } else {
        table_set(state, info, "posX", Val::Num(0.0));
        table_set(state, info, "posY", Val::Num(0.0));
        table_set(state, info, "type", Val::Num(0.0));
        table_set(state, info, "flags", Val::Num(0.0));
        let entry_ids = create_table(state);
        table_set(state, info, "entryIDs", entry_ids);
        let entry_ids_with_committed_ranks = create_table(state);
        table_set(
            state,
            info,
            "entryIDsWithCommittedRanks",
            entry_ids_with_committed_ranks,
        );
        let condition_ids = create_table(state);
        table_set(state, info, "conditionIDs", condition_ids);
        let group_ids = create_table(state);
        table_set(state, info, "groupIDs", group_ids);
        let visible_edges = create_table(state);
        table_set(state, info, "visibleEdges", visible_edges);
        table_set(state, info, "maxRanks", Val::Num(0.0));
        table_set(state, info, "totalMaxRanks", Val::Num(0.0));
        table_set(state, info, "isVisible", Val::Bool(false));
        table_set(state, info, "isAvailable", Val::Bool(false));
        table_set(state, info, "isDisplayError", Val::Bool(false));
        table_set(state, info, "canPurchaseRank", Val::Bool(false));
        table_set(state, info, "canRefundRank", Val::Bool(false));
        table_set(state, info, "meetsEdgeRequirements", Val::Bool(false));
        table_set(state, info, "isCascadeRepurchasable", Val::Bool(false));
        table_set(state, info, "cascadeRepurchaseEntryID", Val::Nil);
        table_set(state, info, "subTreeID", Val::Nil);
        table_set(state, info, "subTreeActive", Val::Nil);
    }
}

fn push_node_info(state: &mut LuaState, node_id: i32) -> Val {
    let info = create_table(state);
    table_set(state, info, "ID", Val::Num(node_id as f64));
    table_set(state, info, "id", Val::Num(node_id as f64));
    let lookup_node_id = u32::try_from(node_id).ok();
    push_node_active_entry(state, info, lookup_node_id);
    push_node_rank_fields(state, info, lookup_node_id);
    push_node_db_fields(state, info, lookup_node_id);
    info
}

fn config_ids_for_spec_id(spec_id: u32) -> Vec<i32> {
    talent_state::seeded_class_talent_configs(spec_id)
        .iter()
        .map(|config| config.id)
        .collect()
}

fn current_config_ids(state: &LuaState) -> Vec<i32> {
    current_spec_id(state)
        .map(config_ids_for_spec_id)
        .unwrap_or_default()
}

fn register_c_traits_query_fns(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table_ref,
        "GenerateImportString",
        c_traits_generate_import_string,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetConfigIDBySystemID",
        c_traits_get_config_id_by_system_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetConfigIDByTreeID",
        c_traits_get_config_id_by_tree_id,
    )?;
    table_set_rust_fn(state, table_ref, "GetConfigInfo", c_traits_get_config_info)?;
    table_set_rust_fn(state, table_ref, "GetNodeInfo", c_traits_get_node_info)?;
    table_set_rust_fn(state, table_ref, "GetEntryInfo", c_traits_get_entry_info)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetDefinitionInfo",
        c_traits_get_definition_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetTraitDescription",
        c_traits_get_trait_description,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetConditionInfo",
        c_traits_get_condition_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "InitializeViewLoadout",
        c_traits_initialize_view_loadout,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetTreeCurrencyInfo",
        c_traits_get_tree_currency_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetTraitCurrencyInfo",
        c_traits_get_trait_currency_info,
    )?;
    table_set_rust_fn(state, table_ref, "GetTreeInfo", c_traits_get_tree_info)?;
    table_set_rust_fn(state, table_ref, "GetTreeNodes", c_traits_get_tree_nodes)?;
    Ok(())
}

fn register_c_traits_action_fns(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn(state, table_ref, "GetAllTreeIDs", c_traits_get_all_tree_ids)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetTraitSystemFlags",
        c_traits_get_trait_system_flags,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "CanPurchaseRank",
        c_traits_can_purchase_rank,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetLoadoutSerializationVersion",
        c_traits_get_loadout_serialization_version,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "ConfigHasStagedChanges",
        c_traits_config_has_staged_changes,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetStagedChanges",
        c_traits_get_staged_changes,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetStagedChangesCost",
        c_traits_get_staged_changes_cost,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSubTreeInfo",
        c_traits_get_subtree_info,
    )?;
    table_set_rust_fn(state, table_ref, "GetNodeCost", c_traits_get_node_cost)?;
    table_set_rust_fn(state, table_ref, "SetSelection", c_traits_set_selection)?;
    table_set_rust_fn(state, table_ref, "PurchaseRank", c_traits_purchase_rank)?;
    table_set_rust_fn(state, table_ref, "RefundRank", c_traits_refund_rank)?;
    Ok(())
}

fn register_c_traits(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Traits")?;
    register_c_traits_query_fns(state, table_ref)?;
    register_c_traits_action_fns(state, table_ref)?;
    Ok(())
}

fn register_c_class_talents_hero_fns(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table_ref,
        "GetHeroTalentSpecsForClassSpec",
        c_class_talents_get_hero_talent_specs_for_class_spec,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetActiveHeroTalentSpec",
        c_class_talents_get_active_hero_talent_spec,
    )?;
    Ok(())
}

const CLASS_TALENTS_CONFIG_METHODS: &[(&str, rilua::RustFn)] = &[
    (
        "GetConfigIDsBySpecID",
        c_class_talents_get_config_ids_by_spec_id,
    ),
    ("GetActiveConfigID", c_class_talents_get_active_config_id),
    (
        "GetLastSelectedSavedConfigID",
        c_class_talents_get_last_selected_saved_config_id,
    ),
    (
        "GetTraitTreeForSpec",
        c_class_talents_get_trait_tree_for_spec,
    ),
    (
        "UpdateLastSelectedSavedConfigID",
        c_class_talents_update_last_selected_saved_config_id,
    ),
    ("CanEditTalents", c_class_talents_can_edit_talents),
    ("CanChangeTalents", c_class_talents_can_change_talents),
    ("GetHasStarterBuild", c_class_talents_get_has_starter_build),
    (
        "IsStarterBuildActive",
        c_class_talents_is_starter_build_active,
    ),
    (
        "GetStarterBuildActive",
        c_class_talents_is_starter_build_active,
    ),
    (
        "SetStarterBuildActive",
        c_class_talents_set_starter_build_active,
    ),
    (
        "GetNextStarterBuildPurchase",
        c_class_talents_get_next_starter_build_purchase,
    ),
    (
        "HasUnspentTalentPoints",
        c_class_talents_has_unspent_talent_points,
    ),
    (
        "HasUnspentHeroTalentPoints",
        c_class_talents_has_unspent_hero_talent_points,
    ),
];

fn register_c_class_talents_config_fns(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    for &(name, func) in CLASS_TALENTS_CONFIG_METHODS {
        table_set_rust_fn(state, table_ref, name, func)?;
    }
    Ok(())
}

fn register_c_class_talents_query_fns(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    register_c_class_talents_hero_fns(state, table_ref)?;
    register_c_class_talents_config_fns(state, table_ref)?;
    Ok(())
}

fn register_c_class_talents_action_fns(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToLoadoutByName",
        c_class_talents_switch_to_loadout_by_name,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToLoadoutByIndex",
        c_class_talents_switch_to_loadout_by_index,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToSpecializationByName",
        c_class_talents_switch_to_specialization_by_name,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToSpecializationByIndex",
        c_class_talents_switch_to_specialization_by_index,
    )?;
    Ok(())
}

fn register_c_class_talents(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_ClassTalents")?;
    register_c_class_talents_query_fns(state, table_ref)?;
    register_c_class_talents_action_fns(state, table_ref)?;
    Ok(())
}

fn c_traits_generate_import_string(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let import = create_string(state, &format!("RILUA:PALADIN:{config_id}"));
    state.push(import);
    Ok(1)
}

fn c_traits_get_config_id_by_system_id(state: &mut LuaState) -> LuaResult<u32> {
    let _system_id = i32::from_stack(state, 1)?;
    state.push(Val::Num(1.0));
    Ok(1)
}

fn c_traits_get_config_id_by_tree_id(state: &mut LuaState) -> LuaResult<u32> {
    let Val::Num(_) = stack_val(state, 1) else {
        return Ok(0);
    };
    state.push(Val::Num(1.0));
    Ok(1)
}

fn c_traits_get_config_info(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let info = create_table(state);
    table_set(state, info, "ID", Val::Num(config_id as f64));
    table_set(state, info, "id", Val::Num(config_id as f64));
    let name = create_string(state, config_name(config_id));
    table_set(state, info, "name", name);
    let tree_ids = push_u32_array(
        state,
        config_spec_id(config_id)
            .map(|spec_id| [c_class_talents_trait_tree_for_spec(spec_id)])
            .into_iter()
            .flatten(),
    );
    table_set(state, info, "treeIDs", tree_ids);
    state.push(info);
    Ok(1)
}

fn c_traits_get_node_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = i32::from_stack(state, 2)?;
    let info = push_node_info(state, node_id);
    state.push(info);
    Ok(1)
}

fn c_traits_get_entry_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let entry_id = u32::from_stack(state, 2)?;
    let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    table_set(state, info, "entryID", Val::Num(entry.id as f64));
    table_set(
        state,
        info,
        "definitionID",
        Val::Num(entry.definition_id as f64),
    );
    table_set(state, info, "type", Val::Num(entry.entry_type as f64));
    table_set(state, info, "maxRanks", Val::Num(entry.max_ranks as f64));
    table_set(state, info, "isAvailable", Val::Bool(true));
    table_set(state, info, "isDisplayError", Val::Bool(false));
    let condition_ids = create_table(state);
    table_set(state, info, "conditionIDs", condition_ids);
    if entry.sub_tree_id == 0 {
        table_set(state, info, "subTreeID", Val::Nil);
    } else {
        table_set(state, info, "subTreeID", Val::Num(entry.sub_tree_id as f64));
    }
    state.push(info);
    Ok(1)
}

fn c_traits_get_definition_info(state: &mut LuaState) -> LuaResult<u32> {
    let definition_id = u32::from_stack(state, 1)?;
    let Some(definition) = TRAIT_DEFINITION_DB.get(&definition_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = create_table(state);
    table_set(
        state,
        info,
        "spellID",
        if definition.spell_id == 0 {
            Val::Nil
        } else {
            Val::Num(definition.spell_id as f64)
        },
    );
    table_set(
        state,
        info,
        "overriddenSpellID",
        if definition.overrides_spell_id == 0 {
            Val::Nil
        } else {
            Val::Num(definition.overrides_spell_id as f64)
        },
    );
    table_set(
        state,
        info,
        "overrideIcon",
        if definition.override_icon == 0 {
            Val::Nil
        } else {
            Val::Num(definition.override_icon as f64)
        },
    );
    let override_name = create_string(state, definition.override_name);
    table_set(state, info, "overrideName", override_name);
    let override_subtext = create_string(state, definition.override_subtext);
    table_set(state, info, "overrideSubtext", override_subtext);
    let override_description = create_string(state, definition.override_description);
    table_set(state, info, "overrideDescription", override_description);
    state.push(info);
    Ok(1)
}

fn c_traits_get_trait_description(state: &mut LuaState) -> LuaResult<u32> {
    let entry_id = u32::from_stack(state, 1)?;
    let _rank = u32::from_stack(state, 2)?;
    let description = TRAIT_ENTRY_DB
        .get(&entry_id)
        .and_then(|entry| TRAIT_DEFINITION_DB.get(&entry.definition_id))
        .map(|definition| {
            if definition.override_description.is_empty() {
                spell_descriptions::get_spell_description(definition.spell_id)
                    .unwrap_or("")
                    .to_string()
            } else {
                definition.override_description.to_string()
            }
        })
        .unwrap_or_default();
    let description = create_string(state, &description);
    state.push(description);
    Ok(1)
}

fn c_traits_get_condition_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let cond_id = u32::from_stack(state, 2)?;
    let Some(cond) = TRAIT_COND_DB.get(&cond_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = create_table(state);
    table_set(state, info, "condID", Val::Num(cond_id as f64));
    table_set(
        state,
        info,
        "ranksGranted",
        Val::Num(cond.granted_ranks as f64),
    );
    table_set(
        state,
        info,
        "isAlwaysMet",
        Val::Bool(cond.currency_id == 0 && cond.spec_set_id == 0),
    );

    let is_met = match borrow_state(state).ok() {
        Some(sim) => match cond.cond_type {
            0 => {
                cond.currency_id == 0
                    || sim.talents.spent_for_currency(cond.currency_id) >= cond.spent_amount
            }
            1 => spec_set_contains_active_spec(cond.spec_set_id, &sim),
            2 => sim.player.level as u32 >= cond.required_level,
            _ => true,
        },
        None => false,
    };
    table_set(state, info, "isMet", Val::Bool(is_met));
    table_set(state, info, "isGate", Val::Bool(cond.currency_id != 0));
    table_set(state, info, "isSufficient", Val::Bool(is_met));
    table_set(state, info, "type", Val::Num(cond.cond_type as f64));
    table_set(
        state,
        info,
        "questID",
        if cond.quest_id == 0 {
            Val::Nil
        } else {
            Val::Num(cond.quest_id as f64)
        },
    );
    table_set(
        state,
        info,
        "achievementID",
        if cond.achievement_id == 0 {
            Val::Nil
        } else {
            Val::Num(cond.achievement_id as f64)
        },
    );
    table_set(
        state,
        info,
        "specSetID",
        if cond.spec_set_id == 0 {
            Val::Nil
        } else {
            Val::Num(cond.spec_set_id as f64)
        },
    );
    table_set(
        state,
        info,
        "playerLevel",
        if cond.required_level == 0 {
            Val::Nil
        } else {
            Val::Num(cond.required_level as f64)
        },
    );
    table_set(
        state,
        info,
        "traitCurrencyID",
        if cond.currency_id == 0 {
            Val::Nil
        } else {
            Val::Num(cond.currency_id as f64)
        },
    );
    table_set(
        state,
        info,
        "spentAmountRequired",
        if cond.spent_amount == 0 {
            Val::Nil
        } else {
            Val::Num(cond.spent_amount as f64)
        },
    );
    table_set(state, info, "tooltipFormat", Val::Nil);
    table_set(state, info, "traitCondAccountElementID", Val::Nil);
    state.push(info);
    Ok(1)
}

fn c_traits_initialize_view_loadout(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let _tree_id = i32::from_stack(state, 2)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn subtree_trait_currency_id(subtree_id: u32) -> Option<u32> {
    match subtree_id {
        48 => Some(2986),
        49 => Some(2987),
        50 => Some(2988),
        _ => None,
    }
}

fn active_hero_currency_id(state: &LuaState) -> Option<u32> {
    borrow_state(state)
        .ok()
        .and_then(|sim| sim.talents.active_hero_subtree())
        .and_then(subtree_trait_currency_id)
}

fn tree_currency_budget(state: &LuaState, index: usize, currency_id: u32) -> Option<u32> {
    match index {
        0 => Some(31),
        1 => Some(30),
        _ if active_hero_currency_id(state) == Some(currency_id) => Some(11),
        _ => None,
    }
}

fn push_tree_currency_info(
    state: &mut LuaState,
    trait_currency_id: u32,
    quantity: u32,
    max_quantity: Option<u32>,
    spent: u32,
) -> Val {
    let info = create_table(state);
    table_set(
        state,
        info,
        "traitCurrencyID",
        Val::Num(trait_currency_id as f64),
    );
    table_set(state, info, "quantity", Val::Num(quantity as f64));
    match max_quantity {
        Some(max_quantity) => table_set(state, info, "maxQuantity", Val::Num(max_quantity as f64)),
        None => table_set(state, info, "maxQuantity", Val::Nil),
    }
    table_set(state, info, "spent", Val::Num(spent as f64));
    info
}

fn c_traits_get_tree_currency_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let tree_id = u32::from_stack(state, 2)?;
    let _exclude_staged_changes = bool::from_stack(state, 3)?;
    let currencies = create_table(state);
    let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
        state.push(currencies);
        return Ok(1);
    };

    let spent_by_currency = borrow_state(state)
        .ok()
        .map(|sim| sim.talents.currency_spent.clone())
        .unwrap_or_default();

    for (index, &currency_id) in tree.currency_ids.iter().enumerate() {
        let spent = spent_by_currency.get(&currency_id).copied().unwrap_or(0);
        let budget = tree_currency_budget(state, index, currency_id);
        let quantity = budget.unwrap_or(0).saturating_sub(spent);
        let info = push_tree_currency_info(state, currency_id, quantity, budget, spent);
        set_table_array(state, currencies, index as i64 + 1, info);
    }

    state.push(currencies);
    Ok(1)
}

fn c_traits_get_trait_currency_info(state: &mut LuaState) -> LuaResult<u32> {
    let trait_currency_id = u32::from_stack(state, 1)?;
    let Some(currency) = TRAIT_CURRENCY_DB.get(&trait_currency_id) else {
        state.push(Val::Num(0.0));
        state.push(Val::Num(0.0));
        state.push(Val::Nil);
        state.push(Val::Nil);
        return Ok(4);
    };

    state.push(Val::Num(currency.flags as f64));
    state.push(Val::Num(0.0));
    if currency.currency_type == 0 {
        state.push(Val::Nil);
    } else {
        state.push(Val::Num(currency.currency_type as f64));
    }
    state.push(Val::Nil);
    Ok(4)
}

fn c_traits_get_tree_info(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = match stack_val(state, 1) {
        Val::Num(value) => value as i32,
        _ => 0,
    };
    let tree_id = match stack_val(state, 2) {
        Val::Num(value) => value as u32,
        _ => u32::from_stack(state, 1)?,
    };
    let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    table_set(state, info, "ID", Val::Num(tree.id as f64));
    table_set(state, info, "configID", Val::Num(config_id as f64));
    let gates = create_table(state);
    table_set(state, info, "gates", gates);
    table_set(state, info, "hideSinglePurchaseNodes", Val::Bool(false));
    table_set(state, info, "minZoom", Val::Num(0.75));
    table_set(state, info, "maxZoom", Val::Num(1.2));
    table_set(state, info, "buttonSize", Val::Num(40.0));
    table_set(state, info, "isLinkedToActiveConfigID", Val::Bool(true));
    table_set(
        state,
        info,
        "rootNodeID",
        if tree.first_node_id == 0 {
            Val::Nil
        } else {
            Val::Num(tree.first_node_id as f64)
        },
    );
    let currency_ids = push_u32_array(state, tree.currency_ids.iter().copied());
    table_set(state, info, "currencyIDs", currency_ids);
    state.push(info);
    Ok(1)
}

fn c_traits_get_tree_nodes(state: &mut LuaState) -> LuaResult<u32> {
    let tree_id = match stack_val(state, 2) {
        Val::Num(value) => value as u32,
        _ => match stack_val(state, 1) {
            Val::Num(value) => value as u32,
            _ => 0,
        },
    };
    let nodes = TRAIT_TREE_DB
        .get(&tree_id)
        .map(|tree| push_u32_array(state, tree.node_ids.iter().copied()))
        .unwrap_or_else(|| create_table(state));
    state.push(nodes);
    Ok(1)
}

fn c_traits_get_all_tree_ids(state: &mut LuaState) -> LuaResult<u32> {
    let tree_ids = push_u32_array(state, [1, 790, 994]);
    state.push(tree_ids);
    Ok(1)
}

fn c_traits_get_trait_system_flags(state: &mut LuaState) -> LuaResult<u32> {
    let _system_id = i32::from_stack(state, 1)?;
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_traits_can_purchase_rank(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let _node_id = u32::from_stack(state, 2)?;
    let _entry_id = u32::from_stack(state, 3)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_traits_get_loadout_serialization_version(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(2.0));
    Ok(1)
}

fn c_traits_config_has_staged_changes(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_traits_get_staged_changes(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let purchases = create_table(state);
    let refunds = create_table(state);
    let swaps = create_table(state);
    state.push(purchases);
    state.push(refunds);
    state.push(swaps);
    Ok(3)
}

fn c_traits_get_staged_changes_cost(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let costs = create_table(state);
    state.push(costs);
    Ok(1)
}

fn push_subtree_base_fields(
    state: &mut LuaState,
    info: Val,
    subtree: &crate::traits::TraitSubTreeInfo,
) {
    table_set(state, info, "ID", Val::Num(subtree.id as f64));
    table_set(state, info, "id", Val::Num(subtree.id as f64));
    let name = create_string(state, subtree.name);
    table_set(state, info, "name", name);
    let description = create_string(state, subtree.description);
    table_set(state, info, "description", description);
    table_set(
        state,
        info,
        "iconElementID",
        Val::Num(subtree.atlas_element_id as f64),
    );
}

fn push_subtree_hero_fields(state: &mut LuaState, info: Val, subtree_id: u32) {
    let selection_node_ids = push_u32_array(
        state,
        hero_talents::selection_node_ids_for_subtree(subtree_id)
            .into_iter()
            .map(|node_id| node_id as u32),
    );
    table_set(state, info, "subTreeSelectionNodeIDs", selection_node_ids);
    let (pos_x, pos_y) = hero_talents::subtree_position(subtree_id);
    table_set(state, info, "posX", Val::Num(pos_x as f64));
    table_set(state, info, "posY", Val::Num(pos_y as f64));
    let is_active = borrow_state(state)
        .ok()
        .and_then(|sim| sim.talents.active_hero_subtree())
        == Some(subtree_id);
    table_set(state, info, "isActive", Val::Bool(is_active));
    if let Some(currency_id) = subtree_trait_currency_id(subtree_id) {
        table_set(state, info, "traitCurrencyID", Val::Num(currency_id as f64));
    }
}

fn c_traits_get_subtree_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let subtree_id = u32::from_stack(state, 2)?;
    let Some(subtree) = TRAIT_SUBTREE_DB.get(&subtree_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    push_subtree_base_fields(state, info, subtree);
    push_subtree_hero_fields(state, info, subtree_id);
    state.push(info);
    Ok(1)
}

fn c_traits_get_node_cost(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    let costs = create_table(state);
    let currency_id = borrow_state(state)
        .ok()
        .and_then(|sim| sim.talents.node_currency_map.get(&node_id).copied());
    if let Some(currency_id) = currency_id {
        let cost = create_table(state);
        table_set(state, cost, "ID", Val::Num(currency_id as f64));
        table_set(state, cost, "amount", Val::Num(1.0));
        set_table_array(state, costs, 1, cost);
    }
    state.push(costs);
    Ok(1)
}

fn c_traits_set_selection(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    let entry_id = match stack_val(state, 3) {
        Val::Nil => None,
        Val::Num(value) => Some(value as u32),
        _ => None,
    };
    {
        let mut sim = borrow_state_mut(state)?;
        sim.talents.set_node_selection(node_id, entry_id);
        sim.talents
            .set_node_rank(node_id, u32::from(entry_id.is_some()));
    }
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_traits_purchase_rank(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    {
        let mut sim = borrow_state_mut(state)?;
        let next_rank = sim.talents.node_ranks.get(&node_id).copied().unwrap_or(0) + 1;
        sim.talents.set_node_rank(node_id, next_rank);
    }
    fire_named_event_with_arg(state, "TRAIT_NODE_CHANGED", Val::Num(node_id as f64));
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_traits_refund_rank(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.talents.set_node_rank(node_id, 0);
    }
    fire_named_event_with_arg(state, "TRAIT_NODE_CHANGED", Val::Num(node_id as f64));
    state.push(Val::Bool(true));
    Ok(1)
}

fn hero_specs_for_spec(spec_id: u32) -> &'static [u32] {
    match spec_id {
        65 => &[49, 50],
        66 => &[48, 49],
        70 => &[48, 50],
        _ => &[],
    }
}

fn c_class_talents_get_hero_talent_specs_for_class_spec(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = match (stack_val(state, 1), stack_val(state, 2)) {
        (_, Val::Num(value)) => value as u32,
        (Val::Num(value), Val::Nil) => config_spec_id(value as i32)
            .or_else(|| current_spec_id(state))
            .unwrap_or(0),
        _ => current_spec_id(state).unwrap_or(0),
    };
    let hero_specs = push_u32_array(state, hero_specs_for_spec(spec_id).iter().copied());
    state.push(hero_specs);
    state.push(Val::Num(71.0));
    Ok(2)
}

fn c_class_talents_get_active_hero_talent_spec(state: &mut LuaState) -> LuaResult<u32> {
    match borrow_state(state)
        .ok()
        .and_then(|sim| hero_talents::get_active_hero_subtree(&sim))
    {
        Some(subtree_id) => state.push(Val::Num(subtree_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_class_talents_get_config_ids_by_spec_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = u32::from_stack(state, 1)?;
    let config_ids = push_i32_array(state, config_ids_for_spec_id(spec_id));
    state.push(config_ids);
    Ok(1)
}

fn c_class_talents_get_active_config_id(state: &mut LuaState) -> LuaResult<u32> {
    let active_config_id = borrow_state(state)?.talents.active_config_id as f64;
    state.push(Val::Num(active_config_id));
    Ok(1)
}

fn c_class_talents_get_last_selected_saved_config_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = u32::from_stack(state, 1)?;
    let config_id = borrow_state(state)?
        .talents
        .last_selected_config_id_by_spec_id
        .get(&spec_id)
        .copied()
        .or_else(|| talent_state::default_class_talent_config_id(spec_id))
        .unwrap_or(0);
    state.push(Val::Num(config_id as f64));
    Ok(1)
}

fn c_class_talents_update_last_selected_saved_config_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = u32::from_stack(state, 1)?;
    let config_id = match stack_val(state, 2) {
        Val::Nil => None,
        Val::Num(value) => Some(value as i32),
        _ => None,
    };
    let mut sim = borrow_state_mut(state)?;
    match config_id {
        Some(config_id) => {
            sim.talents
                .last_selected_config_id_by_spec_id
                .insert(spec_id, config_id);
        }
        None => {
            sim.talents
                .last_selected_config_id_by_spec_id
                .remove(&spec_id);
        }
    }
    Ok(0)
}

fn c_class_talents_switch_to_loadout_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let config_id = current_config_ids(state)
        .into_iter()
        .find(|config_id| config_name(*config_id) == name)
        .unwrap_or_else(|| {
            borrow_state(state)
                .map(|sim| sim.talents.active_config_id)
                .unwrap_or(0)
        });
    if let Some(spec_id) = current_spec_id(state) {
        borrow_state_mut(state)?
            .talents
            .switch_to_loadout(spec_id, config_id);
    }
    Ok(0)
}

fn c_class_talents_switch_to_loadout_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?.max(1) as usize - 1;
    let configs = current_config_ids(state);
    if let Some(config_id) = configs.get(index).copied()
        && let Some(spec_id) = current_spec_id(state)
    {
        borrow_state_mut(state)?
            .talents
            .switch_to_loadout(spec_id, config_id);
    }
    Ok(0)
}

fn c_class_talents_switch_to_specialization_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let spec_name = String::from_stack(state, 1)?;
    let class_id = borrow_state(state)?.player.class_index as u32;
    let Some((index, spec)) = specializations::specs_for_class(class_id)
        .enumerate()
        .find(|(_, spec)| spec.name == spec_name)
    else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    sim.player.active_spec_index = index as i32 + 1;
    sim.talents.switch_to_spec(spec.id);
    Ok(0)
}

fn c_class_talents_switch_to_specialization_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let spec_index = i32::from_stack(state, 1)?.max(1) as usize - 1;
    let class_id = borrow_state(state)?.player.class_index as u32;
    let Some(spec) = specializations::specs_for_class(class_id).nth(spec_index) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    sim.player.active_spec_index = spec_index as i32 + 1;
    sim.talents.switch_to_spec(spec.id);
    Ok(0)
}

fn c_class_talents_get_trait_tree_for_spec(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(c_class_talents_trait_tree_for_spec(spec_id) as f64));
    Ok(1)
}

fn c_class_talents_can_change_talents(state: &mut LuaState) -> LuaResult<u32> {
    let can_change = borrow_state(state)?.talents.can_change_talents;
    state.push(Val::Bool(can_change));
    state.push(Val::Bool(can_change));
    if can_change {
        state.push(Val::Nil);
    } else {
        let reason = create_string(state, "You can't do that right now.");
        state.push(reason);
    }
    Ok(3)
}

fn c_class_talents_can_edit_talents(state: &mut LuaState) -> LuaResult<u32> {
    let can_edit = borrow_state(state)?.talents.can_change_talents;
    state.push(Val::Bool(can_edit));
    if can_edit {
        state.push(Val::Nil);
    } else {
        let reason = create_string(state, "You can't do that right now.");
        state.push(reason);
    }
    Ok(2)
}

fn c_class_talents_get_has_starter_build(state: &mut LuaState) -> LuaResult<u32> {
    let has_starter = borrow_state(state)?.talents.has_starter_build;
    state.push(Val::Bool(has_starter));
    Ok(1)
}

fn c_class_talents_is_starter_build_active(state: &mut LuaState) -> LuaResult<u32> {
    let is_active = borrow_state(state)?.talents.is_starter_build_active;
    state.push(Val::Bool(is_active));
    Ok(1)
}

fn c_class_talents_set_starter_build_active(state: &mut LuaState) -> LuaResult<u32> {
    let is_active = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.talents.is_starter_build_active = is_active;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_class_talents_get_next_starter_build_purchase(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

fn c_class_talents_has_unspent_talent_points(state: &mut LuaState) -> LuaResult<u32> {
    let has_unspent = {
        let sim = borrow_state(state)?;
        has_unspent_talent_points(&sim)
    };
    state.push(Val::Bool(has_unspent));
    Ok(1)
}

fn c_class_talents_has_unspent_hero_talent_points(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_class_talents_trait_tree_for_spec(_spec_id: u32) -> u32 {
    790
}
