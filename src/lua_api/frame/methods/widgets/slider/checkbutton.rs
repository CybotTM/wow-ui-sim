use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn_static};
use crate::widget::AttributeValue;
use rilua::LuaResult;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

use super::super::shared::val_to_bool;

fn checkbutton_set_checked(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let checked = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    apply_checkbutton_checked(&mut sim, id, checked);
    Ok(0)
}

pub(in crate::lua_api::frame::methods) fn toggle_checkbutton_for_click(
    state: &mut LuaState,
    id: u64,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    if !is_checkbutton(&sim, id) {
        return Ok(());
    }

    let checked = !checkbutton_checked(&sim, id);
    apply_checkbutton_checked(&mut sim, id, checked);
    Ok(())
}

fn is_checkbutton(sim: &crate::lua_api::SimState, id: u64) -> bool {
    sim.widgets
        .get(id)
        .map(|frame| frame.widget_type == crate::widget::WidgetType::CheckButton)
        .unwrap_or(false)
}

fn apply_checkbutton_checked(sim: &mut crate::lua_api::SimState, id: u64, checked: bool) {
    if checkbutton_already_checked(sim, id, checked) {
        return;
    }

    set_checkbutton_checked(sim, id, checked);
    sync_checkbutton_checked_textures(sim, id, checked);
}

fn checkbutton_already_checked(sim: &crate::lua_api::SimState, id: u64, checked: bool) -> bool {
    checkbutton_checked(sim, id) == checked
}

fn checkbutton_checked(sim: &crate::lua_api::SimState, id: u64) -> bool {
    sim.widgets
        .get(id)
        .and_then(|f| f.attributes.get("__checked"))
        .map(|v| matches!(v, AttributeValue::Boolean(true)))
        .unwrap_or(false)
}

fn set_checkbutton_checked(sim: &mut crate::lua_api::SimState, id: u64, checked: bool) {
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.attributes
            .insert("__checked".to_string(), AttributeValue::Boolean(checked));
    }
}

fn sync_checkbutton_checked_textures(sim: &mut crate::lua_api::SimState, id: u64, checked: bool) {
    for key in ["CheckedTexture", "DisabledCheckedTexture"] {
        let visible = sim
            .widgets
            .get(id)
            .map(|frame| checkbutton_texture_visible(frame, key, checked))
            .unwrap_or(false);
        if let Some(tex_id) = sim
            .widgets
            .get(id)
            .and_then(|frame| frame.children_keys.get(key).copied())
        {
            sim.set_frame_visible(tex_id, visible);
        }
    }
}

fn checkbutton_texture_visible(frame: &crate::widget::Frame, key: &str, checked: bool) -> bool {
    let enabled = frame
        .attributes
        .get("__enabled")
        .and_then(|value| match value {
            AttributeValue::Boolean(flag) => Some(*flag),
            _ => None,
        })
        .unwrap_or(true);
    match key {
        "CheckedTexture" => enabled && checked,
        "DisabledCheckedTexture" => !enabled && checked,
        _ => false,
    }
}

fn checkbutton_get_checked(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.attributes.get("__checked"))
        .map(|v| matches!(v, AttributeValue::Boolean(true)))
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(in crate::lua_api::frame::methods::widgets) fn register_checkbutton(
    state: &mut LuaState,
    metatable: GcRef<Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(state, metatable, "SetChecked", checkbutton_set_checked)?;
    table_set_rust_fn_static(state, metatable, "GetChecked", checkbutton_get_checked)?;
    Ok(())
}
