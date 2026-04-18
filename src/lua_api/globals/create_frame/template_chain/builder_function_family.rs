use super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_function_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = build_plain_function_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_function_with_arg_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = build_ancestor_function_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    Ok(None)
}

/// Kind-dispatched bindings: `fn(...)`, `fn(self:GetID())`, `fn(self, event, ...)`, etc.
fn build_plain_function_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::Function(function_name) => {
            Ok(Some(resolve_global_path(state, function_name)))
        }
        FastHandlerRef::FunctionNoArgs(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::NoArgs).map(Some)
        }
        FastHandlerRef::FunctionWithSelfGetTextResult(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::SelfGetText).map(Some)
        }
        FastHandlerRef::FunctionWithSelfIdArg(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::SelfId).map(Some)
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
        _ => Ok(None),
    }
}

/// Per-argument shapes: string / number / global / self+field / global+self combinations.
fn build_function_with_arg_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithStringNumberArgs {
            function_name,
            first,
            second,
        } => build_function_handler_with_string_number_args(state, function_name, first, *second)
            .map(Some),
        FastHandlerRef::FunctionWithTwoGlobalNumberArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third,
        } => build_function_handler_with_two_global_number_args(
            state,
            function_name,
            first_arg_path,
            second_arg_path,
            *third,
        )
        .map(Some),
        FastHandlerRef::FunctionWithStringNilNilGlobalArgs {
            function_name,
            first,
            fourth,
        } => build_function_handler_with_string_nil_nil_global_args(
            state,
            function_name,
            first,
            fourth,
        )
        .map(Some),
        FastHandlerRef::FunctionWithStringArg { function_name, arg } => {
            build_function_handler_with_string_only_arg(state, function_name, arg).map(Some)
        }
        FastHandlerRef::FunctionWithNoArgFunctionResult {
            function_name,
            arg_function_name,
        } => build_function_handler_with_noarg_function_result(
            state,
            function_name,
            arg_function_name,
        )
        .map(Some),
        FastHandlerRef::FunctionWithSelfNoArgsMethodResult {
            function_name,
            method_name,
        } => {
            build_function_handler_with_self_noarg_method_result(state, function_name, method_name)
                .map(Some)
        }
        FastHandlerRef::FunctionWithGlobalMethodNoArgsResult {
            function_name,
            target_path,
            method_name,
        } => build_function_handler_with_global_method_noargs_result(
            state,
            function_name,
            target_path,
            method_name,
        )
        .map(Some),
        FastHandlerRef::FunctionWithSelfStringArg { function_name, arg } => {
            build_function_handler_with_string_arg(state, function_name, arg).map(Some)
        }
        FastHandlerRef::FunctionWithStringSelfStringNumberNumberArgs {
            function_name,
            first,
            third,
            fourth,
            fifth,
        } => build_function_handler_with_string_self_string_number_number_args(
            state,
            function_name,
            first,
            third,
            *fourth,
            *fifth,
        )
        .map(Some),
        FastHandlerRef::FunctionWithSelfNumberArg {
            function_name,
            value,
        } => build_function_handler_with_self_number_arg(state, function_name, *value).map(Some),
        FastHandlerRef::FunctionWithNumberArg {
            function_name,
            value,
        } => build_function_handler_with_number_arg(state, function_name, *value).map(Some),
        FastHandlerRef::FunctionWithGlobalArg {
            function_name,
            arg_path,
        } => build_function_handler_with_global_arg(state, function_name, arg_path).map(Some),
        FastHandlerRef::FunctionWithTwoGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
        } => build_function_handler_with_two_global_args(
            state,
            function_name,
            first_arg_path,
            second_arg_path,
        )
        .map(Some),
        FastHandlerRef::FunctionWithThreeGlobalArgs {
            function_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
        } => build_function_handler_with_three_global_args(
            state,
            function_name,
            first_arg_path,
            second_arg_path,
            third_arg_path,
        )
        .map(Some),
        FastHandlerRef::FunctionWithGlobalSelfMethodSelfMethodBoolArgs {
            function_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            fourth,
        } => build_function_handler_with_global_self_method_self_method_bool_args(
            state,
            function_name,
            first_arg_path,
            second_self_method,
            third_self_method,
            *fourth,
        )
        .map(Some),
        FastHandlerRef::FunctionWithStringGlobalBoolArg {
            function_name,
            first,
            second_arg_path,
            third,
        } => build_function_handler_with_string_global_bool_arg(
            state,
            function_name,
            first,
            second_arg_path,
            *third,
        )
        .map(Some),
        FastHandlerRef::FunctionWithGlobalAndSelfIdArg {
            function_name,
            global_arg_path,
        } => build_function_handler_with_global_and_self_id_arg(
            state,
            function_name,
            global_arg_path,
        )
        .map(Some),
        FastHandlerRef::FunctionWithGlobalAndSelfArg {
            function_name,
            global_arg_path,
        } => build_function_handler_with_global_and_self_arg(state, function_name, global_arg_path)
            .map(Some),
        FastHandlerRef::FunctionWithParentFieldArg {
            function_name,
            field,
        } => build_function_handler_with_parent_field_arg(state, function_name, field).map(Some),
        FastHandlerRef::FunctionWithParentFieldAndNestedParentFieldMethodResult {
            function_name,
            first_field,
            second_field,
            third_field,
            method_name,
        } => build_function_handler_with_parent_field_and_nested_parent_field_method_result(
            state,
            function_name,
            first_field,
            second_field,
            third_field,
            method_name,
        )
        .map(Some),
        FastHandlerRef::FunctionWithSelfAndParentFieldArg {
            function_name,
            field,
        } => build_function_handler_with_self_and_parent_field_arg(state, function_name, field)
            .map(Some),
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
        FastHandlerRef::CopyClubTicketToClipboardFromParent => {
            build_copy_club_ticket_to_clipboard_from_parent_handler(state).map(Some)
        }
        FastHandlerRef::PlaySoundThenCopyClubTicketToClipboardFromParent { sound_path } => {
            build_play_sound_then_copy_club_ticket_to_clipboard_from_parent_handler(
                state, sound_path,
            )
            .map(Some)
        }
        _ => Ok(None),
    }
}

