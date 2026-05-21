//! C_Container temporary default-state shims.
//!
//! Purchase/refund, quest-item flags, bag filtering, battle-pay markers, and
//! direct container item actions depend on game systems that are not modeled
//! yet. Keep their inert compatibility shapes here; the real bag contents and
//! item metadata stay in the state-backed C_Container surface.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{create_table, table_set_static};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_container_default_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Container")?;
    register_query_defaults(state, ns)?;
    register_action_defaults(state, ns)?;
    Ok(())
}

fn register_query_defaults(
    state: &mut LuaState,
    ns: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        ns,
        "GetContainerItemPurchaseInfo",
        get_container_item_purchase_info,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetContainerItemQuestInfo",
        get_container_item_quest_info,
    )?;
    table_set_rust_fn_static(state, ns, "IsContainerFiltered", is_container_filtered)?;
    table_set_rust_fn_static(state, ns, "IsBattlePayItem", is_battle_pay_item)?;
    Ok(())
}

fn register_action_defaults(
    state: &mut LuaState,
    ns: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(state, ns, "UseContainerItem", container_action_noop)?;
    table_set_rust_fn_static(state, ns, "PickupContainerItem", container_action_noop)?;
    table_set_rust_fn_static(state, ns, "SplitContainerItem", container_action_noop)?;
    Ok(())
}

fn get_container_item_purchase_info(state: &mut LuaState) -> LuaResult<u32> {
    let _bag = i32::from_stack(state, 1)?;
    let _slot = i32::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn get_container_item_quest_info(state: &mut LuaState) -> LuaResult<u32> {
    let _bag = i32::from_stack(state, 1)?;
    let _slot = i32::from_stack(state, 2)?;
    let info = create_table(state);
    table_set_static(state, info, "isQuestItem", Val::Bool(false));
    table_set_static(state, info, "questID", Val::Nil);
    table_set_static(state, info, "isActive", Val::Bool(false));
    state.push(info);
    Ok(1)
}

fn is_container_filtered(state: &mut LuaState) -> LuaResult<u32> {
    let _bag = i32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_battle_pay_item(state: &mut LuaState) -> LuaResult<u32> {
    let _bag = i32::from_stack(state, 1)?;
    let _slot = i32::from_stack(state, 2)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn container_action_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
