use super::{
    FastHandlerRef, FastLiteralValue, build_assignment_handler, build_chained_handler,
    load_template,
};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
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
        FastHandlerRef::GlobalMethodWithStringArg {
            target_path,
            method_name,
            arg,
        } => {
            build_global_method_with_string_handler(state, target_path, method_name, arg).map(Some)
        }
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
        FastHandlerRef::GlobalTooltipSetOwnerThenSetText {
            target_path,
            anchor,
            text_path,
            red_path,
            green_path,
            blue_path,
            wrap,
        } => build_global_tooltip_set_owner_then_set_text_handler(
            state,
            target_path,
            anchor,
            text_path,
            red_path,
            green_path,
            blue_path,
            *wrap,
        )
        .map(Some),
        FastHandlerRef::GlobalTooltipSetOwnerThenSetTextLiteral {
            target_path,
            anchor,
            text,
            red,
            green,
            blue,
        } => build_global_tooltip_set_owner_then_set_text_literal_handler(
            state,
            target_path,
            anchor,
            text,
            *red,
            *green,
            *blue,
        )
        .map(Some),
        FastHandlerRef::ConditionalTooltip {
            target_path,
            field,
            anchor,
            red_path,
            green_path,
            blue_path,
        } => build_conditional_tooltip_handler(
            state,
            target_path,
            field,
            anchor,
            red_path,
            green_path,
            blue_path,
        )
        .map(Some),
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

fn build_global_method_with_string_handler(
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
                return target[method_name](target, literal_arg)
            end
        "#,
        "template-global-method-string-handler",
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
        r#"
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
        "#,
        "template-conditional-tooltip-handler",
        &[field, anchor, red_path, green_path, blue_path],
    )
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
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target],
    )
}

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
    call_global_method_builder(
        state,
        target_path,
        "SetText",
        r#"
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
        "#,
        "template-global-tooltip-settext-handler",
        &[
            anchor,
            text_path,
            red_path,
            green_path,
            blue_path,
            Val::Bool(wrap),
        ],
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
