//! Button-specific methods: SetNormalTexture, SetPushedTexture, font objects, etc.

use super::super::handle::FrameRef;
use super::methods_button_state;
pub(crate) use super::methods_button_texture::button_texture_should_show;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state, sync_child_to_lua};
use mlua::Value;

/// Add button-specific methods to the shared methods table.
pub fn add_button_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_font_object_methods(methods);
    add_pushed_text_offset_methods(methods);
    super::methods_button_texture::add_button_texture_methods(methods);
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
            let store = get_or_create_button_font_store(lua)?;
            let key = format!("{}:{}", id, state_key);
            store.set(key, font_object)?;
            Ok(())
        });

        methods.add_method(get_name, move |lua, this, ()| {
            let id = this.0;
            let store = get_or_create_button_font_store(lua)?;
            let key = format!("{}:{}", id, state_key);
            let font: Value = store.get(key)?;
            Ok(font)
        });
    }
}

/// Return the `_G.__button_font_objects` table, creating + caching on first access.
fn get_or_create_button_font_store(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    if let Ok(t) = lua.named_registry_value::<mlua::Table>("__button_font_objects_cache") {
        return Ok(t);
    }
    let globals = lua.globals();
    let table = match globals.get::<Value>("__button_font_objects")? {
        Value::Table(t) => t,
        _ => {
            let t = lua.create_table()?;
            globals.set("__button_font_objects", t.clone())?;
            t
        }
    };
    lua.set_named_registry_value("__button_font_objects_cache", table.clone())?;
    Ok(table)
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
