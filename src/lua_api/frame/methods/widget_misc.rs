//! Miscellaneous widget methods: drag/move/resize, SimpleHTML, and stubs.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use super::methods_rect::{resolve_and_extract, to_wow_rect};
use crate::lua_api::frame::handle::get_sim_state;
use crate::render::font::WowFontSystem;
use mlua::Value;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub fn add_drag_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_drag_move_methods(methods);
    add_drag_movable_resizable_methods(methods);
    add_drag_clamp_methods(methods);
    add_drag_resize_methods(methods);
}

pub fn add_simplehtml_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_simplehtml_hyperlink_methods(methods);
    add_simplehtml_content_methods(methods);
    methods.add_method("GetIndentedWordWrap", |_, _, ()| Ok(false));
}

pub fn add_misc_widget_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_misc_stubs_simple(methods);
    add_misc_stubs_mixin(methods);
}

// --- SimpleHTML ---

fn add_simplehtml_hyperlink_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_simplehtml_string_setter(methods, "SetHyperlinkFormat", |data, value| {
        data.hyperlink_format = value
    });
    add_simplehtml_string_getter(methods, "GetHyperlinkFormat", "|H%s|h%s|h", |data| {
        data.hyperlink_format.clone()
    });
    add_set_hyperlinks_enabled(methods);
    add_simplehtml_bool_getter(methods, "GetHyperlinksEnabled", true, |data| {
        data.hyperlinks_enabled
    });
}

fn add_set_hyperlinks_enabled<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHyperlinksEnabled", |lua, this, value: bool| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "SetHyperlinksEnabled") {
                return Ok(());
            }
        }
        update_simplehtml_data(lua, id, |data| data.hyperlinks_enabled = value);
        Ok(())
    });
}

fn update_simplehtml_data<F>(lua: &mlua::Lua, id: u64, update: F)
where
    F: FnOnce(&mut crate::lua_api::simple_html::SimpleHtmlData),
{
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        update(data);
    }
}

fn read_simplehtml_data<T, F>(lua: &mlua::Lua, id: u64, default: T, read: F) -> T
where
    F: FnOnce(&crate::lua_api::simple_html::SimpleHtmlData) -> T,
{
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state.simple_htmls.get(&id).map(read).unwrap_or(default)
}

fn add_simplehtml_string_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::lua_api::simple_html::SimpleHtmlData, String) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: String| {
        update_simplehtml_data(lua, this.0, |data| setter(data, value));
        Ok(())
    });
}

fn add_simplehtml_string_getter<M, F>(
    methods: &mut M,
    name: &'static str,
    default: &'static str,
    getter: F,
) where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::lua_api::simple_html::SimpleHtmlData) -> String + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        Ok(read_simplehtml_data(
            lua,
            this.0,
            default.to_string(),
            getter,
        ))
    });
}

fn add_simplehtml_bool_getter<M, F>(methods: &mut M, name: &'static str, default: bool, getter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::lua_api::simple_html::SimpleHtmlData) -> bool + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        Ok(read_simplehtml_data(lua, this.0, default, getter))
    });
}

fn add_simplehtml_content_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetContentHeight", |lua, this, ()| {
        measure_simplehtml_content_height(lua, this.0)
    });

    methods.add_method("GetTextData", |lua, _this, ()| {
        Ok(Value::Table(lua.create_table()?))
    });
}

fn measure_simplehtml_content_height(lua: &mlua::Lua, id: u64) -> mlua::Result<f64> {
    let resolved = resolve_and_extract(lua, id);
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let frame = match state.widgets.get(id) {
        Some(frame) => frame,
        None => return Ok(0.0),
    };
    let text = match frame.text_stripped.as_deref().or(frame.text.as_deref()) {
        Some(text) if !text.is_empty() => text.to_string(),
        _ => return Ok(0.0),
    };
    let font_path = frame.font.clone();
    let font_size = frame.font_size;
    let wrap_width = frame.word_wrap.then(|| {
        resolved
            .as_ref()
            .map(|rect| {
                let (_, _, width, _) = to_wow_rect(rect);
                width
            })
            .filter(|width| *width > 0.0)
            .unwrap_or(frame.width)
    });
    drop(state);

    Ok(
        measure_simplehtml_text_height(lua, &text, font_path.as_deref(), font_size, wrap_width)
            as f64,
    )
}

fn measure_simplehtml_text_height(
    lua: &mlua::Lua,
    text: &str,
    font_path: Option<&str>,
    font_size: f32,
    wrap_width: Option<f32>,
) -> f32 {
    if let Some(font_system_rc) = lua.app_data_ref::<Rc<RefCell<WowFontSystem>>>() {
        let mut font_system = font_system_rc.borrow_mut();
        return font_system.measure_text_height(text, font_path, font_size, wrap_width);
    }

    let mut font_system = WowFontSystem::new(&default_fonts_dir());
    font_system.measure_text_height(text, font_path, font_size, wrap_width)
}

fn default_fonts_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts")
}

// --- Drag/Move/Resize ---

fn add_drag_move_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_start_moving(methods);
    add_stop_moving_or_sizing(methods);
    methods.add_method("SetMovable", |lua, this, movable: bool| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0)
        {
            frame.movable = movable;
        }
        Ok(())
    });
    methods.add_method("IsMovable", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0)
        {
            return Ok(frame.movable);
        }
        Ok(false)
    });
}