/// Parent / grandparent / parent-id shapes.
fn build_ancestor_function_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::FunctionWithParentArg(function_name) => {
            build_ancestor_function_handler(state, function_name, 1).map(Some)
        }
        FastHandlerRef::FunctionWithGrandparentArg(function_name) => {
            build_ancestor_function_handler(state, function_name, 2).map(Some)
        }
        FastHandlerRef::FunctionWithParentIdArg(function_name) => {
            build_ancestor_id_function_handler(state, function_name, 1).map(Some)
        }
        _ => Ok(None),
    }
}

enum FunctionHandlerKind {
    NoArgs,
    SelfGetText,
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
    let (source, tag) = function_handler_template(kind);
    let builder = load_template(state, source, tag)?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[target])
}

// ── Per-kind Lua function-handler templates ──────────────────────────────────
//
// Each template closes over `fn` (the resolved global) and returns a
// wrapper function that forwards the right argument shape. Kept as
// named consts so `function_handler_template` is a trivial dispatch.

const NOARGS_TEMPLATE: &str = r#"
    local fn = ...
    return function(self, ...)
        return fn()
    end
"#;

const SELF_GETTEXT_TEMPLATE: &str = r#"
    local fn = ...
    return function(self, ...)
        return fn(self:GetText())
    end
"#;

const SELF_ID_TEMPLATE: &str = r#"
    local fn = ...
    return function(self, ...)
        return fn(self:GetID())
    end
"#;

const EVENT_VARARGS_TEMPLATE: &str = r#"
    local fn = ...
    return function(self, event, ...)
        return fn(self, event, ...)
    end
"#;

const BUTTON_TEMPLATE: &str = r#"
    local fn = ...
    return function(self, button, ...)
        return fn(self, button, ...)
    end
"#;

const ELAPSED_TEMPLATE: &str = r#"
    local fn = ...
    return function(self, elapsed, ...)
        return fn(self, elapsed, ...)
    end
"#;

