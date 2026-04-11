//! MessageFrame callback stubs, message manipulation, and message-core helpers.

use super::super::handle::FrameRef;
use super::methods_helpers::get_mixin_override;
use super::methods_text::get_frame_font_object;
use crate::lua_api::SimState;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use crate::widget::{Frame, WidgetType};
use mlua::Value;

pub(super) fn add_message_frame_callback_stubs<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
) {
    methods.add_method("GetIndentedWordWrap", |lua, this, ()| {
        if let Some((func, self_value)) = get_mixin_override(lua, this.0, "GetIndentedWordWrap") {
            return func.call(self_value);
        }
        if let Some(font_object) = get_frame_font_object(lua, this.0)?
            && let Ok(getter) = font_object.get::<mlua::Function>("GetIndentedWordWrap")
        {
            return getter.call(font_object);
        }
        Ok(false)
    });
    methods.add_method("SetOnScrollChangedCallback", |lua, this, func: Value| {
        set_message_frame_callback(lua, this.0, "onScrollChangedCallback", func)
    });
    methods.add_method("SetOnLineRightClickedCallback", |lua, this, func: Value| {
        set_message_frame_callback(lua, this.0, "onLineRightClickedCallback", func)
    });
    methods.add_method("AddOnDisplayRefreshedCallback", |lua, this, func: Value| {
        add_message_frame_display_refreshed_callback(lua, this.0, func)
    });
    methods.add_method(
        "RemoveMessagesByPredicate",
        |lua, this, predicate: Value| {
            let Some(predicate) = as_lua_function(predicate) else {
                return Ok(());
            };
            if remove_messages_by_predicate(lua, this.0, predicate)? {
                mark_message_frame_display_dirty(lua, this.0)?;
            }
            Ok(())
        },
    );
    methods.add_method("TransformMessages", |lua, this, args: mlua::MultiValue| {
        let Some((predicate, transform)) = parse_transform_message_callbacks(args) else {
            return Ok(());
        };
        if transform_messages(lua, this.0, predicate, transform)? {
            mark_message_frame_display_dirty(lua, this.0)?;
        }
        Ok(())
    });
    methods.add_method("AdjustMessageColors", |lua, this, transform: Value| {
        let Some(transform) = as_lua_function(transform) else {
            return Ok(());
        };
        if adjust_message_colors(lua, this.0, transform)? {
            mark_message_frame_display_dirty(lua, this.0)?;
        }
        Ok(())
    });
    methods.add_method("GetFontStringByID", |lua, this, message_id: i64| {
        get_font_string_by_message_id(lua, this.0, message_id)
    });
    methods.add_method("ResetMessageFadeByID", |lua, this, message_id: i64| {
        if reset_message_fade_by_id(lua, this.0, message_id) {
            mark_message_frame_display_dirty(lua, this.0)?;
        }
        Ok(())
    });
    methods.add_method("ResetAllFadeTimes", |lua, this, ()| {
        reset_all_message_fade_times(lua, this.0);
        mark_message_frame_display_dirty(lua, this.0)
    });
    methods.add_method("MarkDisplayDirty", |lua, this, ()| {
        mark_message_frame_display_dirty(lua, this.0)
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

pub(super) fn call_message_frame_callback(
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

pub(super) fn mark_message_frame_display_dirty(
    lua: &mlua::Lua,
    frame_id: u64,
) -> mlua::Result<()> {
    {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let data = state
            .message_frames
            .entry(frame_id)
            .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
        data.display_dirty = true;
    }
    call_message_frame_display_refreshed_callbacks(lua, frame_id)
}

fn reset_all_message_fade_times(lua: &mlua::Lua, frame_id: u64) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let now = state.start_time.elapsed().as_secs_f64();
    let data = state
        .message_frames
        .entry(frame_id)
        .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
    data.override_fade_timestamp = now;
}

fn reset_message_fade_by_id(lua: &mlua::Lua, frame_id: u64, message_id: i64) -> bool {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let now = state.start_time.elapsed().as_secs_f64();
    let Some(data) = state.message_frames.get_mut(&frame_id) else {
        return false;
    };
    let Some(message) = data
        .messages
        .iter_mut()
        .rev()
        .find(|message| message.message_id == Some(message_id))
    else {
        return false;
    };
    message.timestamp = now;
    true
}

fn get_font_string_by_message_id(
    lua: &mlua::Lua,
    frame_id: u64,
    message_id: i64,
) -> mlua::Result<Value> {
    let Some((font_string_id, message)) = resolve_message_font_string(lua, frame_id, message_id)
    else {
        return Ok(Value::Nil);
    };
    update_message_font_string(lua, font_string_id, &message);
    frame_ref(lua, font_string_id)
}

fn resolve_message_font_string(
    lua: &mlua::Lua,
    frame_id: u64,
    message_id: i64,
) -> Option<(u64, crate::lua_api::message_frame::Message)> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let (message, existing_font_string_id) = {
        let data = state.message_frames.get_mut(&frame_id)?;
        let message = data
            .messages
            .iter()
            .rev()
            .find(|message| message.message_id == Some(message_id))
            .cloned()?;
        let existing_font_string_id = data.message_font_strings.get(&message_id).copied();
        (message, existing_font_string_id)
    };
    let font_string_id = existing_font_string_id
        .unwrap_or_else(|| create_message_font_string(&mut state, frame_id, message_id));
    state
        .message_frames
        .get_mut(&frame_id)?
        .message_font_strings
        .insert(message_id, font_string_id);
    Some((font_string_id, message))
}

fn create_message_font_string(state: &mut SimState, parent_id: u64, message_id: i64) -> u64 {
    let mut font_string = Frame::new(WidgetType::FontString, None, Some(parent_id));
    font_string.visible = false;
    font_string.object_type_name = Some("FontString".to_string());
    font_string.parent_key = Some(format!("MessageID{message_id}"));
    let font_string_id = font_string.id;
    state.widgets.register(font_string);
    state.widgets.add_child(parent_id, font_string_id);
    let parent_props = state
        .widgets
        .get(parent_id)
        .map(|parent| (parent.frame_strata, parent.frame_level));
    if let Some((parent_strata, parent_level)) = parent_props
        && let Some(frame) = state.widgets.get_mut_visual(font_string_id)
    {
        frame.frame_strata = parent_strata;
        frame.frame_level = parent_level + 1;
    }
    font_string_id
}

fn update_message_font_string(
    lua: &mlua::Lua,
    font_string_id: u64,
    message: &crate::lua_api::message_frame::Message,
) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let Some(font_string) = state.widgets.get_mut_visual(font_string_id) else {
        return;
    };
    font_string.text = Some(message.text.clone());
    font_string.text_stripped = Some(crate::dump::strip_wow_escapes(&message.text));
    font_string.text_color = crate::widget::Color::new(message.r, message.g, message.b, message.a);
}

fn as_lua_function(value: Value) -> Option<mlua::Function> {
    match value {
        Value::Function(function) => Some(function),
        _ => None,
    }
}

fn parse_transform_message_callbacks(
    args: mlua::MultiValue,
) -> Option<(mlua::Function, mlua::Function)> {
    let mut values = args.into_iter();
    let predicate = as_lua_function(values.next()?)?;
    let transform = as_lua_function(values.next()?)?;
    Some((predicate, transform))
}

fn remove_messages_by_predicate(
    lua: &mlua::Lua,
    frame_id: u64,
    predicate: mlua::Function,
) -> mlua::Result<bool> {
    let snapshot = get_message_frame_snapshot(lua, frame_id);
    if snapshot.is_empty() {
        return Ok(false);
    }
    let mut kept_messages = Vec::with_capacity(snapshot.len());
    let mut removed_any = false;
    for message in snapshot {
        if call_message_predicate(lua, &predicate, &message)? {
            removed_any = true;
        } else {
            kept_messages.push(message);
        }
    }
    if removed_any {
        replace_message_frame_messages(lua, frame_id, kept_messages);
    }
    Ok(removed_any)
}

fn transform_messages(
    lua: &mlua::Lua,
    frame_id: u64,
    predicate: mlua::Function,
    transform: mlua::Function,
) -> mlua::Result<bool> {
    let snapshot = get_message_frame_snapshot(lua, frame_id);
    if snapshot.is_empty() {
        return Ok(false);
    }
    let mut transformed_messages = Vec::with_capacity(snapshot.len());
    let mut changed_any = false;
    for message in snapshot {
        if call_message_predicate(lua, &predicate, &message)? {
            transformed_messages.push(call_message_transform(lua, &transform, &message)?);
            changed_any = true;
        } else {
            transformed_messages.push(message);
        }
    }
    if changed_any {
        replace_message_frame_messages(lua, frame_id, transformed_messages);
    }
    Ok(changed_any)
}

fn adjust_message_colors(
    lua: &mlua::Lua,
    frame_id: u64,
    transform: mlua::Function,
) -> mlua::Result<bool> {
    let snapshot = get_message_frame_snapshot(lua, frame_id);
    if snapshot.is_empty() {
        return Ok(false);
    }
    let mut recolored_messages = Vec::with_capacity(snapshot.len());
    let mut changed_any = false;
    for message in snapshot {
        let Some((r, g, b)) = call_color_transform(lua, &transform, &message)? else {
            recolored_messages.push(message);
            continue;
        };
        let mut updated = message;
        updated.r = r;
        updated.g = g;
        updated.b = b;
        recolored_messages.push(updated);
        changed_any = true;
    }
    if changed_any {
        replace_message_frame_messages(lua, frame_id, recolored_messages);
    }
    Ok(changed_any)
}

fn get_message_frame_snapshot(
    lua: &mlua::Lua,
    frame_id: u64,
) -> Vec<crate::lua_api::message_frame::Message> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .message_frames
        .get(&frame_id)
        .map(|data| data.messages.clone())
        .unwrap_or_default()
}