fn add_start_moving<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("StartMoving", |lua, this, ()| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "StartMoving") {
                return Ok(());
            }
        }
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(id)
            && frame.movable
        {
            frame.is_moving = true;
        }
        Ok(())
    });
}

fn add_stop_moving_or_sizing<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("StopMovingOrSizing", |lua, this, ()| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "StopMovingOrSizing") {
                return Ok(());
            }
        }
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(id)
        {
            frame.is_moving = false;
        }
        Ok(())
    });
}

fn add_drag_movable_resizable_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetResizable", |lua, this, resizable: bool| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0)
        {
            frame.resizable = resizable;
        }
        Ok(())
    });
    methods.add_method("IsResizable", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0)
        {
            return Ok(frame.resizable);
        }
        Ok(false)
    });
}

fn add_drag_clamp_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetClampedToScreen", |lua, this, clamped: bool| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "SetClampedToScreen") {
                return Ok(());
            }
        }
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(id)
        {
            frame.clamped_to_screen = clamped;
        }
        Ok(())
    });
    methods.add_method("IsClampedToScreen", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0)
        {
            return Ok(frame.clamped_to_screen);
        }
        Ok(false)
    });
    methods.add_method("SetClampRectInsets", |lua, this, args: mlua::MultiValue| {
        let mut values = args.into_iter();
        let left = next_inset_value(&mut values);
        let right = next_inset_value(&mut values);
        let top = next_inset_value(&mut values);
        let bottom = next_inset_value(&mut values);
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0)
        {
            frame.clamp_rect_insets = (left, right, top, bottom);
        }
        Ok(())
    });
}

fn next_inset_value(values: &mut impl Iterator<Item = mlua::Value>) -> f32 {
    match values.next() {
        Some(mlua::Value::Number(n)) => n as f32,
        Some(mlua::Value::Integer(n)) => n as f32,
        _ => 0.0,
    }
}

fn add_drag_resize_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetResizeBounds", |lua, this, args: mlua::MultiValue| {
        let mut values = args.into_iter();
        let min_width = next_required_resize_bound(&mut values);
        let min_height = next_required_resize_bound(&mut values);
        let max_width = next_optional_resize_bound(&mut values);
        let max_height = next_optional_resize_bound(&mut values);
        let resize_bounds_max = match (max_width, max_height) {
            (Some(width), Some(height)) => Some((width, height)),
            _ => None,
        };

        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.resize_bounds_min = (min_width, min_height);
            frame.resize_bounds_max = resize_bounds_max;
        }
        Ok(())
    });
    methods.add_method("GetResizeBounds", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            let (min_width, min_height) = frame.resize_bounds_min;
            let (max_width, max_height) = frame
                .resize_bounds_max
                .map(|(width, height)| (Some(width), Some(height)))
                .unwrap_or((None, None));
            return Ok((min_width, min_height, max_width, max_height));
        }
        Ok((0.0_f32, 0.0_f32, None::<f32>, None::<f32>))
    });
    methods.add_method("SetMinResize", |lua, this, (width, height): (f32, f32)| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.resize_bounds_min = (width, height);
        }
        Ok(())
    });
    methods.add_method("SetMaxResize", |lua, this, (width, height): (f32, f32)| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.resize_bounds_max = Some((width, height));
        }
        Ok(())
    });
    methods.add_method("StartSizing", |_, _this, _point: Option<String>| Ok(()));
    methods.add_method(
        "RegisterForDrag",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("SetUserPlaced", |lua, this, user_placed: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.user_placed = user_placed;
        }
        Ok(())
    });
    methods.add_method("IsUserPlaced", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.user_placed)
            .unwrap_or(false))
    });
    methods.add_method("SetDontSavePosition", |lua, this, dont_save: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.dont_save_position = dont_save;
        }
        Ok(())
    });
}

fn next_required_resize_bound(values: &mut impl Iterator<Item = mlua::Value>) -> f32 {
    next_optional_resize_bound(values).unwrap_or(0.0)
}

fn next_optional_resize_bound(values: &mut impl Iterator<Item = mlua::Value>) -> Option<f32> {
    match values.next() {
        Some(mlua::Value::Number(n)) => Some(n as f32),
        Some(mlua::Value::Integer(n)) => Some(n as f32),
        _ => None,
    }
}

// --- Misc stubs ---

fn add_misc_stubs_simple<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetupMenu", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetAlertContainer", |_, _this, _container: Value| Ok(()));
    methods.add_method("SetColorFill", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetTextToFit", |lua, this, text: Option<String>| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.text = text;
        }
        Ok(())
    });
    methods.add_method("SetSelectionTranslator", |_, _this, _func: Value| Ok(()));
    methods.add_method("SetItemButtonScale", |_, _this, _scale: Value| Ok(()));
    methods.add_method(
        "UpdateItemContextMatching",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("UpdateHeight", |_, _this, ()| Ok(()));
    methods.add_method("SetDefaultText", |_, _this, _text: Value| Ok(()));
    methods.add_method("SetVisuals", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method(
        "RegisterForWidgetSet",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method(
        "UnregisterForWidgetSet",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
}

fn add_misc_stubs_mixin<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetRotationIncrement", |lua, this, inc: Value| {
        let id = this.0;
        if let Some((func, frame_ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "SetRotationIncrement")
        {
            return func.call::<()>((frame_ud, inc));
        }
        Ok(())
    });
}
