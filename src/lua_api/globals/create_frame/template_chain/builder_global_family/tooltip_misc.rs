//! Tooltip-setup shapes (`GameTooltip:SetOwner` + `:SetText`) and the
//! miscellaneous remainder (toggle visibility, suffix-named global methods,
//! method-then-assign chains).

use super::super::{
    FastHandlerRef, FastLiteralValue, build_assignment_handler, build_chained_handler,
    load_template,
};
use super::call_global_method_builder;
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Tooltip group ──────────────────────────────────────────────────────────

/// `GameTooltip:SetOwner(...)` + `:SetText(...)` shapes — both the
/// resolved-global and the literal-text/colour variants, plus the
/// conditional tooltip used by checkbox/option templates.
pub(super) fn build_global_tooltip_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = try_global_tooltip_set_owner_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = try_global_tooltip_set_owner_literal_variant(state, handler_ref)? {
        return Ok(Some(result));
    }
    try_conditional_tooltip_variant(state, handler_ref)
}

fn try_global_tooltip_set_owner_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let FastHandlerRef::GlobalTooltipSetOwnerThenSetText {
        target_path,
        anchor,
        text_path,
        red_path,
        green_path,
        blue_path,
        wrap,
    } = handler_ref
    else {
        return Ok(None);
    };
    build_global_tooltip_set_owner_then_set_text_handler(
        state,
        target_path,
        anchor,
        text_path,
        red_path,
        green_path,
        blue_path,
        *wrap,
    )
    .map(Some)
}

fn try_global_tooltip_set_owner_literal_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let FastHandlerRef::GlobalTooltipSetOwnerThenSetTextLiteral {
        target_path,
        anchor,
        text,
        red,
        green,
        blue,
    } = handler_ref
    else {
        return Ok(None);
    };
    build_global_tooltip_set_owner_then_set_text_literal_handler(
        state,
        target_path,
        anchor,
        text,
        *red,
        *green,
        *blue,
    )
    .map(Some)
}

fn try_conditional_tooltip_variant(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    let FastHandlerRef::ConditionalTooltip {
        target_path,
        field,
        anchor,
        red_path,
        green_path,
        blue_path,
    } = handler_ref
    else {
        return Ok(None);
    };
    build_conditional_tooltip_handler(
        state,
        target_path,
        field,
        anchor,
        red_path,
        green_path,
        blue_path,
    )
    .map(Some)
}

const TEMPLATE_CONDITIONAL_TOOLTIP: &str = r#"
    local target_ref, _ignored_method_name, field, anchor, red_path, green_path, blue_path = ...
    local function resolve_global(path)
        local value = _G
        for segment in string.gmatch(path, "[^%.]+") do
            value = value and value[segment]
        end
        return value
    end
    return function(self, ...)
        local target = target_ref
        if type(target) == "string" then
            target = resolve_global(target)
        end
        if not target then
            return
        end
        local text = self[field]
        if not text then
            return
        end
        target:SetOwner(self, anchor)
        return target:SetText(
            text,
            resolve_global(red_path),
            resolve_global(green_path),
            resolve_global(blue_path)
        )
    end
"#;

fn build_conditional_tooltip_handler(
    state: &mut LuaState,
    target_path: &str,
    field: &str,
    anchor: &str,
    red_path: &str,
    green_path: &str,
    blue_path: &str,
) -> LuaResult<Val> {
    let field = create_string(state, field);
    let anchor = create_string(state, anchor);
    let red_path = create_string(state, red_path);
    let green_path = create_string(state, green_path);
    let blue_path = create_string(state, blue_path);
    call_global_method_builder(
        state,
        target_path,
        "SetText",
        TEMPLATE_CONDITIONAL_TOOLTIP,
        "template-conditional-tooltip-handler",
        &[field, anchor, red_path, green_path, blue_path],
    )
}

const TEMPLATE_GLOBAL_TOOLTIP_SET_OWNER_THEN_SET_TEXT: &str = r#"
    local target_ref, _ignored_method_name, anchor, text_path, red_path, green_path, blue_path, wrap = ...
    local function resolve_global(path)
        local value = _G
        for segment in string.gmatch(path, "[^%.]+") do
            value = value and value[segment]
        end
        return value
    end
    return function(self, ...)
        local target = target_ref
        if type(target) == "string" then
            target = resolve_global(target)
        end
        if not target then
            return
        end
        target:SetOwner(self, anchor)
        return target:SetText(
            resolve_global(text_path),
            resolve_global(red_path),
            resolve_global(green_path),
            resolve_global(blue_path),
            nil,
            wrap
        )
    end
