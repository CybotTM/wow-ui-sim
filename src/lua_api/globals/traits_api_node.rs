//! Node/entry/definition/condition info builders for C_Traits.
//!
//! Split from traits_api.rs — these are the read-side data accessors.

use crate::lua_api::SimState;
use crate::traits::{TRAIT_COND_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TraitNodeInfo};
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub fn trait_entry_display_spell_id(def: &crate::traits::TraitDefInfo) -> Option<u32> {
    [def.visible_spell_id, def.spell_id, def.overrides_spell_id]
        .into_iter()
        .find(|id| *id != 0)
}

pub fn trait_entry_name(entry_id: u32) -> Option<String> {
    let entry = TRAIT_ENTRY_DB.get(&entry_id)?;
    let def = crate::traits::TRAIT_DEFINITION_DB.get(&entry.definition_id)?;

    if !def.override_name.is_empty() {
        return Some(def.override_name.to_string());
    }

    let spell_id = trait_entry_display_spell_id(def)?;
    crate::spells::get_spell(spell_id).map(|spell| spell.name.to_string())
}

pub fn trait_entry_description(entry_id: u32, _rank: u32) -> Option<String> {
    let entry = TRAIT_ENTRY_DB.get(&entry_id)?;
    let def = crate::traits::TRAIT_DEFINITION_DB.get(&entry.definition_id)?;

    if !def.override_description.is_empty() {
        return Some(def.override_description.to_string());
    }

    None
}

pub fn create_node_info(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    _config_id: Value,
    node_id: Value,
) -> Result<Value> {
    let node_id = match &node_id {
        Value::Integer(n) => *n as i32,
        Value::Number(n) => *n as i32,
        _ => return build_empty_node_info(lua, 0),
    };
    let Some(node) = TRAIT_NODE_DB.get(&(node_id as u32)) else {
        return build_empty_node_info(lua, node_id);
    };
    let info = lua.create_table()?;
    set_node_static_fields(lua, &info, node, node_id)?;
    set_node_contract_defaults(lua, &info, node)?;
    set_node_dynamic_fields(lua, &info, node, node_id as u32, state)?;
    Ok(Value::Table(info))
}

/// Static node fields that don't depend on talent state.
fn set_node_static_fields(
    lua: &Lua,
    info: &mlua::Table,
    node: &TraitNodeInfo,
    node_id: i32,
) -> Result<()> {
    info.set("ID", node_id)?;
    info.set("posX", node.pos_x)?;
    info.set("posY", node.pos_y)?;
    info.set("type", node.node_type as i32)?;
    info.set("flags", node.flags as i32)?;
    if node.sub_tree_id != 0 {
        info.set("subTreeID", node.sub_tree_id as i64)?;
    }
    build_node_entry_ids(lua, info, node)?;
    build_node_cond_ids(lua, info, node)?;
    build_node_group_ids(lua, info, node)?;
    info.set("isCascadeRepurchasable", false)?;
    Ok(())
}

/// Populate the documented non-nil TraitNodeInfo fields that Blizzard UI reads
/// unconditionally, even when a node is hidden or otherwise inactive.
fn set_node_contract_defaults(lua: &Lua, info: &mlua::Table, node: &TraitNodeInfo) -> Result<()> {
    info.set("entryIDsWithCommittedRanks", lua.create_table()?)?;
    info.set("isDisplayError", false)?;
    info.set("ranksIncreased", 0)?;
    info.set("entryIDToRanksIncreased", lua.create_table()?)?;
    info.set("totalMaxRanks", total_node_max_ranks(node))?;
    Ok(())
}

