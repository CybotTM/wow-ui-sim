use super::Value;

/// Fire a script handler on a frame (e.g. OnTooltipCleared).
pub(crate) fn fire_tooltip_script(
    lua: &mlua::Lua,
    frame_id: u64,
    handler: &str,
) -> mlua::Result<()> {
    fire_tooltip_script_with_args(lua, frame_id, handler, Vec::new())
}

pub(super) fn fire_tooltip_script_with_args(
    lua: &mlua::Lua,
    frame_id: u64,
    handler: &str,
    extra_args: Vec<Value>,
) -> mlua::Result<()> {
    if let Some(func) = crate::lua_api::script_helpers::get_script(lua, frame_id, handler)
        && let Some(frame_ud) = crate::lua_api::script_helpers::get_frame_ref(lua, frame_id)
    {
        let mut call_args = mlua::MultiValue::with_capacity(extra_args.len() + 1);
        call_args.push_back(frame_ud);
        for arg in extra_args {
            call_args.push_back(arg);
        }
        if let Err(e) = func.call::<()>(call_args) {
            crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
        }
    }
    Ok(())
}

/// Extract f32 from a Lua Value, returning default if nil/absent.
pub(crate) fn val_to_f32(val: Option<Value>, default: f32) -> f32 {
    match val {
        Some(Value::Number(n)) => n as f32,
        Some(Value::Integer(n)) => n as f32,
        _ => default,
    }
}
