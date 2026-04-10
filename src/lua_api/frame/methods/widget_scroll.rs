//! ScrollFrame and ScrollBox widget methods.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use super::methods_helpers::get_mixin_override;
use super::methods_hierarchy::reparent_widget;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use mlua::{Function, MultiValue, Table, Value};

pub fn add_scrollframe_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_scrollframe_child_methods(methods);
    add_scrollframe_offset_methods(methods);
    add_scrollframe_range_methods(methods);
    methods.add_method("SetMaxLines", |_, _, _: mlua::Variadic<Value>| Ok(()));
}

pub fn add_scrollbox_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("RegisterCallback", |lua, this, args: MultiValue| {
        if let Some(result) =
            call_scrollbox_mixin_override(lua, this.0, "RegisterCallback", args.clone())?
        {
            return Ok(result);
        }
        register_scrollbox_callback_fallback(lua, this.0, args)
    });
    methods.add_method("ForEachFrame", |lua, this, cb: Function| {
        if let Some(result) = call_scrollbox_mixin_override(
            lua,
            this.0,
            "ForEachFrame",
            MultiValue::from_vec(vec![Value::Function(cb.clone())]),
        )? {
            return Ok(result);
        }
        for_each_scrollbox_frame_fallback(lua, this.0, cb)
    });
    methods.add_method("UnregisterCallback", |lua, this, args: MultiValue| {
        if let Some(result) =
            call_scrollbox_mixin_override(lua, this.0, "UnregisterCallback", args.clone())?
        {
            return Ok(result);
        }
        unregister_scrollbox_callback_fallback(lua, this.0, args)
    });
    methods.add_method("CanInterpolateScroll", |_, _this, ()| Ok(false));
    methods.add_method("SetInterpolateScroll", |_, _this, _enabled: bool| Ok(()));
}

fn call_scrollbox_mixin_override(
    lua: &mlua::Lua,
    frame_id: u64,
    method_name: &str,
    args: MultiValue,
) -> mlua::Result<Option<Value>> {
    let Some((override_fn, self_value)) = get_mixin_override(lua, frame_id, method_name) else {
        return Ok(None);
    };
    let mut call_args = MultiValue::new();
    call_args.push_back(self_value);
    for arg in args {
        call_args.push_back(arg);
    }
    override_fn.call(call_args).map(Some)
}

