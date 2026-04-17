//! FontString methods: GetFontString, SetFontString, CreateFontString.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack, frame_ref,
    sync_child_to_rilua,
};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::shared::{bind_named_child_global, opt_string};

/// GetFontString() -> fontstring
pub(super) fn get_font_string(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(tid) = find_existing_text_child(state, id) {
        let val = frame_ref(state, tid)?;
        state.push(val);
        return Ok(1);
    }
    create_synthetic_text_child(state, id)
}

fn find_existing_text_child(state: &mut LuaState, id: u64) -> Option<u64> {
    let sim = borrow_state(state).ok()?;
    sim.widgets.get(id).and_then(|frame| {
        frame.children_keys.get("Text").copied().or_else(|| {
            let fallback_name = frame.name.as_ref().map(|name| format!("{name}Text"))?;
            let child_id = sim.widgets.get_id_by_name(&fallback_name)?;
            let child = sim.widgets.get(child_id)?;
            (child.parent_id == Some(id)
                && child.widget_type == crate::widget::WidgetType::FontString)
                .then_some(child_id)
        })
    })
}

fn create_synthetic_text_child(state: &mut LuaState, id: u64) -> LuaResult<u32> {
    let fallback = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(|frame| {
            (
                matches!(
                    frame.widget_type,
                    crate::widget::WidgetType::Button | crate::widget::WidgetType::CheckButton
                ),
                frame.name.as_ref().map(|name| format!("{name}Text")),
                frame.text.clone().unwrap_or_default(),
            )
        })
    };
    let Some((is_button, child_name, text_value)) = fallback else {
        state.push(Val::Nil);
        return Ok(1);
    };
    if !is_button {
        state.push(Val::Nil);
        return Ok(1);
    }

    let child_id = register_font_string_child(state, id, child_name, text_value)?;
    let _ = sync_child_to_rilua(state, id, "Text", child_id);
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

fn register_font_string_child(
    state: &mut LuaState,
    id: u64,
    child_name: Option<String>,
    text_value: String,
) -> LuaResult<u64> {
    use crate::widget::{Frame, WidgetType};
    let mut font_string = Frame::new(WidgetType::FontString, child_name, Some(id));
    font_string.parent_key = Some("Text".to_string());
    font_string.text = Some(text_value.clone());
    font_string.text_stripped = Some(crate::render::strip_wow_markup(&text_value));
    super::super::methods_helpers::set_all_points_anchors_pub(&mut font_string, id);
    let child_id = font_string.id;

    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(font_string);
    sim.widgets.add_child(id, child_id);
    if let Some(button) = sim.widgets.get_mut_visual(id) {
        button.children_keys.insert("Text".to_string(), child_id);
    }
    sim.invalidate_strata_buckets();
    Ok(child_id)
}

/// SetFontString(fontstring)
pub(super) fn set_font_string(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let fontstring_val = stack_val(state, 2);
    let fs_id_opt = extract_frame_id(state, fontstring_val);
    if let Some(fs_id) = fs_id_opt {
        attach_font_string_to_button(state, id, fs_id)?;
    } else {
        let mut sim = borrow_state_mut(state)?;
        if let Some(btn) = sim.widgets.get_mut_visual(id) {
            btn.children_keys.remove("Text");
        }
    }
    Ok(0)
}

fn attach_font_string_to_button(state: &mut LuaState, id: u64, fs_id: u64) -> LuaResult<()> {
    {
        let mut sim = borrow_state_mut(state)?;
        super::super::methods_hierarchy::reparent_widget(&mut sim.widgets, fs_id, Some(id));
        if let Some(fs) = sim.widgets.get_mut_visual(fs_id) {
            fs.anchors.clear();
            super::super::methods_helpers::set_all_points_anchors_pub(fs, id);
        }
        if let Some(btn) = sim.widgets.get_mut_visual(id) {
            btn.children_keys.insert("Text".to_string(), fs_id);
        }
        if let Some(fs) = sim.widgets.get_mut_visual(fs_id) {
            fs.parent_key = Some("Text".to_string());
        }
    }
    let _ = sync_child_to_rilua(state, id, "Text", fs_id);
    Ok(())
}

/// CreateFontString([name [, layer [, inherits]]]) -> fontstring
pub(super) fn create_font_string(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{DrawLayer, Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let layer = opt_string(state, 3);
    let _inherits = opt_string(state, 4);

    let name = resolve_child_name(state, name_raw, parent_id);

    let mut fontstring = Frame::new(WidgetType::FontString, name.clone(), Some(parent_id));
    if let Some(layer_str) = layer {
        if let Some(draw_layer) = DrawLayer::from_str(&layer_str) {
            fontstring.draw_layer = draw_layer;
        }
    }
    let child_id = fontstring.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(fontstring);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    if let Some(ref n) = name {
        bind_named_child_global(state, n, child_id)?;
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

pub(super) fn resolve_child_name(
    state: &mut LuaState,
    name_raw: Option<String>,
    parent_id: u64,
) -> Option<String> {
    name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(parent_id), &sim)
        } else {
            n
        }
    })
}
