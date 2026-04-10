//! Button-specific methods: SetNormalTexture, SetPushedTexture, font objects, etc.

use super::super::handle::FrameRef;
use super::methods_button_state;
use super::methods_helpers::get_or_create_button_texture;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state, sync_child_to_lua};
use mlua::Value;

/// Add button-specific methods to the shared methods table.
pub fn add_button_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_font_object_methods(methods);
    add_pushed_text_offset_methods(methods);
    add_texture_getter_methods(methods);
    add_texture_setter_methods(methods);
    add_texture_setter_methods_2(methods);
    add_atlas_setter_methods(methods);
    add_checked_texture_methods(methods);
    add_clear_texture_methods(methods);
    add_three_slice_methods(methods);
    add_font_string_methods(methods);
    methods_button_state::add_button_state_methods(methods);
}

/// Set/Get font objects for normal, highlight, and disabled states.
///
/// Stores font objects in `_G.__button_font_objects` keyed by `"{frame_id}:{state}"`.
fn add_font_object_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    for (set_name, get_name, state_key) in [
        ("SetNormalFontObject", "GetNormalFontObject", "normal"),
        (
            "SetHighlightFontObject",
            "GetHighlightFontObject",
            "highlight",
        ),
        ("SetDisabledFontObject", "GetDisabledFontObject", "disabled"),
    ] {
        methods.add_method(set_name, move |lua, this, font_object: Value| {
            let id = this.0;
            let store: mlua::Table = lua
                .load("_G.__button_font_objects = _G.__button_font_objects or {}; return _G.__button_font_objects")
                .eval()?;
            let key = format!("{}:{}", id, state_key);
            store.set(key, font_object)?;
            Ok(())
        });

        methods.add_method(get_name, move |lua, this, ()| {
            let id = this.0;
            let store: mlua::Table = lua.load("return _G.__button_font_objects or {}").eval()?;
            let key = format!("{}:{}", id, state_key);
            let font: Value = store.get(key)?;
            Ok(font)
        });
    }
}

/// SetPushedTextOffset / GetPushedTextOffset.
fn add_pushed_text_offset_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPushedTextOffset", |lua, this, (x, y): (f64, f64)| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.pushed_text_offset = (x as f32, y as f32);
        }
        Ok(())
    });
    methods.add_method("GetPushedTextOffset", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let (x, y) = state
            .widgets
            .get(this.0)
            .map(|frame| frame.pushed_text_offset)
            .unwrap_or((0.0, 0.0));
        Ok((f64::from(x), f64::from(y)))
    });
}

/// Get{Normal,Highlight,Pushed,Disabled}Texture — return existing texture child or nil.
fn add_texture_getter_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    for (method_name, parent_key) in [
        ("GetNormalTexture", "NormalTexture"),
        ("GetHighlightTexture", "HighlightTexture"),
        ("GetPushedTexture", "PushedTexture"),
        ("GetDisabledTexture", "DisabledTexture"),
    ] {
        methods.add_method(method_name, move |lua, this, ()| {
            get_existing_button_texture(lua, this.0, parent_key)
        });
    }
}

fn get_existing_button_texture(
    lua: &mlua::Lua,
    button_id: u64,
    parent_key: &str,
) -> mlua::Result<Value> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let tex_id = state
        .widgets
        .get(button_id)
        .and_then(|frame| frame.children_keys.get(parent_key).copied());
    match tex_id {
        Some(texture_id) => frame_ref(lua, texture_id),
        None => Ok(Value::Nil),
    }
}

/// Extract texture path from a Lua Value.
fn extract_texture_path(texture: &Value) -> Result<Option<String>, mlua::Error> {
    Ok(super::methods_helpers::resolve_file_data_id_or_path(
        texture,
    ))
}

/// Resolved texture info: file path and optional UV coords (from atlas lookup).
struct ResolvedTexture {
    path: String,
    tex_coords: Option<(f32, f32, f32, f32)>,
}

