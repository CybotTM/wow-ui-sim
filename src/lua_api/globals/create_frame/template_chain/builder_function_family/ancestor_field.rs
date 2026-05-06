//! Handlers that read `:GetParent()` fields (possibly nested).

use super::super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_ancestor_field_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithParentFieldArg {
            function_name,
            field,
        } => build_function_handler_with_parent_field_arg(state, function_name, field).map(Some),
        FastHandlerRef::FunctionWithParentFieldAndNestedParentFieldMethodResult {
            function_name,
            first_field,
            second_field,
            third_field,
            method_name,
        } => build_function_handler_with_parent_field_and_nested_parent_field_method_result(
            state,
            function_name,
            first_field,
            second_field,
            third_field,
            method_name,
        )
        .map(Some),
        FastHandlerRef::FunctionWithSelfAndParentFieldArg {
            function_name,
            field,
        } => build_function_handler_with_self_and_parent_field_arg(state, function_name, field)
            .map(Some),
        _ => Ok(None),
    }
}

fn build_function_handler_with_parent_field_arg(
    state: &mut LuaState,
    function_name: &str,
    field: &str,
) -> LuaResult<Val> {
    build_function_handler_with_ancestor_field_arg(state, function_name, field, false)
}

fn build_function_handler_with_parent_field_and_nested_parent_field_method_result(
    state: &mut LuaState,
    function_name: &str,
    first_field: &str,
    second_field: &str,
    third_field: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first_field, second_field, third_field, method_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                local second = parent[second_field]
                local third = second and second[third_field]
                if not third or not third[method_name] then
                    return
                end
                return fn(parent[first_field], third[method_name](third))
            end
        "#,
        "template-inline-function-parent-field-nested-parent-method-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let first_field = create_string(state, first_field);
    let second_field = create_string(state, second_field);
    let third_field = create_string(state, third_field);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first_field, second_field, third_field, method_name],
    )
}

fn build_function_handler_with_self_and_parent_field_arg(
    state: &mut LuaState,
    function_name: &str,
    field: &str,
) -> LuaResult<Val> {
    build_function_handler_with_ancestor_field_arg(state, function_name, field, true)
}

fn build_function_handler_with_ancestor_field_arg(
    state: &mut LuaState,
    function_name: &str,
    field: &str,
    pass_self: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        ancestor_field_arg_template(pass_self),
        "template-inline-function-ancestor-field-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, field_name],
    )
}

fn ancestor_field_arg_template(pass_self: bool) -> &'static str {
    if pass_self {
        r#"
            local fn, field_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                return fn(self, parent[field_name])
            end
        "#
    } else {
        r#"
            local fn, field_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                return fn(parent[field_name])
            end
        "#
    }
}
