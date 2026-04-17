use super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_method_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = build_direct_method_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_self_field_method_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_ancestor_method_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    Ok(None)
}

/// `self[method](self, ...)` shapes.
fn build_direct_method_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::Method(method_name) => build_method_handler(state, method_name).map(Some),
        FastHandlerRef::MethodWithBoolArg { method_name, value } => {
            build_method_with_bool_arg_handler(state, method_name, *value).map(Some)
        }
        FastHandlerRef::MethodWithStringArg { method_name, arg } => {
            build_method_with_string_arg_handler(state, method_name, arg).map(Some)
        }
        _ => Ok(None),
    }
}

/// `self[field][method](target, ...)` shapes that cover the child-field
/// method bindings used by XML templates.
fn build_self_field_method_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::SelfFieldMethod { field, method_name } => {
            build_self_field_method_handler(state, field, method_name).map(Some)
        }
        FastHandlerRef::SelfFieldMethodWithStringArg {
            field,
            method_name,
            arg,
        } => build_self_field_method_with_string_arg_handler(state, field, method_name, arg)
            .map(Some),
        FastHandlerRef::SelfFieldMethodWithNumberArg {
            field,
            method_name,
            value,
        } => build_self_field_method_with_number_arg_handler(state, field, method_name, *value)
            .map(Some),
        FastHandlerRef::SelfFieldMethodWithGlobalArg {
            field,
            method_name,
            arg_path,
        } => build_self_field_method_with_global_arg_handler(state, field, method_name, arg_path)
            .map(Some),
        FastHandlerRef::SelfFieldMethodWithSelfFieldArg {
            field,
            method_name,
            arg_field,
        } => build_self_field_method_with_self_field_arg_handler(
            state,
            field,
            method_name,
            arg_field,
        )
        .map(Some),
        FastHandlerRef::SelfFieldMethodWithStringNumberNumberArgs {
            field,
            method_name,
            first,
            second,
            third,
        } => build_self_field_method_with_string_number_number_args_handler(
            state,
            field,
            method_name,
            first,
            *second,
            *third,
        )
        .map(Some),
        _ => Ok(None),
    }
}

/// `parent[method](...)` / `grandparent[method](...)` shapes.
fn build_ancestor_method_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::ParentMethod(method_name) => {
            build_parent_method_handler(state, method_name).map(Some)
        }
        FastHandlerRef::ParentMethodWithStringArg { method_name, arg } => {
            build_parent_method_with_string_arg_handler(state, method_name, arg).map(Some)
        }
        FastHandlerRef::GrandparentMethod(method_name) => {
            build_ancestor_method_handler(state, method_name, 2).map(Some)
        }
        _ => Ok(None),
    }
}

fn build_method_handler(state: &mut LuaState, method_name: &str) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local method_name = ...
            return function(self, ...)
                return self[method_name](self, ...)
            end
        "#,
        "template-method-handler",
    )?;
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name],
    )
}

fn build_method_with_bool_arg_handler(
    state: &mut LuaState,
    method_name: &str,
    value: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local method_name, value = ...
            return function(self, ...)
                return self[method_name](self, value)
            end
        "#,
        "template-method-bool-handler",
    )?;
    let method_name = create_string(state, method_name);
    let value = Val::Bool(value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name, value],
    )
}

fn build_method_with_string_arg_handler(
    state: &mut LuaState,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local method_name, literal_arg = ...
            return function(self, ...)
                return self[method_name](self, literal_arg)
            end
        "#,
        "template-method-string-handler",
    )?;
    let method_name = create_string(state, method_name);
    let literal_arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name, literal_arg],
    )
}

fn build_self_field_method_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, ...)
            end
        "#,
        "template-self-field-method-handler",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, method_name],
    )
}

fn build_self_field_method_with_string_arg_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name, literal_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, literal_arg)
            end
        "#,
        "template-self-field-method-string-handler",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    let literal_arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, method_name, literal_arg],
    )
}

fn build_self_field_method_with_number_arg_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
    value: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name, number_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, number_arg)
            end
        "#,
        "template-self-field-method-number-handler",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, method_name, Val::Num(value)],
    )
}

fn build_self_field_method_with_global_arg_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
    arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name, resolved_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, resolved_arg)
            end
        "#,
        "template-self-field-method-global-handler",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    let resolved_arg = resolve_global_path(state, arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, method_name, resolved_arg],
    )
}

fn build_self_field_method_with_self_field_arg_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
    arg_field: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name, arg_field_name = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, self[arg_field_name])
            end
        "#,
        "template-self-field-method-self-field-handler",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    let arg_field_name = create_string(state, arg_field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, method_name, arg_field_name],
    )
}

fn build_self_field_method_with_string_number_number_args_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
    first: &str,
    second: f64,
    third: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name, first_arg, second_arg, third_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, first_arg, second_arg, third_arg)
            end
        "#,
        "template-self-field-method-string-number-number-handler",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    let first_arg = create_string(state, first);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            field_name,
            method_name,
            first_arg,
            Val::Num(second),
            Val::Num(third),
        ],
    )
}

fn build_parent_method_handler(state: &mut LuaState, method_name: &str) -> LuaResult<Val> {
    build_ancestor_method_handler(state, method_name, 1)
}

fn build_parent_method_with_string_arg_handler(
    state: &mut LuaState,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local method_name, literal_arg = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                return parent[method_name](parent, literal_arg)
            end
        "#,
        "template-parent-method-string-handler",
    )?;
    let method_name = create_string(state, method_name);
    let literal_arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name, literal_arg],
    )
}

fn build_ancestor_method_handler(
    state: &mut LuaState,
    method_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local method_name, depth = ...
            return function(self, ...)
                local target = self
                for _ = 1, depth do
                    target = target and target:GetParent()
                end
                if not target then
                    return
                end
                return target[method_name](target, ...)
            end
        "#,
        "template-ancestor-method-handler",
    )?;
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name, Val::Num(depth as f64)],
    )
}
