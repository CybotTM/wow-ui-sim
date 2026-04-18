//! Multi-arg arms of the global-method family: 3+ value args past
//! `(target, method)`, including function-result forwarding and
//! self-method/self-method shapes.

use super::super::FastHandlerRef;
use super::super::load_template;
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_global_method_multi_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::GlobalMethodWithSelfStringNumberNumberArgs {
            target_path,
            method_name,
            first,
            second,
            third,
        } => build_global_method_with_self_string_number_number_handler(
            state,
            target_path,
            method_name,
            first,
            *second,
            *third,
        )
        .map(Some),
        FastHandlerRef::GlobalMethodWithStringGlobalBoolArgs {
            target_path,
            method_name,
            first,
            second_arg_path,
            third,
        } => build_global_method_with_string_global_bool_args_handler(
            state,
            target_path,
            method_name,
            first,
            second_arg_path,
            *third,
        )
        .map(Some),
        FastHandlerRef::GlobalMethodWithGlobalThreeGlobalBoolArgs {
            target_path,
            method_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
            fourth_arg_path,
            fifth,
        } => build_global_method_with_global_three_global_bool_args_handler(
            state,
            target_path,
            method_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
            fourth_arg_path,
            *fifth,
        )
        .map(Some),
        FastHandlerRef::GlobalMethodWithGlobalNilNilNilNilBoolArgs {
            target_path,
            method_name,
            first_arg_path,
            sixth,
        } => build_global_method_with_global_nil_nil_nil_nil_bool_args_handler(
            state,
            target_path,
            method_name,
            first_arg_path,
            *sixth,
        )
        .map(Some),
        FastHandlerRef::GlobalMethodWithFourGlobalArgs {
            target_path,
            method_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
            fourth_arg_path,
        } => build_global_method_with_four_global_args_handler(
            state,
            target_path,
            method_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
            fourth_arg_path,
        )
        .map(Some),
        FastHandlerRef::GlobalMethodWithStringStringFunctionResultAndThreeNumberArgs {
            target_path,
            method_name,
            function_name,
            first,
            second,
            third,
            fourth,
            fifth,
        } => build_global_method_with_string_string_function_result_and_three_number_args_handler(
            state,
            target_path,
            method_name,
            function_name,
            first,
            second,
            *third,
            *fourth,
            *fifth,
        )
        .map(Some),
        FastHandlerRef::GlobalMethodWithGlobalStringFunctionResultAndThreeNumberArgs {
            target_path,
            method_name,
            function_name,
            first_arg_path,
            second,
            third,
            fourth,
            fifth,
        } => build_global_method_with_global_string_function_result_and_three_number_args_handler(
            state,
            target_path,
            method_name,
            function_name,
            first_arg_path,
            second,
            *third,
            *fourth,
            *fifth,
        )
        .map(Some),
        FastHandlerRef::GlobalMethodWithGlobalSelfMethodSelfMethodBoolArgs {
            target_path,
            method_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            fourth,
        } => build_global_method_with_global_self_method_self_method_bool_args_handler(
            state,
            target_path,
            method_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            *fourth,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn build_global_method_with_self_string_number_number_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    first: &str,
    second: f64,
    third: f64,
) -> LuaResult<Val> {
    let first = create_string(state, first);
    super::call_global_method_builder(
        state,
        target_path,
        method_name,
        r#"
            local target_ref, method_name, first, second, third = ...
            return function(self, ...)
                local target = target_ref
                if type(target) == "string" then
                    local env = getfenv(0) or _G
                    for segment in string.gmatch(target, "[^%.]+") do
                        env = env and env[segment]
                    end
                    target = env
                end
                if not target then
                    return
                end
                return target[method_name](target, self, first, second, third)
            end
        "#,
        "template-global-method-self-string-number-number-handler",
        &[first, Val::Num(second), Val::Num(third)],
    )
}

