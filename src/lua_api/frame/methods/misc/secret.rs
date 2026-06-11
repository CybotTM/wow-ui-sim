//! Secret, protected, and anchoring restriction methods.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, mt, "HasAnySecretAspect", has_any_secret_aspect)?;
    table_set_rust_fn_static(state, mt, "HasSecretAspect", has_secret_aspect)?;
    table_set_rust_fn_static(state, mt, "HasSecretValues", has_secret_values)?;
    table_set_rust_fn_static(state, mt, "IsAnchoringRestricted", is_anchoring_restricted)?;
    table_set_rust_fn_static(state, mt, "IsAnchoringSecret", is_anchoring_secret)?;
    table_set_rust_fn_static(
        state,
        mt,
        "IsPreventingSecretValues",
        is_preventing_secret_values,
    )?;
    table_set_rust_fn_static(state, mt, "IsProtected", is_protected)?;
    table_set_rust_fn_static(
        state,
        mt,
        "SetPreventSecretValues",
        set_prevent_secret_values,
    )?;
    Ok(())
}

pub fn has_any_secret_aspect(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        frame_has_secret_values(&sim.widgets, id) || frame_is_anchoring_restricted(&sim.widgets, id)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn has_secret_aspect(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let aspect_num: Option<i64> = match stack_val(state, 2) {
        Val::Num(n) => Some(n as i64),
        _ => None,
    };
    let result = {
        let sim = borrow_state(state)?;
        let has_any = frame_has_secret_values(&sim.widgets, id)
            || frame_is_anchoring_restricted(&sim.widgets, id);
        aspect_num.is_some_and(|v| v == 1 && has_any)
    };
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn has_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = { frame_has_secret_values(&borrow_state(state)?.widgets, id) };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_anchoring_restricted(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = { frame_is_anchoring_restricted(&borrow_state(state)?.widgets, id) };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_anchoring_secret(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = { frame_has_secret_values(&borrow_state(state)?.widgets, id) };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_preventing_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        borrow_state(state)?
            .widgets
            .get(id)
            .map(|f| f.prevent_secret_values)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_protected(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let protected = {
        borrow_state(state)?
            .widgets
            .get(id)
            .map(|f| f.is_protected)
            .unwrap_or(false)
    };
    state.push(Val::Bool(protected));
    state.push(Val::Bool(protected));
    Ok(2)
}

pub fn set_prevent_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let prevent = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.prevent_secret_values = prevent;
    }
    Ok(0)
}

fn frame_has_secret_values(widgets: &crate::widget::WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|f| f.prevent_secret_values)
        .unwrap_or(false)
}

fn frame_is_anchoring_restricted(widgets: &crate::widget::WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|f| f.forbidden || f.is_protected)
        .unwrap_or(false)
}