fn replace_message_frame_messages(
    lua: &mlua::Lua,
    frame_id: u64,
    messages: Vec<crate::lua_api::message_frame::Message>,
) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let data = state
        .message_frames
        .entry(frame_id)
        .or_insert_with(crate::lua_api::message_frame::MessageFrameData::default);
    data.messages = messages;
    data.scroll_offset = data
        .scroll_offset
        .clamp(0, super::widget_message_frame_scroll::message_frame_scroll_limit(data));
}

fn call_message_predicate(
    lua: &mlua::Lua,
    predicate: &mlua::Function,
    message: &crate::lua_api::message_frame::Message,
) -> mlua::Result<bool> {
    predicate.call(build_message_callback_args(lua, message)?)
}

fn call_message_transform(
    lua: &mlua::Lua,
    transform: &mlua::Function,
    message: &crate::lua_api::message_frame::Message,
) -> mlua::Result<crate::lua_api::message_frame::Message> {
    let results: mlua::MultiValue = transform.call(build_message_callback_args(lua, message)?)?;
    Ok(message_from_transform_results(message, results))
}

fn call_color_transform(
    lua: &mlua::Lua,
    transform: &mlua::Function,
    message: &crate::lua_api::message_frame::Message,
) -> mlua::Result<Option<(f32, f32, f32)>> {
    let results: mlua::MultiValue = transform.call(build_message_callback_args(lua, message)?)?;
    let mut values = results.into_iter();
    let Some(change_color) = values.next() else {
        return Ok(None);
    };
    if !is_lua_truthy(&change_color) {
        return Ok(None);
    }
    Ok(Some((
        value_to_optional_f32(values.next()).unwrap_or(message.r),
        value_to_optional_f32(values.next()).unwrap_or(message.g),
        value_to_optional_f32(values.next()).unwrap_or(message.b),
    )))
}

