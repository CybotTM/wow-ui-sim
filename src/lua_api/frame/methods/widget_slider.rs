//! Slider and CheckButton methods plus shared SetValue/GetValue/SetMinMaxValues.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use crate::widget::{AttributeValue, WidgetType};
use mlua::Value;

pub fn add_slider_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_slider_step_methods(methods);
    add_slider_orientation_methods(methods);
    add_slider_thumb_methods(methods);
    add_slider_drag_methods(methods);
    methods.add_method("GetObeyStepOnDrag", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.slider_obey_step_on_drag)
            .unwrap_or(false))
    });
    methods.add_method("IsDraggingThumb", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.active_slider_thumb_drag_frame == Some(this.0))
    });
}

pub fn add_checkbutton_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_checked_method(methods);
    methods.add_method("GetChecked", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0)
            && let Some(AttributeValue::Boolean(checked)) = frame.attributes.get("__checked")
        {
            return Ok(*checked);
        }
        Ok(false)
    });
    methods.add_method("GetCheckedTexture", |lua, this, ()| {
        get_or_create_child_texture(lua, this.0, "CheckedTexture")
    });
}

fn add_set_checked_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetChecked", |lua, this, checked: bool| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let already = state
            .widgets
            .get(id)
            .and_then(|f| f.attributes.get("__checked"))
            .map(|v| matches!(v, AttributeValue::Boolean(b) if *b == checked))
            .unwrap_or(false);
        if already {
            return Ok(());
        }
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame
                .attributes
                .insert("__checked".to_string(), AttributeValue::Boolean(checked));
        }
        let checked_tex_id = state
            .widgets
            .get(id)
            .and_then(|f| f.children_keys.get("CheckedTexture").copied());
        if let Some(tex_id) = checked_tex_id {
            state.set_frame_visible(tex_id, checked);
        }
        Ok(())
    });
}

/// Shared SetValue/GetValue/SetMinMaxValues/GetMinMaxValues.
pub fn add_shared_value_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_shared_set_value(methods);
    add_shared_get_value(methods);
    add_shared_set_min_max(methods);
    add_shared_get_min_max(methods);
}

// --- Slider methods ---

fn add_slider_step_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetValueStep", |lua, this, step: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.slider_step = step;
        }
        Ok(())
    });
    methods.add_method("GetValueStep", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.slider_step)
            .unwrap_or(1.0))
    });
}

fn add_slider_orientation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetOrientation", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        if let Some((func, frame_ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "SetOrientation")
        {
            let mut call_args = vec![frame_ud];
            call_args.extend(args);
            return func
                .call::<Value>(mlua::MultiValue::from_iter(call_args))
                .map(|_| ());
        }
        if let Some(Value::String(s)) = args.into_iter().next() {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(id) {
                frame.slider_orientation = s
                    .to_str()
                    .map(|s| s.to_uppercase())
                    .unwrap_or_else(|_| "HORIZONTAL".to_string());
            }
        }
        Ok(())
    });
    methods.add_method("GetOrientation", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.slider_orientation.clone())
            .unwrap_or_else(|| "HORIZONTAL".to_string()))
    });
}

fn add_slider_thumb_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_thumb_texture_method(methods);
    methods.add_method("GetThumbTexture", |lua, this, ()| {
        let store: mlua::Table = lua.load("return _G.__slider_thumbs or {}").eval()?;
        let thumb: Value = store.get(this.0)?;
        Ok(thumb)
    });
}

fn add_set_thumb_texture_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetThumbTexture", |lua, this, arg: Value| {
        let id = this.0;
        let store: mlua::Table = lua
            .load("_G.__slider_thumbs = _G.__slider_thumbs or {}; return _G.__slider_thumbs")
            .eval()?;
        match arg {
            Value::UserData(_) => {
                store.set(id, arg)?;
            }
            Value::Integer(_) | Value::Number(_) | Value::String(_) => {
                let existing: Value = store.get(id)?;
                let thumb_ud = if let Value::UserData(_) = existing {
                    existing
                } else {
                    let new_thumb = get_or_create_child_texture(lua, id, "ThumbTexture")?;
                    store.set(id, new_thumb.clone())?;
                    new_thumb
                };
                call_set_texture(lua, &thumb_ud, &arg)?;
            }
            _ => {}
        }
        Ok(())
    });
}

fn call_set_texture(lua: &mlua::Lua, texture_ud: &Value, arg: &Value) -> mlua::Result<()> {
    lua.load("local t, v = ...; t:SetTexture(v)")
        .call::<()>((texture_ud.clone(), arg.clone()))?;
    Ok(())
}

fn add_slider_drag_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetObeyStepOnDrag", |lua, this, obey: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.slider_obey_step_on_drag = obey;
        }
        Ok(())
    });
    methods.add_method("SetStepsPerPage", |lua, this, steps: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.slider_steps_per_page = steps;
        }
        Ok(())
    });
    methods.add_method("GetStepsPerPage", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.slider_steps_per_page)
            .unwrap_or(1))
    });
}

