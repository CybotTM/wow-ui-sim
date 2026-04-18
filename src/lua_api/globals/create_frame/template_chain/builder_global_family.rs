use super::{
    FastHandlerRef, FastLiteralValue, build_assignment_handler, build_chained_handler,
    load_template,
};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_global_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = build_global_method_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_global_tooltip_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_global_misc_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    Ok(None)
}

/// LFG branch + the `target[method](...)` shapes (plain, with self/string/
/// global/self-id/self-field args).
fn build_global_method_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::GetLfgModeBranch {
            category_path,
            slot_path,
            leave_function,
            join_function,
        } => build_get_lfg_mode_branch_handler(
            state,
            category_path,
            *slot_path,
            leave_function,
            join_function,
        )
        .map(Some),
        FastHandlerRef::LocalGlobalPathConditionalMethod {
            target_path,
            method_name,
        } => build_local_global_path_conditional_method_handler(state, target_path, method_name)
            .map(Some),
        FastHandlerRef::GlobalMethod {
            target_path,
            method_name,
        } => build_global_method_handler(state, target_path, method_name).map(Some),
        FastHandlerRef::GlobalMethodWithSelfStringArg {
            target_path,
            method_name,
            arg,
        } => build_global_method_with_self_string_handler(state, target_path, method_name, arg)
            .map(Some),
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
        FastHandlerRef::GlobalMethodWithStringArg {
            target_path,
            method_name,
            arg,
        } => {
            build_global_method_with_string_handler(state, target_path, method_name, arg).map(Some)
        }
        FastHandlerRef::GlobalMethodWithGlobalArg {
            target_path,
            method_name,
            arg_path,
        } => build_global_method_with_global_handler(state, target_path, method_name, arg_path)
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
        FastHandlerRef::GlobalMethodWithSelfIdArg {
            target_path,
            method_name,
        } => build_global_method_with_self_id_handler(state, target_path, method_name).map(Some),
        FastHandlerRef::GlobalMethodWithSelfFieldArg {
            target_path,
            method_name,
            field,
        } => build_global_method_with_self_field_handler(state, target_path, method_name, field)
            .map(Some),
        _ => Ok(None),
    }
}

/// `GameTooltip:SetOwner(...)` + `:SetText(...)` shapes — both the
/// resolved-global and the literal-text/colour variants, plus the
/// conditional tooltip used by checkbox/option templates.
fn build_global_tooltip_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::GlobalTooltipSetOwnerThenSetText {
            target_path,
            anchor,
            text_path,
            red_path,
            green_path,
            blue_path,
            wrap,
        } => build_global_tooltip_set_owner_then_set_text_handler(
            state,
            target_path,
            anchor,
            text_path,
            red_path,
            green_path,
            blue_path,
            *wrap,
        )
        .map(Some),
        FastHandlerRef::GlobalTooltipSetOwnerThenSetTextLiteral {
            target_path,
            anchor,
            text,
            red,
            green,
            blue,
        } => build_global_tooltip_set_owner_then_set_text_literal_handler(
            state,
            target_path,
            anchor,
            text,
            *red,
            *green,
            *blue,
        )
        .map(Some),
        FastHandlerRef::ConditionalTooltip {
            target_path,
            field,
            anchor,
            red_path,
            green_path,
            blue_path,
        } => build_conditional_tooltip_handler(
            state,
            target_path,
            field,
            anchor,
            red_path,
            green_path,
            blue_path,
        )
        .map(Some),
        _ => Ok(None),
    }
}

/// Toggle visibility, suffix-named global methods, and "call method then
/// assign field" — the misc shapes that don't fit either the method or
/// tooltip family.
fn build_global_misc_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::ToggleGlobalVisibility { target_path } => {
            build_toggle_global_visibility_handler(state, target_path).map(Some)
        }
        FastHandlerRef::NamedGlobalMethodWithGlobalArg {
            suffix,
            method_name,
            arg_path,
        } => {
            build_named_global_method_with_global_arg_handler(state, suffix, method_name, arg_path)
                .map(Some)
        }
        FastHandlerRef::GlobalMethodThenAssignLiteral {
            target_path,
            method_name,
            field,
            value,
        } => {
            build_global_method_then_assign_handler(state, target_path, method_name, field, *value)
                .map(Some)
        }
        _ => Ok(None),
    }
}

fn build_local_global_path_conditional_method_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target_path, method_name = ...
            local function resolve_global(path)
                local value = getfenv(0) or _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local target = resolve_global(target_path)
                if target and target[method_name] then
                    return target[method_name](target)
                end
            end
        "#,
        "template-local-global-path-conditional-method-handler",
    )?;
    let target_path = create_string(state, target_path);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target_path, method_name],
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

