use super::{FastHandlerRef, FastLiteralValue, FastScriptInstall};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::{create_string, frame_ref, table_set};
use crate::lua_api::script_helpers::{get_script, set_script};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

fn build_method_handler(state: &mut LuaState, method_name: &str) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local method_name = ...
            return function(self, ...)
                return self[method_name](self, ...)
            end
        "#,
        "template-method-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local method_name, value = ...
            return function(self, ...)
                return self[method_name](self, value)
            end
        "#,
        "template-method-bool-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let method_name = create_string(state, method_name);
    let value = if value {
        Val::Bool(true)
    } else {
        Val::Bool(false)
    };
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local method_name, literal_arg = ...
            return function(self, ...)
                return self[method_name](self, literal_arg)
            end
        "#,
        "template-method-string-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, method_name = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, ...)
            end
        "#,
        "template-self-field-method-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, method_name, literal_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, literal_arg)
            end
        "#,
        "template-self-field-method-string-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, method_name, number_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, number_arg)
            end
        "#,
        "template-self-field-method-number-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, method_name, resolved_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, resolved_arg)
            end
        "#,
        "template-self-field-method-global-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, method_name, arg_field_name = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, self[arg_field_name])
            end
        "#,
        "template-self-field-method-self-field-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, method_name, first_arg, second_arg, third_arg = ...
            return function(self, ...)
                local target = self[field_name]
                return target[method_name](target, first_arg, second_arg, third_arg)
            end
        "#,
        "template-self-field-method-string-number-number-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local method_name, literal_arg = ...
            return function(self, ...)
                local target = self:GetParent()
                if not target then
                    return
                end
                return target[method_name](target, literal_arg)
            end
        "#,
        "template-parent-method-string-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
    let builder = crate::loader::chunk_cache::load_chunk(
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
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name, Val::Num(depth as f64)],
    )
}

fn build_global_method_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local target, method_name = ...
            return function(self, ...)
                if not target then
                    return
                end
                return target[method_name](target, ...)
            end
        "#,
        "template-global-method-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name],
    )
}

fn build_global_method_with_self_string_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local target, method_name, literal_arg = ...
            return function(self, ...)
                if not target then
                    return
                end
                return target[method_name](target, self, literal_arg)
            end
        "#,
        "template-global-method-self-string-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let literal_arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name, literal_arg],
    )
}

fn build_global_method_with_self_id_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local target, method_name = ...
            return function(self, ...)
                if not target then
                    return
                end
                return target[method_name](target, self:GetID())
            end
        "#,
        "template-global-method-self-id-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name],
    )
}

fn build_global_method_with_self_field_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    field: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local target, method_name, field_name = ...
            return function(self, ...)
                if not target then
                    return
                end
                return target[method_name](target, self[field_name])
            end
        "#,
        "template-global-method-self-field-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name, field_name],
    )
}

fn build_global_method_then_assign_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let call = build_global_method_handler(state, target_path, method_name)?;
    let assign = build_assignment_handler(state, field, value)?;
    build_chained_handler(state, call, assign, "template-global-method-assign", false)
}

fn build_register_for_clicks_handler(
    state: &mut LuaState,
    first: &str,
    second: Option<&str>,
    third: Option<&str>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local first, second, third = ...
            return function(self, ...)
                return self:RegisterForClicks(first, second, third)
            end
        "#,
        "template-register-for-clicks-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let first = create_string(state, first);
    let second = second
        .map(|value| create_string(state, value))
        .unwrap_or(Val::Nil);
    let third = third
        .map(|value| create_string(state, value))
        .unwrap_or(Val::Nil);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[first, second, third],
    )
}

fn build_register_for_drag_handler(state: &mut LuaState, button: &str) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local button = ...
            return function(self, ...)
                return self:RegisterForDrag(button)
            end
        "#,
        "template-register-for-drag-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let button = create_string(state, button);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[button])
}

fn build_set_alpha_handler(state: &mut LuaState, alpha: f64) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local alpha = ...
            return function(self, ...)
                return self:SetAlpha(alpha)
            end
        "#,
        "template-set-alpha-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[Val::Num(alpha)],
    )
}

