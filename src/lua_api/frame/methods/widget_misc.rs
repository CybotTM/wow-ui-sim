//! Miscellaneous widget methods: drag/move/resize, SimpleHTML, and stubs.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use super::methods_helpers::get_mixin_override;
use super::methods_rect::{resolve_and_extract, to_wow_rect};
use super::methods_text::get_frame_font_object;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use crate::render::font::WowFontSystem;
use crate::widget::Color;
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
    methods.add_method(
        "GetIndentedWordWrap",
        |lua, this, args: mlua::MultiValue| {
            if let Some(Value::String(text_type)) = args.front() {
                let text_type = text_type.to_string_lossy().to_string();
                return Ok(read_simplehtml_indented_word_wrap(lua, this.0, &text_type));
            }
            if let Some((func, self_value)) = get_mixin_override(lua, this.0, "GetIndentedWordWrap")
            {
                return func.call(self_value);
            }
            if let Some(font_object) = get_frame_font_object(lua, this.0)?
                && let Ok(getter) = font_object.get::<mlua::Function>("GetIndentedWordWrap")
            {
                return getter.call(font_object);
            }
            Ok(false)
        },
    );
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

fn read_simplehtml_indented_word_wrap(lua: &mlua::Lua, id: u64, text_type: &str) -> bool {
    read_simplehtml_data(lua, id, false, |data| {
        data.text_styles
            .get(text_type)
            .map(|style| style.indented_word_wrap)
            .unwrap_or(false)
    })
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
            if frame.is_moving || frame.is_sizing {
                frame.user_placed = true;
            }
            frame.is_moving = false;
            frame.is_sizing = false;
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
    add_resize_bounds_methods(methods);
    add_start_sizing_method(methods);
    add_drag_registration_methods(methods);
    add_placement_methods(methods);
}

fn add_resize_bounds_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetResizeBounds", |lua, this, args: mlua::MultiValue| {
        let (min, max) = parse_resize_bounds(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.resize_bounds_min = min;
            frame.resize_bounds_max = max;
        }
        Ok(())
    });
    methods.add_method("GetResizeBounds", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let Some(frame) = state.widgets.get(this.0) else {
            return Ok((0.0_f32, 0.0_f32, None::<f32>, None::<f32>));
        };
        let (min_w, min_h) = frame.resize_bounds_min;
        let (max_w, max_h) = frame
            .resize_bounds_max
            .map(|(w, h)| (Some(w), Some(h)))
            .unwrap_or((None, None));
        Ok((min_w, min_h, max_w, max_h))
    });
    add_resize_bounds_legacy_methods(methods);
}

/// Parse SetResizeBounds varargs: (minW, minH [, maxW, maxH]).
fn parse_resize_bounds(args: mlua::MultiValue) -> ((f32, f32), Option<(f32, f32)>) {
    let mut values = args.into_iter();
    let min_width = next_required_resize_bound(&mut values);
    let min_height = next_required_resize_bound(&mut values);
    let max = match (
        next_optional_resize_bound(&mut values),
        next_optional_resize_bound(&mut values),
    ) {
        (Some(w), Some(h)) => Some((w, h)),
        _ => None,
    };
    ((min_width, min_height), max)
}

/// Deprecated SetMinResize/SetMaxResize (superseded by SetResizeBounds).
fn add_resize_bounds_legacy_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
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
}

fn add_start_sizing_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("StartSizing", |lua, this, point: Option<String>| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "StartSizing") {
                return Ok(());
            }
        }
        let sizing_point = point
            .as_deref()
            .and_then(crate::widget::AnchorPoint::from_str)
            .unwrap_or(crate::widget::AnchorPoint::BottomRight);
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(id)
            && frame.resizable
        {
            frame.is_sizing = true;
            frame.sizing_point = sizing_point;
        }
        Ok(())
    });
}

fn add_drag_registration_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("RegisterForDrag", |lua, this, args: mlua::MultiValue| {
        let buttons = args
            .into_iter()
            .filter_map(|value| match value {
                mlua::Value::String(button) => button.to_str().ok().map(|s| s.to_string()),
                _ => None,
            })
            .collect();

        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.registered_drag_buttons = buttons;
        }
        Ok(())
    });
}

fn add_placement_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
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
    add_misc_mixin_field_stubs(methods);
    add_misc_mixin_only_stubs(methods);
    add_misc_state_setters(methods);
    add_misc_vararg_stubs(methods);
}

/// Methods that delegate to a mixin override or fall back to setting a Lua field.
fn add_misc_mixin_field_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetupMenu", |lua, this, generator: Value| {
        if let Some((func, self_value)) = get_mixin_override(lua, this.0, "SetupMenu") {
            return func.call::<()>((self_value, generator));
        }
        widget_fields(lua, this.0)?.set("menuGenerator", generator)?;
        Ok(())
    });
    methods.add_method("SetSelectionTranslator", |lua, this, translator: Value| {
        if let Some((func, self_value)) = get_mixin_override(lua, this.0, "SetSelectionTranslator")
        {
            return func.call::<()>((self_value, translator));
        }
        widget_fields(lua, this.0)?.set("selectionTranslator", translator)?;
        Ok(())
    });
    methods.add_method("SetItemButtonScale", |lua, this, scale: Value| {
        if let Some((func, self_value)) = get_mixin_override(lua, this.0, "SetItemButtonScale") {
            return func.call::<()>((self_value, scale));
        }
        widget_fields(lua, this.0)?.set("itemButtonScale", scale)?;
        Ok(())
    });
    methods.add_method("SetDefaultText", |lua, this, text: Value| {
        if let Some((func, self_value)) = get_mixin_override(lua, this.0, "SetDefaultText") {
            return func.call::<()>((self_value, text));
        }
        widget_fields(lua, this.0)?.set("defaultText", text)?;
        Ok(())
    });
}

