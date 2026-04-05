//! C_Traits namespace - talent/loadout system (Dragonflight+).
//!
//! Backed by static data from `data/traits.rs` and runtime state in `TalentState`.

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Build and return the C_Traits Lua table.
pub fn register_c_traits(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    register_c_traits_config(&t, lua, Rc::clone(&state))?;
    register_c_traits_tree(&t, lua, Rc::clone(&state))?;
    register_c_traits_node(&t, lua, state)?;
    Ok(t)
}

/// C_Traits config-level APIs.
fn register_c_traits_config(
    t: &mlua::Table,
    lua: &Lua,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_config_stubs(t, lua)?;
    register_config_mutations(t, lua, state)?;
    Ok(())
}

/// Stateless config stubs.
fn register_config_stubs(t: &mlua::Table, lua: &Lua) -> Result<()> {
    t.set(
        "GenerateImportString",
        lua.create_function(|_, _id: i32| Ok("dummy_talent_string".to_string()))?,
    )?;
    t.set(
        "GetConfigIDBySystemID",
        lua.create_function(|_, _id: i32| Ok(1i32))?,
    )?;
    t.set(
        "GetConfigIDByTreeID",
        lua.create_function(|_, _id: i32| Ok(1i32))?,
    )?;
    t.set("GetConfigInfo", lua.create_function(create_config_info)?)?;
    t.set(
        "CanPurchaseRank",
        lua.create_function(|_, (_a, _b, _c): (i32, i32, i32)| Ok(false))?,
    )?;
    t.set(
        "GetLoadoutSerializationVersion",
        lua.create_function(|_, ()| Ok(2i32))?,
    )?;
    t.set("CommitConfig", lua.create_function(|_, _id: i32| Ok(true))?)?;
    t.set(
        "RollbackConfig",
        lua.create_function(|_, _id: i32| Ok(true))?,
    )?;
    t.set(
        "GetStagedChanges",
        lua.create_function(|lua, _id: i32| {
            Ok((
                lua.create_table()?,
                lua.create_table()?,
                lua.create_table()?,
            ))
        })?,
    )?;
    t.set(
        "GetStagedChangesCost",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    t.set(
        "RefundAllRanks",
        lua.create_function(|_, (_a, _b): (i32, i32)| Ok(false))?,
    )?;
    t.set(
        "CascadeRepurchaseRanks",
        lua.create_function(|_, (_a, _b): (i32, i32)| Ok(false))?,
    )?;
    t.set(
        "ClearCascadeRepurchaseHistory",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    t.set(
        "GenerateInspectImportString",
        lua.create_function(|_, _unit: String| Ok("".to_string()))?,
    )?;
    t.set(
        "GetTreeHash",
        lua.create_function(|_, _id: i32| Ok("0".to_string()))?,
    )?;
    Ok(())
}

/// State-aware config mutations: PurchaseRank, RefundRank, SetSelection, Reset, etc.
fn register_config_mutations(
    t: &mlua::Table,
    lua: &Lua,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    let st = Rc::clone(&state);
    t.set(
        "PurchaseRank",
        lua.create_function(move |lua, (config_id, node_id): (i32, i32)| {
            purchase_rank(&st, lua, config_id, node_id as u32)
        })?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "RefundRank",
        lua.create_function(move |lua, (config_id, node_id): (i32, i32)| {
            refund_rank(&st, lua, config_id, node_id as u32)
        })?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "SetSelection",
        lua.create_function(
            move |lua, (config_id, node_id, entry_id): (i32, i32, Option<i32>)| {
                set_selection(
                    &st,
                    lua,
                    config_id,
                    node_id as u32,
                    entry_id.map(|id| id as u32),
                )
            },
        )?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "ConfigHasStagedChanges",
        lua.create_function(move |_, _id: i32| {
            Ok(st.borrow().talents.node_ranks.values().any(|&r| r > 0))
        })?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "ResetTree",
        lua.create_function(move |lua, config_id: i32| reset_tree(&st, lua, config_id))?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "ResetTreeByCurrency",
        lua.create_function(move |lua, (config_id, currency_id): (i32, i32)| {
            reset_tree_by_currency(&st, lua, config_id, currency_id as u32)
        })?,
    )?;

    Ok(())
}

fn purchase_rank(
    state: &Rc<RefCell<SimState>>,
    lua: &Lua,
    _config_id: i32,
    node_id: u32,
) -> Result<bool> {
    use crate::traits::TRAIT_NODE_DB;
    let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
        return Ok(false);
    };
    let max_ranks = super::traits_api_node::node_max_ranks(node);
    let mut s = state.borrow_mut();
    let current = *s.talents.node_ranks.get(&node_id).unwrap_or(&0);
    if current >= max_ranks as u32 {
        return Ok(false);
    }
    let old_spent = currency_spent_before_change(&s, node_id);
    s.talents.set_node_rank(node_id, current + 1);
    let affected = compute_affected_nodes(node_id, &s, old_spent);
    drop(s);
    fire_trait_nodes_changed_for(lua, &affected)
}

fn refund_rank(
    state: &Rc<RefCell<SimState>>,
    lua: &Lua,
    _config_id: i32,
    node_id: u32,
) -> Result<bool> {
    let mut s = state.borrow_mut();
    let current = *s.talents.node_ranks.get(&node_id).unwrap_or(&0);
    if current == 0 {
        return Ok(false);
    }
    let old_spent = currency_spent_before_change(&s, node_id);
    if current == 1 {
        s.talents.set_node_rank(node_id, 0);
        s.talents.set_node_selection(node_id, None);
    } else {
        s.talents.set_node_rank(node_id, current - 1);
    }
    let affected = compute_affected_nodes(node_id, &s, old_spent);
    drop(s);
    fire_trait_nodes_changed_for(lua, &affected)
}

fn set_selection(
    state: &Rc<RefCell<SimState>>,
    lua: &Lua,
    _config_id: i32,
    node_id: u32,
    entry_id: Option<u32>,
) -> Result<bool> {
    let mut s = state.borrow_mut();
    let old_spent = currency_spent_before_change(&s, node_id);
    match entry_id {
        Some(eid) => {
            // Select an entry: set selection and ensure rank >= 1.
            s.talents.set_node_selection(node_id, Some(eid));
            let current = *s.talents.node_ranks.get(&node_id).unwrap_or(&0);
            if current == 0 {
                s.talents.set_node_rank(node_id, 1);
            }
        }
        None => {
            // nil entry_id = deselect/refund the selection node.
            let current = *s.talents.node_ranks.get(&node_id).unwrap_or(&0);
            if current == 0 {
                return Ok(false);
            }
            s.talents.set_node_rank(node_id, 0);
            s.talents.set_node_selection(node_id, None);
        }
    }
    let affected = compute_affected_nodes(node_id, &s, old_spent);
    drop(s);
    fire_trait_nodes_changed_for(lua, &affected)?;
    fire_sub_tree_changed_if_needed(lua, node_id, entry_id)?;
    Ok(true)
}

/// Fire TRAIT_SUB_TREE_CHANGED when a SubTreeSelection node (type 3) gets a new entry.
fn fire_sub_tree_changed_if_needed(
    lua: &Lua,
    node_id: u32,
    entry_id: Option<u32>,
) -> Result<()> {
    use crate::traits::{TRAIT_ENTRY_DB, TRAIT_NODE_DB};
    let Some(eid) = entry_id else { return Ok(()) };
    let Some(node) = TRAIT_NODE_DB.get(&node_id) else { return Ok(()) };
    if node.node_type != 3 { return Ok(()) }
    let Some(entry) = TRAIT_ENTRY_DB.get(&eid) else { return Ok(()) };
    if entry.sub_tree_id == 0 { return Ok(()) }
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    fire.call::<()>((
        lua.create_string("TRAIT_SUB_TREE_CHANGED")?,
        entry.sub_tree_id as i64,
    ))
}

fn reset_tree(state: &Rc<RefCell<SimState>>, lua: &Lua, config_id: i32) -> Result<bool> {
    let mut s = state.borrow_mut();
    s.talents.clear_ranks();
    s.talents.node_selections.clear();
    s.talents.active_hero_subtree_id = None;
    drop(s);
    fire_trait_config_updated(lua, config_id)
}

fn reset_tree_by_currency(
    state: &Rc<RefCell<SimState>>,
    lua: &Lua,
    config_id: i32,
    currency_id: u32,
) -> Result<bool> {
    let mut s = state.borrow_mut();
    let nodes_to_clear: Vec<u32> = s
        .talents
        .node_ranks
        .keys()
        .filter(|nid| s.talents.node_currency_map.get(nid) == Some(&currency_id))
        .copied()
        .collect();
    for nid in &nodes_to_clear {
        s.talents.set_node_rank(*nid, 0);
        s.talents.set_node_selection(*nid, None);
    }
    drop(s);
    fire_trait_config_updated(lua, config_id)
}

/// Get the spent amount for the changed node's currency before mutation.
fn currency_spent_before_change(state: &SimState, node_id: u32) -> Option<u32> {
    state
        .talents
        .node_currency_map
        .get(&node_id)
        .map(|&cid| state.talents.spent_for_currency(cid))
}

/// Compute the set of nodes affected by a change to `changed_node_id`:
/// - The node itself
/// - Nodes with edges pointing to it (dependents whose meetsEdgeRequirements may flip)
/// - Nodes with gate conditions whose threshold is crossed by this point change
fn compute_affected_nodes(
    changed_node_id: u32,
    state: &SimState,
    old_spent: Option<u32>,
) -> Vec<u32> {
    use crate::traits::{TRAIT_COND_DB, TRAIT_NODE_DB, TRAIT_TREE_DB};
    let Some(tree) = TRAIT_TREE_DB.get(&790) else {
        return vec![changed_node_id];
    };
    let changed_currency = state
        .talents
        .node_currency_map
        .get(&changed_node_id)
        .copied();
    let new_spent = changed_currency.map(|cid| state.talents.spent_for_currency(cid));

    let mut affected = vec![changed_node_id];
    for &nid in tree.node_ids {
        if nid == changed_node_id {
            continue;
        }
        let Some(node) = TRAIT_NODE_DB.get(&nid) else {
            continue;
        };
        let is_dependent = node
            .edges
            .iter()
            .any(|e| e.source_node_id == changed_node_id && e.edge_type > 0);
        // Only fire for gate nodes whose threshold is actually crossed.
        let is_gate_crossed = changed_currency.map_or(false, |ccy| {
            node.cond_ids.iter().any(|&cid| {
                TRAIT_COND_DB.get(&cid).map_or(false, |c| {
                    c.cond_type == 0
                        && c.currency_id == ccy
                        && gate_threshold_crossed(old_spent, new_spent, c.spent_amount)
                })
            })
        });
        if is_dependent || is_gate_crossed {
            affected.push(nid);
        }
    }
    affected
}

/// True if the gate threshold was crossed: met before but not after, or vice versa.
fn gate_threshold_crossed(old_spent: Option<u32>, new_spent: Option<u32>, threshold: u32) -> bool {
    match (old_spent, new_spent) {
        (Some(old), Some(new)) => (old >= threshold) != (new >= threshold),
        _ => false,
    }
}

/// Fire TRAIT_NODE_CHANGED for a specific set of affected nodes.
///
/// Does NOT fire TRAIT_TREE_CURRENCY_INFO_UPDATED — in WoW, that event fires
/// after CommitConfig (server confirms), not on individual staging changes.
fn fire_trait_nodes_changed_for(lua: &Lua, affected: &[u32]) -> Result<bool> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let event = lua.create_string("TRAIT_NODE_CHANGED")?;
    for &nid in affected {
        fire.call::<()>((event.clone(), nid as i64))?;
    }
    Ok(true)
}

