//! Child creation methods: CreateTexture, CreateFontString, CreateAnimationGroup, etc.

use super::super::handle::FrameRef;
use crate::lua_api::animation::AnimGroupState;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use crate::lua_api::globals::create_frame::apply_parent_sub;
use crate::widget::{Frame, WidgetType};
use mlua::Value;

/// Resolve a raw name with $parent substitution using the parent widget's ancestor chain.
fn resolve_child_name(lua: &mlua::Lua, name_raw: Option<String>, parent_id: u64) -> Option<String> {
    name_raw.map(|n| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        apply_parent_sub(&n, Some(parent_id), &state)
    })
}

/// Extract an optional string from the first element of a MultiValue args list.
fn extract_string_arg(args: &[Value], index: usize) -> Option<String> {
    args.get(index).and_then(|v| {
        if let Value::String(s) = v {
            Some(s.to_string_lossy().to_string())
        } else {
            None
        }
    })
}

/// Register a child widget in the state and cache its FrameRef UserData in `_G`.
fn register_child_widget(
    lua: &mlua::Lua,
    parent_id: u64,
    child: Frame,
    name: &Option<String>,
) -> mlua::Result<Value> {
    let child_id = child.id;

    {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.widgets.register(child);
        state.widgets.add_child(parent_id, child_id);

        let parent_props = state
            .widgets
            .get(parent_id)
            .map(|p| (p.frame_strata, p.frame_level));
        if let Some((parent_strata, parent_level)) = parent_props {
            if let Some(f) = state.widgets.get_mut_visual(child_id) {
                f.frame_strata = parent_strata;
                f.frame_level = parent_level + 1;
            }
        }
    }

    let ud = frame_ref(lua, child_id)?;

    if let Some(n) = name {
        lua.globals().raw_set(n.as_str(), ud.clone())?;
    }

    Ok(ud)
}

/// Add child creation methods to the shared methods table.
pub fn add_create_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_create_texture_method(methods);
    add_create_mask_texture_method(methods);
    add_create_line_method(methods);
    add_create_font_string_method(methods);
    add_create_animation_group_method(methods);
    add_get_animation_groups_method(methods);
    add_create_animation_method(methods);
    add_create_control_point_method(methods);
}

/// CreateTexture(name, layer, inherits, subLevel)
fn add_create_texture_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateTexture", |lua, this, args: mlua::MultiValue| {
        use crate::widget::DrawLayer;

        let id = this.0;
        let args: Vec<Value> = args.into_iter().collect();
        let name_raw = extract_string_arg(&args, 0);
        let layer = extract_string_arg(&args, 1);
        let name = resolve_child_name(lua, name_raw, id);

        let mut texture = Frame::new(WidgetType::Texture, name.clone(), Some(id));

        if let Some(layer_str) = layer
            && let Some(draw_layer) = DrawLayer::from_str(&layer_str)
        {
            texture.draw_layer = draw_layer;
        }

        register_child_widget(lua, id, texture, &name)
    });
}

/// CreateMaskTexture(layer, inherits, subLevel) - create a mask texture.
fn add_create_mask_texture_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateMaskTexture", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args: Vec<Value> = args.into_iter().collect();
        let name_raw = extract_string_arg(&args, 0);
        let name = resolve_child_name(lua, name_raw, id);
        let mut texture = Frame::new(WidgetType::Texture, name.clone(), Some(id));
        texture.is_mask = true;
        texture.object_type_name = Some("MaskTexture".to_string());
        register_child_widget(lua, id, texture, &name)
    });
}

/// CreateLine(name, layer, inherits, subLevel) - create a Line.
fn add_create_line_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateLine", |lua, this, args: mlua::MultiValue| {
        use crate::widget::DrawLayer;

        let id = this.0;
        let args: Vec<Value> = args.into_iter().collect();
        let name_raw = extract_string_arg(&args, 0);
        let layer = extract_string_arg(&args, 1);
        let name = resolve_child_name(lua, name_raw, id);

        let mut line = Frame::new(WidgetType::Line, name.clone(), Some(id));

        if let Some(layer_str) = layer
            && let Some(draw_layer) = DrawLayer::from_str(&layer_str)
        {
            line.draw_layer = draw_layer;
        }

        register_child_widget(lua, id, line, &name)
    });
}

/// CreateFontString(name, layer, inherits)
fn add_create_font_string_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateFontString", |lua, this, args: mlua::MultiValue| {
        use crate::widget::DrawLayer;

        let id = this.0;
        let args: Vec<Value> = args.into_iter().collect();
        let name_raw = extract_string_arg(&args, 0);
        let layer = extract_string_arg(&args, 1);
        let inherits = extract_string_arg(&args, 2);
        let name = resolve_child_name(lua, name_raw, id);

        let mut fontstring = Frame::new(WidgetType::FontString, name.clone(), Some(id));

        if let Some(layer_str) = layer
            && let Some(draw_layer) = DrawLayer::from_str(&layer_str)
        {
            fontstring.draw_layer = draw_layer;
        }

        apply_font_inherit(lua, &mut fontstring, inherits.as_deref());

        register_child_widget(lua, id, fontstring, &name)
    });
}

