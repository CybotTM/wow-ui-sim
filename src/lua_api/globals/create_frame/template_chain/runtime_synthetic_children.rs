use crate::lua_api::frame::methods::methods_helpers::set_all_points_anchors_pub;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_ref, sync_child_to_rilua};
use crate::widget::{Frame, WidgetType};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn ensure_runtime_slider_children(state: &mut LuaState, frame_id: u64) -> LuaResult<()> {
    if !is_slider(state, frame_id)? {
        return Ok(());
    }

    for key in ["Low", "High", "Text"] {
        ensure_named_child(state, frame_id, key, WidgetType::FontString)?;
    }
    ensure_named_child(state, frame_id, "ThumbTexture", WidgetType::Texture)
}

fn is_slider(state: &LuaState, frame_id: u64) -> LuaResult<bool> {
    let sim = borrow_state(state)?;
    Ok(sim
        .widgets
        .get(frame_id)
        .is_some_and(|widget| widget.widget_type == WidgetType::Slider))
}

fn ensure_named_child(
    state: &mut LuaState,
    parent_id: u64,
    key: &str,
    widget_type: WidgetType,
) -> LuaResult<()> {
    let child_id = get_or_create_named_child(state, parent_id, key, widget_type)?;
    sync_child_to_rilua(state, parent_id, key, child_id)?;
    register_child_global(state, child_id)
}

fn get_or_create_named_child(
    state: &mut LuaState,
    parent_id: u64,
    key: &str,
    widget_type: WidgetType,
) -> LuaResult<u64> {
    let mut sim = borrow_state_mut(state)?;
    if let Some(child_id) = sim
        .widgets
        .get(parent_id)
        .and_then(|parent| parent.children_keys.get(key).copied())
    {
        ensure_existing_child_name(&mut sim, parent_id, child_id, key);
        return Ok(child_id);
    }

    let child_name = runtime_default_child_name(&sim, parent_id, key);
    let mut child = Frame::new(widget_type, child_name, Some(parent_id));
    child.parent_key = Some(key.to_string());
    set_all_points_anchors_pub(&mut child, parent_id);
    let child_id = child.id;
    sim.widgets.register(child);
    sim.widgets.add_child(parent_id, child_id);
    if let Some(parent) = sim.widgets.get_mut_visual(parent_id) {
        parent.children_keys.insert(key.to_string(), child_id);
    }
    Ok(child_id)
}

fn ensure_existing_child_name(
    sim: &mut crate::lua_api::SimState,
    parent_id: u64,
    child_id: u64,
    key: &str,
) {
    let child_name = runtime_default_child_name(sim, parent_id, key);
    if let Some(child) = sim.widgets.get_mut_visual(child_id)
        && child.name.is_none()
    {
        child.name = child_name;
    }
}

fn runtime_default_child_name(
    sim: &crate::lua_api::SimState,
    parent_id: u64,
    key: &str,
) -> Option<String> {
    sim.widgets
        .get(parent_id)
        .and_then(|parent| parent.name.as_ref())
        .map(|parent_name| format!("{parent_name}{key}"))
}

fn register_child_global(state: &mut LuaState, child_id: u64) -> LuaResult<()> {
    let child_name = borrow_state(state)?
        .widgets
        .get(child_id)
        .and_then(|child| child.name.clone());
    let Some(child_name) = child_name else {
        return Ok(());
    };

    let child_ref = frame_ref(state, child_id)?;
    let key = state.gc.intern_string(child_name.as_bytes());
    let global = state.global;
    if let Some(globals) = state.gc.tables.get_mut(global) {
        let _ = globals.raw_set(Val::Str(key), child_ref, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    crate::lua_api::global_slots::refresh_installed_slots_for_name(state, &child_name);
    Ok(())
}