/// Fire all events after a config commit or full reset:
/// - TRAIT_NODE_CHANGED for ALL nodes (full invalidation)
/// - TRAIT_TREE_CURRENCY_INFO_UPDATED + TRAIT_CONFIG_UPDATED
fn fire_trait_config_updated(lua: &Lua, config_id: i32) -> Result<bool> {
    use crate::traits::TRAIT_TREE_DB;
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let event = lua.create_string("TRAIT_NODE_CHANGED")?;
    if let Some(tree) = TRAIT_TREE_DB.get(&790) {
        for &nid in tree.node_ids {
            fire.call::<()>((event.clone(), nid as i64))?;
        }
    }
    fire_currency_updated_event(lua)?;
    fire.call::<()>((lua.create_string("TRAIT_CONFIG_UPDATED")?, config_id as i64))?;
    Ok(true)
}

fn fire_currency_updated_event(lua: &Lua) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    fire.call::<()>((
        lua.create_string("TRAIT_TREE_CURRENCY_INFO_UPDATED")?,
        790i64,
    ))?;
    Ok(())
}

/// C_Traits tree-level APIs.
fn register_c_traits_tree(t: &mlua::Table, lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set(
        "InitializeViewLoadout",
        lua.create_function(|_, (_a, _b): (i32, i32)| Ok(true))?,
    )?;
    t.set("GetTreeInfo", lua.create_function(create_tree_info)?)?;
    t.set("GetTreeNodes", lua.create_function(create_tree_nodes)?)?;
    t.set(
        "GetAllTreeIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetTraitSystemFlags",
        lua.create_function(|_, _id: i32| Ok(0))?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "GetTreeCurrencyInfo",
        lua.create_function(move |lua, (_config_id, tree_id): (i32, i32)| {
            create_tree_currency_info(lua, &st, tree_id)
        })?,
    )?;

    Ok(())
}

