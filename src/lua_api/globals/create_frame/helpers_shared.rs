//! Minimal CreateFrame helpers kept alive while the implementation moves to rilua.

use crate::lua_api::SimState;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_ref};
use crate::widget::{Frame, WidgetType};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn create_frame_instance(
    state: &mut LuaState,
    widget_type: WidgetType,
    frame_type: &str,
    name: Option<String>,
    parent_id: Option<u64>,
    parent_explicit: bool,
    id: Option<i32>,
) -> LuaResult<u64> {
    let mut frame = Frame::new(widget_type, name.clone(), parent_id);
    if widget_type.as_str() != frame_type {
        frame.object_type_name = Some(frame_type.to_string());
    }
    apply_initial_visibility(state, &mut frame)?;
    apply_addon_ownership(state, parent_id, &mut frame)?;

    if let Some(user_id) = id {
        frame.user_id = user_id;
    }

    let frame_id = frame.id;
    register_and_attach_parent(state, frame, parent_id, parent_explicit, frame_id)?;
    register_global_name(state, name, frame_id)?;

    Ok(frame_id)
}

fn apply_initial_visibility(state: &mut LuaState, frame: &mut Frame) -> LuaResult<()> {
    let initial_hidden = borrow_state(state)?
        .create_frame_initial_hidden
        .unwrap_or(false);
    if initial_hidden {
        frame.visible = false;
        frame.effective_alpha = 0.0;
    }
    Ok(())
}

fn apply_addon_ownership(
    state: &mut LuaState,
    parent_id: Option<u64>,
    frame: &mut Frame,
) -> LuaResult<()> {
    let sim = borrow_state_mut(state)?;
    frame.owner_addon = sim
        .loading_addon_index
        .or(sim.executing_addon_index)
        .or_else(|| parent_id.and_then(|pid| sim.widgets.get(pid).and_then(|f| f.owner_addon)));
    frame.forbidden = sim.loading_forbidden;
    Ok(())
}

fn register_and_attach_parent(
    state: &mut LuaState,
    frame: Frame,
    parent_id: Option<u64>,
    parent_explicit: bool,
    frame_id: u64,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(frame);
    let Some(parent_id) = parent_id else {
        return Ok(());
    };
    sim.widgets.add_child(parent_id, frame_id);
    let Some(parent) = sim.widgets.get(parent_id) else {
        return Ok(());
    };
    let parent_strata = parent.frame_strata;
    let parent_level = parent.frame_level;
    let parent_alpha = parent.effective_alpha;
    let parent_scale = parent.effective_scale;
    if let Some(child) = sim.widgets.get_mut_visual(frame_id) {
        child.frame_strata = parent_strata;
        if parent_explicit {
            child.frame_level = parent_level + 1;
        }
        child.effective_alpha = if child.visible {
            parent_alpha * child.alpha
        } else {
            0.0
        };
        child.effective_scale = parent_scale * child.scale;
    }
    Ok(())
}

fn register_global_name(
    state: &mut LuaState,
    name: Option<String>,
    frame_id: u64,
) -> LuaResult<()> {
    let Some(name) = name else {
        return Ok(());
    };
    let frame_val = frame_ref(state, frame_id)?;
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(globals) = state.gc.tables.get_mut(global) {
        let _ = globals.raw_set(Val::Str(key), frame_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    Ok(())
}

pub(crate) fn apply_parent_sub(name: &str, parent_id: Option<u64>, state: &SimState) -> String {
    if name.len() < 7 || !name[..7].eq_ignore_ascii_case("$parent") {
        return name.to_string();
    }

    let mut current_id = parent_id;
    while let Some(id) = current_id {
        let Some(frame) = state.widgets.get(id) else {
            break;
        };
        if let Some(frame_name) = &frame.name
            && !frame_name.is_empty()
            && frame_name != "UIParent"
        {
            return format!("{frame_name}{}", &name[7..]);
        }
        current_id = frame.parent_id;
    }

    format!("Top{}", &name[7..])
}
