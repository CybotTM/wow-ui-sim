//! Minimal CreateFrame helpers kept alive while the implementation moves to rilua.

use crate::lua_api::SimState;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_ref, sync_child_to_rilua};
use crate::widget::{Frame, FrameStrata, WidgetType};
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
    if widget_type == WidgetType::GameTooltip {
        frame.frame_strata = FrameStrata::Tooltip;
        frame.has_fixed_frame_strata = true;
    }
    if should_preserve_object_type_name(widget_type, frame_type) {
        frame.object_type_name = Some(frame_type.to_string());
    }
    apply_initial_visibility(state, &mut frame)?;
    apply_addon_ownership(state, parent_id, &mut frame)?;

    if let Some(user_id) = id {
        frame.user_id = user_id;
    }

    let preserve_existing_name_binding = should_preserve_existing_root_frame_name(state, &name)?;
    let frame_id = frame.id;
    register_and_attach_parent(
        state,
        frame,
        parent_id,
        parent_explicit,
        frame_id,
        preserve_existing_name_binding,
    )?;
    install_default_children(state, widget_type, frame_id)?;
    if widget_type == WidgetType::MessageFrame {
        crate::lua_api::frame::methods::widgets::message_frame::install_message_frame_fields(
            state, frame_id,
        )?;
    }
    register_global_name(state, name, frame_id, preserve_existing_name_binding)?;

    Ok(frame_id)
}

fn install_default_children(
    state: &mut LuaState,
    widget_type: WidgetType,
    frame_id: u64,
) -> LuaResult<()> {
    if widget_type != WidgetType::Slider {
        return Ok(());
    }

    for (key, child_type) in [
        ("Low", WidgetType::FontString),
        ("High", WidgetType::FontString),
        ("Text", WidgetType::FontString),
        ("ThumbTexture", WidgetType::Texture),
    ] {
        register_named_child(state, frame_id, key, child_type)?;
    }
    Ok(())
}

fn register_named_child(
    state: &mut LuaState,
    parent_id: u64,
    key: &str,
    child_type: WidgetType,
) -> LuaResult<u64> {
    let mut child = Frame::new(child_type, None, Some(parent_id));
    child.parent_key = Some(key.to_string());
    let child_id = child.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(child);
        sim.widgets.add_child(parent_id, child_id);
        if let Some(parent) = sim.widgets.get_mut_visual(parent_id) {
            parent.children_keys.insert(key.to_string(), child_id);
        }
    }
    sync_child_to_rilua(state, parent_id, key, child_id)?;
    Ok(child_id)
}

fn should_preserve_object_type_name(widget_type: WidgetType, frame_type: &str) -> bool {
    if widget_type.as_str().eq_ignore_ascii_case(frame_type) {
        return false;
    }

    !matches!(
        frame_type.to_ascii_lowercase().as_str(),
        "containedalertframe"
            | "dropdownbutton"
            | "dropdowntogglebutton"
            | "eventbutton"
            | "itembutton"
    )
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
    let parent_forbidden = parent_id
        .and_then(|pid| sim.widgets.get(pid))
        .is_some_and(|parent| parent.forbidden);
    frame.owner_addon = sim
        .loading_addon_index
        .or(sim.executing_addon_index)
        .or_else(|| parent_id.and_then(|pid| sim.widgets.get(pid).and_then(|f| f.owner_addon)));
    frame.forbidden = sim.loading_forbidden || parent_forbidden;
    Ok(())
}

fn register_and_attach_parent(
    state: &mut LuaState,
    frame: Frame,
    parent_id: Option<u64>,
    parent_explicit: bool,
    frame_id: u64,
    preserve_existing_name_binding: bool,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    if preserve_existing_name_binding {
        sim.widgets.register_preserving_existing_name(frame);
    } else {
        sim.widgets.register(frame);
    }
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
        if !child.has_fixed_frame_strata {
            child.frame_strata = parent_strata;
        }
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
    sim.invalidate_strata_buckets();
    Ok(())
}

fn register_global_name(
    state: &mut LuaState,
    name: Option<String>,
    frame_id: u64,
    preserve_existing_name_binding: bool,
) -> LuaResult<()> {
    let Some(name) = name else {
        return Ok(());
    };
    if preserve_existing_name_binding {
        return Ok(());
    }
    let frame_val = frame_ref(state, frame_id)?;
    migrate_existing_global_frame_fields(state, &name, frame_val);
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(globals) = state.gc.tables.get_mut(global) {
        let _ = globals.raw_set(Val::Str(key), frame_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    crate::lua_api::global_slots::refresh_installed_slots_for_name(state, &name);
    Ok(())
}

fn should_preserve_existing_root_frame_name(
    state: &mut LuaState,
    name: &Option<String>,
) -> LuaResult<bool> {
    let Some(name) = name.as_deref() else {
        return Ok(false);
    };
    if !matches!(name, "UIParent" | "WorldFrame") {
        return Ok(false);
    }
    Ok(borrow_state(state)?.widgets.get_id_by_name(name).is_some())
}

fn migrate_existing_global_frame_fields(state: &mut LuaState, name: &str, new_frame: Val) {
    let Val::Table(new_ref) = new_frame else {
        return;
    };
    let key = state.gc.intern_string(name.as_bytes());
    let existing = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    let Val::Table(existing_ref) = existing else {
        return;
    };

    let array_values = state
        .gc
        .tables
        .get(existing_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let hash_entries = state
        .gc
        .tables
        .get(existing_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default();

    if let Some(new_table) = state.gc.tables.get_mut(new_ref) {
        for (index, value) in array_values.into_iter().enumerate() {
            let _ = new_table.raw_set(Val::Num((index + 1) as f64), value, &state.gc.string_arena);
        }
        for (entry_key, value) in hash_entries {
            let _ = new_table.raw_set(entry_key, value, &state.gc.string_arena);
        }
    }
    state.gc.barrier_back(new_ref);
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
