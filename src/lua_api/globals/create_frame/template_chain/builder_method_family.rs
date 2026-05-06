use super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[path = "builder_method_family/direct_method.rs"]
mod direct_method;

pub(super) fn build_method_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = direct_method::build_direct_method_variants(state, handler_ref)? {
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

/// `self[field][method](target, ...)` shapes that cover the child-field
/// method bindings used by XML templates.
fn build_self_field_method_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = build_simple_self_field_method_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    build_multi_arg_self_field_method_variant(state, handler_ref)
}

fn build_simple_self_field_method_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = build_literal_self_field_method_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    build_reference_self_field_method_variant(state, handler_ref)
}

fn build_literal_self_field_method_variant(
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
        _ => Ok(None),
    }
}

fn build_reference_self_field_method_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
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
        _ => Ok(None),
    }
}

fn build_multi_arg_self_field_method_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = build_string_number_self_field_method_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    build_string_self_string_number_self_field_method_variant(state, handler_ref)
}

fn build_string_number_self_field_method_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let FastHandlerRef::SelfFieldMethodWithStringNumberNumberArgs {
        field,
        method_name,
        first,
        second,
        third,
    } = handler_ref
    else {
        return Ok(None);
    };

    build_self_field_method_with_string_number_number_args_handler(
        state,
        field,
        method_name,
        first,
        *second,
        *third,
    )
    .map(Some)
}

fn build_string_self_string_number_self_field_method_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let FastHandlerRef::SelfFieldMethodWithStringSelfStringNumberNumberArgs {
        field,
        method_name,
        first,
        third,
        fourth,
        fifth,
    } = handler_ref
    else {
        return Ok(None);
    };

    build_self_field_method_with_string_self_string_number_number_args_handler(
        state,
        field,
        method_name,
        first,
        third,
        *fourth,
        *fifth,
    )
    .map(Some)
}

/// `parent[method](...)` / `grandparent[method](...)` shapes.
fn build_ancestor_method_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::ParentFieldLocalToggleShown { field } => {
            build_parent_field_local_toggle_shown_handler(state, field).map(Some)
        }
        FastHandlerRef::ParentFieldLocalClickIfEnabled { field } => {
            build_parent_field_local_click_if_enabled_handler(state, field).map(Some)
        }
        FastHandlerRef::ParentMethod(method_name) => {
            build_parent_method_handler(state, method_name).map(Some)
        }
        FastHandlerRef::ParentMethodWithStringArg { method_name, arg } => {
            build_parent_method_with_string_arg_handler(state, method_name, arg).map(Some)
        }
        FastHandlerRef::ParentFieldMethodWithSelfNoArgMethodResult {
            field,
            method_name,
            self_method_name,
        } => build_parent_field_method_with_self_noarg_method_result_handler(
            state,
            field,
            method_name,
            self_method_name,
        )
        .map(Some),
        FastHandlerRef::GrandparentFieldMethod { field, method_name } => {
            build_grandparent_field_method_handler(state, field, method_name).map(Some)
        }
        FastHandlerRef::GrandparentMethodWithNotSelfCheckedArg { method_name } => {
            build_grandparent_method_with_not_self_checked_arg_handler(state, method_name).map(Some)
        }
        FastHandlerRef::GrandparentMethod(method_name) => {
            build_ancestor_method_handler(state, method_name, 2).map(Some)
        }
        _ => Ok(None),
    }
}

fn build_parent_field_local_toggle_shown_handler(
    state: &mut LuaState,
    field: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                local infoFrame = parent[field_name]
                if not infoFrame then
                    return
                end
                return infoFrame:SetShown(not infoFrame:IsShown())
            end
        "#,
        "template-parent-field-local-toggle-shown",
    )?;
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name],
    )
}

fn build_self_field_method_with_string_self_string_number_number_args_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
    first: &str,
    third: &str,
    fourth: f64,
    fifth: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name, first, third, fourth, fifth = ...
            return function(self, ...)
                local target = self[field_name]
                if not target or not target[method_name] then
                    return
                end
                return target[method_name](target, first, self, third, fourth, fifth)
            end
        "#,
        "template-self-field-method-string-self-string-number-number-args",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    let first = create_string(state, first);
    let third = create_string(state, third);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            field_name,
            method_name,
            first,
            third,
            Val::Num(fourth),
            Val::Num(fifth),
        ],
    )
}

fn build_parent_field_local_click_if_enabled_handler(
    state: &mut LuaState,
    field: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                local button = parent[field_name]
                if not button or not button:IsEnabled() then
                    return
                end
                local on_click = button:GetScript("OnClick")
                if on_click then
                    return on_click(button)
                end
            end
        "#,
        "template-parent-field-local-click-if-enabled",
    )?;
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name],
    )
}

fn build_grandparent_method_with_not_self_checked_arg_handler(
    state: &mut LuaState,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local method_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                local grandparent = parent and parent:GetParent()
                if not grandparent then
                    return
                end
                return grandparent[method_name](grandparent, not self:GetChecked())
            end
        "#,
        "template-grandparent-method-not-self-checked-arg",
    )?;
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name],
    )
}

fn build_grandparent_field_method_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                local grandparent = parent and parent:GetParent()
                local target = grandparent and grandparent[field_name]
                if not target then
                    return
                end
                return target[method_name](target, ...)
            end
        "#,
        "template-grandparent-field-method",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, method_name],
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

fn build_parent_field_method_with_self_noarg_method_result_handler(
    state: &mut LuaState,
    field: &str,
    method_name: &str,
    self_method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, method_name, self_method_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                local target = parent[field_name]
                if not target then
                    return
                end
                local value = self[self_method_name](self)
                return target[method_name](target, value)
            end
        "#,
        "template-parent-field-method-self-noarg-result-handler",
    )?;
    let field_name = create_string(state, field);
    let method_name = create_string(state, method_name);
    let self_method_name = create_string(state, self_method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, method_name, self_method_name],
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