/// Mixin-only delegates with no field fallback.
fn add_misc_mixin_only_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("UpdateItemContextMatching", |lua, this, ()| {
        if let Some((func, self_value)) =
            get_mixin_override(lua, this.0, "UpdateItemContextMatching")
        {
            return func.call::<()>(self_value);
        }
        Ok(())
    });
    methods.add_method("UpdateHeight", |lua, this, ()| {
        if let Some((func, self_value)) = get_mixin_override(lua, this.0, "UpdateHeight") {
            return func.call::<()>(self_value);
        }
        Ok(())
    });
}

/// Simple field and state setters without mixin delegation.
fn add_misc_state_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAlertContainer", |lua, this, container: Value| {
        widget_fields(lua, this.0)?.set("alertContainer", container)?;
        Ok(())
    });
    methods.add_method("SetColorFill", |lua, this, args: mlua::MultiValue| {
        let color = parse_statusbar_color(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(bar_id) = statusbar_child_id(&state.widgets, this.0)
            && let Some(bar) = state.widgets.get_mut_visual(bar_id)
        {
            bar.vertex_color = Some(color);
        }
        Ok(())
    });
    methods.add_method("SetTextToFit", |lua, this, text: Option<String>| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.text = text;
        }
        Ok(())
    });
}

/// Vararg methods that delegate to mixin overrides or store args.
fn add_misc_vararg_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetVisuals", |lua, this, args: mlua::MultiValue| {
        if let Some((func, self_value)) = get_mixin_override(lua, this.0, "SetVisuals") {
            return call_mixin_with_varargs(func, self_value, args);
        }
        let stored_args = lua.create_table()?;
        for (index, value) in args.into_iter().enumerate() {
            stored_args.set(index + 1, value)?;
        }
        widget_fields(lua, this.0)?.set("visualArgs", stored_args)?;
        Ok(())
    });
    methods.add_method(
        "RegisterForWidgetSet",
        |lua, this, args: mlua::MultiValue| {
            if let Some((func, self_value)) =
                get_mixin_override(lua, this.0, "RegisterForWidgetSet")
            {
                return call_mixin_with_varargs(func, self_value, args);
            }
            set_widget_set_registration(lua, this.0, args)
        },
    );
    methods.add_method(
        "UnregisterForWidgetSet",
        |lua, this, args: mlua::MultiValue| {
            if let Some((func, self_value)) =
                get_mixin_override(lua, this.0, "UnregisterForWidgetSet")
            {
                return call_mixin_with_varargs(func, self_value, args);
            }
            clear_widget_set_registration(lua, this.0)
        },
    );
}

/// Forward a vararg call to a mixin override, prepending `self`.
fn call_mixin_with_varargs(
    func: mlua::Function,
    self_value: Value,
    args: mlua::MultiValue,
) -> mlua::Result<()> {
    let mut call_args = mlua::MultiValue::new();
    call_args.push_back(self_value);
    for value in args {
        call_args.push_back(value);
    }
    func.call::<()>(call_args)
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

fn widget_fields(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<mlua::Table> {
    match frame_ref(lua, frame_id)? {
        Value::UserData(ud) => ud.user_value(),
        _ => lua.create_table(),
    }
}

fn set_widget_set_registration(
    lua: &mlua::Lua,
    frame_id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<()> {
    let mut values = args.into_iter();
    let widget_set_id = values.next().unwrap_or(Value::Nil);
    if widget_set_id.is_nil() {
        return clear_widget_set_registration(lua, frame_id);
    }

    let registration = lua.create_table()?;
    registration.set("widgetSetID", widget_set_id)?;
    registration.set("widgetLayoutFunction", values.next().unwrap_or(Value::Nil))?;
    registration.set("widgetInitFunction", values.next().unwrap_or(Value::Nil))?;
    registration.set("attachedUnitInfo", values.next().unwrap_or(Value::Nil))?;
    widget_fields(lua, frame_id)?.set("widgetSetRegistration", registration)?;
    Ok(())
}

fn clear_widget_set_registration(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<()> {
    widget_fields(lua, frame_id)?.set("widgetSetRegistration", Value::Nil)?;
    Ok(())
}

fn parse_statusbar_color(args: mlua::MultiValue) -> Color {
    let mut it = args.into_iter();
    let r = statusbar_color_arg(it.next(), 1.0);
    let g = statusbar_color_arg(it.next(), 1.0);
    let b = statusbar_color_arg(it.next(), 1.0);
    let a = statusbar_color_arg(it.next(), 1.0);
    Color::new(r, g, b, a)
}

fn statusbar_color_arg(value: Option<Value>, default: f32) -> f32 {
    match value {
        Some(Value::Number(value)) => value as f32,
        Some(Value::Integer(value)) => value as f32,
        _ => default,
    }
}

fn statusbar_child_id(widgets: &crate::widget::WidgetRegistry, id: u64) -> Option<u64> {
    let bar_id = widgets.get(id).and_then(|frame| frame.statusbar_bar_id)?;
    widgets
        .get(bar_id)
        .filter(|frame| frame.parent_id == Some(id))
        .map(|_| bar_id)
}