pub(super) fn build_fast_handler(
    state: &mut LuaState,
    handler_ref: FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::NoOp => Ok(None),
        FastHandlerRef::Sequence2(parts) => {
            let (first_ref, second_ref) = &*parts;
            let first = build_fast_handler(state, first_ref.clone())?;
            let second = build_fast_handler(state, second_ref.clone())?;
            match (first, second) {
                (Some(first), Some(second)) => {
                    build_chained_handler(state, first, second, "inline-sequence", false).map(Some)
                }
                (Some(first), None) => Ok(Some(first)),
                (None, Some(second)) => Ok(Some(second)),
                (None, None) => Ok(None),
            }
        }
        FastHandlerRef::Sequence3(parts) => {
            let (first_ref, second_ref, third_ref) = &*parts;
            let first = build_fast_handler(state, first_ref.clone())?;
            let second = build_fast_handler(state, second_ref.clone())?;
            let third = build_fast_handler(state, third_ref.clone())?;
            match (first, second, third) {
                (Some(first), Some(second), Some(third)) => {
                    let chained =
                        build_chained_handler(state, first, second, "inline-sequence", false)?;
                    build_chained_handler(state, chained, third, "inline-sequence", false).map(Some)
                }
                (Some(first), Some(second), None) => {
                    build_chained_handler(state, first, second, "inline-sequence", false).map(Some)
                }
                (Some(first), None, Some(third)) => {
                    build_chained_handler(state, first, third, "inline-sequence", false).map(Some)
                }
                (None, Some(second), Some(third)) => {
                    build_chained_handler(state, second, third, "inline-sequence", false).map(Some)
                }
                (Some(first), None, None) => Ok(Some(first)),
                (None, Some(second), None) => Ok(Some(second)),
                (None, None, Some(third)) => Ok(Some(third)),
                (None, None, None) => Ok(None),
            }
        }
        FastHandlerRef::Method(method_name) => build_method_handler(state, method_name).map(Some),
        FastHandlerRef::MethodWithBoolArg { method_name, value } => {
            build_method_with_bool_arg_handler(state, method_name, value).map(Some)
        }
        FastHandlerRef::MethodWithStringArg { method_name, arg } => {
            build_method_with_string_arg_handler(state, method_name, arg).map(Some)
        }
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
        } => build_self_field_method_with_number_arg_handler(state, field, method_name, value)
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
            second,
            third,
        )
        .map(Some),
        FastHandlerRef::ParentMethod(method_name) => {
            build_parent_method_handler(state, method_name).map(Some)
        }
        FastHandlerRef::ParentMethodWithStringArg { method_name, arg } => {
            build_parent_method_with_string_arg_handler(state, method_name, arg).map(Some)
        }
        FastHandlerRef::GrandparentMethod(method_name) => {
            build_ancestor_method_handler(state, method_name, 2).map(Some)
        }
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
        FastHandlerRef::GlobalMethodThenAssignLiteral {
            target_path,
            method_name,
            field,
            value,
        } => build_global_method_then_assign_handler(state, target_path, method_name, field, value)
            .map(Some),
        FastHandlerRef::Function(function_name) => {
            Ok(Some(resolve_global_path(state, function_name)))
        }
        FastHandlerRef::FunctionNoArgs(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::NoArgs).map(Some)
        }
        FastHandlerRef::FunctionWithSelfIdArg(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::SelfId).map(Some)
        }
        FastHandlerRef::FunctionWithSelfStringArg { function_name, arg } => {
            build_function_handler_with_string_arg(state, function_name, arg).map(Some)
        }
        FastHandlerRef::FunctionWithNumberArg {
            function_name,
            value,
        } => build_function_handler_with_number_arg(state, function_name, value).map(Some),
        FastHandlerRef::FunctionWithGlobalArg {
            function_name,
            arg_path,
        } => build_function_handler_with_global_arg(state, function_name, arg_path).map(Some),
        FastHandlerRef::FunctionWithGlobalAndSelfArg {
            function_name,
            global_arg_path,
        } => build_function_handler_with_global_and_self_arg(state, function_name, global_arg_path)
            .map(Some),
        FastHandlerRef::FunctionWithSelfAndParentFieldArg {
            function_name,
            field,
        } => build_function_handler_with_self_and_parent_field_arg(state, function_name, field)
            .map(Some),
        FastHandlerRef::FunctionWithParentArg(function_name) => {
            build_ancestor_function_handler(state, function_name, 1).map(Some)
        }
        FastHandlerRef::FunctionWithGrandparentArg(function_name) => {
            build_ancestor_function_handler(state, function_name, 2).map(Some)
        }
        FastHandlerRef::FunctionWithParentIdArg(function_name) => {
            build_ancestor_id_function_handler(state, function_name, 1).map(Some)
        }
        FastHandlerRef::FunctionWithEventVarargs(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::EventVarargs)
                .map(Some)
        }
        FastHandlerRef::FunctionWithButton(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::Button).map(Some)
        }
        FastHandlerRef::FunctionWithElapsed(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::Elapsed).map(Some)
        }
        FastHandlerRef::RegisterForClicks {
            first,
            second,
            third,
        } => build_register_for_clicks_handler(state, first, second, third).map(Some),
        FastHandlerRef::RegisterForDrag(button) => {
            build_register_for_drag_handler(state, button).map(Some)
        }
        FastHandlerRef::SetAlpha(alpha) => build_set_alpha_handler(state, alpha).map(Some),
        FastHandlerRef::SetFrameLevelFromParent(delta) => {
            build_set_frame_level_from_parent_handler(state, delta).map(Some)
        }
        FastHandlerRef::AssignAncestorRef { field, depth } => {
            build_ancestor_assignment_handler(state, field, depth).map(Some)
        }
        FastHandlerRef::AssignLiteral { field, value } => {
            build_assignment_handler(state, field, value).map(Some)
        }
        FastHandlerRef::AssignNestedLiteral {
            parent_field,
            field,
            value,
        } => build_nested_assignment_handler(state, parent_field, field, value).map(Some),
        FastHandlerRef::AssignParentField { field, value } => {
            build_parent_assignment_handler(state, field, value).map(Some)
        }
    }
}