/// Apply font properties from an inherited Font object to a fontstring widget.
fn apply_font_inherit(lua: &mlua::Lua, frame: &mut Frame, inherits: Option<&str>) {
    let Some(name) = inherits else { return };
    let Ok(globals) = lua.globals().get::<Value>(name) else { return };
    let Value::Table(tbl) = globals else { return };
    if let Ok(path) = tbl.get::<String>("__font") {
        frame.font = Some(path);
    }
    if let Ok(height) = tbl.get::<f64>("__height") {
        frame.font_size = height as f32;
    }
    if let Ok(outline) = tbl.get::<String>("__outline") {
        frame.font_outline = crate::widget::TextOutline::from_wow_str(&outline);
    }
    if let Ok(h) = tbl.get::<String>("__justifyH") {
        frame.justify_h = crate::widget::TextJustify::from_wow_str(&h);
    }
}

/// GetAnimationGroups() — return all animation groups owned by this frame.
fn add_get_animation_groups_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetAnimationGroups", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        // Find child frame IDs that are animation groups for this frame.
        let ag_frame_ids: Vec<u64> = state.anim_frame_to_group.iter()
            .filter(|&(_, &gid)| {
                state.animation_groups.get(&gid).is_some_and(|g| g.owner_frame_id == this.0)
            })
            .map(|(&fid, _)| fid)
            .collect();
        drop(state);
        let mut values = Vec::with_capacity(ag_frame_ids.len());
        for fid in ag_frame_ids {
            values.push(frame_ref(lua, fid)?);
        }
        Ok(mlua::MultiValue::from_vec(values))
    });
}

/// CreateAnimationGroup(name, inherits) — returns FrameRef with object_type_name "AnimationGroup".
fn add_create_animation_group_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateAnimationGroup", |lua, this, (name_raw, _inherits): (Option<String>, Option<String>)| {
        let id = this.0;
        let name = resolve_child_name(lua, name_raw, id);
        let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(id));
        child.object_type_name = Some("AnimationGroup".to_string());
        let child_id = child.id;
        let state_rc = get_sim_state(lua);
        let group_id = {
            let mut state = state_rc.borrow_mut();
            let gid = state.next_anim_group_id;
            state.next_anim_group_id += 1;
            let mut group = AnimGroupState::new(id);
            group.name = name.clone();
            group.frame_id = Some(child_id);
            state.animation_groups.insert(gid, group);
            state.anim_frame_to_group.insert(child_id, gid);
            gid
        };
        let _ = group_id;
        register_child_widget(lua, id, child, &name)
    });
}

/// CreateAnimation(type, name) on an AnimationGroup FrameRef.
fn add_create_animation_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateAnimation", |lua, this, args: mlua::MultiValue| {
        let args: Vec<Value> = args.into_iter().collect();
        let anim_type_str = extract_string_arg(&args, 0);
        let anim_name_raw = extract_string_arg(&args, 1);
        create_animation_on_group(lua, this.0, anim_type_str.as_deref(), anim_name_raw)
    });
}

/// Shared logic for creating an animation child on a group FrameRef.
fn create_animation_on_group(
    lua: &mlua::Lua, group_frame_id: u64,
    anim_type_str: Option<&str>, anim_name_raw: Option<String>,
) -> mlua::Result<Value> {
    use crate::lua_api::animation::{AnimState, AnimationType};
    let state_rc = get_sim_state(lua);
    let group_id = {
        let state = state_rc.borrow();
        state.anim_frame_to_group.get(&group_frame_id).copied()
            .ok_or_else(|| mlua::Error::runtime("CreateAnimation called on non-AnimationGroup"))?
    };
    let anim_type = AnimationType::from_str(anim_type_str.unwrap_or("Animation"));
    let type_name = anim_type.as_str().to_string();
    let name = resolve_child_name(lua, anim_name_raw, group_frame_id);
    let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(group_frame_id));
    child.object_type_name = Some(type_name);
    let child_id = child.id;
    let mut anim = AnimState::new(anim_type);
    anim.name = name.clone();
    {
        let mut state = state_rc.borrow_mut();
        let group = state.animation_groups.get_mut(&group_id)
            .ok_or_else(|| mlua::Error::runtime("Animation group not found"))?;
        let idx = group.animations.len();
        group.animations.push(anim);
        state.anim_frame_to_anim.insert(child_id, (group_id, idx));
    }
    register_child_widget(lua, group_frame_id, child, &name)
}

/// CreateControlPoint() on a Path animation FrameRef.
fn add_create_control_point_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateControlPoint", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args: Vec<Value> = args.into_iter().collect();
        let name_raw = extract_string_arg(&args, 0);
        let name = resolve_child_name(lua, name_raw, id);
        let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(id));
        child.object_type_name = Some("ControlPoint".to_string());
        register_child_widget(lua, id, child, &name)
    });
}
