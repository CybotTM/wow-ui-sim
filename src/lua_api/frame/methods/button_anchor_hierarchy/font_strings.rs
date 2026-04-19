//! FontString methods: GetFontString, SetFontString, CreateFontString.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack, frame_ref,
    sync_child_to_rilua, table_get, val_to_string,
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
                frame.text.clone(),
            )
        })
    };
    let Some((is_button, _child_name, text_value)) = fallback else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let has_normal_font_object = super::buttons::has_normal_font_object(state, id);
    if !is_button || (text_value.is_none() && !has_normal_font_object) {
        state.push(Val::Nil);
        return Ok(1);
    }

    let Some(child_id) = ensure_button_text_child(state, id)? else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

pub(super) fn ensure_button_text_child(state: &mut LuaState, id: u64) -> LuaResult<Option<u64>> {
    if let Some(tid) = find_existing_text_child(state, id) {
        return Ok(Some(tid));
    }

    let fallback = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(|frame| {
            (
                matches!(
                    frame.widget_type,
                    crate::widget::WidgetType::Button | crate::widget::WidgetType::CheckButton
                ),
                frame.name.as_ref().map(|name| format!("{name}Text")),
                frame.text.clone(),
            )
        })
    };

    let Some((is_button, child_name, text_value)) = fallback else {
        return Ok(None);
    };
    if !is_button {
        return Ok(None);
    }

    let child_id = register_font_string_child(state, id, child_name, text_value)?;
    let _ = sync_child_to_rilua(state, id, "Text", child_id);
    Ok(Some(child_id))
}

fn register_font_string_child(
    state: &mut LuaState,
    id: u64,
    child_name: Option<String>,
    text_value: Option<String>,
) -> LuaResult<u64> {
    use crate::widget::{Frame, WidgetType};
    let mut font_string = Frame::new(WidgetType::FontString, child_name, Some(id));
    font_string.parent_key = Some("Text".to_string());
    if let Some(text_value) = text_value {
        font_string.text_stripped = Some(crate::render::strip_wow_markup(&text_value));
        font_string.text = Some(text_value);
    }
    super::super::methods_helpers::set_all_points_anchors_pub(&mut font_string, id);
    let child_id = font_string.id;
    let child_global_name = font_string.name.clone();

    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(font_string);
    sim.widgets.add_child(id, child_id);
    if let Some(button) = sim.widgets.get_mut_visual(id) {
        button.children_keys.insert("Text".to_string(), child_id);
    }
    sim.invalidate_strata_buckets();
    drop(sim);
    if let Some(child_global_name) = child_global_name {
        bind_named_child_global(state, &child_global_name, child_id)?;
    }
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
    let inherits = opt_string(state, 4);

    let name = resolve_child_name(state, name_raw, parent_id);

    let mut fontstring = Frame::new(WidgetType::FontString, name.clone(), Some(parent_id));
    if let Some(layer_str) = layer {
        if let Some(draw_layer) = DrawLayer::from_str(&layer_str) {
            fontstring.draw_layer = draw_layer;
        }
    }
    apply_font_inherit(state, &mut fontstring, inherits.as_deref());
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

fn apply_font_inherit(
    state: &mut LuaState,
    fontstring: &mut crate::widget::Frame,
    inherits: Option<&str>,
) {
    let Some(inherits) = inherits else { return };
    let font_object = table_get(state, Val::Table(state.global), inherits);
    if !matches!(font_object, Val::Table(_)) {
        return;
    }

    if let Some(path) = font_field_string(state, font_object.clone(), "__font", "__fontPath") {
        fontstring.font = Some(path);
    }
    if let Some(height) = font_field_number(state, font_object.clone(), "__height", "__fontHeight")
    {
        fontstring.font_size = height as f32;
    }
    if let Some(outline) = font_field_string(state, font_object.clone(), "__outline", "__fontFlags")
    {
        fontstring.font_outline = crate::widget::TextOutline::from_wow_str(&outline);
    }
    if let Some(justify_h) =
        font_field_string(state, font_object.clone(), "__justifyH", "__justifyH")
    {
        fontstring.justify_h = crate::widget::TextJustify::from_wow_str(&justify_h);
    }
    if let Some(justify_v) =
        font_field_string(state, font_object.clone(), "__justifyV", "__justifyV")
    {
        fontstring.justify_v = crate::widget::TextJustify::from_wow_str(&justify_v);
    }
    if let Some(text_color) = read_color(state, font_object.clone(), "__textColor") {
        fontstring.text_color = text_color;
    }
    if let Some(shadow_color) = read_color(state, font_object.clone(), "__shadowColor") {
        fontstring.shadow_color = shadow_color;
    }
    if let Some(shadow_offset) = read_shadow_offset(state, font_object) {
        fontstring.shadow_offset = shadow_offset;
    }
}

fn font_field_string(
    state: &mut LuaState,
    table: Val,
    primary: &str,
    fallback: &str,
) -> Option<String> {
    let primary_value = table_get(state, table.clone(), primary);
    match primary_value {
        Val::Str(_) => val_to_string(state, primary_value),
        _ => {
            let fallback_value = table_get(state, table, fallback);
            val_to_string(state, fallback_value)
        }
    }
}

fn font_field_number(
    state: &mut LuaState,
    table: Val,
    primary: &str,
    fallback: &str,
) -> Option<f64> {
    match table_get(state, table.clone(), primary) {
        Val::Num(value) => Some(value),
        _ => match table_get(state, table, fallback) {
            Val::Num(value) => Some(value),
            _ => None,
        },
    }
}

fn read_color(state: &mut LuaState, table: Val, prefix: &str) -> Option<crate::widget::Color> {
    let r = font_field_number(
        state,
        table.clone(),
        &format!("{prefix}R"),
        &format!("{prefix}R"),
    )?;
    let g = font_field_number(
        state,
        table.clone(),
        &format!("{prefix}G"),
        &format!("{prefix}G"),
    )?;
    let b = font_field_number(
        state,
        table.clone(),
        &format!("{prefix}B"),
        &format!("{prefix}B"),
    )?;
    let a = font_field_number(state, table, &format!("{prefix}A"), &format!("{prefix}A"))
        .unwrap_or(1.0);
    Some(crate::widget::Color::new(
        r as f32, g as f32, b as f32, a as f32,
    ))
}

fn read_shadow_offset(state: &mut LuaState, table: Val) -> Option<(f32, f32)> {
    let x = font_field_number(state, table.clone(), "__shadowOffsetX", "__shadowOffsetX")?;
    let y = font_field_number(state, table, "__shadowOffsetY", "__shadowOffsetY")?;
    Some((x as f32, y as f32))
}