/// C_Traits node/entry/definition-level APIs.
fn register_c_traits_node(t: &mlua::Table, lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let st = Rc::clone(&state);
    t.set(
        "GetNodeInfo",
        lua.create_function(move |lua, (cfg, nid): (Value, Value)| {
            super::traits_api_node::create_node_info(lua, &st, cfg, nid)
        })?,
    )?;

    t.set(
        "GetEntryInfo",
        lua.create_function(super::traits_api_node::create_entry_info)?,
    )?;
    t.set(
        "GetDefinitionInfo",
        lua.create_function(super::traits_api_node::create_definition_info)?,
    )?;
    t.set(
        "GetTraitDescription",
        lua.create_function(|_, (entry_id, rank): (i32, i32)| {
            Ok(
                super::traits_api_node::trait_entry_description(entry_id as u32, rank as u32)
                    .unwrap_or_default(),
            )
        })?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "GetConditionInfo",
        lua.create_function(move |lua, (_cfg, cid): (i32, i32)| {
            super::traits_api_node::create_condition_info(lua, &st, cid)
        })?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "GetSubTreeInfo",
        lua.create_function(move |lua, (config_id, sub_tree_id): (i32, i32)| {
            super::traits_api_node::create_sub_tree_info(lua, &st, config_id, sub_tree_id)
        })?,
    )?;

    let st = Rc::clone(&state);
    t.set(
        "GetNodeCost",
        lua.create_function(move |lua, (_cfg, node_id): (i32, i32)| {
            create_node_cost(lua, &st, node_id as u32)
        })?,
    )?;

    Ok(())
}

