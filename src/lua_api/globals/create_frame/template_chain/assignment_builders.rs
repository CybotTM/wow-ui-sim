use super::load_template;
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::globals::create_frame::template_chain::FastLiteralValue;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_set_frame_level_from_parent_handler(
    state: &mut LuaState,
    delta: i32,
) -> LuaResult<Val> {
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

pub(super) fn build_ancestor_assignment_handler(
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

pub(in crate::lua_api::globals::create_frame::template_chain) fn build_assignment_handler(
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

pub(super) fn build_global_assignment_handler(
    state: &mut LuaState,
    target_path: &str,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target, field_name, assigned_value = ...
            return function(self, ...)
                if not target then
                    return
                end
                target[field_name] = assigned_value
            end
        "#,
        "template-inline-global-assignment",
    )?;
    let target = resolve_global_path(state, target_path);
    let field_name = create_string(state, field);
    let assigned_value = fast_literal_value(state, value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, field_name, assigned_value],
    )
}

pub(super) fn build_nested_assignment_handler(
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

pub(super) fn build_nested_global_pair_table_assignment_handler(
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

pub(super) fn build_parent_assignment_handler(
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