fn function_handler_template(kind: FunctionHandlerKind) -> (&'static str, &'static str) {
    match kind {
        FunctionHandlerKind::NoArgs => (NOARGS_TEMPLATE, "template-inline-function-noargs"),
        FunctionHandlerKind::SelfGetText => (
            SELF_GETTEXT_TEMPLATE,
            "template-inline-function-self-gettext",
        ),
        FunctionHandlerKind::SelfId => (SELF_ID_TEMPLATE, "template-inline-function-self-id"),
        FunctionHandlerKind::EventVarargs => (
            EVENT_VARARGS_TEMPLATE,
            "template-inline-function-event-varargs",
        ),
        FunctionHandlerKind::Button => (BUTTON_TEMPLATE, "template-inline-function-button"),
        FunctionHandlerKind::Elapsed => (ELAPSED_TEMPLATE, "template-inline-function-elapsed"),
    }
}

fn build_function_handler_with_string_arg(
    state: &mut LuaState,
    function_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, literal_arg = ...
            return function(self, ...)
                return fn(self, literal_arg)
            end
        "#,
        "template-inline-function-self-string",
    )?;
    let target = resolve_global_path(state, function_name);
    let arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg],
    )
}

fn build_function_handler_with_string_only_arg(
    state: &mut LuaState,
    function_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, literal_arg = ...
            return function(self, ...)
                return fn(literal_arg)
            end
        "#,
        "template-inline-function-string-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg],
    )
}

fn build_function_handler_with_string_number_args(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    second: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second = ...
            return function(self, ...)
                return fn(first, second)
            end
        "#,
        "template-inline-function-string-number-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, Val::Num(second)],
    )
}

fn build_function_handler_with_two_global_number_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
    third: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second, third = ...
            return function(self, ...)
                return fn(first, second, third)
            end
        "#,
        "template-inline-function-two-global-number-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = resolve_global_path(state, first_arg_path);
    let second = resolve_global_path(state, second_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, second, Val::Num(third)],
    )
}

fn build_play_sound_then_copy_club_ticket_to_clipboard_from_parent_handler(
    state: &mut LuaState,
    sound_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local sound = ...
            return function(self, ...)
                PlaySound(sound)
                local clubId = self:GetParent():GetClubId()
                local clubInfo = clubId and C_Club.GetClubInfo(clubId)
                if clubInfo then
                    return CopyToClipboard(
                        ClubTicketUtil.FormatTicket(
                            clubInfo,
                            self:GetParent().LinkIDText:GetText()
                        )
                    )
                end
            end
        "#,
        "template-play-sound-then-copy-club-ticket",
    )?;
    let sound = resolve_global_path(state, sound_path);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[sound])
}

fn build_function_handler_with_three_global_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
    third_arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second, third = ...
            return function(self, ...)
                return fn(first, second, third)
            end
        "#,
        "template-inline-function-three-global-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = resolve_global_path(state, first_arg_path);
    let second = resolve_global_path(state, second_arg_path);
    let third = resolve_global_path(state, third_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, second, third],
    )
}

fn build_function_handler_with_string_self_string_number_number_args(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    third: &str,
    fourth: f64,
    fifth: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, third, fourth, fifth = ...
            return function(self, ...)
                return fn(first, self, third, fourth, fifth)
            end
        "#,
        "template-inline-function-string-self-string-number-number-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    let third = create_string(state, third);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, third, Val::Num(fourth), Val::Num(fifth)],
    )
}

fn build_function_handler_with_string_global_bool_arg(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    second_arg_path: &str,
    third: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second, third = ...
            return function(self, ...)
                return fn(first, second, third)
            end
        "#,
        "template-inline-function-string-global-bool-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    let second = resolve_global_path(state, second_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, second, Val::Bool(third)],
    )
}

fn build_function_handler_with_global_self_method_self_method_bool_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_self_method: &str,
    third_self_method: &str,
    fourth: bool,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, second_method, third_method, fourth = ...
            return function(self, ...)
                return fn(first, self[second_method](self), self[third_method](self), fourth)
            end
        "#,
        "template-inline-function-global-self-method-self-method-bool-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = resolve_global_path(state, first_arg_path);
    let second_method = create_string(state, second_self_method);
    let third_method = create_string(state, third_self_method);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            target,
            first,
            second_method,
            third_method,
            Val::Bool(fourth),
        ],
    )
}

fn build_function_handler_with_string_nil_nil_global_args(
    state: &mut LuaState,
    function_name: &str,
    first: &str,
    fourth: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first, fourth = ...
            return function(self, ...)
                return fn(first, nil, nil, fourth)
            end
        "#,
        "template-inline-function-string-nil-nil-global-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first = create_string(state, first);
    let fourth = resolve_global_path(state, fourth);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first, fourth],
    )
}

