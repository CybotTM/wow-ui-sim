//! `C_AnimaDiversion` namespace — Shadowlands anima-diversion UI surface
//! consumed by the LoD `Blizzard_AnimaDiversionUI` addon.
//!
//! Backs the seven probes documented in
//! `vendor/wow-ui-source/Interface/AddOns/Blizzard_APIDocumentationGenerated/AnimaDiversionUIDocumentation.lua`:
//! `GetAnimaDiversionNodes`, `GetOriginPosition`, `GetReinforceProgress`,
//! `GetTextureKit`, `OpenAnimaDiversionUI`, `SelectAnimaNode`, `CloseUI`.
//! The frame info shape (`textureKit`, `title`, `mapID`) is published to
//! the event queue when `OpenAnimaDiversionUI` fires `ANIMA_DIVERSION_OPEN`.

use super::{ensure_namespace, set_table_array};
use crate::event::{Event, EventArg};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set,
};
use crate::lua_api::state::{AnimaDiversionCostInfo, AnimaDiversionNodeInfo};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_anima_diversion_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AnimaDiversion")?;
    table_set_rust_fn_static(state, table_ref, "GetAnimaDiversionNodes", get_nodes)?;
    table_set_rust_fn_static(state, table_ref, "GetOriginPosition", get_origin_position)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetReinforceProgress",
        get_reinforce_progress,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetTextureKit", get_texture_kit)?;
    table_set_rust_fn_static(state, table_ref, "OpenAnimaDiversionUI", open_ui)?;
    table_set_rust_fn_static(state, table_ref, "SelectAnimaNode", select_anima_node)?;
    table_set_rust_fn_static(state, table_ref, "CloseUI", close_ui)?;
    Ok(())
}

fn get_nodes(state: &mut LuaState) -> LuaResult<u32> {
    let nodes = borrow_state(state)?.anima_diversion.nodes.clone();
    let array = create_table(state);
    for (index, node) in nodes.into_iter().enumerate() {
        let entry = anima_node_table(state, node);
        set_table_array(state, array, index as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn anima_node_table(state: &mut LuaState, node: AnimaDiversionNodeInfo) -> Val {
    let entry = create_table(state);
    let name = create_string(state, &node.name);
    let description = create_string(state, &node.description);
    table_set(state, entry, "talentID", Val::Num(node.talent_id as f64));
    table_set(state, entry, "name", name);
    table_set(state, entry, "description", description);
    table_set(
        state,
        entry,
        "currencyID",
        Val::Num(node.currency_id as f64),
    );
    table_set(state, entry, "icon", Val::Num(node.icon as f64));
    table_set(state, entry, "state", Val::Num(node.state as f64));
    let position = node_position_table(state, &node);
    let costs = node_costs_table(state, node.costs);
    table_set(state, entry, "normalizedPosition", position);
    table_set(state, entry, "costs", costs);
    entry
}

fn node_position_table(state: &mut LuaState, node: &AnimaDiversionNodeInfo) -> Val {
    let position = create_table(state);
    table_set(state, position, "x", Val::Num(node.normalized_position_x));
    table_set(state, position, "y", Val::Num(node.normalized_position_y));
    position
}

fn node_costs_table(state: &mut LuaState, costs: Vec<AnimaDiversionCostInfo>) -> Val {
    let costs_table = create_table(state);
    for (index, cost) in costs.into_iter().enumerate() {
        let cost_entry = node_cost_table(state, cost);
        set_table_array(state, costs_table, index as i64 + 1, cost_entry);
    }
    costs_table
}

fn node_cost_table(state: &mut LuaState, cost: AnimaDiversionCostInfo) -> Val {
    let cost_entry = create_table(state);
    table_set(
        state,
        cost_entry,
        "currencyID",
        Val::Num(cost.currency_id as f64),
    );
    table_set(
        state,
        cost_entry,
        "quantity",
        Val::Num(cost.quantity as f64),
    );
    cost_entry
}

fn get_origin_position(state: &mut LuaState) -> LuaResult<u32> {
    let origin = borrow_state(state)?.anima_diversion.origin_position;
    let Some((x, y)) = origin else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let position = create_table(state);
    table_set(state, position, "x", Val::Num(x));
    table_set(state, position, "y", Val::Num(y));
    state.push(position);
    Ok(1)
}

fn get_reinforce_progress(state: &mut LuaState) -> LuaResult<u32> {
    let progress = borrow_state(state)?.anima_diversion.reinforce_progress;
    state.push(Val::Num(progress));
    Ok(1)
}

fn get_texture_kit(state: &mut LuaState) -> LuaResult<u32> {
    let kit = borrow_state(state)?.anima_diversion.texture_kit.clone();
    let value = create_string(state, &kit);
    state.push(value);
    Ok(1)
}

fn open_ui(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let texture_kit = sim.anima_diversion.texture_kit.clone();
    let title = sim.anima_diversion.title.clone();
    let map_id = sim.anima_diversion.map_id;
    sim.events.push(Event {
        name: "ANIMA_DIVERSION_OPEN".to_string(),
        args: vec![
            EventArg::String(texture_kit),
            EventArg::String(title),
            EventArg::Number(map_id as f64),
        ],
    });
    Ok(0)
}

fn select_anima_node(state: &mut LuaState) -> LuaResult<u32> {
    let talent_id = i64::from_stack(state, 1)?;
    let temporary = bool::from_stack(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    sim.anima_diversion.last_selected_talent_id = Some(talent_id);
    sim.anima_diversion.last_selected_temporary = Some(temporary);
    sim.events.push(Event {
        name: "ANIMA_DIVERSION_TALENT_UPDATED".to_string(),
        args: vec![
            EventArg::Number(talent_id as f64),
            EventArg::Boolean(temporary),
        ],
    });
    Ok(0)
}

fn close_ui(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.events.push(Event {
        name: "ANIMA_DIVERSION_CLOSE".to_string(),
        args: vec![],
    });
    Ok(0)
}
