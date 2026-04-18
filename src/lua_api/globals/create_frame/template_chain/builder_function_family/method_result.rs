//! Handlers that forward the result of some method call as the argument.

use super::super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_method_result_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithNoArgFunctionResult {
            function_name,
            arg_function_name,
        } => build_function_handler_with_noarg_function_result(
            state,
            function_name,
            arg_function_name,
        )
        .map(Some),
        FastHandlerRef::FunctionWithSelfNoArgsMethodResult {
            function_name,
            method_name,
        } => {
            build_function_handler_with_self_noarg_method_result(state, function_name, method_name)
                .map(Some)
        }
        FastHandlerRef::FunctionWithGlobalMethodNoArgsResult {
            function_name,
            target_path,
            method_name,
        } => build_function_handler_with_global_method_noargs_result(
            state,
            function_name,
            target_path,
            method_name,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn build_function_handler_with_noarg_function_result(
    state: &mut LuaState,
    function_name: &str,
    arg_function_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, arg_fn = ...
            return function(self, ...)
                return fn(arg_fn())
            end
        "#,
        "template-inline-function-noarg-function-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let arg_function = resolve_global_path(state, arg_function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg_function],
    )
}

fn build_function_handler_with_self_noarg_method_result(
    state: &mut LuaState,
    function_name: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, method_name = ...
            return function(self, ...)
                return fn(self[method_name](self))
            end
        "#,
        "template-inline-function-self-noarg-method-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name],
    )
}

fn build_function_handler_with_global_method_noargs_result(
    state: &mut LuaState,
    function_name: &str,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, target, method_name = ...
            return function(self, ...)
                return fn(target[method_name](target))
            end
        "#,
        "template-inline-function-global-method-noargs-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let method_target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_target, method_name],
    )
}