fn create_config_info(lua: &Lua, _config_id: i32) -> Result<Value> {
    let info = lua.create_table()?;
    let tree_ids = lua.create_table()?;
    tree_ids.set(1, 790)?;
    info.set("treeIDs", tree_ids)?;
    info.set("ID", 1)?;
    info.set("type", 1)?;
    info.set("name", "")?;
    Ok(Value::Table(info))
}

fn create_tree_info(lua: &Lua, (config_id, tree_id): (i32, i32)) -> Result<Value> {
    use crate::traits::TRAIT_TREE_DB;
    if TRAIT_TREE_DB.get(&(tree_id as u32)).is_none() {
        return Ok(Value::Nil);
    }
    let info = lua.create_table()?;
    info.set("ID", tree_id)?;
    info.set("gates", lua.create_table()?)?;
    info.set("hideSinglePurchaseNodes", false)?;
    info.set("configID", config_id)?;
    info.set("minZoom", 0.75)?;
    info.set("maxZoom", 1.2)?;
    info.set("buttonSize", 40)?;
    info.set("isLinkedToActiveConfigID", true)?;
    Ok(Value::Table(info))
}

fn create_tree_nodes(lua: &Lua, tree_id: i32) -> Result<mlua::Table> {
    use crate::traits::TRAIT_TREE_DB;
    let t = lua.create_table()?;
    if let Some(tree) = TRAIT_TREE_DB.get(&(tree_id as u32)) {
        for (i, &node_id) in tree.node_ids.iter().enumerate() {
            t.set(i as i64 + 1, node_id as i64)?;
        }
    }
    Ok(t)
}