fn build_function_handler_with_noarg_function_result(
    state: &mut LuaState,
    function_name: &str,
    arg_function_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, arg_fn = ...
            return function(self, ...)
                return fn(arg_fn())
            end
        "#,
        "template-inline-function-noarg-function-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let arg_function = resolve_global_path(state, arg_function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg_function],
    )
}

fn build_function_handler_with_self_noarg_method_result(
    state: &mut LuaState,
    function_name: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, method_name = ...
            return function(self, ...)
                return fn(self[method_name](self))
            end
        "#,
        "template-inline-function-self-noarg-method-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name],
    )
}

fn build_function_handler_with_global_method_noargs_result(
    state: &mut LuaState,
    function_name: &str,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, target, method_name = ...
            return function(self, ...)
                return fn(target[method_name](target))
            end
        "#,
        "template-inline-function-global-method-noargs-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let method_target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_target, method_name],
    )
}

fn build_function_handler_with_number_arg(
    state: &mut LuaState,
    function_name: &str,
    value: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, number_arg = ...
            return function(self, ...)
                return fn(number_arg)
            end
        "#,
        "template-inline-function-number-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(value)],
    )
}

fn build_function_handler_with_self_number_arg(
    state: &mut LuaState,
    function_name: &str,
    value: f64,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, number_arg = ...
            return function(self, ...)
                return fn(self, number_arg)
            end
        "#,
        "template-inline-function-self-number-arg",
    )?;
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
    let builder = load_template(
        state,
        r#"
            local fn, resolved_arg = ...
            return function(self, ...)
                return fn(resolved_arg)
            end
        "#,
        "template-inline-function-global-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let arg = resolve_global_path(state, arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, arg],
    )
}

fn build_function_handler_with_two_global_args(
    state: &mut LuaState,
    function_name: &str,
    first_arg_path: &str,
    second_arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first_arg, second_arg = ...
            return function(self, ...)
                return fn(first_arg, second_arg)
            end
        "#,
        "template-inline-function-two-global-args",
    )?;
    let target = resolve_global_path(state, function_name);
    let first_arg = resolve_global_path(state, first_arg_path);
    let second_arg = resolve_global_path(state, second_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first_arg, second_arg],
    )
}

fn build_function_handler_with_global_and_self_arg(
    state: &mut LuaState,
    function_name: &str,
    global_arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, global_arg = ...
            return function(self, ...)
                return fn(global_arg, self)
            end
        "#,
        "template-inline-function-global-self-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let global_arg = resolve_global_path(state, global_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, global_arg],
    )
}

fn build_function_handler_with_global_and_self_id_arg(
    state: &mut LuaState,
    function_name: &str,
    global_arg_path: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, global_arg = ...
            return function(self, ...)
                return fn(global_arg, self:GetID())
            end
        "#,
        "template-inline-function-global-self-id-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let global_arg = resolve_global_path(state, global_arg_path);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, global_arg],
    )
}

fn build_function_handler_with_parent_field_arg(
    state: &mut LuaState,
    function_name: &str,
    field: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, field_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                return fn(parent[field_name])
            end
        "#,
        "template-inline-function-parent-field-arg",
    )?;
    let target = resolve_global_path(state, function_name);
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, field_name],
    )
}

fn build_function_handler_with_parent_field_and_nested_parent_field_method_result(
    state: &mut LuaState,
    function_name: &str,
    first_field: &str,
    second_field: &str,
    third_field: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            local fn, first_field, second_field, third_field, method_name = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                local second = parent[second_field]
                local third = second and second[third_field]
                if not third or not third[method_name] then
                    return
                end
                return fn(parent[first_field], third[method_name](third))
            end
        "#,
        "template-inline-function-parent-field-nested-parent-method-result",
    )?;
    let target = resolve_global_path(state, function_name);
    let first_field = create_string(state, first_field);
    let second_field = create_string(state, second_field);
    let third_field = create_string(state, third_field);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, first_field, second_field, third_field, method_name],
    )
}

