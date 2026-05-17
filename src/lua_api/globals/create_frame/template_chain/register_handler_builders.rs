use super::load_template;
use crate::lua_api::methods::{call_function_state, create_string};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_register_for_clicks_handler(
    state: &mut LuaState,
    first: &str,
    second: Option<&str>,
    third: Option<&str>,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local first, second, third = ...
            return function(self, ...)
                return self:RegisterForClicks(first, second, third)
            end
        "#,
        "template-register-for-clicks-handler",
    )?;
    let first = create_string(state, first);
    let second = second
        .map(|value| create_string(state, value))
        .unwrap_or(Val::Nil);
    let third = third
        .map(|value| create_string(state, value))
        .unwrap_or(Val::Nil);
    call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[first, second, third],
    )
}

pub(super) fn build_register_for_drag_handler(
    state: &mut LuaState,
    button: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local button = ...
            return function(self, ...)
                return self:RegisterForDrag(button)
            end
        "#,
        "template-register-for-drag-handler",
    )?;
    let button = create_string(state, button);
    call_function_state(state, Val::Function(builder.gc_ref()), &[button])
}
