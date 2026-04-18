//! Handlers that write `self:GetChecked()` into a global field, then fire
//! one or more callbacks. All four templates share a `resolve_global(path)`
//! helper and the same `if target then ... end` / `if cb then cb() end` idiom.

use super::super::FastHandlerRef;
use super::instantiate_template_with_args;
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_checked_assignment_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let handler = try_build_checked_assignment_callback_variants(state, handler_ref)?
        .or(try_build_checked_assignment_two_callback_variant(
            state,
            handler_ref,
        )?)
        .or(try_build_checked_number_assignment_variant(
            state,
            handler_ref,
        )?);
    Ok(handler)
}

fn try_build_checked_assignment_callback_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::CheckedAssignmentThenCallbacks {
            target_path,
            field,
            on_change_function,
            on_sound_function,
        } => build_checked_assignment_then_callbacks_handler(
            state,
            target_path,
            field,
            on_change_function,
            on_sound_function,
        )
        .map(Some),
        FastHandlerRef::CheckedAssignments3ThenCallbacks {
            first_target_path,
            first_field,
            second_target_path,
            second_field,
            third_target_path,
            third_field,
            on_change_function,
            on_sound_function,
        } => build_checked_assignments3_then_callbacks_handler(
            state,
            first_target_path,
            first_field,
            second_target_path,
            second_field,
            third_target_path,
            third_field,
            on_change_function,
            on_sound_function,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn try_build_checked_assignment_two_callback_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::CheckedAssignmentThenTwoCallbacks {
            target_path,
            field,
            first_callback,
            second_callback,
            on_sound_function,
        } => build_checked_assignment_then_two_callbacks_handler(
            state,
            target_path,
            field,
            first_callback,
            second_callback,
            on_sound_function,
        )
        .map(Some),
        _ => Ok(None),
    }
}

fn try_build_checked_number_assignment_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::CheckedNumberAssignmentThenCallbacks {
            target_path,
            field,
            value,
            on_change_function,
            on_sound_function,
        } => build_checked_number_assignment_then_callbacks_handler(
            state,
            target_path,
            field,
            *value,
            on_change_function,
            on_sound_function,
        )
        .map(Some),
        _ => Ok(None),
    }
}

const TEMPLATE_CHECKED_ASSIGNMENT: &str = r#"
    local target_path, field_name, on_change, on_sound = ...
    local function resolve_global(path)
        local value = getfenv(0) or _G
        for segment in string.gmatch(path, "[^%.]+") do
            value = value and value[segment]
        end
        return value
    end
    return function(self, ...)
        local checked = self:GetChecked()
        local target = resolve_global(target_path)
        if target then
            target[field_name] = checked and true or false
        end
        if on_change then
            on_change()
        end
        if on_sound then
            on_sound(checked)
        end
    end
"#;

const TEMPLATE_CHECKED_ASSIGNMENTS3: &str = r#"
    local first_target_path, first_field, second_target_path, second_field,
        third_target_path, third_field, on_change, on_sound = ...
    local function resolve_global(path)
        local value = getfenv(0) or _G
        for segment in string.gmatch(path, "[^%.]+") do
            value = value and value[segment]
        end
        return value
    end
    return function(self, ...)
        local checked = self:GetChecked()
        local first_target = resolve_global(first_target_path)
        local second_target = resolve_global(second_target_path)
        local third_target = resolve_global(third_target_path)
        if first_target then
            first_target[first_field] = checked and true or false
        end
        if second_target then
            second_target[second_field] = checked and true or false
        end
        if third_target then
            third_target[third_field] = checked and true or false
        end
        if on_change then
            on_change()
        end
        if on_sound then
            on_sound(checked)
        end
    end
"#;

const TEMPLATE_CHECKED_ASSIGNMENT_TWO_CALLBACKS: &str = r#"
    local target_path, field_name, first_callback, second_callback, on_sound = ...
    local function resolve_global(path)
        local value = getfenv(0) or _G
        for segment in string.gmatch(path, "[^%.]+") do
            value = value and value[segment]
        end
        return value
    end
    return function(self, ...)
        local checked = self:GetChecked()
        local target = resolve_global(target_path)
        if target then
            target[field_name] = checked and true or false
        end
        if first_callback then
            first_callback()
        end
        if second_callback then
            second_callback()
        end
        if on_sound then
            on_sound(checked)
        end
    end