fn frame_user_value(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<Table> {
    let Value::UserData(ud) = frame_ref(lua, frame_id)? else {
        return Err(mlua::Error::runtime("frame reference is not userdata"));
    };
    ud.user_value::<Table>()
}

fn callback_type_keys(lua: &mlua::Lua) -> (Value, Value) {
    let globals = lua.globals();
    let Ok(callback_type) = globals.get::<Table>("CallbackType") else {
        return (Value::Integer(1), Value::Integer(2));
    };
    let closure_key = callback_type
        .get::<Value>("Closure")
        .unwrap_or(Value::Integer(1));
    let function_key = callback_type
        .get::<Value>("Function")
        .unwrap_or(Value::Integer(2));
    (closure_key, function_key)
}

fn ensure_scrollbox_callback_tables(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<Table> {
    let fields = frame_user_value(lua, frame_id)?;
    if let Ok(Value::Table(callback_tables)) = fields.get::<Value>("callbackTables") {
        return Ok(callback_tables);
    }

    let callback_tables = lua.create_table()?;
    let (closure_key, function_key) = callback_type_keys(lua);
    callback_tables.raw_set(closure_key, lua.create_table()?)?;
    callback_tables.raw_set(function_key, lua.create_table()?)?;
    fields.set("callbackTables", callback_tables.clone())?;
    fields.set("executingEvents", lua.create_table()?)?;
    fields.set("deferredCallbacks", lua.create_table()?)?;
    Ok(callback_tables)
}

fn callback_bucket(
    lua: &mlua::Lua,
    callback_tables: &Table,
    callback_type_key: Value,
    event: &str,
) -> mlua::Result<Table> {
    let bucket = match callback_tables.raw_get::<Value>(callback_type_key.clone())? {
        Value::Table(table) => table,
        _ => {
            let table = lua.create_table()?;
            callback_tables.raw_set(callback_type_key, table.clone())?;
            table
        }
    };
    match bucket.raw_get::<Value>(event)? {
        Value::Table(callbacks) => Ok(callbacks),
        _ => {
            let callbacks = lua.create_table()?;
            bucket.raw_set(event, callbacks.clone())?;
            Ok(callbacks)
        }
    }
}

fn register_scrollbox_callback_fallback(
    lua: &mlua::Lua,
    frame_id: u64,
    args: MultiValue,
) -> mlua::Result<Value> {
    let mut it = args.into_iter();
    let Some(Value::String(event)) = it.next() else {
        return Err(mlua::Error::runtime(
            "CallbackRegistryMixin::RegisterCallback 'event' requires string type.",
        ));
    };
    let event = event.to_string_lossy().to_string();
    let Some(Value::Function(func)) = it.next() else {
        return Err(mlua::Error::runtime(
            "CallbackRegistryMixin::RegisterCallback 'func' requires function type.",
        ));
    };

    let owner = match it.next() {
        Some(Value::Nil) | None => Value::Table(lua.create_table()?),
        Some(value) => value,
    };
    unregister_scrollbox_callback_by_owner(lua, frame_id, &event, &owner)?;

    let callback_tables = ensure_scrollbox_callback_tables(lua, frame_id)?;
    let remaining: Vec<Value> = it.collect();
    let (closure_key, function_key) = callback_type_keys(lua);
    if remaining.is_empty() {
        let callbacks = callback_bucket(lua, &callback_tables, function_key, &event)?;
        callbacks.raw_set(owner.clone(), func)?;
    } else {
        let owner_arg = owner.clone();
        let closure_func = func.clone();
        let closure = lua.create_function(move |_, event_args: MultiValue| {
            let mut call_args = MultiValue::new();
            call_args.push_back(owner_arg.clone());
            for extra in &remaining {
                call_args.push_back(extra.clone());
            }
            for event_arg in event_args {
                call_args.push_back(event_arg);
            }
            closure_func.call::<Value>(call_args)
        })?;
        let callbacks = callback_bucket(lua, &callback_tables, closure_key, &event)?;
        callbacks.raw_set(owner.clone(), closure)?;
    }
    Ok(owner)
}

fn unregister_scrollbox_callback_fallback(
    lua: &mlua::Lua,
    frame_id: u64,
    args: MultiValue,
) -> mlua::Result<Value> {
    let mut it = args.into_iter();
    let Some(Value::String(event)) = it.next() else {
        return Err(mlua::Error::runtime(
            "CallbackRegistryMixin:UnregisterCallback 'event' requires string type.",
        ));
    };
    let Some(owner) = it.next() else {
        return Err(mlua::Error::runtime(
            "CallbackRegistryMixin:UnregisterCallback 'owner' is required.",
        ));
    };
    if matches!(owner, Value::Nil) {
        return Err(mlua::Error::runtime(
            "CallbackRegistryMixin:UnregisterCallback 'owner' is required.",
        ));
    }
    unregister_scrollbox_callback_by_owner(lua, frame_id, &event.to_string_lossy(), &owner)?;
    Ok(Value::Nil)
}

fn unregister_scrollbox_callback_by_owner(
    lua: &mlua::Lua,
    frame_id: u64,
    event: &str,
    owner: &Value,
) -> mlua::Result<()> {
    let callback_tables = ensure_scrollbox_callback_tables(lua, frame_id)?;
    let (closure_key, function_key) = callback_type_keys(lua);
    for callback_type_key in [closure_key, function_key] {
        let callbacks = callback_bucket(lua, &callback_tables, callback_type_key, event)?;
        callbacks.raw_set(owner.clone(), Value::Nil)?;
    }
    Ok(())
}

fn for_each_scrollbox_frame_fallback(
    lua: &mlua::Lua,
    frame_id: u64,
    callback: Function,
) -> mlua::Result<Value> {
    let fields = frame_user_value(lua, frame_id)?;
    let Value::Table(view) = fields.get::<Value>("view").unwrap_or(Value::Nil) else {
        return Ok(Value::Nil);
    };

    if let Ok(Value::Function(for_each_frame)) = view.raw_get::<Value>("ForEachFrame") {
        return for_each_frame.call((view, callback));
    }

    let frames = match view.raw_get::<Value>("GetFrames") {
        Ok(Value::Function(get_frames)) => match get_frames.call::<Value>(view.clone())? {
            Value::Table(frames) => frames,
            _ => return Ok(Value::Nil),
        },
        _ => return Ok(Value::Nil),
    };

    for index in 1..=frames.raw_len() {
        let frame = frames.raw_get::<Value>(index)?;
        let element_data = frame_element_data(&frame)?;
        let result: Value = callback.call((frame.clone(), element_data))?;
        if !matches!(result, Value::Nil | Value::Boolean(false)) {
            return Ok(result);
        }
    }
    Ok(Value::Nil)
}

fn frame_element_data(frame: &Value) -> mlua::Result<Value> {
    let maybe_func = match frame {
        Value::UserData(ud) => ud.user_value::<Table>().ok().and_then(|fields| {
            fields
                .raw_get::<Value>("GetElementData")
                .ok()
                .and_then(|value| match value {
                    Value::Function(func) => Some(func),
                    _ => None,
                })
        }),
        Value::Table(table) => table
            .raw_get::<Value>("GetElementData")
            .ok()
            .and_then(|value| match value {
                Value::Function(func) => Some(func),
                _ => None,
            }),
        _ => None,
    };

    match maybe_func {
        Some(func) => func.call(frame.clone()),
        None => Ok(Value::Nil),
    }
}

fn add_scrollframe_child_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScrollChild", |lua, this, child: Value| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "SetScrollChild") {
                return Ok(());
            }
        }
        let child_id = match extract_frame_id(&child) {
            Some(cid) => cid,
            None => {
                return Err(mlua::Error::runtime(
                    "Usage: ScrollFrame:SetScrollChild(child)",
                ));
            }
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.scroll_child_id = Some(child_id);
        }
        reparent_widget(&mut state.widgets, child_id, Some(id));
        state.visible_on_update_cache = None;
        state.invalidate_layout(child_id);
        Ok(())
    });

    methods.add_method("GetScrollChild", |lua, this, ()| {
        let child_id = {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            state.widgets.get(this.0).and_then(|f| f.scroll_child_id)
        };
        match child_id {
            Some(cid) => frame_ref(lua, cid),
            None => Ok(Value::Nil),
        }
    });

    methods.add_method("UpdateScrollChildRect", |_, _this, ()| Ok(()));
}

