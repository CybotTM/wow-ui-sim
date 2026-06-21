use super::*;
mod registration;

static SUBTREE_CURRENCY_IDS: std::sync::LazyLock<std::collections::HashMap<u32, u32>> =
    std::sync::LazyLock::new(build_subtree_currency_ids);

const CONFIG_INFO_HASH_FIELDS: usize = 4;
const ENTRY_INFO_HASH_FIELDS: usize = 8;
const DEFINITION_INFO_HASH_FIELDS: usize = 6;
const CONDITION_INFO_HASH_FIELDS: usize = 15;

pub(super) fn register_c_traits(state: &mut LuaState) -> LuaResult<()> {
    registration::register_c_traits(state)
}

fn c_traits_generate_import_string(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let import = create_string(state, &format!("RILUA:PALADIN:{config_id}"));
    state.push(import);
    Ok(1)
}

fn c_traits_get_config_id_by_system_id(state: &mut LuaState) -> LuaResult<u32> {
    let system_id = i32::from_stack(state, 1)?;
    let Some(config_id) = config_id_for_system_id(state, system_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Num(config_id as f64));
    Ok(1)
}

fn c_traits_get_config_id_by_tree_id(state: &mut LuaState) -> LuaResult<u32> {
    let tree_id = u32::from_stack(state, 1)?;
    let Some(config_id) = config_id_for_tree_id(state, tree_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Num(config_id as f64));
    Ok(1)
}

fn c_traits_get_config_info(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let info = create_table_with_capacity(state, CONFIG_INFO_HASH_FIELDS);
    table_set(state, info, "ID", Val::Num(config_id as f64));
    table_set(state, info, "id", Val::Num(config_id as f64));
    let name = create_string(state, config_name(config_id));
    table_set(state, info, "name", name);
    let tree_ids = if config_id == DELVES_COMPANION_CONFIG_ID {
        push_u32_array(state, [DELVES_COMPANION_TRAIT_TREE_ID])
    } else {
        push_u32_array(
            state,
            config_spec_id(config_id).and_then(class_talent_tree_for_spec),
        )
    };
    table_set(state, info, "treeIDs", tree_ids);
    state.push(info);
    Ok(1)
}

fn c_traits_get_node_info(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let node_id = i32::from_stack(state, 2)?;
    let info = push_node_info(state, config_id, node_id);
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
    let info = create_table_with_capacity(state, ENTRY_INFO_HASH_FIELDS);
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
    let entry_cost = create_table(state);
    table_set(state, info, "entryCost", entry_cost);
    if entry.sub_tree_id == 0 {
        table_set(state, info, "subTreeID", Val::Nil);
    } else {
        table_set(state, info, "subTreeID", Val::Num(entry.sub_tree_id as f64));
    }
    state.push(info);
    Ok(1)
}

fn optional_u32_number(value: u32) -> Val {
    if value == 0 {
        Val::Nil
    } else {
        Val::Num(value as f64)
    }
}

fn push_definition_numeric_fields(
    state: &mut LuaState,
    info: Val,
    definition: &crate::traits::TraitDefInfo,
) {
    table_set(
        state,
        info,
        "spellID",
        optional_u32_number(definition.spell_id),
    );
    table_set(
        state,
        info,
        "overriddenSpellID",
        optional_u32_number(definition.overrides_spell_id),
    );
    table_set(
        state,
        info,
        "overrideIcon",
        optional_u32_number(definition.override_icon),
    );
}

fn push_definition_text_fields(
    state: &mut LuaState,
    info: Val,
    definition: &crate::traits::TraitDefInfo,
) {
    let override_name = create_string(state, definition.override_name);
    table_set(state, info, "overrideName", override_name);
    let override_subtext = create_string(state, definition.override_subtext);
    table_set(state, info, "overrideSubtext", override_subtext);
    let override_description = create_string(state, definition.override_description);
    table_set(state, info, "overrideDescription", override_description);
}