fn build_message_callback_args(
    lua: &mlua::Lua,
    message: &crate::lua_api::message_frame::Message,
) -> mlua::Result<mlua::MultiValue> {
    let mut args = mlua::MultiValue::new();
    args.push_back(Value::String(lua.create_string(&message.text)?));
    args.push_back(Value::Number(message.r as f64));
    args.push_back(Value::Number(message.g as f64));
    args.push_back(Value::Number(message.b as f64));
    args.push_back(Value::Number(message.a as f64));
    match message.message_id {
        Some(message_id) => args.push_back(Value::Integer(message_id)),
        None => args.push_back(Value::Nil),
    }
    args.push_back(Value::Number(message.timestamp));
    Ok(args)
}

fn message_from_transform_results(
    original: &crate::lua_api::message_frame::Message,
    results: mlua::MultiValue,
) -> crate::lua_api::message_frame::Message {
    let mut values = results.into_iter();
    crate::lua_api::message_frame::Message {
        text: value_to_optional_string(values.next()).unwrap_or_else(|| original.text.clone()),
        r: value_to_optional_f32(values.next()).unwrap_or(original.r),
        g: value_to_optional_f32(values.next()).unwrap_or(original.g),
        b: value_to_optional_f32(values.next()).unwrap_or(original.b),
        a: value_to_optional_f32(values.next()).unwrap_or(original.a),
        message_id: value_to_optional_i64(values.next()).or(original.message_id),
        timestamp: value_to_optional_f64(values.next()).unwrap_or(original.timestamp),
    }
}

fn value_to_optional_string(value: Option<Value>) -> Option<String> {
    match value {
        Some(Value::String(text)) => Some(text.to_string_lossy().to_string()),
        _ => None,
    }
}

fn value_to_optional_f32(value: Option<Value>) -> Option<f32> {
    match value {
        Some(Value::Integer(number)) => Some(number as f32),
        Some(Value::Number(number)) => Some(number as f32),
        _ => None,
    }
}

fn value_to_optional_f64(value: Option<Value>) -> Option<f64> {
    match value {
        Some(Value::Integer(number)) => Some(number as f64),
        Some(Value::Number(number)) => Some(number),
        _ => None,
    }
}

fn value_to_optional_i64(value: Option<Value>) -> Option<i64> {
    match value {
        Some(Value::Integer(number)) => Some(number),
        Some(Value::Number(number)) => Some(number as i64),
        _ => None,
    }
}

fn is_lua_truthy(value: &Value) -> bool {
    !matches!(value, Value::Nil | Value::Boolean(false))
}

fn log_message(state: &SimState, id: u64, text: &str) {
    let name = state
        .widgets
        .get(id)
        .and_then(|w| w.name.as_deref())
        .unwrap_or("?");
    let clean = crate::dump::strip_wow_escapes(text);
    eprintln!("[{name}] {clean}");
}

pub(super) fn add_message_core(
    state: &mut SimState,
    id: u64,
    args: mlua::MultiValue,
    log: bool,
) {
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
    super::widget_message_frame_scroll::truncate_messages(data);
    data.display_dirty = true;
}

pub(super) fn backfill_message(state: &mut SimState, id: u64, args: mlua::MultiValue) {
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
    data.display_dirty = true;
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