/// Resolve a texture string as either an atlas name or a file path.
fn resolve_texture_string(name: &str) -> ResolvedTexture {
    if let Some(lookup) = crate::atlas::get_atlas_info(name) {
        let info = lookup.info;
        ResolvedTexture {
            path: info.file.to_string(),
            tex_coords: Some((
                info.left_tex_coord,
                info.right_tex_coord,
                info.top_tex_coord,
                info.bottom_tex_coord,
            )),
        }
    } else {
        ResolvedTexture {
            path: name.to_string(),
            tex_coords: None,
        }
    }
}

/// Apply a resolved texture (path + optional atlas UVs) to a button field and its child texture.
#[allow(clippy::type_complexity)]
fn apply_button_texture_setter(
    lua: &mlua::Lua,
    state: &mut crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
    texture: &Value,
    set_button_field: fn(&mut crate::widget::Frame, Option<String>, Option<(f32, f32, f32, f32)>),
) -> Result<(), mlua::Error> {
    if let Value::UserData(_) = texture {
        if let Some(tex_id) = crate::lua_api::frame::extract_frame_id(texture) {
            apply_set_button_texture(lua, state, button_id, parent_key, tex_id);
        }
    } else {
        apply_set_button_texture_path(
            lua,
            state,
            button_id,
            parent_key,
            texture,
            set_button_field,
        )?;
    }
    Ok(())
}

/// Assign a texture userdata as a button's texture child.
fn apply_set_button_texture(
    lua: &mlua::Lua,
    state: &mut crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
    tex_id: u64,
) {
    let current_parent = state.widgets.get(tex_id).and_then(|f| f.parent_id);
    if current_parent != Some(button_id) {
        super::methods_hierarchy::reparent_widget(&mut state.widgets, tex_id, Some(button_id));
    }
    if let Some(tex) = state.widgets.get_mut_visual(tex_id) {
        tex.anchors.clear();
        super::methods_helpers::set_all_points_anchors_pub(tex, button_id);
        tex.parent_key = Some(parent_key.to_string());
    }
    if let Some(btn) = state.widgets.get_mut_visual(button_id) {
        btn.children_keys.insert(parent_key.to_string(), tex_id);
    }
    if parent_key == "HighlightTexture" {
        if let Some(tex) = state.widgets.get_mut_visual(tex_id) {
            tex.draw_layer = crate::widget::DrawLayer::Highlight;
            tex.alpha_mode = Some("ADD".to_string());
            tex.blend_mode = crate::render::BlendMode::Additive;
        }
    }
    let _ = sync_child_to_lua(lua, button_id, parent_key, tex_id);
    let should_show = button_texture_should_show(state, button_id, parent_key);
    state.widgets.set_visible(tex_id, should_show);
}

/// Set a button texture by path/atlas/fileDataID (non-userdata).
#[allow(clippy::type_complexity)]
fn apply_set_button_texture_path(
    lua: &mlua::Lua,
    state: &mut crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
    texture: &Value,
    set_button_field: fn(&mut crate::widget::Frame, Option<String>, Option<(f32, f32, f32, f32)>),
) -> Result<(), mlua::Error> {
    let file_data_id = match texture {
        Value::Integer(n) => Some(*n as i64),
        Value::Number(n) => Some(*n as i64),
        _ => None,
    };
    let path = extract_texture_path(texture)?;
    let resolved = path.as_ref().map(|p| resolve_texture_string(p));
    let resolved_path = resolved.as_ref().map(|r| r.path.clone());
    let tex_coords = resolved.as_ref().and_then(|r| r.tex_coords);
    if let Some(frame) = state.widgets.get_mut_visual(button_id) {
        set_button_field(frame, resolved_path.clone(), tex_coords);
    }
    let tex_id = get_or_create_button_texture(lua, state, button_id, parent_key);
    if let Some(tex) = state.widgets.get_mut_visual(tex_id) {
        tex.texture = resolved_path;
        tex.tex_coords = tex_coords;
        tex.atlas_tex_coords = tex_coords;
        tex.texture_file_data_id = file_data_id;
    }
    let should_show = button_texture_should_show(state, button_id, parent_key);
    state.widgets.set_visible(tex_id, should_show);
    Ok(())
}

