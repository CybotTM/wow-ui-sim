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
    if let Some(result) = try_build_global_method_string_arg_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = try_build_global_method_global_arg_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = try_build_global_method_function_result_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    try_build_global_method_self_method_variant(state, handler_ref)
}

fn try_build_global_method_string_arg_variants(
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
        _ => Ok(None),
    }
}

fn try_build_global_method_global_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = try_build_global_method_global_bool_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = try_build_global_method_global_nil_bool_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    try_build_global_method_four_global_variant(state, handler_ref)
}

fn try_build_global_method_global_bool_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
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
        _ => Ok(None),
    }
}

fn try_build_global_method_global_nil_bool_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
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
        _ => Ok(None),
    }
}

fn try_build_global_method_four_global_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
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
        _ => Ok(None),
    }
}

fn try_build_global_method_function_result_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let Some(variant) = function_result_variant(handler_ref) else {
        return Ok(None);
    };
    build_function_result_variant(state, variant)
}

fn function_result_variant<'a>(
    handler_ref: &'a FastHandlerRef<'a>,
) -> Option<FunctionResultVariant<'a>> {
    string_function_result_variant(handler_ref)
        .or_else(|| global_function_result_variant(handler_ref))
}

fn string_function_result_variant<'a>(
    handler_ref: &'a FastHandlerRef<'a>,
) -> Option<FunctionResultVariant<'a>> {
    let FastHandlerRef::GlobalMethodWithStringStringFunctionResultAndThreeNumberArgs {
        target_path,
        method_name,
        function_name,
        first,
        second,
        third,
        fourth,
        fifth,
    } = handler_ref
    else {
        return None;
    };
    Some(FunctionResultVariant::new(
        target_path,
        method_name,
        function_name,
        FunctionResultFirstArg::Literal(first),
        second,
        (*third, *fourth, *fifth),
    ))
}

fn global_function_result_variant<'a>(
    handler_ref: &'a FastHandlerRef<'a>,
) -> Option<FunctionResultVariant<'a>> {
    let FastHandlerRef::GlobalMethodWithGlobalStringFunctionResultAndThreeNumberArgs {
        target_path,
        method_name,
        function_name,
        first_arg_path,
        second,
        third,
        fourth,
        fifth,
    } = handler_ref
    else {
        return None;
    };
    Some(FunctionResultVariant::new(
        target_path,
        method_name,
        function_name,
        FunctionResultFirstArg::Global(first_arg_path),
        second,
        (*third, *fourth, *fifth),
    ))
}

struct FunctionResultVariant<'a> {
    target_path: &'a str,
    method_name: &'a str,
    function_name: &'a str,
    first_arg: FunctionResultFirstArg<'a>,
    second: &'a str,
    numbers: (f64, f64, f64),
}

impl<'a> FunctionResultVariant<'a> {
    fn new(
        target_path: &'a str,
        method_name: &'a str,
        function_name: &'a str,
        first_arg: FunctionResultFirstArg<'a>,
        second: &'a str,
        numbers: (f64, f64, f64),
    ) -> Self {
        Self {
            target_path,
            method_name,
            function_name,
            first_arg,
            second,
            numbers,
        }
    }
}

enum FunctionResultFirstArg<'a> {
    Literal(&'a str),
    Global(&'a str),
}

fn build_function_result_variant(
    state: &mut LuaState,
    variant: FunctionResultVariant<'_>,
) -> LuaResult<Option<Val>> {
    let first = resolve_function_result_first_arg(state, variant.first_arg);
    build_global_method_with_function_result_and_three_number_args_handler(
        state,
        variant.target_path,
        variant.method_name,
        variant.function_name,
        first,
        variant.second,
        variant.numbers.0,
        variant.numbers.1,
        variant.numbers.2,
    )
    .map(Some)
}

fn resolve_function_result_first_arg(
    state: &mut LuaState,
    first_arg: FunctionResultFirstArg<'_>,
) -> Val {
    match first_arg {
        FunctionResultFirstArg::Literal(value) => create_string(state, value),
        FunctionResultFirstArg::Global(path) => resolve_global_path(state, path),
    }
}

fn try_build_global_method_self_method_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
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
    let method_args = build_global_self_method_bool_args(
        state,
        target_path,
        method_name,
        first_arg_path,
        second_self_method,
        third_self_method,
        fourth,
    );
    let builder = load_template(
        state,
        global_self_method_bool_template(),
        "template-global-method-global-self-method-self-method-bool-args",
    )?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &method_args,
    )
}

fn build_global_self_method_bool_args(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    first_arg_path: &str,
    second_self_method: &str,
    third_self_method: &str,
    fourth: bool,
) -> [Val; 6] {
    [
        create_string(state, target_path),
        create_string(state, method_name),
        create_string(state, first_arg_path),
        create_string(state, second_self_method),
        create_string(state, third_self_method),
        Val::Bool(fourth),
    ]
}

fn global_self_method_bool_template() -> &'static str {
    r#"
        local target_path, method_name, first_path, second_method, third_method, fourth = ...
        local function resolve_global(path)
            local value = _G
            for segment in string.gmatch(path, "[^%.]+") do
                value = value and value[segment]
            end
            return value
        end
        return function(self, ...)
            local target = resolve_global(target_path)
            if not target then
                return
            end
            return target[method_name](
                target,
                resolve_global(first_path),
                self[second_method](self),
                self[third_method](self),
                fourth
            )
        end
    "#
}

fn build_global_method_with_function_result_and_three_number_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    function_name: &str,
    first: Val,
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
        "template-global-method-function-result-three-number-args",
    )?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let function_name = resolve_global_path(state, function_name);
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