fn build_get_lfg_mode_branch_handler(
    state: &mut LuaState,
    category_path: &str,
    slot_path: Option<&str>,
    leave_function: &str,
    join_function: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local category_path, slot_path, leave_fn, join_fn = ...
            local function resolve_global(path)
                local value = getfenv(0) or _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local category = resolve_global(category_path)
                local slot = slot_path ~= nil and resolve_global(slot_path) or nil
                local mode, subMode
                if slot_path ~= nil then
                    mode, subMode = GetLFGMode(category, slot)
                else
                    mode, subMode = GetLFGMode(category)
                end
                if mode == "queued" or mode == "listed" or mode == "rolecheck" or mode == "suspended" then
                    if slot_path ~= nil then
                        return leave_fn(category, slot)
                    end
                    return leave_fn(category)
                end
                return join_fn()
            end
        "#,
        "template-get-lfg-mode-branch-handler",
    )?;
    let category_path = create_string(state, category_path);
    let slot_path = slot_path
        .map(|path| create_string(state, path))
        .unwrap_or(Val::Nil);
    let leave_function = resolve_global_path(state, leave_function);
    let join_function = resolve_global_path(state, join_function);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[category_path, slot_path, leave_function, join_function],
    )
}

fn build_global_method_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    build_global_method_with_mode(
        state,
        target_path,
        method_name,
        GlobalMethodMode::Passthrough,
    )
}

fn build_global_method_with_self_string_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let literal_arg = create_string(state, arg);
    call_global_method_builder(
        state,
        target_path,
        method_name,
        r#"
            local target_ref, method_name, literal_arg = ...
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
                return target[method_name](target, self, literal_arg)
            end
        "#,
        "template-global-method-self-string-handler",
        &[literal_arg],
    )
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
    call_global_method_builder(
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

fn build_global_method_with_string_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let literal_arg = create_string(state, arg);
    call_global_method_builder(
        state,
        target_path,
        method_name,
        r#"
            local target_ref, method_name, literal_arg = ...
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
                return target[method_name](target, literal_arg)
            end
        "#,
        "template-global-method-string-handler",
        &[literal_arg],
    )
}

fn build_global_method_with_global_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg_path: &str,
) -> LuaResult<Val> {
    let arg_path = create_string(state, arg_path);
    call_global_method_builder(
        state,
        target_path,
        method_name,
        r#"
            local target_ref, method_name, arg_path = ...
            local function resolve_global(path)
                local value = getfenv(0) or _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
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
                return target[method_name](target, resolve_global(arg_path))
            end
        "#,
        "template-global-method-global-arg-handler",
        &[arg_path],
    )
}

fn build_global_method_with_self_id_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    build_global_method_with_mode(state, target_path, method_name, GlobalMethodMode::SelfId)
}

fn build_global_method_with_self_field_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    field: &str,
) -> LuaResult<Val> {
    let field_name = create_string(state, field);
    call_global_method_builder(
        state,
        target_path,
        method_name,
        r#"
            local target_ref, method_name, field_name = ...
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
                return target[method_name](target, self[field_name])
            end
        "#,
        "template-global-method-self-field-handler",
        &[field_name],
    )
}

fn build_global_method_then_assign_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let method = build_global_method_handler(state, target_path, method_name)?;
    let assign = build_assignment_handler(state, field, value)?;
    build_chained_handler(state, method, assign, "inline-global-method-assign", false)
}

fn build_conditional_tooltip_handler(
    state: &mut LuaState,
    target_path: &str,
    field: &str,
    anchor: &str,
    red_path: &str,
    green_path: &str,
    blue_path: &str,
) -> LuaResult<Val> {
    let field = create_string(state, field);
    let anchor = create_string(state, anchor);
    let red_path = create_string(state, red_path);
    let green_path = create_string(state, green_path);
    let blue_path = create_string(state, blue_path);
    call_global_method_builder(
        state,
        target_path,
        "SetText",
        r#"
            local target_ref, _ignored_method_name, field, anchor, red_path, green_path, blue_path = ...
            local function resolve_global(path)
                local value = _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local target = target_ref
                if type(target) == "string" then
                    target = resolve_global(target)
                end
                if not target then
                    return
                end
                local text = self[field]
                if not text then
                    return
                end
                target:SetOwner(self, anchor)
                return target:SetText(
                    text,
                    resolve_global(red_path),
                    resolve_global(green_path),
                    resolve_global(blue_path)
                )
            end
        "#,
        "template-conditional-tooltip-handler",
        &[field, anchor, red_path, green_path, blue_path],
    )
}

fn build_toggle_global_visibility_handler(
    state: &mut LuaState,
    target_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target_ref = ...
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
                if target:IsShown() then
                    return target:Hide()
                end
                return target:Show()
            end
        "#,
        "template-toggle-global-visibility-handler",
    )?;
    let target = resolve_global_path(state, target_path);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[target])
}

