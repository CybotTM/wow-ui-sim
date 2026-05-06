//! Handlers whose args come from global-path lookups (possibly mixed with
//! literals or `self`).

use super::super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::hot_literals::{
    TEMPLATE_INLINE_FUNCTION_GLOBAL_ARG, TEMPLATE_INLINE_FUNCTION_TWO_GLOBAL_ARGS,
};
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_global_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let handler = try_build_plain_global_variants(state, handler_ref)?
        .or(try_build_mixed_global_variants(state, handler_ref)?)
        .or(try_build_global_self_method_variant(state, handler_ref)?)
        .or(try_build_global_self_variants(state, handler_ref)?);
    Ok(handler)
}

fn try_build_plain_global_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithGlobalArg {
            function_name,
            arg_path,
        } => build_function_handler_with_global_arg(state, function_name, arg_path).map(Some),
        FastHandlerRef::FunctionWithTwoGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
        } => build_function_handler_with_two_global_args(
            state,
            function_name,
            first_arg_path,
            second_arg_path,
        )
        .map(Some),
        FastHandlerRef::FunctionWithThreeGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
        } => build_function_handler_with_three_global_args(
            state,
            function_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn try_build_mixed_global_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = try_build_global_number_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    try_build_string_global_variants(state, handler_ref)
}

fn try_build_global_number_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithTwoGlobalNumberArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third,
        } => build_function_handler_with_two_global_number_args(
            state,
            function_name,
            first_arg_path,
            second_arg_path,
            *third,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn try_build_string_global_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithStringNilNilGlobalArgs {
            function_name,
            first,
            fourth,
        } => build_function_handler_with_string_nil_nil_global_args(
            state,
            function_name,
            first,
            fourth,
        )
        .map(Some),
        FastHandlerRef::FunctionWithStringGlobalBoolArg {
            function_name,
            first,
            second_arg_path,
            third,
        } => build_function_handler_with_string_global_bool_arg(
            state,
            function_name,
            first,
            second_arg_path,
            *third,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn try_build_global_self_method_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithGlobalSelfMethodSelfMethodBoolArgs {
            function_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            fourth,
        } => build_function_handler_with_global_self_method_self_method_bool_args(
            state,
            function_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            *fourth,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn try_build_global_self_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithGlobalAndSelfIdArg {
            function_name,
            global_arg_path,
        } => build_function_handler_with_global_and_self_value_arg(
            state,
            function_name,
            global_arg_path,
            true,
        )
        .map(Some),
        FastHandlerRef::FunctionWithGlobalAndSelfArg {
            function_name,
            global_arg_path,
        } => build_function_handler_with_global_and_self_value_arg(
            state,
            function_name,
            global_arg_path,
            false,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn build_function_handler_with_two_global_number_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
    third: f64,
) -> LuaResult<Val> {
    let first = resolve_global_path(state, first_arg_path);
    let second = resolve_global_path(state, second_arg_path);
    build_function_handler_with_three_bound_args(
        state,
        function_name,
        first,
        second,
        Val::Num(third),
    )
}

fn build_function_handler_with_three_bound_args(
    state: &mut LuaState,
    function_name: &str,
    first: Val,
    second: Val,
    third: Val,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second, third = ...
            return function(self, ...)
                return fn(first, second, third)
            end
        "#,
        "template-inline-function-three-bound-args",
    )?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, second, third],
    )
}

fn build_function_handler_with_three_global_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
    third_arg_path: &str,
) -> LuaResult<Val> {
    let first = resolve_global_path(state, first_arg_path);
    let second = resolve_global_path(state, second_arg_path);
    let third = resolve_global_path(state, third_arg_path);
    build_function_handler_with_three_bound_args(state, function_name, first, second, third)
}

fn build_function_handler_with_string_global_bool_arg(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    second_arg_path: &str,
    third: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second, third = ...
            return function(self, ...)
                return fn(first, second, third)
            end
        "#,
        "template-inline-function-string-global-bool-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    let second = resolve_global_path(state, second_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, second, Val::Bool(third)],
    )
}

fn build_function_handler_with_global_self_method_self_method_bool_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_self_method: &str,
    third_self_method: &str,
    fourth: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second_method, third_method, fourth = ...
            return function(self, ...)
                return fn(first, self[second_method](self), self[third_method](self), fourth)
            end
        "#,
        "template-inline-function-global-self-method-self-method-bool-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = resolve_global_path(state, first_arg_path);
    let second_method = create_string(state, second_self_method);
    let third_method = create_string(state, third_self_method);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            target,
            first,
            second_method,
            third_method,
            Val::Bool(fourth),
        ],
    )
}

fn build_function_handler_with_string_nil_nil_global_args(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    fourth: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, fourth = ...
            return function(self, ...)
                return fn(first, nil, nil, fourth)
            end
        "#,
        "template-inline-function-string-nil-nil-global-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    let fourth = resolve_global_path(state, fourth);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, fourth],
    )
}

fn build_function_handler_with_global_arg(
    state: &mut LuaState,
    function_name: &str,
    arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, resolved_arg = ...
            return function(self, ...)
                return fn(resolved_arg)
            end
        "#,
        TEMPLATE_INLINE_FUNCTION_GLOBAL_ARG,
    )?;
    let target = resolve_global_path(state, function_name);
    let arg = resolve_global_path(state, arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg],
    )
}

fn build_function_handler_with_two_global_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first_arg, second_arg = ...
            return function(self, ...)
                return fn(first_arg, second_arg)
            end
        "#,
        TEMPLATE_INLINE_FUNCTION_TWO_GLOBAL_ARGS,
    )?;
    let target = resolve_global_path(state, function_name);
    let first_arg = resolve_global_path(state, first_arg_path);
    let second_arg = resolve_global_path(state, second_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first_arg, second_arg],
    )
}

fn build_function_handler_with_global_and_self_value_arg(
    state: &mut LuaState,
    function_name: &str,
    global_arg_path: &str,
    pass_self_id: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        global_and_self_value_arg_template(pass_self_id),
        "template-inline-function-global-self-value-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let global_arg = resolve_global_path(state, global_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, global_arg],
    )
}

fn global_and_self_value_arg_template(pass_self_id: bool) -> &'static str {
    if pass_self_id {
        r#"
            local fn, global_arg = ...
            return function(self, ...)
                return fn(global_arg, self:GetID())
            end
        "#
    } else {
        r#"
            local fn, global_arg = ...
            return function(self, ...)
                return fn(global_arg, self)
            end
        "#
    }
}
