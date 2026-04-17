use super::{
    FastHandlerRef, FastLiteralValue, build_assignment_handler, build_chained_handler,
    load_template,
};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::globals::create_frame::template_chain::FastValueExpr;
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
        FastHandlerRef::GlobalMethodWithRuntimeArgs {
            target_path,
            method_name,
            args,
        } => build_global_method_with_runtime_args_handler(state, target_path, method_name, args)
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

fn build_global_method_with_runtime_args_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    args: &[FastValueExpr<'_>],
) -> LuaResult<Val> {
    let mut captured_args = Vec::with_capacity(args.len());
    for arg in args {
        captured_args.push(resolve_fast_value_expr(state, arg));
    }
    call_global_method_builder(
        state,
        target_path,
        method_name,
        r#"
            local target_ref, method_name, arg_count = ...
            local args = { select(4, ...) }
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
                return target[method_name](target, unpack(args, 1, arg_count))
            end
        "#,
        "template-global-method-runtime-args-handler",
        &{
            let mut args = Vec::with_capacity(1 + captured_args.len());
            args.push(Val::Num(args.len() as f64));
            args.extend(captured_args);
            args
        },
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

fn global_method_template(mode: GlobalMethodMode) -> (&'static str, &'static str) {
    match mode {
        GlobalMethodMode::Passthrough => (
            r#"
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
            "#,
            "template-global-method-handler",
        ),
        GlobalMethodMode::SelfId => (
            r#"
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

fn resolve_fast_value_expr(state: &mut LuaState, arg: &FastValueExpr<'_>) -> Val {
    match arg {
        FastValueExpr::String(value) => create_string(state, value),
        FastValueExpr::Literal(FastLiteralValue::Global(path)) => resolve_global_path(state, path),
        FastValueExpr::Literal(FastLiteralValue::Number(value)) => Val::Num(*value),
        FastValueExpr::Literal(FastLiteralValue::Bool(value)) => Val::Bool(*value),
        FastValueExpr::Literal(FastLiteralValue::Nil) => Val::Nil,
    }
}
