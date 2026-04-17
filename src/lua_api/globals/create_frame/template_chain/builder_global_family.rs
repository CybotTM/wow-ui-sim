use super::{
    FastHandlerRef, FastLiteralValue, build_assignment_handler, build_chained_handler,
    load_template,
};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::globals::create_frame::template_chain::FastValueArg;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_global_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
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
        FastHandlerRef::GlobalMethodWithLiteralArgs {
            target_path,
            method_name,
            args,
        } => build_global_method_with_literal_args_handler(state, target_path, method_name, args)
            .map(Some),
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
            local target, method_name, literal_arg = ...
            return function(self, ...)
                return target[method_name](target, self, literal_arg)
            end
        "#,
        "template-global-method-self-string-handler",
        &[literal_arg],
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
            local target, method_name, field_name = ...
            return function(self, ...)
                return target[method_name](target, self[field_name])
            end
        "#,
        "template-global-method-self-field-handler",
        &[field_name],
    )
}

fn build_global_method_with_literal_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    args: &[FastValueArg<'_>],
) -> LuaResult<Val> {
    let mut captured_args = Vec::with_capacity(args.len());
    for arg in args {
        captured_args.push(resolve_fast_value_arg(state, arg));
    }
    call_global_method_builder(
        state,
        target_path,
        method_name,
        r#"
            local target, method_name = ...
            local args = {select(3, ...)}
            return function(self, ...)
                return target[method_name](target, unpack(args))
            end
        "#,
        "template-global-method-literal-args-handler",
        &captured_args,
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

fn global_method_template(mode: GlobalMethodMode) -> (&'static str, &'static str) {
    match mode {
        GlobalMethodMode::Passthrough => (
            r#"
                local target, method_name = ...
                return function(self, ...)
                    return target[method_name](target, ...)
                end
            "#,
            "template-global-method-handler",
        ),
        GlobalMethodMode::SelfId => (
            r#"
                local target, method_name = ...
                return function(self, ...)
                    return target[method_name](target, self:GetID())
                end
            "#,
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
    let mut args = Vec::with_capacity(2 + extra_args.len());
    args.push(resolve_global_path(state, target_path));
    args.push(create_string(state, method_name));
    args.extend_from_slice(extra_args);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &args)
}

fn resolve_fast_value_arg(state: &mut LuaState, arg: &FastValueArg<'_>) -> Val {
    match arg {
        FastValueArg::String(value) => create_string(state, value),
        FastValueArg::Literal(value) => match *value {
            FastLiteralValue::Global(path) => resolve_global_path(state, path),
            FastLiteralValue::Number(value) => Val::Num(value),
            FastLiteralValue::Nil => Val::Nil,
            FastLiteralValue::Bool(value) => Val::Bool(value),
        },
    }
}