fn build_global_tooltip_set_owner_then_set_text_handler(
    state: &mut LuaState,
    target_path: &str,
    anchor: &str,
    text_path: &str,
    red_path: &str,
    green_path: &str,
    blue_path: &str,
    wrap: bool,
) -> LuaResult<Val> {
    let anchor = create_string(state, anchor);
    let text_path = create_string(state, text_path);
    let red_path = create_string(state, red_path);
    let green_path = create_string(state, green_path);
    let blue_path = create_string(state, blue_path);
    call_global_method_builder(
        state,
        target_path,
        "SetText",
        r#"
            local target_ref, _ignored_method_name, anchor, text_path, red_path, green_path, blue_path, wrap = ...
            local function resolve_global(path)
                local value = _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local target = target_ref
                if type(target) == "string" then
                    target = resolve_global(target)
                end
                if not target then
                    return
                end
                target:SetOwner(self, anchor)
                return target:SetText(
                    resolve_global(text_path),
                    resolve_global(red_path),
                    resolve_global(green_path),
                    resolve_global(blue_path),
                    nil,
                    wrap
                )
            end
        "#,
        "template-global-tooltip-settext-handler",
        &[
            anchor,
            text_path,
            red_path,
            green_path,
            blue_path,
            Val::Bool(wrap),
        ],
    )
}

fn build_global_tooltip_set_owner_then_set_text_literal_handler(
    state: &mut LuaState,
    target_path: &str,
    anchor: &str,
    text: &str,
    red: f64,
    green: f64,
    blue: f64,
) -> LuaResult<Val> {
    let anchor = create_string(state, anchor);
    let text = create_string(state, text);
    call_global_method_builder(
        state,
        target_path,
        "SetText",
        r#"
            local target_ref, _ignored_method_name, anchor, text, red, green, blue = ...
            local function resolve_global(path)
                local value = _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local target = target_ref
                if type(target) == "string" then
                    target = resolve_global(target)
                end
                if not target then
                    return
                end
                target:SetOwner(self, anchor)
                return target:SetText(text, red, green, blue)
            end
        "#,
        "template-global-tooltip-settext-literal-handler",
        &[anchor, text, Val::Num(red), Val::Num(green), Val::Num(blue)],
    )
}

fn build_named_global_method_with_global_arg_handler(
    state: &mut LuaState,
    suffix: &str,
    method_name: &str,
    arg_path: &str,
) -> LuaResult<Val> {
    let suffix = create_string(state, suffix);
    let method_name = create_string(state, method_name);
    let arg_path = create_string(state, arg_path);
    let builder = load_template(
        state,
        r#"
            local suffix, method_name, arg_path = ...
            local function resolve_global(path)
                local value = _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local target = _G[self:GetName() .. suffix]
                if not target then
                    return
                end
                return target[method_name](target, resolve_global(arg_path))
            end
        "#,
        "template-named-global-method-global-arg-handler",
    )?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[suffix, method_name, arg_path],
    )
}

enum GlobalMethodMode {
    Passthrough,
    SelfId,
}

fn build_global_method_with_mode(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    mode: GlobalMethodMode,
) -> LuaResult<Val> {
    let (source, tag) = global_method_template(mode);
    call_global_method_builder(state, target_path, method_name, source, tag, &[])
}

// ── Per-mode Lua global-method dispatch templates ────────────────────────────
//
// Each template closes over `target_ref` (either a pre-resolved value or
// a dotted path string) + `method_name`, then forwards either `...` or
// `self:GetID()` as the trailing argument shape.

const GLOBAL_METHOD_PASSTHROUGH_TEMPLATE: &str = r#"
    local target_ref, method_name = ...
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
        return target[method_name](target, ...)
    end
"#;

const GLOBAL_METHOD_SELF_ID_TEMPLATE: &str = r#"
    local target_ref, method_name = ...
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
        return target[method_name](target, self:GetID())
    end
"#;

fn global_method_template(mode: GlobalMethodMode) -> (&'static str, &'static str) {
    match mode {
        GlobalMethodMode::Passthrough => (
            GLOBAL_METHOD_PASSTHROUGH_TEMPLATE,
            "template-global-method-handler",
        ),
        GlobalMethodMode::SelfId => (
            GLOBAL_METHOD_SELF_ID_TEMPLATE,
            "template-global-method-self-id-handler",
        ),
    }
}

fn call_global_method_builder(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    source: &str,
    tag: &str,
    extra_args: &[Val],
) -> LuaResult<Val> {
    let builder = load_template(state, source, tag)?;
    let target = resolve_global_path(state, target_path);
    let mut args = Vec::with_capacity(2 + extra_args.len());
    args.push(if target == Val::Nil {
        create_string(state, target_path)
    } else {
        target
    });
    args.push(create_string(state, method_name));
    args.extend_from_slice(extra_args);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &args)
}
