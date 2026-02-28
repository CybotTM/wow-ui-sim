//! Child creation methods: CreateTexture, CreateFontString, CreateAnimationGroup, etc.

use super::super::handle::FrameRef;
use crate::lua_api::animation::{AnimGroupHandle, AnimGroupState};
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use crate::lua_api::globals::create_frame::apply_parent_sub;
use crate::widget::{Frame, WidgetType};
use mlua::Value;
use std::rc::Rc;

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
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let group_ids: Vec<u64> = state
            .animation_groups
            .iter()
            .filter(|(_, g)| g.owner_frame_id == id)
            .map(|(&gid, _)| gid)
            .collect();
        drop(state);
        let mut values = Vec::with_capacity(group_ids.len());
        for group_id in group_ids {
            let handle = AnimGroupHandle {
                group_id,
                state: Rc::clone(&state_rc),
            };
            values.push(mlua::Value::UserData(lua.create_userdata(handle)?));
        }
        Ok(mlua::MultiValue::from_vec(values))
    });
}

/// CreateAnimationGroup(name, inherits)
fn add_create_animation_group_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateAnimationGroup", |lua, this, (name, _inherits): (Option<String>, Option<String>)| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let group_id;
        {
            let mut state = state_rc.borrow_mut();
            group_id = state.next_anim_group_id;
            state.next_anim_group_id += 1;
            let mut group = AnimGroupState::new(id);
            group.name = name;
            state.animation_groups.insert(group_id, group);
        }

        let handle = AnimGroupHandle {
            group_id,
            state: Rc::clone(&state_rc),
        };
        lua.create_userdata(handle)
    });
}
