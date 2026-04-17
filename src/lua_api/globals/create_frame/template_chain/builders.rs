use super::{FastHandlerRef, FastLiteralValue, FastScriptInstall};
#[path = "builder_function_family.rs"]
mod builder_function_family;
#[path = "builder_global_family.rs"]
mod builder_global_family;
#[path = "builder_method_family.rs"]
mod builder_method_family;

use self::builder_function_family::build_function_family_handler;
use self::builder_global_family::build_global_family_handler;
use self::builder_method_family::build_method_family_handler;
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::{create_string, frame_ref, table_set};
use crate::lua_api::script_helpers::{get_script, set_script};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Load a cached Lua chunk, converting loader errors into runtime errors.
pub(super) fn load_template(
    state: &mut LuaState,
    source: &str,
    tag: &str,
) -> LuaResult<rilua::Function> {
    crate::loader::chunk_cache::load_chunk(state, source, tag)
        .map_err(|error| rilua::runtime_error(error.to_string()))
}

fn build_register_for_clicks_handler(
    state: &mut LuaState,
    first: &str,
    second: Option<&str>,
    third: Option<&str>,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local first, second, third = ...
            return function(self, ...)
                return self:RegisterForClicks(first, second, third)
            end
        "#,
        "template-register-for-clicks-handler",
    )?;
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
    let builder = load_template(
        state,
        r#"
            local button = ...
            return function(self, ...)
                return self:RegisterForDrag(button)
            end
        "#,
        "template-register-for-drag-handler",
    )?;
    let button = create_string(state, button);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[button])
}

fn build_set_alpha_handler(state: &mut LuaState, alpha: f64) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local alpha = ...
            return function(self, ...)
                return self:SetAlpha(alpha)
            end
        "#,
        "template-set-alpha-handler",
    )?;
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
    if let Some(result) = try_family_dispatchers(state, &handler_ref)? {
        return Ok(result);
    }
    build_terminal_fast_handler(state, handler_ref)
}

/// Try each family dispatcher in turn; return `Some(outer)` on first
/// match (where the inner option encodes NoOp-style non-results).
fn try_family_dispatchers(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Option<Val>>> {
    if let Some(result) = build_sequence_fast_handler(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_method_family_handler(state, handler_ref)? {
        return Ok(Some(Some(result)));
    }
    if let Some(result) = build_global_family_handler(state, handler_ref)? {
        return Ok(Some(Some(result)));
    }
    if let Some(result) = build_function_family_handler(state, handler_ref)? {
        return Ok(Some(Some(result)));
    }
    Ok(None)
}

/// Final match for variants that don't belong to a family dispatcher
/// (click/drag registration, alpha/frame-level setters, assignments).
fn build_terminal_fast_handler(
    state: &mut LuaState,
    handler_ref: FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::NoOp => Ok(None),
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
        FastHandlerRef::AssignNestedGlobalPairTable {
            parent_field,
            field,
            first_path,
            second_path,
        } => build_nested_global_pair_table_assignment_handler(
            state,
            parent_field,
            field,
            first_path,
            second_path,
        )
        .map(Some),
        FastHandlerRef::AssignParentField { field, value } => {
            build_parent_assignment_handler(state, field, value).map(Some)
        }
        _ => unreachable!(
            "FastHandlerRef variant not dispatched by any family handler or the terminal match; \
             this is a bug — every variant must be handled"
        ),
    }
}

fn build_sequence_fast_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Option<Val>>> {
    match handler_ref {
        FastHandlerRef::Sequence2(parts) => {
            let (first_ref, second_ref) = &**parts;
            let first = build_fast_handler(state, first_ref.clone())?;
            let second = build_fast_handler(state, second_ref.clone())?;
            Ok(Some(chain_optional_handlers(state, first, second)?))
        }
        FastHandlerRef::Sequence3(parts) => {
            let (first_ref, second_ref, third_ref) = &**parts;
            let first = build_fast_handler(state, first_ref.clone())?;
            let second = build_fast_handler(state, second_ref.clone())?;
            let third = build_fast_handler(state, third_ref.clone())?;
            let first_pair = chain_optional_handlers(state, first, second)?;
            Ok(Some(chain_optional_handlers(state, first_pair, third)?))
        }
        _ => Ok(None),
    }
}

fn chain_optional_handlers(
    state: &mut LuaState,
    first: Option<Val>,
    second: Option<Val>,
) -> LuaResult<Option<Val>> {
    match (first, second) {
        (Some(first), Some(second)) => {
            build_chained_handler(state, first, second, "inline-sequence", false).map(Some)
        }
        (Some(first), None) => Ok(Some(first)),
        (None, Some(second)) => Ok(Some(second)),
        (None, None) => Ok(None),
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

pub(super) fn build_chained_handler(
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
    // Root both handlers before any allocation so the returned closure cannot
    // capture a dangling function ref in its upvalues.
    let stack_slot = root_vals_on_stack(state, first, second);
    let result = invoke_chained_template(state, handler_name, first, second);
    state.top = stack_slot;
    result
}

/// Push two values onto the Lua stack to root them for GC safety.
/// Returns the original stack top so the caller can restore it.
fn root_vals_on_stack(state: &mut LuaState, first: Val, second: Val) -> usize {
    let stack_slot = state.top;
    state.ensure_stack(stack_slot + 2);
    state.stack_set(stack_slot, first);
    state.stack_set(stack_slot + 1, second);
    state.top = stack_slot + 2;
    stack_slot
}

/// Load the chained-handler template and call it with `(handler_name, first, second)`.
fn invoke_chained_template(
    state: &mut LuaState,
    handler_name: &str,
    first: Val,
    second: Val,
) -> LuaResult<Val> {
    let (source, tag) = chained_handler_template();
    let builder = load_template(state, source, tag)?;
    let handler_name = create_string(state, handler_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[handler_name, first, second],
    )
}

fn chained_handler_template() -> (&'static str, &'static str) {
    (
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
}

fn build_set_frame_level_from_parent_handler(state: &mut LuaState, delta: i32) -> LuaResult<Val> {
    let builder = load_template(
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
    )?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[Val::Num(delta as f64)],
    )
}

fn build_ancestor_assignment_handler(
    state: &mut LuaState,
    field: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = load_template(
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
    )?;
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, Val::Num(depth as f64)],
    )
}

pub(super) fn build_assignment_handler(
    state: &mut LuaState,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local field_name, assigned_value = ...
            return function(self, ...)
                self[field_name] = assigned_value
            end
        "#,
        "template-inline-assignment",
    )?;
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
    let builder = load_template(
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
    )?;
    let parent_field_name = create_string(state, parent_field);
    let field_name = create_string(state, field);
    let assigned_value = fast_literal_value(state, value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[parent_field_name, field_name, assigned_value],
    )
}

fn build_nested_global_pair_table_assignment_handler(
    state: &mut LuaState,
    parent_field: &str,
    field: &str,
    first_path: &str,
    second_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local parent_field_name, field_name, first_value, second_value = ...
            return function(self, ...)
                local target = self[parent_field_name]
                if not target then
                    return
                end
                target[field_name] = { first_value, second_value }
            end
        "#,
        "template-inline-nested-global-pair-table-assignment",
    )?;
    let parent_field_name = create_string(state, parent_field);
    let field_name = create_string(state, field);
    let first_value = resolve_global_path(state, first_path);
    let second_value = resolve_global_path(state, second_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[parent_field_name, field_name, first_value, second_value],
    )
}

fn build_parent_assignment_handler(
    state: &mut LuaState,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = load_template(
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
    )?;
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