// --- Shared value methods ---

fn add_shared_set_value<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetValue", |lua, this, value: f64| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let wtype = {
            let s = state_rc.borrow();
            s.widgets.get(id).map(|f| f.widget_type)
        };
        match wtype {
            Some(WidgetType::Slider) => set_slider_value(lua, id, value)?,
            Some(WidgetType::StatusBar) => set_statusbar_value(lua, id, value)?,
            _ => {}
        }
        Ok(())
    });
}

fn add_shared_get_value<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetValue", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok(match frame.widget_type {
                WidgetType::Slider => frame.slider_value,
                WidgetType::StatusBar => frame.statusbar_value,
                _ => 0.0,
            });
        }
        Ok(0.0_f64)
    });
}

fn add_shared_set_min_max<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetMinMaxValues", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let (min, max) = parse_min_max_args(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if !min_max_changed(&state, id, min, max) {
            return Ok(());
        }
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            apply_min_max(frame, min, max);
        }
        Ok(())
    });
}

fn add_shared_get_min_max<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetMinMaxValues", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok(match frame.widget_type {
                WidgetType::Slider => (frame.slider_min, frame.slider_max),
                WidgetType::StatusBar => (frame.statusbar_min, frame.statusbar_max),
                _ => (0.0, 1.0),
            });
        }
        Ok((0.0_f64, 1.0_f64))
    });
}

// --- Helper functions ---

fn set_slider_value(lua: &mlua::Lua, id: u64, value: f64) -> mlua::Result<()> {
    let clamped = {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let Some(frame) = state.widgets.get(id) else {
            return Ok(());
        };
        let clamped = value.clamp(frame.slider_min, frame.slider_max);
        if clamped == frame.slider_value {
            return Ok(());
        }
        state.widgets.get_mut_visual(id).unwrap().slider_value = clamped;
        clamped
    };
    fire_value_changed(lua, id, clamped)
}

fn set_statusbar_value(lua: &mlua::Lua, id: u64, value: f64) -> mlua::Result<()> {
    let clamped = {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let Some(frame) = state.widgets.get(id) else {
            return Ok(());
        };
        let clamped = value.clamp(frame.statusbar_min, frame.statusbar_max);
        if clamped == frame.statusbar_value {
            return Ok(());
        }
        state.widgets.get_mut_visual(id).unwrap().statusbar_value = clamped;
        clamped
    };
    fire_value_changed(lua, id, clamped)
}

fn fire_value_changed(lua: &mlua::Lua, frame_id: u64, value: f64) -> mlua::Result<()> {
    if let Some(func) = crate::lua_api::script_helpers::get_script(lua, frame_id, "OnValueChanged")
        && let Some(frame_ud) = crate::lua_api::script_helpers::get_frame_ref(lua, frame_id)
        && let Err(e) = func.call::<()>((frame_ud, value))
    {
        crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
    }
    Ok(())
}

fn parse_min_max_args(args: mlua::MultiValue) -> (f64, f64) {
    let mut it = args.into_iter();
    let min = match it.next() {
        Some(Value::Number(n)) => n,
        Some(Value::Integer(n)) => n as f64,
        _ => 0.0,
    };
    let max = match it.next() {
        Some(Value::Number(n)) => n,
        Some(Value::Integer(n)) => n as f64,
        _ => 1.0,
    };
    (min, max)
}

fn min_max_changed(state: &crate::lua_api::SimState, id: u64, min: f64, max: f64) -> bool {
    state
        .widgets
        .get(id)
        .map(|frame| match frame.widget_type {
            WidgetType::Slider => frame.slider_min != min || frame.slider_max != max,
            WidgetType::StatusBar => frame.statusbar_min != min || frame.statusbar_max != max,
            _ => false,
        })
        .unwrap_or(false)
}

fn apply_min_max(frame: &mut crate::widget::Frame, min: f64, max: f64) {
    match frame.widget_type {
        WidgetType::Slider => {
            frame.slider_min = min;
            frame.slider_max = max;
            frame.slider_value = frame.slider_value.clamp(min, max);
        }
        WidgetType::StatusBar => {
            frame.statusbar_min = min;
            frame.statusbar_max = max;
            frame.statusbar_value = frame.statusbar_value.clamp(min, max);
        }
        _ => {}
    }
}

/// Look up or create a child texture by key and return it as a UserData Value.
pub(super) fn get_or_create_child_texture(
    lua: &mlua::Lua,
    id: u64,
    key: &str,
) -> mlua::Result<Value> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let tex_id = super::methods_helpers::get_or_create_button_texture(lua, &mut state, id, key);
    drop(state);
    frame_ref(lua, tex_id)
}
