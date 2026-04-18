//! Fast-path builders for XML inline global handlers. Each submodule owns
//! one shape-family (simple/multi-arg method calls, tooltip, misc) and
//! exports its own `pub(super) fn build_*_variants` dispatcher.

use super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

mod method_multi;
mod method_simple;
mod tooltip_misc;

pub(super) fn build_global_family_handler(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = build_global_method_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    if let Some(result) = tooltip_misc::build_global_tooltip_variants(state, handler_ref)? {
        return Ok(Some(result));
    }
    tooltip_misc::build_global_misc_variants(state, handler_ref)
}

/// LFG branch + the `target[method](...)` shapes (plain, with self/string/
/// global/self-id/self-field args).
///
/// Dispatches through two themed sub-helpers: single-/zero-arg variants and
/// multi-arg variants. Each sub-helper returns `Ok(None)` for arms it does
/// not handle so the chain falls through to the next.
fn build_global_method_variants(
    state: &mut LuaState,
    handler_ref: &FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    if let Some(result) = method_simple::build_global_method_simple_arg_variants(state, handler_ref)?
    {
        return Ok(Some(result));
    }
    method_multi::build_global_method_multi_arg_variants(state, handler_ref)
}

// ── Shared global-method dispatch helpers ──────────────────────────────────

pub(super) enum GlobalMethodMode {
    Passthrough,
    SelfId,
}

pub(super) fn build_global_method_with_mode(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    mode: GlobalMethodMode,
) -> LuaResult<Val> {
    let (source, tag) = global_method_template(mode);
    call_global_method_builder(state, target_path, method_name, source, tag, &[])
}

// ── Per-mode Lua global-method dispatch templates ────────────────────────────
//
// Each template closes over `target_ref` (either a pre-resolved value or
// a dotted path string) + `method_name`, then forwards either `...` or
// `self:GetID()` as the trailing argument shape.

const GLOBAL_METHOD_PASSTHROUGH_TEMPLATE: &str = r#"
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
"#;

const GLOBAL_METHOD_SELF_ID_TEMPLATE: &str = r#"
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
"#;

fn global_method_template(mode: GlobalMethodMode) -> (&'static str, &'static str) {
    match mode {
        GlobalMethodMode::Passthrough => (
            GLOBAL_METHOD_PASSTHROUGH_TEMPLATE,
            "template-global-method-handler",
        ),
        GlobalMethodMode::SelfId => (
            GLOBAL_METHOD_SELF_ID_TEMPLATE,
            "template-global-method-self-id-handler",
        ),
    }
}

pub(super) fn call_global_method_builder(
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
