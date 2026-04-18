//! Hierarchy methods (parent/children/regions) and create-region methods.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack, frame_ref,
};
use crate::lua_bridge::{FromStack, IntoStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::font_strings::resolve_child_name;
use super::shared::{bind_named_child_global, opt_string};

// ── Hierarchy methods ─────────────────────────────────────────────────────────

pub(super) fn get_parent(state: &mut LuaState) -> LuaResult<u32> {
    use super::shared::frame_global_or_ref;
    let id = frame_id_from_stack(state, 1)?;
    let parent_id = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|f| f.parent_id)
    };
    match parent_id {
        Some(pid) => {
            let val = frame_global_or_ref(state, pid)?;
            state.push(val);
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

pub(super) fn set_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let parent_val = stack_val(state, 2);
    let new_parent_id = extract_frame_id(state, parent_val);
    let mut sim = borrow_state_mut(state)?;
    super::super::methods_hierarchy::reparent_widget(&mut sim.widgets, id, new_parent_id);
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.default_parent = false;
    }
    sim.visible_on_update_cache = None;
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

pub(super) fn get_num_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(|f| f.children.len()).unwrap_or(0) as i32
    };
    count.into_stack(state)
}

pub(super) fn get_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let children = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default()
    };
    let count = children.len() as u32;
    for child_id in children {
        let val = frame_ref(state, child_id)?;
        state.push(val);
    }
    Ok(count)
}

pub(super) fn get_num_regions(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| {
                f.children
                    .iter()
                    .filter(|&&cid| is_region_type(&sim, cid))
                    .count()
            })
            .unwrap_or(0) as i32
    };
    count.into_stack(state)
}

fn is_region_type(sim: &crate::lua_api::SimState, cid: u64) -> bool {
    use crate::widget::WidgetType;
    sim.widgets
        .get(cid)
        .map(|c| {
            matches!(
                c.widget_type,
                WidgetType::Texture | WidgetType::FontString | WidgetType::Line
            )
        })
        .unwrap_or(false)
}

pub(super) fn get_regions(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let children = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default()
    };
    let mut count = 0u32;
    for child_id in children {
        let is_region = {
            let sim = borrow_state(state)?;
            is_region_type(&sim, child_id)
        };
        if is_region {
            let val = frame_ref(state, child_id)?;
            state.push(val);
            count += 1;
        }
    }
    Ok(count)
}

pub(super) fn get_additional_regions(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id_from_stack(state, 1)?;
    Ok(0)
}

pub(super) fn get_parent_key(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::methods::create_string;
    let id = frame_id_from_stack(state, 1)?;
    let key = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|f| f.parent_key.clone())
    };
    match key {
        Some(k) => {
            let val = create_string(state, &k);
            state.push(val);
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

pub(super) fn set_parent_key(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let key = String::from_stack(state, 2)?;
    let _remove_old = bool::from_stack(state, 3)?;
    let parent_id = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|f| f.parent_id)
    };
    let Some(pid) = parent_id else {
        return Ok(0);
    };
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(child) = sim.widgets.get_mut(id) {
            child.parent_key = Some(key.clone());
        }
    }
    crate::lua_api::methods::sync_child_to_rilua(state, pid, &key, id)?;
    Ok(0)
}

// ── Create methods ────────────────────────────────────────────────────────────

fn apply_draw_layer(frame: &mut crate::widget::Frame, layer: Option<String>) {
    use crate::widget::DrawLayer;
    if let Some(layer_str) = layer {
        if let Some(draw_layer) = DrawLayer::from_str(&layer_str) {
            frame.draw_layer = draw_layer;
        }
    }
}

