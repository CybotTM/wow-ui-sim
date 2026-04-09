//! Miscellaneous widget methods: drag/move/resize, SimpleHTML, and stubs.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::Value;

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
    add_simplehtml_bool_setter(methods, "SetHyperlinksEnabled", |data, value| {
        data.hyperlinks_enabled = value
    });
    add_simplehtml_bool_getter(methods, "GetHyperlinksEnabled", true, |data| {
        data.hyperlinks_enabled
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

fn add_simplehtml_bool_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::lua_api::simple_html::SimpleHtmlData, bool) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: bool| {
        update_simplehtml_data(lua, this.0, |data| setter(data, value));
        Ok(())
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
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = match state.widgets.get(this.0) {
            Some(f) => f,
            None => return Ok(0.0_f64),
        };
        let text = match &frame.text {
            Some(t) if !t.is_empty() => t,
            _ => return Ok(0.0_f64),
        };
        let font_size = frame.font_size.max(12.0) as f64;
        let line_height = font_size * 1.2;
        let width = frame.width.max(200.0) as f64;
        let chars_per_line = (width / (font_size * 0.6)).max(1.0);
        let estimated_lines = (text.len() as f64 / chars_per_line).ceil().max(1.0);
        Ok(estimated_lines * line_height)
    });

    methods.add_method("GetTextData", |lua, _this, ()| {
        Ok(Value::Table(lua.create_table()?))
    });
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
    methods.add_method("SetClampRectInsets", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
}

fn add_drag_resize_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetResizeBounds",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("GetResizeBounds", |_, _this, ()| {
        Ok((0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32))
    });
    methods.add_method("SetMinResize", |_, _this, (_w, _h): (f32, f32)| Ok(()));
    methods.add_method("SetMaxResize", |_, _this, (_w, _h): (f32, f32)| Ok(()));
    methods.add_method("StartSizing", |_, _this, _point: Option<String>| Ok(()));
    methods.add_method(
        "RegisterForDrag",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("SetUserPlaced", |_, _this, _user_placed: bool| Ok(()));
    methods.add_method("IsUserPlaced", |_, _this, ()| Ok(false));
    methods.add_method("SetDontSavePosition", |_, _this, _dont_save: bool| Ok(()));
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