fn build_function_handler_with_self_and_parent_field_arg(
    state: &mut LuaState,
    function_name: &str,
    field: &str,
) -> LuaResult<Val> {
    let builder = load_template(
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
    )?;
    let target = resolve_global_path(state, function_name);
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, field_name],
    )
}

fn build_checked_assignment_then_callbacks_handler(
    state: &mut LuaState,
    target_path: &str,
    field: &str,
    on_change_function: &str,
    on_sound_function: &str,
) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
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
        "#,
        "template-inline-checked-assignment-then-callbacks",
    )?;
    let target_path = create_string(state, target_path);
    let field_name = create_string(state, field);
    let on_change = resolve_global_path(state, on_change_function);
    let on_sound = resolve_global_path(state, on_sound_function);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
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
    let builder = load_template(
        state,
        r#"
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
        "#,
        "template-inline-checked-assignments3-then-callbacks",
    )?;
    let first_target_path = create_string(state, first_target_path);
    let first_field = create_string(state, first_field);
    let second_target_path = create_string(state, second_target_path);
    let second_field = create_string(state, second_field);
    let third_target_path = create_string(state, third_target_path);
    let third_field = create_string(state, third_field);
    let on_change = resolve_global_path(state, on_change_function);
    let on_sound = resolve_global_path(state, on_sound_function);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
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
    let builder = load_template(
        state,
        r#"
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
        "#,
        "template-inline-checked-assignment-two-callbacks",
    )?;
    let target_path = create_string(state, target_path);
    let field_name = create_string(state, field);
    let first_callback = resolve_global_path(state, first_callback);
    let second_callback = resolve_global_path(state, second_callback);
    let on_sound = resolve_global_path(state, on_sound_function);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
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
    let builder = load_template(
        state,
        r#"
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
        "#,
        "template-inline-checked-number-assignment-then-callbacks",
    )?;
    let target_path = create_string(state, target_path);
    let field_name = create_string(state, field);
    let on_change = resolve_global_path(state, on_change_function);
    let on_sound = resolve_global_path(state, on_sound_function);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[
            target_path,
            field_name,
            Val::Num(value),
            on_change,
            on_sound,
        ],
    )
}

fn build_copy_club_ticket_to_clipboard_from_parent_handler(state: &mut LuaState) -> LuaResult<Val> {
    let builder = load_template(
        state,
        r#"
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                local clubId = parent:GetClubId()
                local clubInfo = clubId and C_Club.GetClubInfo(clubId)
                if clubInfo and parent.LinkIDText and parent.LinkIDText.GetText then
                    return CopyToClipboard(ClubTicketUtil.FormatTicket(clubInfo, parent.LinkIDText:GetText()))
                end
            end
        "#,
        "template-inline-copy-club-ticket-to-clipboard-from-parent",
    )?;
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[])
}

fn build_ancestor_function_handler(
    state: &mut LuaState,
    function_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    build_ancestor_function_handler_with_mode(state, function_name, depth, AncestorArgMode::Target)
}

fn build_ancestor_id_function_handler(
    state: &mut LuaState,
    function_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    build_ancestor_function_handler_with_mode(state, function_name, depth, AncestorArgMode::Id)
}

enum AncestorArgMode {
    Target,
    Id,
}

fn build_ancestor_function_handler_with_mode(
    state: &mut LuaState,
    function_name: &str,
    depth: usize,
    mode: AncestorArgMode,
) -> LuaResult<Val> {
    let (source, tag) = ancestor_function_handler_template(mode);
    let builder = load_template(state, source, tag)?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(depth as f64)],
    )
}

// ── Per-mode Lua ancestor-walk templates ─────────────────────────────────────
//
// Both close over `fn` + `depth`, walk `:GetParent()` `depth` times, and
// forward either the ancestor itself or its `:GetID()`.

const ANCESTOR_TARGET_TEMPLATE: &str = r#"
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
"#;

const ANCESTOR_ID_TEMPLATE: &str = r#"
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
"#;

fn ancestor_function_handler_template(mode: AncestorArgMode) -> (&'static str, &'static str) {
    match mode {
        AncestorArgMode::Target => (
            ANCESTOR_TARGET_TEMPLATE,
            "template-inline-function-ancestor",
        ),
        AncestorArgMode::Id => (ANCESTOR_ID_TEMPLATE, "template-inline-function-ancestor-id"),
    }
}