fn add_scrollframe_offset_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_scroll_offset_setter(methods, "SetHorizontalScroll", |frame, offset| {
        frame.scroll_horizontal = offset
    });
    add_scroll_offset_getter(methods, "GetHorizontalScroll", |frame| {
        frame.scroll_horizontal
    });
    add_scroll_offset_setter(methods, "SetVerticalScroll", |frame, offset| {
        frame.scroll_vertical = offset
    });
    add_scroll_offset_getter(methods, "GetVerticalScroll", |frame| frame.scroll_vertical);
}

fn add_scroll_offset_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::widget::Frame, f64) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, offset: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            setter(frame, offset);
        }
        Ok(())
    });
}

fn add_scroll_offset_getter<M, F>(methods: &mut M, name: &'static str, getter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::widget::Frame) -> f64 + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(getter).unwrap_or(0.0))
    });
}

fn add_scrollframe_range_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetHorizontalScrollRange", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = match state.widgets.get(this.0) {
            Some(f) => f,
            None => return Ok(0.0_f64),
        };
        let child_width = frame
            .scroll_child_id
            .and_then(|cid| state.widgets.get(cid))
            .map(|c| c.width as f64)
            .unwrap_or(0.0);
        Ok((child_width - frame.width as f64).max(0.0))
    });

    methods.add_method("GetVerticalScrollRange", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = match state.widgets.get(this.0) {
            Some(f) => f,
            None => return Ok(0.0_f64),
        };
        let child_height = frame
            .scroll_child_id
            .and_then(|cid| state.widgets.get(cid))
            .map(|c| c.height as f64)
            .unwrap_or(0.0);
        Ok((child_height - frame.height as f64).max(0.0))
    });
}