/// Determine if a button texture child should be visible based on button state.
pub(super) fn button_texture_should_show(
    state: &crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
) -> bool {
    let (enabled, button_state) = state
        .widgets
        .get(button_id)
        .map(|f| {
            let en = f
                .attributes
                .get("__enabled")
                .and_then(|v| match v {
                    crate::widget::AttributeValue::Boolean(b) => Some(*b),
                    _ => None,
                })
                .unwrap_or(true);
            (en, f.button_state)
        })
        .unwrap_or((true, 0));
    match parent_key {
        "NormalTexture" => enabled && button_state == 0,
        "PushedTexture" => enabled && button_state == 1,
        "DisabledTexture" => !enabled,
        _ => true,
    }
}

/// Set{Normal,Highlight}Texture - set texture by path, atlas name, or userdata.
fn add_texture_setter_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetNormalTexture", |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        apply_button_texture_setter(
            lua,
            &mut state,
            this.0,
            "NormalTexture",
            &texture,
            |f, path, coords| {
                f.normal_texture = path;
                f.normal_tex_coords = coords;
            },
        )
    });

    methods.add_method("SetHighlightTexture", |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        apply_button_texture_setter(
            lua,
            &mut state,
            this.0,
            "HighlightTexture",
            &texture,
            |f, path, coords| {
                f.highlight_texture = path;
                f.highlight_tex_coords = coords;
            },
        )
    });
}

/// Set{Pushed,Disabled}Texture - set texture by path, atlas name, or userdata.
fn add_texture_setter_methods_2<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPushedTexture", |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        apply_button_texture_setter(
            lua,
            &mut state,
            this.0,
            "PushedTexture",
            &texture,
            |f, path, coords| {
                f.pushed_texture = path;
                f.pushed_tex_coords = coords;
            },
        )
    });

    methods.add_method("SetDisabledTexture", |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        apply_button_texture_setter(
            lua,
            &mut state,
            this.0,
            "DisabledTexture",
            &texture,
            |f, path, coords| {
                f.disabled_texture = path;
                f.disabled_tex_coords = coords;
            },
        )
    });
}

/// Set{Normal,Pushed,Disabled,Highlight}Atlas.
fn add_atlas_setter_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_normal_atlas_method(methods);
    add_pushed_atlas_method(methods);
    add_disabled_atlas_method(methods);
    add_highlight_atlas_method(methods);
}

fn add_normal_atlas_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetNormalAtlas", |lua, this, args: mlua::MultiValue| {
        if let Some(atlas_name) = extract_string_arg(&args) {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            let tex_id = get_or_create_button_texture(lua, &mut state, this.0, "NormalTexture");
            apply_atlas_to_button(
                &mut state,
                this.0,
                tex_id,
                &atlas_name,
                |f, file, coords| {
                    f.normal_texture = Some(file);
                    f.normal_tex_coords = Some(coords);
                },
            );
        }
        Ok(())
    });
}

fn add_pushed_atlas_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPushedAtlas", |lua, this, args: mlua::MultiValue| {
        if let Some(atlas_name) = extract_string_arg(&args) {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            let tex_id = get_or_create_button_texture(lua, &mut state, this.0, "PushedTexture");
            apply_atlas_to_button(
                &mut state,
                this.0,
                tex_id,
                &atlas_name,
                |f, file, coords| {
                    f.pushed_texture = Some(file);
                    f.pushed_tex_coords = Some(coords);
                },
            );
        }
        Ok(())
    });
}

fn add_disabled_atlas_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDisabledAtlas", |lua, this, args: mlua::MultiValue| {
        if let Some(atlas_name) = extract_string_arg(&args) {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            let tex_id = get_or_create_button_texture(lua, &mut state, this.0, "DisabledTexture");
            apply_atlas_to_button(
                &mut state,
                this.0,
                tex_id,
                &atlas_name,
                |f, file, coords| {
                    f.disabled_texture = Some(file);
                    f.disabled_tex_coords = Some(coords);
                },
            );
        }
        Ok(())
    });
}