"#;

fn build_global_tooltip_set_owner_then_set_text_handler(
    state: &mut LuaState,
    target_path: &str,
    anchor: &str,
    text_path: &str,
    red_path: &str,
    green_path: &str,
    blue_path: &str,
    wrap: bool,
) -> LuaResult<Val> {
    let anchor = create_string(state, anchor);
    let text_path = create_string(state, text_path);
    let red_path = create_string(state, red_path);
    let green_path = create_string(state, green_path);
    let blue_path = create_string(state, blue_path);
    let args = [anchor, text_path, red_path, green_path, blue_path, Val::Bool(wrap)];
    call_global_method_builder(
        state,
        target_path,
        "SetText",
        TEMPLATE_GLOBAL_TOOLTIP_SET_OWNER_THEN_SET_TEXT,
        "template-global-tooltip-settext-handler",
        &args,
    )
}

fn build_global_tooltip_set_owner_then_set_text_literal_handler(
    state: &mut LuaState,
    target_path: &str,
    anchor: &str,
    text: &str,
    red: f64,
    green: f64,
    blue: f64,
) -> LuaResult<Val> {
    let anchor = create_string(state, anchor);
    let text = create_string(state, text);
    call_global_method_builder(
        state,
        target_path,
        "SetText",
        r#"
            local target_ref, _ignored_method_name, anchor, text, red, green, blue = ...
            local function resolve_global(path)
                local value = _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local target = target_ref
                if type(target) == "string" then
                    target = resolve_global(target)
                end
                if not target then
                    return
                end
                target:SetOwner(self, anchor)
                return target:SetText(text, red, green, blue)
            end
        "#,
        "template-global-tooltip-settext-literal-handler",
        &[anchor, text, Val::Num(red), Val::Num(green), Val::Num(blue)],
    )
}

// ── Misc group ─────────────────────────────────────────────────────────────

/// Toggle visibility, suffix-named global methods, and "call method then
/// assign field" — the misc shapes that don't fit either the method or
/// tooltip family.
pub(super) fn build_global_misc_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::ToggleGlobalVisibility { target_path } => {
            build_toggle_global_visibility_handler(state, target_path).map(Some)
        }
        FastHandlerRef::NamedGlobalMethodWithGlobalArg {
            suffix,
            method_name,
            arg_path,
        } => {
            build_named_global_method_with_global_arg_handler(state, suffix, method_name, arg_path)
                .map(Some)
        }
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

fn build_toggle_global_visibility_handler(
    state: &mut LuaState,
    target_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local target_ref = ...
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
                if target:IsShown() then
                    return target:Hide()
                end
                return target:Show()
            end
        "#,
        "template-toggle-global-visibility-handler",
    )?;
    let target = resolve_global_path(state, target_path);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[target])
}

fn build_named_global_method_with_global_arg_handler(
    state: &mut LuaState,
    suffix: &str,
    method_name: &str,
    arg_path: &str,
) -> LuaResult<Val> {
    let suffix = create_string(state, suffix);
    let method_name = create_string(state, method_name);
    let arg_path = create_string(state, arg_path);
    let builder = load_template(
        state,
        r#"
            local suffix, method_name, arg_path = ...
            local function resolve_global(path)
                local value = _G
                for segment in string.gmatch(path, "[^%.]+") do
                    value = value and value[segment]
                end
                return value
            end
            return function(self, ...)
                local target = _G[self:GetName() .. suffix]
                if not target then
                    return
                end
                return target[method_name](target, resolve_global(arg_path))
            end
        "#,
        "template-named-global-method-global-arg-handler",
    )?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[suffix, method_name, arg_path],
    )
}

fn build_global_method_then_assign_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let method = super::build_global_method_with_mode(
        state,
        target_path,
        method_name,
        super::GlobalMethodMode::Passthrough,
    )?;
    let assign = build_assignment_handler(state, field, value)?;
    build_chained_handler(state, method, assign, "inline-global-method-assign", false)
}