fn register_child_with_strata(
    state: &mut LuaState,
    parent_id: u64,
    child_id: u64,
    widget: crate::widget::Frame,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let parent_props = sim
        .widgets
        .get(parent_id)
        .map(|p| (p.frame_strata, p.frame_level));
    sim.widgets.register(widget);
    sim.widgets.add_child(parent_id, child_id);
    sim.invalidate_strata_buckets();
    if let Some((parent_strata, parent_level)) = parent_props {
        if let Some(f) = sim.widgets.get_mut_visual(child_id) {
            f.frame_strata = parent_strata;
            f.frame_level = parent_level + 1;
        }
    }
    Ok(())
}

pub(super) fn create_texture(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let layer = opt_string(state, 3);
    let _inherits = opt_string(state, 4);
    let sub_level = super::shared::opt_f32(state, 5).map(|n| n as i32);

    let name = resolve_child_name(state, name_raw, parent_id);
    let mut texture = Frame::new(WidgetType::Texture, name.clone(), Some(parent_id));
    apply_draw_layer(&mut texture, layer);
    if let Some(sub_level) = sub_level {
        texture.draw_sub_layer = sub_level;
    }

    let child_id = texture.id;
    register_child_with_strata(state, parent_id, child_id, texture)?;

    if let Some(ref n) = name {
        bind_named_child_global(state, n, child_id)?;
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

fn register_child_widget(
    state: &mut LuaState,
    parent_id: u64,
    child_id: u64,
    widget: crate::widget::Frame,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(widget);
    sim.widgets.add_child(parent_id, child_id);
    sim.invalidate_strata_buckets();
    Ok(())
}

pub(super) fn create_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let name = resolve_child_name(state, name_raw, parent_id);
    let mut texture = Frame::new(WidgetType::Texture, name.clone(), Some(parent_id));
    texture.is_mask = true;
    texture.object_type_name = Some("MaskTexture".to_string());
    let child_id = texture.id;
    register_child_widget(state, parent_id, child_id, texture)?;
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

pub(super) fn add_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let Some(mask_id) = extract_frame_id(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    let is_mask = sim.widgets.get(mask_id).map(|f| f.is_mask).unwrap_or(false);
    if !is_mask {
        return Ok(0);
    }
    if let Some(texture) = sim.widgets.get_mut_visual(texture_id)
        && !texture.mask_textures.contains(&mask_id)
    {
        texture.mask_textures.push(mask_id);
    }
    Ok(0)
}

pub(super) fn remove_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let Some(mask_id) = extract_frame_id(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(texture) = sim.widgets.get_mut_visual(texture_id) {
        texture.mask_textures.retain(|id| *id != mask_id);
    }
    Ok(0)
}

pub(super) fn get_num_mask_textures(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(texture_id)
            .map(|f| f.mask_textures.len())
            .unwrap_or(0)
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub(super) fn get_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let index = i64::from_stack(state, 2).unwrap_or(1);
    let mask_id = {
        let sim = borrow_state(state)?;
        if index <= 0 {
            None
        } else {
            sim.widgets
                .get(texture_id)
                .and_then(|f| f.mask_textures.get((index - 1) as usize).copied())
        }
    };
    if let Some(mask_id) = mask_id {
        let mask_ref = frame_ref(state, mask_id)?;
        state.push(mask_ref);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

pub(super) fn create_line(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let layer = opt_string(state, 3);
    let _inherits = opt_string(state, 4);
    let name = resolve_child_name(state, name_raw, parent_id);
    let mut line = Frame::new(WidgetType::Line, name.clone(), Some(parent_id));
    apply_draw_layer(&mut line, layer);
    let child_id = line.id;
    register_child_widget(state, parent_id, child_id, line)?;
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

pub(super) fn attach_texture(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let texture = Frame::new(WidgetType::Texture, None, Some(parent_id));
    let child_id = texture.id;
    register_child_widget(state, parent_id, child_id, texture)?;
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

pub(super) fn attach_font_string(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let fontstring = Frame::new(WidgetType::FontString, None, Some(parent_id));
    let child_id = fontstring.id;
    register_child_widget(state, parent_id, child_id, fontstring)?;
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}
