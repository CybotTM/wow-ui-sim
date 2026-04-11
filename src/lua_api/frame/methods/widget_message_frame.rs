//! MessageFrame widget methods: AddMessage, scrolling, fading, message history.

use super::super::handle::FrameRef;
use crate::lua_api::SimState;
use crate::lua_api::frame::handle::get_sim_state;

pub fn add_message_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_message_frame_add_methods(methods);
    add_message_frame_count_methods(methods);
    add_message_frame_fade_methods(methods);
    add_message_frame_fade_duration_methods(methods);
    add_message_frame_insert_methods(methods);
    super::widget_message_frame_scroll::add_message_frame_scroll_methods(methods);
    add_message_frame_misc_methods(methods);
    super::widget_message_frame_callbacks::add_message_frame_callback_stubs(methods);
}

fn add_message_frame_add_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddMessage", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        super::widget_message_frame_callbacks::add_message_core(
            &mut state_rc.borrow_mut(),
            this.0,
            args,
            true,
        );
        Ok(())
    });
    methods.add_method("AddMsg", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        super::widget_message_frame_callbacks::add_message_core(
            &mut state_rc.borrow_mut(),
            this.0,
            args,
            true,
        );
        Ok(())
    });
    methods.add_method("_AddMessageSilent", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        super::widget_message_frame_callbacks::add_message_core(
            &mut state_rc.borrow_mut(),
            this.0,
            args,
            false,
        );
        Ok(())
    });
    methods.add_method("BackFillMessage", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        super::widget_message_frame_callbacks::backfill_message(
            &mut state_rc.borrow_mut(),
            this.0,
            args,
        );
        Ok(())
    });
    methods.add_method("Clear", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(data) = state.message_frames.get_mut(&this.0) {
            data.messages.clear();
            data.scroll_offset = 0;
        }
        Ok(())
    });
    methods.add_method("ClearText", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(data) = state.message_frames.get_mut(&this.0) {
            data.messages.clear();
            data.scroll_offset = 0;
        }
        Ok(())
    });
}

fn add_message_frame_count_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_num_messages(methods);
    add_set_max_lines(methods);
    add_get_max_lines(methods);
}

fn add_get_num_messages<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetNumMessages", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(message_frame_message_count(&state, this.0) as i32)
    });
}

fn add_set_max_lines<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetMaxLines", |lua, this, max_lines: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        set_message_frame_max_lines(&mut state, this.0, max_lines);
        Ok(())
    });
}

fn add_get_max_lines<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetMaxLines", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(message_frame_max_lines(&state, this.0) as i32)
    });
}

fn message_frame_message_count(state: &SimState, id: u64) -> usize {
    state
        .message_frames
        .get(&id)
        .map(|data| data.messages.len())
        .unwrap_or(0)
}

fn set_message_frame_max_lines(state: &mut SimState, id: u64, max_lines: i32) {
    let data = state
        .message_frames
        .entry(id)
        .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
    data.max_lines = max_lines.max(1) as usize;
    while data.messages.len() > data.max_lines {
        super::widget_message_frame_scroll::truncate_messages(data);
    }
    data.scroll_offset = data.scroll_offset.clamp(
        0,
        super::widget_message_frame_scroll::message_frame_scroll_limit(data),
    );
    data.display_dirty = true;
}

fn message_frame_max_lines(state: &SimState, id: u64) -> usize {
    state
        .message_frames
        .get(&id)
        .map(|data| data.max_lines)
        .unwrap_or(120)
}

fn add_message_frame_fade_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_message_frame_bool_setter(methods, "SetFading", |data, value| data.fading = value);
    add_message_frame_bool_getter(methods, "GetFading", true, |data| data.fading);
    add_message_frame_f64_setter(methods, "SetTimeVisible", |data, value| {
        data.time_visible = value
    });
    add_message_frame_f64_getter(methods, "GetTimeVisible", 10.0, |data| data.time_visible);
}

fn add_message_frame_bool_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::lua_api::message_frame::MessageFrameData, bool) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let data = state
            .message_frames
            .entry(this.0)
            .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
        setter(data, value);
        Ok(())
    });
}

fn add_message_frame_bool_getter<M, F>(
    methods: &mut M,
    name: &'static str,
    default: bool,
    getter: F,
) where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::lua_api::message_frame::MessageFrameData) -> bool + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .message_frames
            .get(&this.0)
            .map(getter)
            .unwrap_or(default))
    });
}

fn add_message_frame_f64_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::lua_api::message_frame::MessageFrameData, f64) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let data = state
            .message_frames
            .entry(this.0)
            .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
        setter(data, value);
        Ok(())
    });
}

fn add_message_frame_f64_getter<M, F>(methods: &mut M, name: &'static str, default: f64, getter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::lua_api::message_frame::MessageFrameData) -> f64 + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .message_frames
            .get(&this.0)
            .map(getter)
            .unwrap_or(default))
    });
}

fn add_message_frame_fade_duration_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_message_frame_f64_setter(methods, "SetFadeDuration", |data, value| {
        data.fade_duration = value
    });
    add_message_frame_f64_getter(methods, "GetFadeDuration", 3.0, |data| data.fade_duration);
    add_message_frame_f64_setter(methods, "SetFadePower", |data, value| {
        data.fade_power = value
    });
    add_message_frame_f64_getter(methods, "GetFadePower", 1.0, |data| data.fade_power);
}

fn add_message_frame_insert_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetInsertMode", |lua, this, mode: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state
            .message_frames
            .entry(this.0)
            .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default)
            .insert_mode = mode;
        Ok(())
    });
    methods.add_method("GetInsertMode", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let mode = state
            .message_frames
            .get(&this.0)
            .map(|d| d.insert_mode.clone())
            .unwrap_or_else(|| "BOTTOM".to_string());
        Ok(mode)
    });
}

fn add_message_frame_misc_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTextCopyable", |lua, this, copyable: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state
            .message_frames
            .entry(this.0)
            .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default)
            .text_copyable = copyable;
        Ok(())
    });
    methods.add_method("IsTextCopyable", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .message_frames
            .get(&this.0)
            .map(|d| d.text_copyable)
            .unwrap_or(false))
    });
    methods.add_method("HasMessageByID", |lua, this, msg_id: i64| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let has = state
            .message_frames
            .get(&this.0)
            .map(|d| d.messages.iter().any(|m| m.message_id == Some(msg_id)))
            .unwrap_or(false);
        Ok(has)
    });
    add_get_message_info_method(methods);
}

fn add_get_message_info_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetMessageInfo", |lua, this, index: i32| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(data) = state.message_frames.get(&this.0) {
            let idx = (index - 1) as usize;
            if let Some(msg) = data.messages.get(idx) {
                return Ok((
                    msg.text.clone(),
                    msg.r as f64,
                    msg.g as f64,
                    msg.b as f64,
                    msg.a as f64,
                    msg.timestamp,
                ));
            }
        }
        Ok((String::new(), 1.0_f64, 1.0_f64, 1.0_f64, 1.0_f64, 0.0_f64))
    });
}