pub(super) fn install_fast_handler(
    state: &mut LuaState,
    frame_id: u64,
    handler_name: &'static str,
    install: FastScriptInstall<'_>,
) -> LuaResult<()> {
    match install {
        FastScriptInstall::Set(handler_ref) => {
            if let Some(handler) = build_fast_handler(state, handler_ref)? {
                set_script(state, frame_id, handler_name, handler);
            }
        }
        FastScriptInstall::Intrinsic(handler_ref) => {
            let Some(handler) = build_fast_handler(state, handler_ref)? else {
                return Ok(());
            };
            let frame = frame_ref(state, frame_id)?;
            let intrinsic_name = format!("{handler_name}_Intrinsic");
            table_set(state, frame, &intrinsic_name, handler);
        }
        FastScriptInstall::Chain { handler, new_first } => {
            let Some(new_handler) = build_fast_handler(state, handler)? else {
                return Ok(());
            };
            let Some(old_handler) = get_script(state, frame_id, handler_name) else {
                set_script(state, frame_id, handler_name, new_handler);
                return Ok(());
            };
            let chained =
                build_chained_handler(state, old_handler, new_handler, handler_name, new_first)?;
            set_script(state, frame_id, handler_name, chained);
        }
    }
    Ok(())
}

fn build_chained_handler(
    state: &mut LuaState,
    old_handler: Val,
    new_handler: Val,
    handler_name: &str,
    new_first: bool,
) -> LuaResult<Val> {
    let (first, second) = if new_first {
        (new_handler, old_handler)
    } else {
        (old_handler, new_handler)
    };
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local handler_name, first, second = ...
            local report = debug.getregistry()["__report_script_error"]
            return function(self, ...)
                if securecall then
                    securecall(first, self, ...)
                    securecall(second, self, ...)
                else
                    local ok1, err1 = pcall(first, self, ...)
                    local ok2, err2 = pcall(second, self, ...)
                    if not ok1 then
                        local name = self.GetName and self:GetName() or "?"
                        report("[script:" .. handler_name .. "] " .. name .. ": " .. tostring(err1))
                    end
                    if not ok2 then
                        local name = self.GetName and self:GetName() or "?"
                        report("[script:" .. handler_name .. "] " .. name .. ": " .. tostring(err2))
                    end
                end
            end
        "#,
        "template-chained-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let handler_name = create_string(state, handler_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[handler_name, first, second],
    )
}

enum FunctionHandlerKind {
    NoArgs,
    SelfId,
    EventVarargs,
    Button,
    Elapsed,
}

fn build_function_handler(
    state: &mut LuaState,
    function_name: &str,
    kind: FunctionHandlerKind,
) -> LuaResult<Val> {
    let (source, tag) = match kind {
        FunctionHandlerKind::NoArgs => (
            r#"
                local fn = ...
                return function(self, ...)
                    return fn()
                end
            "#,
            "template-inline-function-noargs",
        ),
        FunctionHandlerKind::SelfId => (
            r#"
                local fn = ...
                return function(self, ...)
                    return fn(self:GetID())
                end
            "#,
            "template-inline-function-self-id",
        ),
        FunctionHandlerKind::EventVarargs => (
            r#"
                local fn = ...
                return function(self, event, ...)
                    return fn(self, event, ...)
                end
            "#,
            "template-inline-function-event-varargs",
        ),
        FunctionHandlerKind::Button => (
            r#"
                local fn = ...
                return function(self, button, ...)
                    return fn(self, button, ...)
                end
            "#,
            "template-inline-function-button",
        ),
        FunctionHandlerKind::Elapsed => (
            r#"
                local fn = ...
                return function(self, elapsed, ...)
                    return fn(self, elapsed, ...)
                end
            "#,
            "template-inline-function-elapsed",
        ),
    };
    let builder = crate::loader::chunk_cache::load_chunk(state, source, tag)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[target])
}

