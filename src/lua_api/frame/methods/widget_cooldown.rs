//! Cooldown widget methods: SetCooldown, swipe/edge/bling display, pause/resume.

use super::super::handle::FrameRef;
use super::widget_tooltip::val_to_f32;
use crate::lua_api::frame::handle::get_sim_state;
use crate::widget::AttributeValue;
use mlua::Value;

pub fn add_cooldown_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_cooldown_set_methods(methods);
    add_cooldown_get_methods(methods);
    add_cooldown_display_methods(methods);
    add_cooldown_bool_display_methods(methods);
    add_cooldown_texture_methods(methods);
    add_cooldown_state_methods(methods);
    add_cooldown_stubs(methods);
}

fn add_cooldown_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCooldownDisplayDuration", |_, _, ()| Ok(0.0_f64));
    methods.add_method("GetCountdownFontString", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetDrawBling", |_, _, ()| Ok(true));
    methods.add_method("GetDrawEdge", |_, _, ()| Ok(false));
    methods.add_method("GetDrawSwipe", |_, _, ()| Ok(true));
    methods.add_method("GetEdgeScale", |_, _, ()| Ok(1.0_f64));
    methods.add_method("GetHideCountdownNumbers", |_, _, ()| Ok(false));
    methods.add_method("GetMinimumCountdownDuration", |_, _, ()| Ok(0.0_f64));
    methods.add_method("GetUseAuraDisplayTime", |_, _, ()| Ok(false));
    methods.add_method("SetCooldownFromDurationObject", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetCooldownFromExpirationTime", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetEdgeColor", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetMinimumCountdownDuration", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetTexCoordRange", |_, _, _: mlua::Variadic<Value>| Ok(()));
}

fn parse_f64_arg(val: Option<Value>) -> f64 {
    match val {
        Some(Value::Number(n)) => n,
        Some(Value::Integer(n)) => n as f64,
        _ => 0.0,
    }
}

fn add_cooldown_set_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCooldown", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let start = parse_f64_arg(it.next());
        let duration = parse_f64_arg(it.next());
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_start = start;
            frame.cooldown_duration = duration;
        }
        Ok(())
    });

    methods.add_method("SetCooldownUNIX", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let start = parse_f64_arg(it.next());
        let end = parse_f64_arg(it.next());
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_start = start;
            frame.cooldown_duration = end - start;
        }
        Ok(())
    });
}

fn add_cooldown_get_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCooldownTimes", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok((frame.cooldown_start, frame.cooldown_duration));
        }
        Ok((0.0_f64, 0.0_f64))
    });

    methods.add_method("GetCooldownDuration", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.cooldown_duration).unwrap_or(0.0))
    });
}

fn add_cooldown_display_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSwipeColor", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let r = val_to_f32(it.next(), 0.0);
        let g = val_to_f32(it.next(), 0.0);
        let b = val_to_f32(it.next(), 0.0);
        let a = val_to_f32(it.next(), 0.8);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.attributes.insert(
                "__swipe_color".to_string(),
                AttributeValue::String(format!("{},{},{},{}", r, g, b, a)),
            );
        }
        Ok(())
    });

    methods.add_method("SetHideCountdownNumbers", |lua, this, hide: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_hide_countdown = hide; }
        Ok(())
    });
}

fn add_cooldown_bool_display_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDrawSwipe", |lua, this, draw: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_draw_swipe = draw; }
        Ok(())
    });
    methods.add_method("SetDrawEdge", |lua, this, draw: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_draw_edge = draw; }
        Ok(())
    });
    methods.add_method("SetDrawBling", |lua, this, draw: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_draw_bling = draw; }
        Ok(())
    });
    methods.add_method("SetReverse", |lua, this, reverse: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_reverse = reverse; }
        Ok(())
    });
}

fn add_cooldown_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetEdgeTexture", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetSwipeTexture", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetBlingTexture", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetEdgeScale", |_, _this, _scale: Value| Ok(()));
    methods.add_method("SetUseCircularEdge", |_, _this, _use_circular: bool| Ok(()));
    methods.add_method("SetCountdownAbbrevThreshold", |_, _this, _seconds: Value| Ok(()));
    methods.add_method("SetCountdownFont", |_, _this, _font: Value| Ok(()));
    methods.add_method("SetUseAuraDisplayTime", |_, _this, _use: Value| Ok(()));

    methods.add_method("GetReverse", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.cooldown_reverse).unwrap_or(false))
    });

    methods.add_method("SetCooldownDuration", |lua, this, args: mlua::MultiValue| {
        let duration = match args.into_iter().next() {
            Some(Value::Number(n)) => n,
            Some(Value::Integer(n)) => n as f64,
            _ => 0.0,
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_duration = duration; }
        Ok(())
    });
}

fn add_cooldown_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Clear", |_, _this, _: mlua::MultiValue| Ok(()));

    methods.add_method("Pause", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_paused = true; }
        Ok(())
    });

    methods.add_method("Resume", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.cooldown_paused = false; }
        Ok(())
    });

    methods.add_method("IsPaused", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.cooldown_paused).unwrap_or(false))
    });
}
