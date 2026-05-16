use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack, frame_ref,
};
use crate::lua_bridge::stack_val;
use crate::widget::{Frame, WidgetType};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn get_named_child_texture_id(state: &LuaState, id: u64, key: &str) -> Option<u64> {
    borrow_state(state)
        .ok()?
        .widgets
        .get(id)
        .and_then(|frame| frame.children_keys.get(key).copied())
}

pub(super) fn ensure_named_child_texture(
    state: &mut LuaState,
    id: u64,
    key: &str,
) -> LuaResult<u64> {
    if let Some(child_id) = get_named_child_texture_id(state, id, key) {
        return Ok(child_id);
    }

    let child = Frame::new(WidgetType::Texture, None, Some(id));
    let child_id = child.id;
    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(child);
    sim.widgets.add_child(id, child_id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.children_keys.insert(key.to_string(), child_id);
    }
    Ok(child_id)
}

pub(super) fn assign_texture_payload(
    state: &mut LuaState,
    child_id: u64,
    texture_value: Val,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let Some(child) = sim.widgets.get_mut_visual(child_id) else {
        return Ok(());
    };
    match texture_value {
        Val::Num(value) => {
            child.texture_file_data_id = Some(value as i64);
            child.texture = None;
        }
        Val::Str(value) => {
            let Some(raw) = state.gc.string_arena.get(value) else {
                return Ok(());
            };
            child.texture = Some(String::from_utf8_lossy(raw.data()).to_string());
            child.texture_file_data_id = None;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn set_thumb_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture_value = stack_val(state, 2);
    if let Some(child_id) = extract_frame_id(state, texture_value) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame
                .children_keys
                .insert("ThumbTexture".to_string(), child_id);
        }
        return Ok(0);
    }

    let child_id = ensure_named_child_texture(state, id, "ThumbTexture")?;
    assign_texture_payload(state, child_id, texture_value)?;
    Ok(0)
}

pub(super) fn get_thumb_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    match get_named_child_texture_id(state, id, "ThumbTexture") {
        Some(child_id) => {
            let thumb = frame_ref(state, child_id)?;
            state.push(thumb);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}