fn build_function_handler_with_string_arg(
    state: &mut LuaState,
    function_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, literal_arg = ...
            return function(self, ...)
                return fn(self, literal_arg)
            end
        "#,
        "template-inline-function-self-string",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    let arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg],
    )
}

fn build_function_handler_with_number_arg(
    state: &mut LuaState,
    function_name: &str,
    value: f64,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, number_arg = ...
            return function(self, ...)
                return fn(number_arg)
            end
        "#,
        "template-inline-function-number-arg",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(value)],
    )
}

fn build_function_handler_with_global_arg(
    state: &mut LuaState,
    function_name: &str,
    arg_path: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, resolved_arg = ...
            return function(self, ...)
                return fn(resolved_arg)
            end
        "#,
        "template-inline-function-global-arg",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    let arg = resolve_global_path(state, arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg],
    )
}

fn build_function_handler_with_global_and_self_arg(
    state: &mut LuaState,
    function_name: &str,
    global_arg_path: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, global_arg = ...
            return function(self, ...)
                return fn(global_arg, self)
            end
        "#,
        "template-inline-function-global-self-arg",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    let global_arg = resolve_global_path(state, global_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, global_arg],
    )
}

fn build_function_handler_with_self_and_parent_field_arg(
    state: &mut LuaState,
    function_name: &str,
    field: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, field_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                return fn(self, parent[field_name])
            end
        "#,
        "template-inline-function-self-parent-field-arg",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, field_name],
    )
}

fn build_ancestor_function_handler(
    state: &mut LuaState,
    function_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, depth = ...
            return function(self, ...)
                local target = self
                for _ = 1, depth do
                    target = target and target:GetParent()
                end
                if not target then
                    return
                end
                return fn(target)
            end
        "#,
        "template-inline-function-ancestor",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(depth as f64)],
    )
}

fn build_set_frame_level_from_parent_handler(state: &mut LuaState, delta: i32) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local delta = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                return self:SetFrameLevel(parent:GetFrameLevel() + delta)
            end
        "#,
        "template-set-frame-level-parent-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[Val::Num(delta as f64)],
    )
}

fn build_ancestor_id_function_handler(
    state: &mut LuaState,
    function_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, depth = ...
            return function(self, ...)
                local target = self
                for _ = 1, depth do
                    target = target and target:GetParent()
                end
                if not target then
                    return
                end
                return fn(target:GetID())
            end
        "#,
        "template-inline-function-ancestor-id",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(depth as f64)],
    )
}

fn build_ancestor_assignment_handler(
    state: &mut LuaState,
    field: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, depth = ...
            return function(self, ...)
                local target = self
                for _ = 1, depth do
                    target = target and target:GetParent()
                end
                self[field_name] = target
            end
        "#,
        "template-inline-ancestor-assignment",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, Val::Num(depth as f64)],
    )
}

fn build_assignment_handler(
    state: &mut LuaState,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, assigned_value = ...
            return function(self, ...)
                self[field_name] = assigned_value
            end
        "#,
        "template-inline-assignment",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let field_name = create_string(state, field);
    let assigned_value = fast_literal_value(state, value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, assigned_value],
    )
}

fn build_nested_assignment_handler(
    state: &mut LuaState,
    parent_field: &str,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local parent_field_name, field_name, assigned_value = ...
            return function(self, ...)
                local target = self[parent_field_name]
                if not target then
                    return
                end
                target[field_name] = assigned_value
            end
        "#,
        "template-inline-nested-assignment",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let parent_field_name = create_string(state, parent_field);
    let field_name = create_string(state, field);
    let assigned_value = fast_literal_value(state, value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[parent_field_name, field_name, assigned_value],
    )
}

fn build_parent_assignment_handler(
    state: &mut LuaState,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, assigned_value = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                parent[field_name] = assigned_value
            end
        "#,
        "template-parent-assignment",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let field_name = create_string(state, field);
    let assigned_value = fast_literal_value(state, value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, assigned_value],
    )
}

fn fast_literal_value(state: &mut LuaState, value: FastLiteralValue<'_>) -> Val {
    match value {
        FastLiteralValue::Global(path) => resolve_global_path(state, path),
        FastLiteralValue::Number(value) => Val::Num(value),
        FastLiteralValue::Nil => Val::Nil,
        FastLiteralValue::Bool(value) => Val::Bool(value),
    }
}
