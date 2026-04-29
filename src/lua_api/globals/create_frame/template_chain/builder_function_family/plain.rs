//! Plain function handlers: `fn(...)`, `fn(self:GetText())`, `fn(self:GetID())`,
//! `fn(self, event, ...)`, `fn(self, button, ...)`, `fn(self, elapsed, ...)`.

use super::super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::hot_literals::{
    TEMPLATE_INLINE_FUNCTION_BUTTON, TEMPLATE_INLINE_FUNCTION_ELAPSED,
    TEMPLATE_INLINE_FUNCTION_EVENT_VARARGS, TEMPLATE_INLINE_FUNCTION_NOARGS,
    TEMPLATE_INLINE_FUNCTION_SELF_ID,
};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Kind-dispatched bindings: `fn(...)`, `fn(self:GetID())`, `fn(self, event, ...)`, etc.
pub(super) fn build_plain_function_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::Function(function_name) => {
            let target = resolve_global_path(state, function_name);
            if matches!(target, Val::Nil) {
                build_missing_global_handler(state, function_name).map(Some)
            } else {
                Ok(Some(target))
            }
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
    let target = resolve_global_path(state, function_name);
    if matches!(target, Val::Nil) {
        // Nil at install means the global isn't defined. Building the
        // normal wrapper would capture `fn = nil` and error at call time
        // with the unhelpful `attempt to call upvalue 'fn' (a nil value)`
        // message. Install a stub instead that errors with the actual
        // global name, matching what the slow Lua chunk path produces.
        return build_missing_global_handler(state, function_name);
    }
    let (source, tag) = function_handler_template(kind);
    let builder = load_template(state, source, tag)?;
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[target])
}

const MISSING_GLOBAL_TEMPLATE: &str = r#"
    local fn_name = ...
    return function(self, ...)
        error("attempt to call global '" .. fn_name .. "' (a nil value)", 2)
    end
"#;

fn build_missing_global_handler(state: &mut LuaState, function_name: &str) -> LuaResult<Val> {
    let builder = load_template(state, MISSING_GLOBAL_TEMPLATE, "template-missing-global")?;
    let name = crate::lua_api::methods::create_string(state, function_name);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[name])
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
        FunctionHandlerKind::NoArgs => (NOARGS_TEMPLATE, TEMPLATE_INLINE_FUNCTION_NOARGS),
        FunctionHandlerKind::SelfGetText => (
            SELF_GETTEXT_TEMPLATE,
            "template-inline-function-self-gettext",
        ),
        FunctionHandlerKind::SelfId => (SELF_ID_TEMPLATE, TEMPLATE_INLINE_FUNCTION_SELF_ID),
        FunctionHandlerKind::EventVarargs => (
            EVENT_VARARGS_TEMPLATE,
            TEMPLATE_INLINE_FUNCTION_EVENT_VARARGS,
        ),
        FunctionHandlerKind::Button => (BUTTON_TEMPLATE, TEMPLATE_INLINE_FUNCTION_BUTTON),
        FunctionHandlerKind::Elapsed => (ELAPSED_TEMPLATE, TEMPLATE_INLINE_FUNCTION_ELAPSED),
    }
}
