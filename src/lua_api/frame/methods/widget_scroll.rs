//! ScrollFrame and ScrollBox widget methods.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use super::methods_helpers::get_mixin_override;
use super::methods_hierarchy::reparent_widget;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use crate::widget::WidgetRegistry;
use mlua::{Function, MultiValue, Table, Value};

pub fn add_scrollframe_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_scrollframe_child_methods(methods);
    add_scrollframe_offset_methods(methods);
    add_scrollframe_range_methods(methods);
}

pub fn add_scrollbox_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_scrollbox_register_callback_method(methods);
    add_scrollbox_for_each_frame_method(methods);
    add_scrollbox_unregister_callback_method(methods);
    add_scrollbox_can_interpolate_scroll_method(methods);
    add_scrollbox_set_interpolate_scroll_method(methods);
}

fn add_scrollbox_register_callback_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("RegisterCallback", |lua, this, args: MultiValue| {
        if let Some(result) =
            call_scrollbox_mixin_override(lua, this.0, "RegisterCallback", args.clone())?
        {
            return Ok(result);
        }
        register_scrollbox_callback_fallback(lua, this.0, args)
    });
}

fn add_scrollbox_for_each_frame_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ForEachFrame", |lua, this, cb: Function| {
        let args = MultiValue::from_vec(vec![Value::Function(cb.clone())]);
        if let Some(result) = call_scrollbox_mixin_override(lua, this.0, "ForEachFrame", args)? {
            return Ok(result);
        }
        for_each_scrollbox_frame_fallback(lua, this.0, cb)
    });
}

fn add_scrollbox_unregister_callback_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("UnregisterCallback", |lua, this, args: MultiValue| {
        if let Some(result) =
            call_scrollbox_mixin_override(lua, this.0, "UnregisterCallback", args.clone())?
        {
            return Ok(result);
        }
        unregister_scrollbox_callback_fallback(lua, this.0, args)
    });
}

fn add_scrollbox_can_interpolate_scroll_method<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
) {
    methods.add_method("CanInterpolateScroll", |lua, this, ()| {
        if let Some(result) =
            call_scrollbox_mixin_override(lua, this.0, "CanInterpolateScroll", MultiValue::new())?
        {
            return Ok(matches!(result, Value::Boolean(true)));
        }
        can_interpolate_scroll_fallback(lua, this.0)
    });
}

fn add_scrollbox_set_interpolate_scroll_method<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
) {
    methods.add_method("SetInterpolateScroll", |lua, this, enabled: bool| {
        let args = MultiValue::from_vec(vec![Value::Boolean(enabled)]);
        if let Some(result) =
            call_scrollbox_mixin_override(lua, this.0, "SetInterpolateScroll", args)?
        {
            return Ok(result);
        }
        set_interpolate_scroll_fallback(lua, this.0, enabled)?;
        Ok(Value::Nil)
    });
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
    let registration = parse_scrollbox_callback_registration(lua, args)?;
    unregister_scrollbox_callback_by_owner(
        lua,
        frame_id,
        &registration.event,
        &registration.owner,
    )?;
    let callback_tables = ensure_scrollbox_callback_tables(lua, frame_id)?;
    store_scrollbox_callback(lua, &callback_tables, &registration)?;
    Ok(registration.owner)
}

struct ScrollboxCallbackRegistration {
    event: String,
    func: Function,
    owner: Value,
    extra_args: Vec<Value>,
}

fn parse_scrollbox_callback_registration(
    lua: &mlua::Lua,
    args: MultiValue,
) -> mlua::Result<ScrollboxCallbackRegistration> {
    let mut it = args.into_iter();
    let event = parse_scrollbox_callback_event(it.next())?;
    let func = parse_scrollbox_callback_function(it.next())?;
    let owner = scrollbox_callback_owner(lua, it.next())?;
    let extra_args = it.collect();
    Ok(ScrollboxCallbackRegistration {
        event,
        func,
        owner,
        extra_args,
    })
}

fn parse_scrollbox_callback_event(event: Option<Value>) -> mlua::Result<String> {
    let Some(Value::String(event)) = event else {
        return Err(mlua::Error::runtime(
            "CallbackRegistryMixin::RegisterCallback 'event' requires string type.",
        ));
    };
    Ok(event.to_string_lossy().to_string())
}

fn parse_scrollbox_callback_function(func: Option<Value>) -> mlua::Result<Function> {
    let Some(Value::Function(func)) = func else {
        return Err(mlua::Error::runtime(
            "CallbackRegistryMixin::RegisterCallback 'func' requires function type.",
        ));
    };
    Ok(func)
}

