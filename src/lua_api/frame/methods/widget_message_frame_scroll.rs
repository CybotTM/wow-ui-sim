//! MessageFrame scroll methods and helpers.

use super::super::handle::FrameRef;
use crate::lua_api::SimState;
use crate::lua_api::frame::handle::get_sim_state;

pub(super) fn add_message_frame_scroll_methods<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
) {
    methods.add_method("ScrollUp", |lua, this, ()| {
        adjust_message_frame_scroll(get_sim_state(lua), this.0, 1);
        Ok(())
    });
    methods.add_method("ScrollDown", |lua, this, ()| {
        adjust_message_frame_scroll(get_sim_state(lua), this.0, -1);
        Ok(())
    });
    methods.add_method("PageUp", |lua, this, ()| {
        page_message_frame_scroll(get_sim_state(lua), this.0, true);
        Ok(())
    });
    methods.add_method("PageDown", |lua, this, ()| {
        page_message_frame_scroll(get_sim_state(lua), this.0, false);
        Ok(())
    });
    methods.add_method("ScrollToTop", |lua, this, ()| {
        scroll_message_frame_to_edge(get_sim_state(lua), this.0, true);
        Ok(())
    });
    methods.add_method("ScrollToBottom", |lua, this, ()| {
        scroll_message_frame_to_edge(get_sim_state(lua), this.0, false);
        Ok(())
    });
    methods.add_method("AtTop", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(message_frame_is_at_top(&state, this.0))
    });
    methods.add_method("AtBottom", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(message_frame_is_at_bottom(&state, this.0))
    });
    methods.add_method("GetMaxScrollRange", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(message_frame_max_scroll_range(&state, this.0))
    });
    add_scroll_offset_methods(methods);
    add_scroll_allowed_methods(methods);
}

fn add_scroll_offset_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScrollOffset", |lua, this, offset: i32| {
        let state_rc = get_sim_state(lua);
        let changed = {
            let mut state = state_rc.borrow_mut();
            let data = state
                .message_frames
                .entry(this.0)
                .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
            let changed = data.scroll_offset != offset;
            data.scroll_offset = offset;
            changed
        };
        if changed {
            call_message_frame_scroll_changed_callback(lua, this.0, offset)?;
        }
        Ok(())
    });
    methods.add_method("GetScrollOffset", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .message_frames
            .get(&this.0)
            .map(|d| d.scroll_offset)
            .unwrap_or(0))
    });
}

fn add_scroll_allowed_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScrollAllowed", |lua, this, allowed: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state
            .message_frames
            .entry(this.0)
            .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default)
            .scroll_allowed = allowed;
        Ok(())
    });
    methods.add_method("IsScrollAllowed", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .message_frames
            .get(&this.0)
            .map(|d| d.scroll_allowed)
            .unwrap_or(true))
    });
}

fn adjust_message_frame_scroll(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    delta: i32,
) {
    let mut state = state_rc.borrow_mut();
    let Some(data) = state.message_frames.get_mut(&id) else {
        return;
    };
    if !data.scroll_allowed {
        return;
    }
    let max_offset = message_frame_scroll_limit(data);
    data.scroll_offset = (data.scroll_offset + delta).clamp(0, max_offset);
}

fn page_message_frame_scroll(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    towards_top: bool,
) {
    let mut state = state_rc.borrow_mut();
    let Some(data) = state.message_frames.get_mut(&id) else {
        return;
    };
    if !data.scroll_allowed {
        return;
    }
    let page_amount = message_frame_page_amount(data);
    let delta = if towards_top {
        page_amount
    } else {
        -page_amount
    };
    let max_offset = message_frame_scroll_limit(data);
    data.scroll_offset = (data.scroll_offset + delta).clamp(0, max_offset);
}

fn scroll_message_frame_to_edge(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    top: bool,
) {
    let mut state = state_rc.borrow_mut();
    let Some(data) = state.message_frames.get_mut(&id) else {
        return;
    };
    if !data.scroll_allowed {
        return;
    }
    data.scroll_offset = if top {
        message_frame_scroll_limit(data)
    } else {
        0
    };
}

pub(super) fn message_frame_scroll_limit(
    data: &crate::lua_api::message_frame::MessageFrameData,
) -> i32 {
    data.messages.len().min(data.max_lines).saturating_sub(1) as i32
}

fn message_frame_page_amount(data: &crate::lua_api::message_frame::MessageFrameData) -> i32 {
    data.max_lines.max(1) as i32
}

fn message_frame_max_scroll_range(state: &SimState, id: u64) -> i32 {
    state
        .message_frames
        .get(&id)
        .map(message_frame_scroll_limit)
        .unwrap_or(0)
}

fn message_frame_is_at_top(state: &SimState, id: u64) -> bool {
    let Some(data) = state.message_frames.get(&id) else {
        return true;
    };
    data.scroll_offset == message_frame_scroll_limit(data)
}

fn message_frame_is_at_bottom(state: &SimState, id: u64) -> bool {
    state
        .message_frames
        .get(&id)
        .map(|data| data.scroll_offset == 0)
        .unwrap_or(true)
}

pub(super) fn truncate_messages(
    data: &mut crate::lua_api::message_frame::MessageFrameData,
) {
    while data.messages.len() > data.max_lines {
        if data.insert_mode == "TOP" {
            data.messages.pop();
        } else {
            data.messages.remove(0);
        }
    }
    data.scroll_offset = data
        .scroll_offset
        .clamp(0, message_frame_scroll_limit(data));
}

fn call_message_frame_scroll_changed_callback(
    lua: &mlua::Lua,
    frame_id: u64,
    offset: i32,
) -> mlua::Result<()> {
    super::widget_message_frame_callbacks::call_message_frame_callback(
        lua,
        frame_id,
        "onScrollChangedCallback",
        vec![mlua::Value::Integer(offset as i64)],
    )
}
