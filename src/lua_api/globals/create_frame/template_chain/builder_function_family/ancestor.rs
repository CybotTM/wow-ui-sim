//! Parent / grandparent / parent-id shapes. The shared ancestor walker in
//! Lua walks `:GetParent()` `depth` times and forwards either the ancestor
//! itself or its `:GetID()`.

use super::super::{FastHandlerRef, load_template};
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_ancestor_function_variants(
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
