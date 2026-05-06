//! Handlers whose args are literal constants (strings, numbers) with no path
//! resolution.

use super::super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::hot_literals::{
    TEMPLATE_INLINE_FUNCTION_SELF_STRING, TEMPLATE_INLINE_FUNCTION_STRING_ARG,
};
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_literal_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let handler = try_build_string_number_variants(state, handler_ref)?
        .or(try_build_string_variants(state, handler_ref)?)
        .or(try_build_number_variants(state, handler_ref)?);
    Ok(handler)
}

fn try_build_string_number_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithStringNumberArgs {
            function_name,
            first,
            second,
        } => build_function_handler_with_string_number_args(state, function_name, first, *second)
            .map(Some),
        FastHandlerRef::FunctionWithStringSelfStringNumberNumberArgs {
            function_name,
            first,
            third,
            fourth,
            fifth,
        } => build_function_handler_with_string_self_string_number_number_args(
            state,
            function_name,
            first,
            third,
            *fourth,
            *fifth,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn try_build_string_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithStringArg { function_name, arg } => {
            build_function_handler_with_string_only_arg(state, function_name, arg).map(Some)
        }
        FastHandlerRef::FunctionWithSelfStringArg { function_name, arg } => {
            build_function_handler_with_string_arg(state, function_name, arg).map(Some)
        }
        _ => Ok(None),
    }
}

fn try_build_number_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithSelfNumberArg {
            function_name,
            value,
        } => build_function_handler_with_self_number_arg(state, function_name, *value).map(Some),
        FastHandlerRef::FunctionWithNumberArg {
            function_name,
            value,
        } => build_function_handler_with_number_arg(state, function_name, *value).map(Some),
        _ => Ok(None),
    }
}

fn build_function_handler_with_string_arg(
    state: &mut LuaState,
    function_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    build_function_handler_with_string_value_arg(state, function_name, arg, true)
}

fn build_function_handler_with_string_only_arg(
    state: &mut LuaState,
    function_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    build_function_handler_with_string_value_arg(state, function_name, arg, false)
}

fn build_function_handler_with_string_value_arg(
    state: &mut LuaState,
    function_name: &str,
    arg: &str,
    pass_self: bool,
) -> LuaResult<Val> {
    let (template, template_name) = string_value_arg_template(pass_self);
    let builder = load_template(state, template, template_name)?;
    let target = resolve_global_path(state, function_name);
    let arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg],
    )
}

fn string_value_arg_template(pass_self: bool) -> (&'static str, &'static str) {
    if pass_self {
        (
            r#"
                local fn, literal_arg = ...
                return function(self, ...)
                    return fn(self, literal_arg)
                end
            "#,
            TEMPLATE_INLINE_FUNCTION_SELF_STRING,
        )
    } else {
        (
            r#"
                local fn, literal_arg = ...
                return function(self, ...)
                    return fn(literal_arg)
                end
            "#,
            TEMPLATE_INLINE_FUNCTION_STRING_ARG,
        )
    }
}

fn build_function_handler_with_string_number_args(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    second: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second = ...
            return function(self, ...)
                return fn(first, second)
            end
        "#,
        "template-inline-function-string-number-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, Val::Num(second)],
    )
}

fn build_function_handler_with_string_self_string_number_number_args(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    third: &str,
    fourth: f64,
    fifth: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, third, fourth, fifth = ...
            return function(self, ...)
                return fn(first, self, third, fourth, fifth)
            end
        "#,
        "template-inline-function-string-self-string-number-number-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    let third = create_string(state, third);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, third, Val::Num(fourth), Val::Num(fifth)],
    )
}

fn build_function_handler_with_number_arg(
    state: &mut LuaState,
    function_name: &str,
    value: f64,
) -> LuaResult<Val> {
    build_function_handler_with_number_value_arg(state, function_name, value, false)
}

fn build_function_handler_with_self_number_arg(
    state: &mut LuaState,
    function_name: &str,
    value: f64,
) -> LuaResult<Val> {
    build_function_handler_with_number_value_arg(state, function_name, value, true)
}

fn build_function_handler_with_number_value_arg(
    state: &mut LuaState,
    function_name: &str,
    value: f64,
    pass_self: bool,
) -> LuaResult<Val> {
    let (template, template_name) = number_value_arg_template(pass_self);
    let builder = load_template(state, template, template_name)?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(value)],
    )
}

fn number_value_arg_template(pass_self: bool) -> (&'static str, &'static str) {
    if pass_self {
        (
            r#"
                local fn, number_arg = ...
                return function(self, ...)
                    return fn(self, number_arg)
                end
            "#,
            "template-inline-function-self-number-arg",
        )
    } else {
        (
            r#"
                local fn, number_arg = ...
                return function(self, ...)
                    return fn(number_arg)
                end
            "#,
            "template-inline-function-number-arg",
        )
    }
}