fn scrollbox_callback_owner(lua: &mlua::Lua, owner: Option<Value>) -> mlua::Result<Value> {
    match owner {
        Some(Value::Nil) | None => Ok(Value::Table(lua.create_table()?)),
        Some(value) => Ok(value),
    }
}

fn store_scrollbox_callback(
    lua: &mlua::Lua,
    callback_tables: &Table,
    registration: &ScrollboxCallbackRegistration,
) -> mlua::Result<()> {
    if registration.extra_args.is_empty() {
        return store_scrollbox_function_callback(lua, callback_tables, registration);
    }
    store_scrollbox_closure_callback(lua, callback_tables, registration)
}

fn store_scrollbox_function_callback(
    lua: &mlua::Lua,
    callback_tables: &Table,
    registration: &ScrollboxCallbackRegistration,
) -> mlua::Result<()> {
    let (_, function_key) = callback_type_keys(lua);
    let callbacks = callback_bucket(lua, callback_tables, function_key, &registration.event)?;
    callbacks.raw_set(registration.owner.clone(), registration.func.clone())?;
    Ok(())
}

fn store_scrollbox_closure_callback(
    lua: &mlua::Lua,
    callback_tables: &Table,
    registration: &ScrollboxCallbackRegistration,
) -> mlua::Result<()> {
    let (closure_key, _) = callback_type_keys(lua);
    let callbacks = callback_bucket(lua, callback_tables, closure_key, &registration.event)?;
    let closure = build_scrollbox_callback_closure(
        lua,
        &registration.owner,
        &registration.func,
        &registration.extra_args,
    )?;
    callbacks.raw_set(registration.owner.clone(), closure)?;
    Ok(())
}

fn build_scrollbox_callback_closure(
    lua: &mlua::Lua,
    owner: &Value,
    func: &Function,
    extra_args: &[Value],
) -> mlua::Result<Function> {
    let owner_arg = owner.clone();
    let closure_func = func.clone();
    let extra_args = extra_args.to_vec();
    lua.create_function(move |_, event_args: MultiValue| {
        let mut call_args = MultiValue::new();
        call_args.push_back(owner_arg.clone());
        for extra in &extra_args {
            call_args.push_back(extra.clone());
        }
        for event_arg in event_args {
            call_args.push_back(event_arg);
        }
        closure_func.call::<Value>(call_args)
    })
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

fn can_interpolate_scroll_fallback(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<bool> {
    let fields = frame_user_value(lua, frame_id)?;
    Ok(matches!(
        fields.get::<Value>("canInterpolateScroll")?,
        Value::Boolean(true)
    ))
}

fn set_interpolate_scroll_fallback(
    lua: &mlua::Lua,
    frame_id: u64,
    enabled: bool,
) -> mlua::Result<()> {
    let fields = frame_user_value(lua, frame_id)?;
    fields.set("canInterpolateScroll", enabled)
}

pub(crate) fn assign_scroll_child(
    state: &mut crate::lua_api::SimState,
    parent_id: u64,
    child_id: u64,
    should_reparent: bool,
) {
    if let Some(frame) = state.widgets.get_mut_visual(parent_id) {
        frame.scroll_child_id = Some(child_id);
        frame.scroll_child_rect_size = None;
    }
    if should_reparent {
        reparent_widget(&mut state.widgets, child_id, Some(parent_id));
    }
    state.visible_on_update_cache = None;
    state.invalidate_layout(child_id);
}

fn add_scrollframe_child_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_scroll_child_method(methods);
    add_get_scroll_child_method(methods);
    add_update_scroll_child_rect_method(methods);
}

fn add_set_scroll_child_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScrollChild", |lua, this, child: Value| {
        set_scroll_child_fallback(lua, this.0, child)
    });
}

fn add_get_scroll_child_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetScrollChild", |lua, this, ()| {
        get_scroll_child_fallback(lua, this.0)
    });
}

fn add_update_scroll_child_rect_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("UpdateScrollChildRect", |lua, this, ()| {
        update_scroll_child_rect_fallback(lua, this.0)
    });
}

fn set_scroll_child_fallback(lua: &mlua::Lua, frame_id: u64, child: Value) -> mlua::Result<()> {
    if scroll_child_assignment_blocked(lua, frame_id) {
        return Ok(());
    }
    let child_id = require_scroll_child_id(&child)?;
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    assign_scroll_child(&mut state, frame_id, child_id, true);
    Ok(())
}

fn scroll_child_assignment_blocked(lua: &mlua::Lua, frame_id: u64) -> bool {
    let state_rc = get_sim_state(lua);
    combat_lockdown::check_and_fire(lua, &state_rc, frame_id, "SetScrollChild")
}