"#;

const TEMPLATE_CHECKED_NUMBER_ASSIGNMENT: &str = r#"
    local target_path, field_name, value, on_change, on_sound = ...
    local function resolve_global(path)
        local target = getfenv(0) or _G
        for segment in string.gmatch(path, "[^%.]+") do
            target = target and target[segment]
        end
        return target
    end
    return function(self, ...)
        local checked = self:GetChecked()
        local target = resolve_global(target_path)
        if target then
            target[field_name] = value
        end
        if on_change then
            on_change()
        end
        if on_sound then
            on_sound(checked)
        end
    end
"#;

fn build_checked_assignment_then_callbacks_handler(
    state: &mut LuaState,
    target_path: &str,
    field: &str,
    on_change_function: &str,
    on_sound_function: &str,
) -> LuaResult<Val> {
    let target_path = create_string(state, target_path);
    let field_name = create_string(state, field);
    let on_change = resolve_global_path(state, on_change_function);
    let on_sound = resolve_global_path(state, on_sound_function);
    instantiate_template_with_args(
        state,
        TEMPLATE_CHECKED_ASSIGNMENT,
        "template-inline-checked-assignment-then-callbacks",
        &[target_path, field_name, on_change, on_sound],
    )
}

fn build_checked_assignments3_then_callbacks_handler(
    state: &mut LuaState,
    first_target_path: &str,
    first_field: &str,
    second_target_path: &str,
    second_field: &str,
    third_target_path: &str,
    third_field: &str,
    on_change_function: &str,
    on_sound_function: &str,
) -> LuaResult<Val> {
    let first_target_path = create_string(state, first_target_path);
    let first_field = create_string(state, first_field);
    let second_target_path = create_string(state, second_target_path);
    let second_field = create_string(state, second_field);
    let third_target_path = create_string(state, third_target_path);
    let third_field = create_string(state, third_field);
    let on_change = resolve_global_path(state, on_change_function);
    let on_sound = resolve_global_path(state, on_sound_function);
    instantiate_template_with_args(
        state,
        TEMPLATE_CHECKED_ASSIGNMENTS3,
        "template-inline-checked-assignments3-then-callbacks",
        &[
            first_target_path,
            first_field,
            second_target_path,
            second_field,
            third_target_path,
            third_field,
            on_change,
            on_sound,
        ],
    )
}

fn build_checked_assignment_then_two_callbacks_handler(
    state: &mut LuaState,
    target_path: &str,
    field: &str,
    first_callback: &str,
    second_callback: &str,
    on_sound_function: &str,
) -> LuaResult<Val> {
    let target_path = create_string(state, target_path);
    let field_name = create_string(state, field);
    let first_callback = resolve_global_path(state, first_callback);
    let second_callback = resolve_global_path(state, second_callback);
    let on_sound = resolve_global_path(state, on_sound_function);
    instantiate_template_with_args(
        state,
        TEMPLATE_CHECKED_ASSIGNMENT_TWO_CALLBACKS,
        "template-inline-checked-assignment-two-callbacks",
        &[
            target_path,
            field_name,
            first_callback,
            second_callback,
            on_sound,
        ],
    )
}

fn build_checked_number_assignment_then_callbacks_handler(
    state: &mut LuaState,
    target_path: &str,
    field: &str,
    value: f64,
    on_change_function: &str,
    on_sound_function: &str,
) -> LuaResult<Val> {
    let target_path = create_string(state, target_path);
    let field_name = create_string(state, field);
    let on_change = resolve_global_path(state, on_change_function);
    let on_sound = resolve_global_path(state, on_sound_function);
    instantiate_template_with_args(
        state,
        TEMPLATE_CHECKED_NUMBER_ASSIGNMENT,
        "template-inline-checked-number-assignment-then-callbacks",
        &[
            target_path,
            field_name,
            Val::Num(value),
            on_change,
            on_sound,
        ],
    )
}
