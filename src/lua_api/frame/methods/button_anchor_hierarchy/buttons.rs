//! Button state, enable/disable, click, and related methods.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, extract_frame_id,
    frame_id_from_stack, frame_ref, table_get,
};
use crate::lua_api::script_helpers::{
    call_error_handler_state, get_script as get_rilua_script,
};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Button attribute helpers ──────────────────────────────────────────────────

pub(super) fn button_enabled(frame: &crate::widget::Frame) -> bool {
    frame
        .attributes
        .get("__enabled")
        .and_then(|value| match value {
            crate::widget::AttributeValue::Boolean(value) => Some(*value),
            _ => None,
        })
        .unwrap_or(true)
}

pub(super) fn sync_button_slot_visibility(sim: &mut crate::lua_api::SimState, button_id: u64) {
    for key in [
        "NormalTexture",
        "PushedTexture",
        "DisabledTexture",
        "HighlightTexture",
        "CheckedTexture",
        "DisabledCheckedTexture",
    ] {
        let child_id = sim
            .widgets
            .get(button_id)
            .and_then(|button| button.children_keys.get(key).copied());
        if let Some(child_id) = child_id {
            let should_show = super::textures::button_texture_should_show(sim, button_id, key);
            sim.widgets.set_visible(child_id, should_show);
        }
    }
}

pub(super) fn set_button_enabled_value(
    state: &mut LuaState,
    id: u64,
    enabled: bool,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.attributes.insert(
            "__enabled".to_string(),
            crate::widget::AttributeValue::Boolean(enabled),
        );
    }
    sync_button_slot_visibility(&mut sim, id);
    Ok(())
}

// ── Button methods ────────────────────────────────────────────────────────────

pub(super) fn is_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(button_enabled).unwrap_or(true)
    };
    state.push(Val::Bool(enabled));
    Ok(1)
}

pub(super) fn set_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2).ok().unwrap_or(true);
    set_button_enabled_value(state, id, enabled)?;
    Ok(0)
}

pub(super) fn enable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_button_enabled_value(state, id, true)?;
    Ok(0)
}

pub(super) fn disable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_button_enabled_value(state, id, false)?;
    Ok(0)
}

pub(super) fn register_for_clicks(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id_from_stack(state, 1)?;
    Ok(0)
}

pub(super) fn set_button_state(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let state_name = String::from_stack(state, 2)?;
    let pushed = state_name.eq_ignore_ascii_case("PUSHED");
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.button_state = if pushed { 1 } else { 0 };
        }
        sync_button_slot_visibility(&mut sim, id);
    }
    Ok(0)
}

pub(super) fn get_button_state(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let pushed = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.button_state == 1)
            .unwrap_or(false)
    };
    let name = if pushed { "PUSHED" } else { "NORMAL" };
    let name_val = create_string(state, name);
    state.push(name_val);
    Ok(1)
}

pub(super) fn click(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(handler) = get_rilua_script(state, id, "OnClick") else {
        return Ok(0);
    };
    if matches!(handler, Val::Nil) {
        return Ok(0);
    }
    let self_ref = frame_ref(state, id)?;
    let button = create_string(state, "LeftButton");
    let args = [self_ref, button, Val::Bool(false)];
    if let Err(error) = call_function_state(state, handler, &args) {
        call_error_handler_state(state, &error.to_string());
    }
    Ok(0)
}

pub(super) fn set_item_button_scale(state: &mut LuaState) -> LuaResult<u32> {
    let self_table = stack_val(state, 1);
    let scale = f64::from_stack(state, 2)?;
    let count = table_get(state, self_table, "Count");
    if let Some(count_id) = extract_frame_id(state, count) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(count_id) {
            frame.scale = scale as f32;
        }
    }
    Ok(0)
}

pub(super) fn calculate_action(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let result = {
        let sim = borrow_state(state)?;
        let frame = sim.widgets.get(id);
        let button_id = frame.map(|widget| widget.user_id).unwrap_or(0);
        if button_id > 0 {
            button_id
        } else {
            frame
                .and_then(|widget| widget.attributes.get("action"))
                .and_then(|value| match value {
                    crate::widget::AttributeValue::Number(number) => Some(*number as i32),
                    _ => None,
                })
                .unwrap_or(1)
        }
    };
    state.push(Val::Num(result as f64));
    Ok(1)
}

// ── Button font objects ───────────────────────────────────────────────────────

use crate::lua_api::methods::{registry_table_or_create, table_set};

fn get_or_create_button_font_store(state: &mut LuaState) -> Val {
    registry_table_or_create(state, "__button_font_objects")
}

pub(super) fn set_normal_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = stack_val(state, 2);
    let store = get_or_create_button_font_store(state);
    table_set(state, store, &format!("{id}:normal"), font_object);
    Ok(0)
}

pub(super) fn get_normal_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    let font_object = table_get(state, store, &format!("{id}:normal"));
    state.push(font_object);
    Ok(1)
}

pub(super) fn set_highlight_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = stack_val(state, 2);
    let store = get_or_create_button_font_store(state);
    table_set(state, store, &format!("{id}:highlight"), font_object);
    Ok(0)
}

pub(super) fn get_highlight_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    let font_object = table_get(state, store, &format!("{id}:highlight"));
    state.push(font_object);
    Ok(1)
}

pub(super) fn set_disabled_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = stack_val(state, 2);
    let store = get_or_create_button_font_store(state);
    table_set(state, store, &format!("{id}:disabled"), font_object);
    Ok(0)
}

pub(super) fn get_disabled_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    let font_object = table_get(state, store, &format!("{id}:disabled"));
    state.push(font_object);
    Ok(1)
}

// ── Pushed text offset ────────────────────────────────────────────────────────

pub(super) fn set_pushed_text_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = f64::from_stack(state, 2)? as f32;
    let y = f64::from_stack(state, 3)? as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.pushed_text_offset = (x, y);
    }
    Ok(0)
}

pub(super) fn get_pushed_text_offset(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_bridge::IntoStack;
    let id = frame_id_from_stack(state, 1)?;
    let (x, y) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.pushed_text_offset)
            .unwrap_or((0.0, 0.0))
    };
    (x as f64, y as f64).into_stack(state)
}
