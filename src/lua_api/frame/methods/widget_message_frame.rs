//! MessageFrame widget methods: AddMessage, scrolling, fading, message history.

use super::super::handle::FrameRef;
use crate::lua_api::SimState;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::Value;

pub fn add_message_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_message_frame_add_methods(methods);
    add_message_frame_count_methods(methods);
    add_message_frame_fade_methods(methods);
    add_message_frame_fade_duration_methods(methods);
    add_message_frame_insert_methods(methods);
    add_message_frame_scroll_methods(methods);
    add_message_frame_misc_methods(methods);
    add_message_frame_callback_stubs(methods);
}

fn add_message_frame_add_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddMessage", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        add_message_core(&mut state_rc.borrow_mut(), this.0, args, true);
        Ok(())
    });
    methods.add_method("AddMsg", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        add_message_core(&mut state_rc.borrow_mut(), this.0, args, true);
        Ok(())
    });
    methods.add_method("_AddMessageSilent", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        add_message_core(&mut state_rc.borrow_mut(), this.0, args, false);
        Ok(())
    });
    methods.add_method("BackFillMessage", |lua, this, args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        backfill_message(&mut state_rc.borrow_mut(), this.0, args);
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
        truncate_messages(data);
    }
    data.scroll_offset = data
        .scroll_offset
        .clamp(0, message_frame_scroll_limit(data));
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

fn add_message_frame_scroll_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
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

fn message_frame_scroll_limit(data: &crate::lua_api::message_frame::MessageFrameData) -> i32 {
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

fn add_message_frame_callback_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetIndentedWordWrap", |_, _this, ()| Ok(false));
    methods.add_method("SetOnScrollChangedCallback", |lua, this, func: Value| {
        set_message_frame_callback(lua, this.0, "onScrollChangedCallback", func)
    });
    methods.add_method("SetOnLineRightClickedCallback", |lua, this, func: Value| {
        set_message_frame_callback(lua, this.0, "onLineRightClickedCallback", func)
    });
    methods.add_method("AddOnDisplayRefreshedCallback", |lua, this, func: Value| {
        add_message_frame_display_refreshed_callback(lua, this.0, func)
    });
    methods.add_method("RemoveMessagesByPredicate", |_, _this, _func: Value| Ok(()));
    methods.add_method("TransformMessages", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
    methods.add_method("AdjustMessageColors", |_, _this, _func: Value| Ok(()));
    methods.add_method("GetFontStringByID", |_, _this, _id: i64| Ok(Value::Nil));
    methods.add_method("ResetMessageFadeByID", |_, _this, _id: i64| Ok(()));
    methods.add_method("ResetAllFadeTimes", |lua, this, ()| {
        call_message_frame_display_refreshed_callbacks(lua, this.0)
    });
    methods.add_method("MarkDisplayDirty", |lua, this, ()| {
        call_message_frame_display_refreshed_callbacks(lua, this.0)
    });
    add_set_on_text_copied_callback(methods);
}

fn set_message_frame_callback(
    lua: &mlua::Lua,
    frame_id: u64,
    field_name: &str,
    func: Value,
) -> mlua::Result<()> {
    let frame_fields = crate::lua_api::script_helpers::get_or_create_frame_fields(lua, frame_id);
    match func {
        Value::Function(callback) => frame_fields.set(field_name, callback)?,
        _ => frame_fields.set(field_name, Value::Nil)?,
    }
    Ok(())
}

fn add_message_frame_display_refreshed_callback(
    lua: &mlua::Lua,
    frame_id: u64,
    func: Value,
) -> mlua::Result<()> {
    let Value::Function(callback) = func else {
        return Ok(());
    };
    let callbacks = get_or_create_message_frame_callback_list(lua, frame_id)?;
    callbacks.raw_set(callbacks.raw_len() + 1, callback)?;
    Ok(())
}

fn get_or_create_message_frame_callback_list(
    lua: &mlua::Lua,
    frame_id: u64,
) -> mlua::Result<mlua::Table> {
    let frame_fields = crate::lua_api::script_helpers::get_or_create_frame_fields(lua, frame_id);
    if let Ok(callbacks) = frame_fields.get::<mlua::Table>("onDisplayRefreshedCallbacks") {
        return Ok(callbacks);
    }
    let callbacks = lua.create_table()?;
    frame_fields.set("onDisplayRefreshedCallbacks", callbacks.clone())?;
    Ok(callbacks)
}

fn call_message_frame_scroll_changed_callback(
    lua: &mlua::Lua,
    frame_id: u64,
    offset: i32,
) -> mlua::Result<()> {
    call_message_frame_callback(
        lua,
        frame_id,
        "onScrollChangedCallback",
        vec![Value::Integer(offset as i64)],
    )
}

fn call_message_frame_display_refreshed_callbacks(
    lua: &mlua::Lua,
    frame_id: u64,
) -> mlua::Result<()> {
    let frame_fields = crate::lua_api::script_helpers::get_or_create_frame_fields(lua, frame_id);
    let Ok(callbacks) = frame_fields.get::<mlua::Table>("onDisplayRefreshedCallbacks") else {
        return Ok(());
    };
    let frame = crate::lua_api::frame::frame_ref(lua, frame_id)?;
    for index in 1..=callbacks.raw_len() {
        if let Ok(callback) = callbacks.raw_get::<mlua::Function>(index) {
            callback.call::<()>((frame.clone(),))?;
        }
    }
    Ok(())
}

fn call_message_frame_callback(
    lua: &mlua::Lua,
    frame_id: u64,
    field_name: &str,
    mut args: Vec<Value>,
) -> mlua::Result<()> {
    let frame_fields = crate::lua_api::script_helpers::get_or_create_frame_fields(lua, frame_id);
    let Ok(callback) = frame_fields.get::<mlua::Function>(field_name) else {
        return Ok(());
    };
    let frame = crate::lua_api::frame::frame_ref(lua, frame_id)?;
    let mut call_args = mlua::MultiValue::new();
    call_args.push_back(frame);
    for arg in args.drain(..) {
        call_args.push_back(arg);
    }
    callback.call::<()>(call_args)
}

fn add_set_on_text_copied_callback<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetOnTextCopiedCallback", |lua, this, func: Value| {
        let frame_id = this.0;
        let frame_fields =
            crate::lua_api::script_helpers::get_or_create_frame_fields(lua, frame_id);
        match func {
            Value::Function(callback) => {
                frame_fields.set("_onTextCopiedCallback_orig", callback)?;
                let wrapper = lua.create_function(move |lua, args: mlua::Variadic<Value>| {
                    let fields =
                        crate::lua_api::script_helpers::get_or_create_frame_fields(lua, frame_id);
                    if let Ok(orig) = fields.get::<mlua::Function>("_onTextCopiedCallback_orig") {
                        orig.call::<()>(args)?;
                    }
                    Ok(())
                })?;
                frame_fields.set("onTextCopiedCallback", wrapper)?;
            }
            _ => {
                frame_fields.set("onTextCopiedCallback", Value::Nil)?;
                frame_fields.set("_onTextCopiedCallback_orig", Value::Nil)?;
            }
        }
        Ok(())
    });
}

