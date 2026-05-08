//! Simple-arg arms of the global-method family: LFG branch, plain method
//! call, and shapes with 0-1 value args past `(target, method)`.

use super::super::FastHandlerRef;
use super::super::load_template;
use super::{GlobalMethodMode, build_global_method_with_mode, call_global_method_builder};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_global_method_simple_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = try_build_global_method_basic_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = try_build_global_method_literal_arg_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    try_build_global_method_self_arg_variants(state, handler_ref)
}

fn try_build_global_method_basic_variants(
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
        _ => Ok(None),
    }
}

fn try_build_global_method_literal_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::GlobalMethodWithSelfStringArg {
            target_path,
            method_name,
            arg,
        } => build_global_method_with_self_string_arg_handler(state, target_path, method_name, arg)
            .map(Some),
        FastHandlerRef::GlobalMethodWithStringArg {
            target_path,
            method_name,
            arg,
        } => build_global_method_with_string_arg_handler(state, target_path, method_name, arg)
            .map(Some),
        FastHandlerRef::GlobalMethodWithGlobalArg {
            target_path,
            method_name,
            arg_path,
        } => build_global_method_with_global_handler(state, target_path, method_name, arg_path)
            .map(Some),
        _ => Ok(None),
    }
}

fn try_build_global_method_self_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::GlobalMethodWithSelfIdArg {
            target_path,
            method_name,
        } => build_global_method_with_self_id_handler(state, target_path, method_name).map(Some),
        FastHandlerRef::GlobalMethodWithSelfArg {
            target_path,
            method_name,
        } => build_global_method_with_self_arg_handler(state, target_path, method_name).map(Some),
        FastHandlerRef::GlobalMethodWithSelfFieldArg {
            target_path,
            method_name,
            field,
        } => build_global_method_with_self_field_handler(state, target_path, method_name, field)
            .map(Some),
        _ => Ok(None),
    }
}

const TEMPLATE_GET_LFG_MODE_BRANCH: &str = r#"
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
"#;

fn build_get_lfg_mode_branch_handler(
    state: &mut LuaState,
    category_path: &str,
    slot_path: Option<&str>,
    leave_function: &str,
    join_function: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        TEMPLATE_GET_LFG_MODE_BRANCH,
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

fn build_global_method_with_string_value_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg: &str,
    pass_self: bool,
) -> LuaResult<Val> {
    let literal_arg = create_string(state, arg);
    call_global_method_builder(
        state,
        target_path,
        method_name,
        global_method_string_value_template(pass_self),
        "template-global-method-string-value-handler",
        &[literal_arg],
    )
}

fn build_global_method_with_self_string_arg_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    build_global_method_with_string_value_handler(state, target_path, method_name, arg, true)
}

fn build_global_method_with_string_arg_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    build_global_method_with_string_value_handler(state, target_path, method_name, arg, false)
}

fn global_method_string_value_template(pass_self: bool) -> &'static str {
    if pass_self {
        TEMPLATE_GLOBAL_METHOD_WITH_SELF_STRING_ARG
    } else {
        TEMPLATE_GLOBAL_METHOD_WITH_STRING_ARG
    }
}

const TEMPLATE_GLOBAL_METHOD_WITH_SELF_STRING_ARG: &str = r#"
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
"#;

const TEMPLATE_GLOBAL_METHOD_WITH_STRING_ARG: &str = r#"
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
"#;

const TEMPLATE_GLOBAL_METHOD_WITH_GLOBAL_ARG: &str = r#"
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
"#;

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
        TEMPLATE_GLOBAL_METHOD_WITH_GLOBAL_ARG,
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

fn build_global_method_with_self_arg_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    build_global_method_with_mode(state, target_path, method_name, GlobalMethodMode::SelfArg)
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