fn build_global_method_with_four_global_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
    third_arg_path: &str,
    fourth_arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, method_name, first, second, third, fourth = ...
            return function(self, ...)
                return target[method_name](target, first, second, third, fourth)
            end
        "#,
        "template-global-method-four-global-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let first = resolve_global_path(state, first_arg_path);
    let second = resolve_global_path(state, second_arg_path);
    let third = resolve_global_path(state, third_arg_path);
    let fourth = resolve_global_path(state, fourth_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name, first, second, third, fourth],
    )
}

fn build_global_method_with_string_global_bool_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    first: &str,
    second_arg_path: &str,
    third: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, method_name, first, second, third = ...
            return function(self, ...)
                return target[method_name](target, first, second, third)
            end
        "#,
        "template-global-method-string-global-bool-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let first = create_string(state, first);
    let second = resolve_global_path(state, second_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name, first, second, Val::Bool(third)],
    )
}

fn build_global_method_with_global_three_global_bool_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
    third_arg_path: &str,
    fourth_arg_path: &str,
    fifth: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, method_name, first, second, third, fourth, fifth = ...
            return function(self, ...)
                return target[method_name](target, first, second, third, fourth, fifth)
            end
        "#,
        "template-global-method-global-three-global-bool-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let first = resolve_global_path(state, first_arg_path);
    let second = resolve_global_path(state, second_arg_path);
    let third = resolve_global_path(state, third_arg_path);
    let fourth = resolve_global_path(state, fourth_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            target,
            method_name,
            first,
            second,
            third,
            fourth,
            Val::Bool(fifth),
        ],
    )
}

fn build_global_method_with_global_nil_nil_nil_nil_bool_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    first_arg_path: &str,
    sixth: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, method_name, first, sixth = ...
            return function(self, ...)
                return target[method_name](target, first, nil, nil, nil, nil, sixth)
            end
        "#,
        "template-global-method-global-nil-nil-nil-nil-bool-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let first = resolve_global_path(state, first_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name, first, Val::Bool(sixth)],
    )
}

fn build_global_method_with_global_self_method_self_method_bool_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    first_arg_path: &str,
    second_self_method: &str,
    third_self_method: &str,
    fourth: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, method_name, first, second_method, third_method, fourth = ...
            return function(self, ...)
                return target[method_name](
                    target,
                    first,
                    self[second_method](self),
                    self[third_method](self),
                    fourth
                )
            end
        "#,
        "template-global-method-global-self-method-self-method-bool-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let first = resolve_global_path(state, first_arg_path);
    let second_method = create_string(state, second_self_method);
    let third_method = create_string(state, third_self_method);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            target,
            method_name,
            first,
            second_method,
            third_method,
            Val::Bool(fourth),
        ],
    )
}

fn build_global_method_with_string_string_function_result_and_three_number_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    function_name: &str,
    first: &str,
    second: &str,
    third: f64,
    fourth: f64,
    fifth: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, method_name, fn, first, second, third, fourth, fifth = ...
            return function(self, ...)
                return target[method_name](target, fn(first, second), third, fourth, fifth)
            end
        "#,
        "template-global-method-string-string-function-result-three-number-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let function_name = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    let second = create_string(state, second);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            target,
            method_name,
            function_name,
            first,
            second,
            Val::Num(third),
            Val::Num(fourth),
            Val::Num(fifth),
        ],
    )
}

fn build_global_method_with_global_string_function_result_and_three_number_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    function_name: &str,
    first_arg_path: &str,
    second: &str,
    third: f64,
    fourth: f64,
    fifth: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, method_name, fn, first, second, third, fourth, fifth = ...
            return function(self, ...)
                return target[method_name](target, fn(first, second), third, fourth, fifth)
            end
        "#,
        "template-global-method-global-string-function-result-three-number-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let function_name = resolve_global_path(state, function_name);
    let first = resolve_global_path(state, first_arg_path);
    let second = create_string(state, second);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            target,
            method_name,
            function_name,
            first,
            second,
            Val::Num(third),
            Val::Num(fourth),
            Val::Num(fifth),
        ],
    )
}