// --- Helper functions ---

fn log_message(state: &SimState, id: u64, text: &str) {
    let name = state
        .widgets
        .get(id)
        .and_then(|w| w.name.as_deref())
        .unwrap_or("?");
    let clean = crate::dump::strip_wow_escapes(text);
    eprintln!("[{name}] {clean}");
}

fn add_message_core(state: &mut SimState, id: u64, args: mlua::MultiValue, log: bool) {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let text = match args_vec.first() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        _ => return,
    };
    let (r, g, b, a) = extract_rgba(&args_vec, 1);
    let message_id = match args_vec.get(5) {
        Some(Value::Integer(n)) => Some(*n),
        Some(Value::Number(n)) => Some(*n as i64),
        _ => None,
    };
    if log {
        log_message(state, id, &text);
    }
    let timestamp = state.start_time.elapsed().as_secs_f64();
    let data = state.message_frames.entry(id).or_default();
    insert_message(data, text, r, g, b, a, message_id, timestamp);
    truncate_messages(data);
}

fn backfill_message(state: &mut SimState, id: u64, args: mlua::MultiValue) {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let text = match args_vec.first() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        _ => return,
    };
    let (r, g, b, a) = extract_rgba(&args_vec, 1);
    log_message(state, id, &text);
    let timestamp = state.start_time.elapsed().as_secs_f64();
    let data = state
        .message_frames
        .entry(id)
        .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
    data.messages.insert(
        0,
        crate::lua_api::message_frame::Message {
            text,
            r,
            g,
            b,
            a,
            message_id: None,
            timestamp,
        },
    );
    if data.messages.len() > data.max_lines {
        data.messages.pop();
    }
}

fn insert_message(
    data: &mut crate::lua_api::message_frame::MessageFrameData,
    text: String,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    message_id: Option<i64>,
    timestamp: f64,
) {
    let msg = crate::lua_api::message_frame::Message {
        text,
        r,
        g,
        b,
        a,
        message_id,
        timestamp,
    };
    if data.insert_mode == "TOP" {
        data.messages.insert(0, msg);
    } else {
        data.messages.push(msg);
    }
}

fn truncate_messages(data: &mut crate::lua_api::message_frame::MessageFrameData) {
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

fn extract_rgba(args: &[Value], offset: usize) -> (f32, f32, f32, f32) {
    (
        val_to_f32_ref(args.get(offset), 1.0),
        val_to_f32_ref(args.get(offset + 1), 1.0),
        val_to_f32_ref(args.get(offset + 2), 1.0),
        val_to_f32_ref(args.get(offset + 3), 1.0),
    )
}

fn val_to_f32_ref(val: Option<&Value>, default: f32) -> f32 {
    match val {
        Some(Value::Number(n)) => *n as f32,
        Some(Value::Integer(n)) => *n as f32,
        _ => default,
    }
}