/// Dynamic node fields that depend on talent purchase state.
fn set_node_dynamic_fields(
    lua: &Lua,
    info: &mlua::Table,
    node: &TraitNodeInfo,
    node_id: u32,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let max_ranks = node_max_ranks(node);
    let s = state.borrow();

    // SubTreeSelection nodes (type 3): visible only if spec condition met.
    // Must read actual selection state so activation works.
    if node.node_type == 3 {
        let spec_ok = check_spec_conditions_met(node, &s);
        info.set("isVisible", spec_ok)?;
        info.set("isAvailable", spec_ok)?;
        return set_selection_node_ranks(info, lua, node, node_id, max_ranks, spec_ok, &s);
    }

    // Hero subtree nodes: set subTreeActive, then fall through to normal rank logic.
    if node.sub_tree_id != 0 {
        let active_subtree = super::hero_talents::get_active_hero_subtree(&s);
        let sub_tree_active = active_subtree == Some(node.sub_tree_id);
        info.set("subTreeActive", sub_tree_active)?;
    }

    // Nodes with a spec-set condition (condType=1) for the wrong spec are invisible.
    // Hero nodes skip this — their visibility is controlled by subTreeActive, and
    // their group conditions use OR semantics (visible if ANY spec matches).
    if node.sub_tree_id == 0 && !check_spec_conditions_met(node, &s) {
        info.set("isVisible", false)?;
        return set_empty_ranks(info, lua, max_ranks);
    }

    let ranks_purchased = *s.talents.node_ranks.get(&node_id).unwrap_or(&0) as i32;
    let is_available = check_node_available(node, &s);
    let meets_edges = check_edge_requirements(node, &s);
    let has_currency = check_has_currency(node_id, &s);
    let can_purchase = ranks_purchased < max_ranks && is_available && meets_edges && has_currency;

    info.set("currentRank", ranks_purchased)?;
    info.set("activeRank", ranks_purchased)?;
    info.set("ranksPurchased", ranks_purchased)?;
    info.set("maxRanks", max_ranks)?;
    info.set("isVisible", true)?;
    info.set("isAvailable", is_available)?;
    info.set("canPurchaseRank", can_purchase)?;
    info.set("canRefundRank", ranks_purchased > 0)?;
    info.set("meetsEdgeRequirements", meets_edges)?;
    build_node_edges_dynamic(lua, info, node, &s)?;
    build_active_entry(lua, info, node, node_id, ranks_purchased, &s)?;
    Ok(())
}

/// Rank fields for SubTreeSelection nodes.
/// Reads actual selection state so hero spec activation persists.
fn set_selection_node_ranks(
    info: &mlua::Table,
    lua: &Lua,
    node: &TraitNodeInfo,
    node_id: u32,
    max_ranks: i32,
    visible: bool,
    state: &crate::lua_api::SimState,
) -> Result<()> {
    let ranks = *state.talents.node_ranks.get(&node_id).unwrap_or(&0) as i32;
    info.set("currentRank", ranks)?;
    info.set("activeRank", ranks)?;
    info.set("ranksPurchased", ranks)?;
    info.set("maxRanks", max_ranks)?;
    info.set("canPurchaseRank", false)?;
    info.set("canRefundRank", ranks > 0)?;
    info.set("meetsEdgeRequirements", visible)?;
    info.set("visibleEdges", lua.create_table()?)?;
    let ae = lua.create_table()?;
    let entry_id = if ranks > 0 {
        state
            .talents
            .node_selections
            .get(&node_id)
            .copied()
            .unwrap_or_else(|| node.entry_ids.first().copied().unwrap_or(0))
    } else {
        0
    };
    ae.set("entryID", entry_id as i64)?;
    ae.set("rank", ranks)?;
    info.set("activeEntry", ae)?;
    Ok(())
}

/// Minimal rank fields for non-instantiated nodes.
fn set_empty_ranks(info: &mlua::Table, lua: &Lua, max_ranks: i32) -> Result<()> {
    info.set("currentRank", 0)?;
    info.set("activeRank", 0)?;
    info.set("ranksPurchased", 0)?;
    info.set("maxRanks", max_ranks)?;
    info.set("isAvailable", false)?;
    info.set("canPurchaseRank", false)?;
    info.set("canRefundRank", false)?;
    info.set("meetsEdgeRequirements", false)?;
    info.set("visibleEdges", lua.create_table()?)?;
    let ae = lua.create_table()?;
    ae.set("entryID", 0i64)?;
    ae.set("rank", 0)?;
    info.set("activeEntry", ae)?;
    Ok(())
}

fn build_active_entry(
    lua: &Lua,
    info: &mlua::Table,
    node: &TraitNodeInfo,
    node_id: u32,
    ranks_purchased: i32,
    state: &SimState,
) -> Result<()> {
    let active_entry = lua.create_table()?;
    let entry_id = if node.entry_ids.len() > 1 {
        // Choice node: use selected entry or first.
        state
            .talents
            .node_selections
            .get(&node_id)
            .copied()
            .unwrap_or_else(|| node.entry_ids.first().copied().unwrap_or(0))
    } else {
        node.entry_ids.first().copied().unwrap_or(0)
    };
    active_entry.set("entryID", entry_id as i64)?;
    active_entry.set("rank", ranks_purchased)?;
    info.set("activeEntry", active_entry)?;
    Ok(())
}

