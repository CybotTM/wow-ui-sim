use crate::lua_api::script_helpers::ScriptBinding;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

pub(super) fn optional_script_binding_from_stack(
    state: &mut LuaState,
    index: i32,
) -> LuaResult<ScriptBinding> {
    match stack_val(state, index) {
        Val::Nil => Ok(ScriptBinding::Normal),
        Val::Num(raw) if raw.is_finite() => parse_script_binding(raw),
        other => Err(runtime_error(format!(
            "script binding type must be a number, got {}",
            other.type_name()
        ))),
    }
}

fn parse_script_binding(raw: f64) -> LuaResult<ScriptBinding> {
    let binding_index = raw as i32;
    if binding_index as f64 == raw
        && let Some(binding) = ScriptBinding::from_index(binding_index)
    {
        return Ok(binding);
    }
    Err(runtime_error(format!(
        "script binding type must be 0, 1, or 2, got {raw}"
    )))
}

pub(super) fn build_hooked_script(state: &mut LuaState, old: Val, hook: Val) -> LuaResult<Val> {
    let func = state.load(
        r#"
        local old, hook = ...
        if old == nil then
            return hook
        end
        return function(...)
            old(...)
            hook(...)
        end
    "#,
    )?;
    let call_base = state.top;
    state.ensure_stack(call_base + 4);
    state.stack_set(call_base, Val::Function(func.gc_ref()));
    state.stack_set(call_base + 1, old);
    state.stack_set(call_base + 2, hook);
    state.top = call_base + 3;
    state.call_function(call_base, 1)?;
    let result = state.stack_get(call_base);
    state.top = call_base;
    Ok(result)
}

pub(super) fn reject_unsupported_hook_binding(
    state: &mut LuaState,
    binding: ScriptBinding,
) -> bool {
    if binding == ScriptBinding::Normal {
        return false;
    }
    state.push(Val::Bool(false));
    true
}