/// Max points for a currency, derived from currency flags.
/// flags=4 → class (31 points), flags=8 → spec (30 points).
pub(crate) fn max_points_for_currency(currency_id: u32) -> u32 {
    use crate::traits::TRAIT_CURRENCY_DB;
    let Some(c) = TRAIT_CURRENCY_DB.get(&currency_id) else {
        return 0;
    };
    match c.flags {
        4 => 31,
        8 => 30,
        _ => 0,
    }
}

fn create_tree_currency_info(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    tree_id: i32,
) -> Result<Value> {
    use crate::traits::{TRAIT_CURRENCY_DB, TRAIT_TREE_DB};
    let Some(tree) = TRAIT_TREE_DB.get(&(tree_id as u32)) else {
        return Ok(Value::Nil);
    };
    let s = state.borrow();
    let arr = lua.create_table()?;
    for (i, &cid) in tree.currency_ids.iter().enumerate() {
        let entry = lua.create_table()?;
        entry.set("traitCurrencyID", cid as i64)?;
        let max_pts = max_points_for_currency(cid);
        let spent = s.talents.spent_for_currency(cid);
        let quantity = max_pts.saturating_sub(spent);
        entry.set("quantity", quantity as i64)?;
        entry.set("maxQuantity", max_pts as i64)?;
        entry.set("spent", spent as i64)?;
        let flags = TRAIT_CURRENCY_DB.get(&cid).map(|c| c.flags).unwrap_or(0);
        entry.set("flags", flags as i64)?;
        arr.set(i as i64 + 1, entry)?;
    }
    Ok(Value::Table(arr))
}

fn create_node_cost(lua: &Lua, state: &Rc<RefCell<SimState>>, node_id: u32) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    let s = state.borrow();
    if let Some(&cid) = s.talents.node_currency_map.get(&node_id) {
        let cost = lua.create_table()?;
        cost.set("ID", cid as i64)?;
        cost.set("amount", 1)?;
        t.set(1, cost)?;
    }
    Ok(t)
}

/// Check if `HasUnspentTalentPoints` — any class/spec currency has remaining points.
pub fn has_unspent_talent_points(state: &SimState) -> bool {
    use crate::traits::TRAIT_TREE_DB;
    let Some(tree) = TRAIT_TREE_DB.get(&790) else {
        return false;
    };
    tree.currency_ids.iter().any(|&cid| {
        let max_pts = max_points_for_currency(cid);
        max_pts > 0 && state.talents.spent_for_currency(cid) < max_pts
    })
}