fn require_scroll_child_id(child: &Value) -> mlua::Result<u64> {
    extract_frame_id(child)
        .ok_or_else(|| mlua::Error::runtime("Usage: ScrollFrame:SetScrollChild(child)"))
}

fn get_scroll_child_fallback(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<Value> {
    let child_id = scroll_child_id(lua, frame_id);
    match child_id {
        Some(child_id) => frame_ref(lua, child_id),
        None => Ok(Value::Nil),
    }
}

fn scroll_child_id(lua: &mlua::Lua, frame_id: u64) -> Option<u64> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(frame_id)
        .and_then(|frame| frame.scroll_child_id)
}

fn update_scroll_child_rect_fallback(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let bounds = scroll_child_bounds(&mut state, frame_id);
    store_scroll_child_bounds(&mut state.widgets, frame_id, bounds);
    Ok(())
}

fn scroll_child_bounds(state: &mut crate::lua_api::SimState, frame_id: u64) -> Option<(f32, f32)> {
    let scroll_child_id = state
        .widgets
        .get(frame_id)
        .and_then(|frame| frame.scroll_child_id);
    let Some(child_id) = scroll_child_id else {
        return None;
    };
    state.invalidate_layout(child_id);
    state.ensure_layout_rects();
    scroll_child_rect_size(&state.widgets, child_id)
}

fn store_scroll_child_bounds(
    widgets: &mut WidgetRegistry,
    frame_id: u64,
    bounds: Option<(f32, f32)>,
) {
    if let Some(frame) = widgets.get_mut(frame_id) {
        frame.scroll_child_rect_size = bounds;
    }
}

fn scroll_child_rect_size(widgets: &WidgetRegistry, root_id: u64) -> Option<(f32, f32)> {
    let mut stack = vec![root_id];
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut saw_rect = false;

    while let Some(frame_id) = stack.pop() {
        let Some(frame) = widgets.get(frame_id) else {
            continue;
        };
        if let Some(rect) = frame.layout_rect {
            min_x = min_x.min(rect.x);
            min_y = min_y.min(rect.y);
            max_x = max_x.max(rect.x + rect.width);
            max_y = max_y.max(rect.y + rect.height);
            saw_rect = true;
        }
        stack.extend(frame.children.iter().rev().copied());
    }

    saw_rect.then_some((max_x - min_x, max_y - min_y))
}

fn scroll_child_range_size(
    frame: &crate::widget::Frame,
    child_frame: Option<&crate::widget::Frame>,
) -> (f64, f64) {
    if let Some((width, height)) = frame.scroll_child_rect_size {
        return (width as f64, height as f64);
    }
    child_frame
        .map(|child| (child.width as f64, child.height as f64))
        .unwrap_or((0.0, 0.0))
}

fn add_scrollframe_offset_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_scroll_offset_setter(
        methods,
        "SetHorizontalScroll",
        |frame| frame.scroll_horizontal,
        |frame, offset| frame.scroll_horizontal = offset,
    );
    add_scroll_offset_getter(methods, "GetHorizontalScroll", |frame| {
        frame.scroll_horizontal
    });
    add_scroll_offset_setter(
        methods,
        "SetVerticalScroll",
        |frame| frame.scroll_vertical,
        |frame, offset| frame.scroll_vertical = offset,
    );
    add_scroll_offset_getter(methods, "GetVerticalScroll", |frame| frame.scroll_vertical);
}

fn add_scroll_offset_setter<M, R, W>(methods: &mut M, name: &'static str, read: R, write: W)
where
    M: mlua::UserDataMethods<FrameRef>,
    R: Fn(&crate::widget::Frame) -> f64 + Copy + 'static,
    W: Fn(&mut crate::widget::Frame, f64) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, offset: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if state
            .widgets
            .get(this.0)
            .is_some_and(|frame| read(frame) == offset)
        {
            return Ok(());
        }

        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            write(frame, offset);
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
        let child_frame = frame.scroll_child_id.and_then(|cid| state.widgets.get(cid));
        let (child_width, _) = scroll_child_range_size(frame, child_frame);
        Ok((child_width - frame.width as f64).max(0.0))
    });

    methods.add_method("GetVerticalScrollRange", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = match state.widgets.get(this.0) {
            Some(f) => f,
            None => return Ok(0.0_f64),
        };
        let child_frame = frame.scroll_child_id.and_then(|cid| state.widgets.get(cid));
        let (_, child_height) = scroll_child_range_size(frame, child_frame);
        Ok((child_height - frame.height as f64).max(0.0))
    });
}