fn add_highlight_atlas_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHighlightAtlas", |lua, this, args: mlua::MultiValue| {
        if let Some(atlas_name) = extract_string_arg(&args) {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            let tex_id = get_or_create_button_texture(lua, &mut state, this.0, "HighlightTexture");
            apply_atlas_to_button(
                &mut state,
                this.0,
                tex_id,
                &atlas_name,
                |f, file, coords| {
                    f.highlight_texture = Some(file);
                    f.highlight_tex_coords = Some(coords);
                },
            );
        }
        Ok(())
    });
}

/// Extract a string from the first argument of a MultiValue.
fn extract_string_arg(args: &mlua::MultiValue) -> Option<String> {
    args.iter().next().and_then(|v| match v {
        Value::String(s) => Some(s.to_string_lossy().to_string()),
        _ => None,
    })
}

/// Apply atlas info to both the child texture widget and the parent button field.
fn apply_atlas_to_button<F>(
    state: &mut std::cell::RefMut<'_, crate::lua_api::SimState>,
    button_id: u64,
    tex_id: u64,
    atlas_name: &str,
    set_button_field: F,
) where
    F: FnOnce(&mut crate::widget::Frame, String, (f32, f32, f32, f32)),
{
    let already_set = state
        .widgets
        .get(tex_id)
        .map(|t| t.atlas.as_deref() == Some(atlas_name))
        .unwrap_or(false);
    if already_set {
        return;
    }
    if let Some(lookup) = crate::atlas::get_atlas_info(atlas_name) {
        let tex_coords = (
            lookup.info.left_tex_coord,
            lookup.info.right_tex_coord,
            lookup.info.top_tex_coord,
            lookup.info.bottom_tex_coord,
        );
        if let Some(tex) = state.widgets.get_mut_visual(tex_id) {
            tex.atlas = Some(atlas_name.to_string());
            tex.texture = Some(lookup.info.file.to_string());
            tex.tex_coords = Some(tex_coords);
        }
        if let Some(frame) = state.widgets.get_mut_visual(button_id) {
            set_button_field(frame, lookup.info.file.to_string(), tex_coords);
        }
    }
}

/// Set{Checked,DisabledChecked}Texture.
fn add_checked_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCheckedTexture", |lua, this, texture: Value| {
        let id = this.0;
        let path = extract_texture_path(&texture)?;
        let is_userdata = matches!(texture, Value::UserData(_));
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if !is_userdata && let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.checked_texture = path.clone();
        }
        let tex_id = get_or_create_button_texture(lua, &mut state, id, "CheckedTexture");
        if let Some(tex) = state.widgets.get_mut_visual(tex_id) {
            if !is_userdata {
                tex.texture = path;
            }
            tex.visible = false;
        }
        Ok(())
    });

    methods.add_method("SetDisabledCheckedTexture", |lua, this, texture: Value| {
        let id = this.0;
        let path = extract_texture_path(&texture)?;
        let is_userdata = matches!(texture, Value::UserData(_));
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if !is_userdata && let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.disabled_checked_texture = path.clone();
        }
        let tex_id = get_or_create_button_texture(lua, &mut state, id, "DisabledCheckedTexture");
        if let Some(tex) = state.widgets.get_mut_visual(tex_id) {
            if !is_userdata {
                tex.texture = path;
            }
            tex.visible = false;
        }
        Ok(())
    });
    methods.add_method("GetDisabledCheckedTexture", |lua, this, ()| {
        get_existing_button_texture(lua, this.0, "DisabledCheckedTexture")
    });
}

/// Clear{Normal,Highlight,Pushed,Disabled}Texture.
fn add_clear_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    for (method_name, parent_key) in [
        ("ClearNormalTexture", "NormalTexture"),
        ("ClearHighlightTexture", "HighlightTexture"),
        ("ClearPushedTexture", "PushedTexture"),
        ("ClearDisabledTexture", "DisabledTexture"),
    ] {
        methods.add_method(method_name, move |lua, this, _: mlua::Variadic<Value>| {
            clear_button_texture(lua, this.0, parent_key);
            Ok(())
        });
    }
}

fn clear_button_texture(lua: &mlua::Lua, button_id: u64, parent_key: &str) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    clear_button_texture_field(&mut state.widgets, button_id, parent_key);
    clear_button_texture_child(&mut state.widgets, button_id, parent_key);
    state.widgets.mark_rect_dirty(button_id);
}