fn push_definition_info_table(state: &mut LuaState, definition: &crate::traits::TraitDefInfo) {
    let info = create_table_with_capacity(state, DEFINITION_INFO_HASH_FIELDS);
    push_definition_numeric_fields(state, info, definition);
    push_definition_text_fields(state, info, definition);
    state.push(info);
}

fn c_traits_get_definition_info(state: &mut LuaState) -> LuaResult<u32> {
    let definition_id = u32::from_stack(state, 1)?;
    let Some(definition) = TRAIT_DEFINITION_DB.get(&definition_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    push_definition_info_table(state, definition);
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
    let config_id = i32::from_stack(state, 1)?;
    let cond_id = u32::from_stack(state, 2)?;
    let Some(cond) = TRAIT_COND_DB.get(&cond_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = create_table_with_capacity(state, CONDITION_INFO_HASH_FIELDS);
    push_condition_base_fields(state, info, cond_id, cond);
    let is_met = trait_condition_is_met(state, config_id, cond);
    push_condition_state_fields(state, info, cond, is_met);
    push_condition_optional_fields(state, info, cond);
    state.push(info);
    Ok(1)
}

fn push_condition_base_fields(
    state: &mut LuaState,
    info: Val,
    cond_id: u32,
    cond: &crate::traits::TraitCondInfo,
) {
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
}

fn trait_condition_is_met(
    state: &LuaState,
    config_id: i32,
    cond: &crate::traits::TraitCondInfo,
) -> bool {
    match borrow_state(state).ok() {
        Some(sim) => match cond.cond_type {
            0 => {
                cond.currency_id == 0
                    || sim.talents.spent_for_currency(cond.currency_id) >= cond.spent_amount
            }
            1 => {
                spec_set_contains_spec(cond.spec_set_id, spec_id_for_config_query(config_id, &sim))
            }
            2 => sim.player.level as u32 >= cond.required_level,
            _ => true,
        },
        None => false,
    }
}

fn push_condition_state_fields(
    state: &mut LuaState,
    info: Val,
    cond: &crate::traits::TraitCondInfo,
    is_met: bool,
) {
    table_set(state, info, "isMet", Val::Bool(is_met));
    table_set(state, info, "isGate", Val::Bool(cond.currency_id != 0));
    table_set(state, info, "isSufficient", Val::Bool(is_met));
    table_set(state, info, "type", Val::Num(cond.cond_type as f64));
}

fn push_condition_optional_fields(
    state: &mut LuaState,
    info: Val,
    cond: &crate::traits::TraitCondInfo,
) {
    set_optional_trait_u32(state, info, "questID", cond.quest_id);
    set_optional_trait_u32(state, info, "achievementID", cond.achievement_id);
    set_optional_trait_u32(state, info, "specSetID", cond.spec_set_id);
    set_optional_trait_u32(state, info, "playerLevel", cond.required_level);
    set_optional_trait_u32(state, info, "traitCurrencyID", cond.currency_id);
    set_optional_trait_u32(state, info, "spentAmountRequired", cond.spent_amount);
    push_condition_empty_metadata_fields(state, info);
}

fn set_optional_trait_u32(state: &mut LuaState, info: Val, field_name: &str, value: u32) {
    table_set(state, info, field_name, optional_trait_u32(value));
}

fn push_condition_empty_metadata_fields(state: &mut LuaState, info: Val) {
    table_set(state, info, "tooltipFormat", Val::Nil);
    table_set(state, info, "traitCondAccountElementID", Val::Nil);
}

fn optional_trait_u32(value: u32) -> Val {
    if value == 0 {
        Val::Nil
    } else {
        Val::Num(value as f64)
    }
}

fn c_traits_initialize_view_loadout(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let _tree_id = i32::from_stack(state, 2)?;
    if let Some(spec_id) = config_spec_id(config_id) {
        borrow_state_mut(state)?
            .talents
            .initialize_view_loadout(spec_id);
    }
    state.push(Val::Bool(true));
    Ok(1)
}

pub(super) fn subtree_trait_currency_id(subtree_id: u32) -> Option<u32> {
    SUBTREE_CURRENCY_IDS.get(&subtree_id).copied()
}

fn build_subtree_currency_ids() -> std::collections::HashMap<u32, u32> {
    let mut by_tree: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
    for subtree in TRAIT_SUBTREE_DB.values() {
        by_tree.entry(subtree.tree_id).or_default().push(subtree.id);
    }

    let mut map = std::collections::HashMap::new();
    for (tree_id, mut subtree_ids) in by_tree {
        subtree_ids.sort_unstable();
        let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
            continue;
        };
        let Some(currency_ids) = hero_currency_ids(tree.currency_ids, subtree_ids.len()) else {
            continue;
        };
        for (subtree_id, currency_id) in subtree_ids.into_iter().zip(currency_ids.iter().copied()) {
            map.insert(subtree_id, currency_id);
        }
    }
    map
}

fn hero_currency_ids(currency_ids: &'static [u32], subtree_count: usize) -> Option<&'static [u32]> {
    if subtree_count == 0 || currency_ids.len() < subtree_count {
        return None;
    }
    let start = if currency_ids.len() >= subtree_count + 2 {
        2
    } else {
        currency_ids.len() - subtree_count
    };
    currency_ids.get(start..start + subtree_count)
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
        _ if active_hero_currency_id(state) == Some(currency_id) => Some(HERO_TALENT_POINT_BUDGET),
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

fn c_traits_get_tree_hash(state: &mut LuaState) -> LuaResult<u32> {
    let tree_id = u32::from_stack(state, 1)?;
    let hash = TRAIT_TREE_DB
        .get(&tree_id)
        .map(trait_tree_hash_bytes)
        .unwrap_or([0; 16]);
    let hash_table = push_u32_array(state, hash.into_iter().map(u32::from));
    state.push(hash_table);
    Ok(1)
}

fn tree_info_args(state: &mut LuaState) -> LuaResult<(i32, u32)> {
    let config_id = match stack_val(state, 1) {
        Val::Num(value) => value as i32,
        _ => 0,
    };
    let tree_id = match stack_val(state, 2) {
        Val::Num(value) => value as u32,
        _ => u32::from_stack(state, 1)?,
    };
    Ok((config_id, tree_id))
}

fn push_tree_info_table(state: &mut LuaState, config_id: i32, tree: &crate::traits::TraitTreeInfo) {
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
        optional_u32_number(tree.first_node_id),
    );
    let currency_ids = push_u32_array(state, tree.currency_ids.iter().copied());
    table_set(state, info, "currencyIDs", currency_ids);
    state.push(info);
}

fn c_traits_get_tree_info(state: &mut LuaState) -> LuaResult<u32> {
    let (config_id, tree_id) = tree_info_args(state)?;
    let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    push_tree_info_table(state, config_id, tree);
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
    let nodes = if tree_id == DELVES_COMPANION_TRAIT_TREE_ID {
        push_u32_array(state, DELVES_COMPANION_NODE_IDS)
    } else {
        TRAIT_TREE_DB
            .get(&tree_id)
            .map(|tree| push_u32_array(state, tree.node_ids.iter().copied()))
            .unwrap_or_else(|| create_table(state))
    };
    state.push(nodes);
    Ok(1)
}

fn c_traits_get_all_tree_ids(state: &mut LuaState) -> LuaResult<u32> {
    let tree_ids = push_u32_array(state, [1, 790, 994]);
    state.push(tree_ids);
    Ok(1)
}

fn c_traits_get_trait_system_flags(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    state.push(Val::Num(
        trait_system_flags_for_config(state, config_id) as f64
    ));
    Ok(1)
}

fn c_traits_can_purchase_rank(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    let entry_id = u32::from_stack(state, 3)?;
    let can_purchase = borrow_state(state)
        .ok()
        .and_then(|sim| {
            if !sim.talents.is_active_config(config_id) {
                return None;
            }
            TRAIT_NODE_DB.get(&node_id).map(|node| {
                let ranks_purchased = sim.talents.node_ranks.get(&node_id).copied().unwrap_or(0);
                let total_max_ranks = total_node_max_ranks(node);
                let entry_ok = node.entry_ids.is_empty() || node.entry_ids.contains(&entry_id);
                let is_available = check_node_available(node, &sim);
                let meets_edge_requirements = check_edge_requirements(node, &sim);
                let has_currency = check_has_currency(node_id, &sim);
                entry_ok
                    && ranks_purchased < total_max_ranks
                    && is_available
                    && meets_edge_requirements
                    && has_currency
            })
        })
        .unwrap_or(false);
    state.push(Val::Bool(can_purchase));
    Ok(1)
}

fn c_traits_get_loadout_serialization_version(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(2.0));
    Ok(1)
}

fn c_traits_config_has_staged_changes(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let has_changes = borrow_state(state)
        .ok()
        .map(|sim| sim.talents.has_staged_changes(config_id))
        .unwrap_or(false);
    state.push(Val::Bool(has_changes));
    Ok(1)
}

fn c_traits_get_staged_changes(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let Some((purchases, refunds, selection_swaps)) = borrow_state(state).ok().map(|sim| {
        (
            sim.talents.staged_purchases(config_id),
            sim.talents.staged_refunds(config_id),
            sim.talents.staged_selection_swaps(config_id),
        )
    }) else {
        return Ok(0);
    };
    if purchases.is_empty() && refunds.is_empty() && selection_swaps.is_empty() {
        return Ok(0);
    }

    let purchases_table = push_u32_array(state, purchases);
    let refunds_table = push_u32_array(state, refunds);
    let swaps_table = push_u32_array(state, selection_swaps);
    state.push(purchases_table);
    state.push(refunds_table);
    state.push(swaps_table);
    Ok(3)
}

fn c_traits_get_staged_changes_cost(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let Some(costs_data) = borrow_state(state)
        .ok()
        .map(|sim| sim.talents.staged_cost_deltas(config_id))
    else {
        let empty = create_table(state);
        state.push(empty);
        return Ok(1);
    };

    let costs = create_table(state);
    for (index, (currency_id, amount)) in costs_data.into_iter().enumerate() {
        let cost = create_table(state);
        table_set(state, cost, "ID", Val::Num(currency_id as f64));
        table_set(state, cost, "amount", Val::Num(amount as f64));
        set_table_array(state, costs, index as i64 + 1, cost);
    }
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
        hero_talents::selection_node_ids_for_subtree(subtree_id),
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
    fire_trait_node_changed_with_dependents(state, node_id);
    fire_trait_tree_currency_info_updated_for_node(state, node_id);
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_traits_purchase_rank(state: &mut LuaState) -> LuaResult<u32> {
    change_trait_rank(state, TraitRankChange::Purchase)
}

fn c_traits_refund_rank(state: &mut LuaState) -> LuaResult<u32> {
    change_trait_rank(state, TraitRankChange::Refund)
}

enum TraitRankChange {
    Purchase,
    Refund,
}

fn change_trait_rank(state: &mut LuaState, change: TraitRankChange) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    {
        let mut sim = borrow_state_mut(state)?;
        let current_rank = sim.talents.node_ranks.get(&node_id).copied().unwrap_or(0);
        let next_rank = match change {
            TraitRankChange::Purchase => current_rank + 1,
            TraitRankChange::Refund => 0,
        };
        sim.talents.set_node_rank(node_id, next_rank);
    }
    fire_trait_node_changed_with_dependents(state, node_id);
    fire_trait_tree_currency_info_updated_for_node(state, node_id);
    state.push(Val::Bool(true));
    Ok(1)
}