/// Check all gate conditions (cond_type==0) are met for this node.
fn check_node_available(node: &TraitNodeInfo, state: &SimState) -> bool {
    for &cid in node.cond_ids {
        let Some(cond) = TRAIT_COND_DB.get(&cid) else {
            continue;
        };
        if cond.cond_type != 0 {
            continue;
        }
        if cond.currency_id == 0 {
            continue;
        }
        let spent = state.talents.spent_for_currency(cond.currency_id);
        if spent < cond.spent_amount {
            return false;
        }
    }
    true
}

/// Check edge requirements using WoW semantics:
/// - type 2 (SufficientForAvailability): OR — any one satisfied is enough
/// - type 3 (RequiredForAvailability): AND — all must be satisfied
fn check_edge_requirements(node: &TraitNodeInfo, state: &SimState) -> bool {
    let mut has_sufficient = false;
    let mut any_sufficient_met = false;
    for edge in node.edges {
        let purchased = *state
            .talents
            .node_ranks
            .get(&edge.source_node_id)
            .unwrap_or(&0)
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

/// Check the node's currency has remaining points.
fn check_has_currency(node_id: u32, state: &SimState) -> bool {
    let Some(&cid) = state.talents.node_currency_map.get(&node_id) else {
        return true; // No currency mapped → free (hero nodes, etc.)
    };
    let max_pts = super::traits_api::max_points_for_currency(cid);
    if max_pts == 0 {
        return true;
    }
    state.talents.spent_for_currency(cid) < max_pts
}

/// Build edges with dynamic isActive based on source node purchase state.
/// Filters out cross-subtree edges: non-hero nodes only show edges to other
/// non-hero nodes, hero nodes only show edges within the same subtree.
fn build_node_edges_dynamic(
    lua: &Lua,
    info: &mlua::Table,
    node: &TraitNodeInfo,
    state: &SimState,
) -> Result<()> {
    let edges = lua.create_table()?;
    let mut idx = 0i64;
    for edge in node.edges.iter() {
        if !should_show_edge(node.sub_tree_id, edge.source_node_id) {
            continue;
        }
        // Filter edges to nodes hidden by spec conditions.
        if let Some(target) = TRAIT_NODE_DB.get(&edge.source_node_id) {
            if !check_spec_conditions_met(target, state) {
                continue;
            }
        }
        idx += 1;
        let e = lua.create_table()?;
        e.set("targetNode", edge.source_node_id as i64)?;
        e.set("type", edge.edge_type as i32)?;
        e.set("visualStyle", edge.visual_style as i32)?;
        let is_active = *state
            .talents
            .node_ranks
            .get(&edge.source_node_id)
            .unwrap_or(&0)
            > 0;
        e.set("isActive", is_active)?;
        edges.set(idx, e)?;
    }
    info.set("visibleEdges", edges)?;
    Ok(())
}

/// Filter cross-subtree edges: only show edges between nodes in the same
/// subtree (both hero or both non-hero).
fn should_show_edge(this_sub_tree: u32, target_node_id: u32) -> bool {
    let target_sub_tree = TRAIT_NODE_DB
        .get(&target_node_id)
        .map(|n| n.sub_tree_id)
        .unwrap_or(0);
    match (this_sub_tree, target_sub_tree) {
        (0, 0) => true,                     // both non-hero
        (a, b) if a != 0 && a == b => true, // same hero subtree
        _ => false,                         // cross-subtree
    }
}

/// Build a minimal nodeInfo for nodes not in the trait DB.
pub fn build_empty_node_info(lua: &Lua, node_id: i32) -> Result<Value> {
    let info = lua.create_table()?;
    info.set("ID", node_id)?;
    info.set("posX", 0)?;
    info.set("posY", 0)?;
    info.set("type", 0)?;
    info.set("flags", 0)?;
    info.set("entryIDs", lua.create_table()?)?;
    info.set("entryIDsWithCommittedRanks", lua.create_table()?)?;
    info.set("visibleEdges", lua.create_table()?)?;
    info.set("conditionIDs", lua.create_table()?)?;
    info.set("groupIDs", lua.create_table()?)?;
    set_empty_node_state(lua, &info)?;
    Ok(Value::Table(info))
}

fn set_empty_node_state(lua: &Lua, info: &mlua::Table) -> Result<()> {
    info.set("currentRank", 0)?;
    info.set("activeRank", 0)?;
    info.set("ranksPurchased", 0)?;
    info.set("ranksIncreased", 0)?;
    info.set("entryIDToRanksIncreased", lua.create_table()?)?;
    info.set("maxRanks", 0)?;
    info.set("totalMaxRanks", 0)?;
    let active_entry = lua.create_table()?;
    active_entry.set("entryID", 0i64)?;
    active_entry.set("rank", 0)?;
    info.set("activeEntry", active_entry)?;
    info.set("isVisible", false)?;
    info.set("isAvailable", false)?;
    info.set("isDisplayError", false)?;
    info.set("canPurchaseRank", false)?;
    info.set("canRefundRank", false)?;
    info.set("meetsEdgeRequirements", false)?;
    info.set("isCascadeRepurchasable", false)?;
    Ok(())
}

fn build_node_entry_ids(lua: &Lua, info: &mlua::Table, node: &TraitNodeInfo) -> Result<()> {
    let entry_ids = lua.create_table()?;
    for (i, &eid) in node.entry_ids.iter().enumerate() {
        entry_ids.set(i as i64 + 1, eid as i64)?;
    }
    info.set("entryIDs", entry_ids)?;
    Ok(())
}

fn build_node_cond_ids(lua: &Lua, info: &mlua::Table, node: &TraitNodeInfo) -> Result<()> {
    let cond_ids = lua.create_table()?;
    for (i, &cid) in node.cond_ids.iter().enumerate() {
        cond_ids.set(i as i64 + 1, cid as i64)?;
    }
    info.set("conditionIDs", cond_ids)?;
    Ok(())
}

fn build_node_group_ids(lua: &Lua, info: &mlua::Table, node: &TraitNodeInfo) -> Result<()> {
    let group_ids = lua.create_table()?;
    for (i, &gid) in node.group_ids.iter().enumerate() {
        group_ids.set(i as i64 + 1, gid as i64)?;
    }
    info.set("groupIDs", group_ids)?;
    Ok(())
}

/// Get max ranks for a node from its first entry.
pub fn node_max_ranks(node: &TraitNodeInfo) -> i32 {
    node.entry_ids
        .first()
        .and_then(|eid| TRAIT_ENTRY_DB.get(eid))
        .map(|e| e.max_ranks as i32)
        .unwrap_or(1)
}

fn total_node_max_ranks(node: &TraitNodeInfo) -> i32 {
    let total: i32 = node
        .entry_ids
        .iter()
        .filter_map(|eid| TRAIT_ENTRY_DB.get(eid))
        .map(|entry| entry.max_ranks as i32)
        .sum();
    if total > 0 {
        total
    } else {
        node_max_ranks(node)
    }
}

pub fn create_entry_info(lua: &Lua, (_config_id, entry_id): (i32, i32)) -> Result<Value> {
    use crate::traits::TRAIT_ENTRY_DB;
    let Some(entry) = TRAIT_ENTRY_DB.get(&(entry_id as u32)) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    info.set("entryID", entry_id)?;
    info.set("definitionID", entry.definition_id as i64)?;
    info.set("type", entry.entry_type as i32)?;
    info.set("maxRanks", entry.max_ranks as i32)?;
    if entry.sub_tree_id != 0 {
        info.set("subTreeID", entry.sub_tree_id as i64)?;
    }
    info.set("isAvailable", true)?;
    info.set("conditionIDs", lua.create_table()?)?;
    Ok(Value::Table(info))
}

pub fn create_definition_info(lua: &Lua, def_id: i32) -> Result<Value> {
    use crate::traits::TRAIT_DEFINITION_DB;
    let Some(def) = TRAIT_DEFINITION_DB.get(&(def_id as u32)) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    info.set(
        "spellID",
        if def.spell_id != 0 {
            Value::Integer(def.spell_id as i64)
        } else {
            Value::Nil
        },
    )?;
    info.set(
        "overriddenSpellID",
        if def.overrides_spell_id != 0 {
            Value::Integer(def.overrides_spell_id as i64)
        } else {
            Value::Nil
        },
    )?;
    info.set(
        "overrideIcon",
        if def.override_icon != 0 {
            Value::Integer(def.override_icon as i64)
        } else {
            Value::Nil
        },
    )?;
    info.set(
        "visibleSpellID",
        if def.visible_spell_id != 0 {
            Value::Integer(def.visible_spell_id as i64)
        } else {
            Value::Nil
        },
    )?;
    info.set("overrideName", def.override_name)?;
    info.set("overrideSubtext", def.override_subtext)?;
    info.set("overrideDescription", def.override_description)?;
    Ok(Value::Table(info))
}

/// Dynamic condition info — isMet depends on talent state.
pub fn create_condition_info(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    cond_id: i32,
) -> Result<Value> {
    let Some(cond) = TRAIT_COND_DB.get(&(cond_id as u32)) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    set_condition_static_fields(&info, cond, cond_id)?;
    let s = state.borrow();
    let is_met = evaluate_condition(cond, &s);
    info.set("isMet", is_met)?;
    info.set("isSufficient", is_met)?;
    Ok(Value::Table(info))
}

fn set_condition_static_fields(
    info: &mlua::Table,
    cond: &crate::traits::TraitCondInfo,
    cond_id: i32,
) -> Result<()> {
    info.set("condID", cond_id)?;
    info.set("condType", cond.cond_type as i32)?;
    info.set("traitCurrencyID", cond.currency_id as i64)?;
    info.set("spentAmountRequired", cond.spent_amount as i32)?;
    info.set("specSetID", cond.spec_set_id as i32)?;
    info.set("questID", cond.quest_id as i64)?;
    info.set("achievementID", cond.achievement_id as i64)?;
    info.set("requiredLevel", cond.required_level as i32)?;
    info.set("traitNodeGroupID", cond.group_id as i64)?;
    info.set("traitNodeID", cond.node_id as i64)?;
    info.set("grantedRanks", cond.granted_ranks as i32)?;
    Ok(())
}

/// Check if all spec-set conditions (condType=1) on a node are met.
/// Checks both direct node conditions and group-level conditions.
/// Returns false if any condition specifies a spec set that doesn't include the active spec.
fn check_spec_conditions_met(node: &TraitNodeInfo, state: &SimState) -> bool {
    for &cond_id in node.cond_ids.iter().chain(node.group_cond_ids.iter()) {
        if let Some(cond) = TRAIT_COND_DB.get(&cond_id) {
            if cond.cond_type == 1 && !spec_set_contains_active_spec(cond.spec_set_id, state) {
                return false;
            }
        }
    }
    true
}

/// Check if the active spec is a member of the given specSetID.
///
/// Paladin specSet mapping (from SpecSetMember DB2):
///   27 → 65 (Holy), 28 → 66 (Protection), 29 → 70 (Retribution)
fn spec_set_contains_active_spec(spec_set_id: u32, state: &SimState) -> bool {
    if spec_set_id == 0 {
        return true;
    } // No spec restriction
    let active_spec_id = crate::specializations::specs_for_class(state.player.class_index as u32)
        .nth((state.player.active_spec_index - 1).max(0) as usize)
        .map(|s| s.id)
        .unwrap_or(66); // fallback to Protection
    match spec_set_id {
        27 => active_spec_id == 65,
        28 => active_spec_id == 66,
        29 => active_spec_id == 70,
        _ => true, // Unknown spec set — assume visible
    }
}

/// Evaluate whether a trait condition is met based on current talent state.
fn evaluate_condition(cond: &crate::traits::TraitCondInfo, state: &SimState) -> bool {
    match cond.cond_type {
        0 => {
            // Gate: check spent amount for currency
            if cond.currency_id == 0 {
                return true;
            }
            state.talents.spent_for_currency(cond.currency_id) >= cond.spent_amount
        }
        1 => spec_set_contains_active_spec(cond.spec_set_id, state),
        2 => cond.required_level <= 80, // Level check: simulated level 80
        _ => true,                      // Granted ranks, misc: always met
    }
}

pub fn create_sub_tree_info(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    _config_id: i32,
    sub_tree_id: i32,
) -> Result<Value> {
    use super::hero_talents::{
        get_active_hero_subtree, selection_node_ids_for_subtree, subtree_position,
    };
    use crate::traits::TRAIT_SUBTREE_DB;
    let Some(st) = TRAIT_SUBTREE_DB.get(&(sub_tree_id as u32)) else {
        return Ok(Value::Nil);
    };
    let (pos_x, pos_y) = subtree_position(sub_tree_id as u32);
    let active_subtree = get_active_hero_subtree(&state.borrow());
    let is_active = active_subtree == Some(sub_tree_id as u32);
    let info = lua.create_table()?;
    info.set("ID", sub_tree_id)?;
    info.set("name", st.name)?;
    info.set("description", st.description)?;
    info.set("traitTreeID", st.tree_id as i64)?;
    info.set("iconElementID", st.atlas_element_id as i64)?;
    info.set("isActive", is_active)?;
    info.set("posX", pos_x)?;
    info.set("posY", pos_y)?;
    let sel_nodes = selection_node_ids_for_subtree(sub_tree_id as u32);
    let sel_table = lua.create_table()?;
    for (i, &nid) in sel_nodes.iter().enumerate() {
        sel_table.set(i as i64 + 1, nid as i64)?;
    }
    info.set("subTreeSelectionNodeIDs", sel_table)?;
    Ok(Value::Table(info))
}