fn clear_button_texture_field(
    widgets: &mut crate::widget::WidgetRegistry,
    button_id: u64,
    parent_key: &str,
) {
    let Some(button) = widgets.get_mut_visual(button_id) else {
        return;
    };
    match parent_key {
        "NormalTexture" => {
            button.normal_texture = None;
            button.normal_tex_coords = None;
        }
        "HighlightTexture" => {
            button.highlight_texture = None;
            button.highlight_tex_coords = None;
        }
        "PushedTexture" => {
            button.pushed_texture = None;
            button.pushed_tex_coords = None;
        }
        "DisabledTexture" => {
            button.disabled_texture = None;
            button.disabled_tex_coords = None;
        }
        _ => {}
    }
}

fn clear_button_texture_child(
    widgets: &mut crate::widget::WidgetRegistry,
    button_id: u64,
    parent_key: &str,
) {
    let child_id = widgets
        .get(button_id)
        .and_then(|button| button.children_keys.get(parent_key).copied());
    let Some(child_id) = child_id else {
        return;
    };
    let Some(child) = widgets.get_mut_visual(child_id) else {
        return;
    };
    child.texture = None;
    child.texture_file_data_id = None;
    child.tex_coords = None;
    child.tex_coords_quad = None;
    child.atlas_tex_coords = None;
    child.atlas = None;
    child.three_slice_h = None;
}

/// Set{Left,Middle,Right}Texture - three-slice button cap textures.
fn add_three_slice_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetLeftTexture", |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.left_texture = value_to_optional_path(texture)?;
        }
        Ok(())
    });

    methods.add_method("SetMiddleTexture", |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.middle_texture = value_to_optional_path(texture)?;
        }
        Ok(())
    });

    methods.add_method("SetRightTexture", |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.right_texture = value_to_optional_path(texture)?;
        }
        Ok(())
    });
}

/// Convert a Lua Value to an optional texture path string.
fn value_to_optional_path(value: Value) -> Result<Option<String>, mlua::Error> {
    match value {
        Value::String(s) => Ok(Some(s.to_str()?.to_string())),
        _ => Ok(None),
    }
}

/// GetFontString / SetFontString.
fn add_font_string_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetFontString", |lua, this, ()| {
        get_font_string_impl(lua, this.0)
    });
    methods.add_method("SetFontString", |lua, this, fontstring: Value| {
        set_font_string_impl(lua, this.0, fontstring)
    });
}

fn get_font_string_impl(lua: &mlua::Lua, id: u64) -> mlua::Result<Value> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(frame) = state.widgets.get(id)
        && let Some(&text_id) = frame.children_keys.get("Text")
    {
        drop(state);
        return frame_ref(lua, text_id);
    }
    drop(state);
    if let Some((func, ud_val)) =
        super::methods_helpers::get_mixin_override(lua, id, "GetFontString")
    {
        return func.call::<Value>(ud_val).map(Ok)?;
    }
    Ok(Value::Nil)
}

fn set_font_string_impl(lua: &mlua::Lua, button_id: u64, fontstring: Value) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let fs_id_opt = if let Some(fs_id) = super::super::handle::extract_frame_id(&fontstring) {
        let mut state = state_rc.borrow_mut();
        super::methods_hierarchy::reparent_widget(&mut state.widgets, fs_id, Some(button_id));
        if let Some(fs) = state.widgets.get_mut_visual(fs_id) {
            fs.anchors.clear();
            super::methods_helpers::set_all_points_anchors_pub(fs, button_id);
        }
        if let Some(btn) = state.widgets.get_mut_visual(button_id) {
            btn.children_keys.insert("Text".to_string(), fs_id);
        }
        if let Some(fs) = state.widgets.get_mut_visual(fs_id) {
            fs.parent_key = Some("Text".to_string());
        }
        Some(fs_id)
    } else {
        let mut state = state_rc.borrow_mut();
        if let Some(btn) = state.widgets.get_mut_visual(button_id) {
            btn.children_keys.remove("Text");
        }
        None
    };
    if let Some(fs_id) = fs_id_opt {
        sync_child_to_lua(lua, button_id, "Text", fs_id)?;
    }
    Ok(())
}
