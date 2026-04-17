use super::FastHandlerRef;
use crate::lua_api::globals::create_frame::helpers::resolve_global_path;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn build_function_family_handler(
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
        FastHandlerRef::FunctionWithSelfIdArg(function_name) => {
            build_function_handler(state, function_name, FunctionHandlerKind::SelfId).map(Some)
        }
        FastHandlerRef::FunctionWithSelfStringArg { function_name, arg } => {
            build_function_handler_with_string_arg(state, function_name, arg).map(Some)
        }
        FastHandlerRef::FunctionWithNumberArg {
            function_name,
            value,
        } => build_function_handler_with_number_arg(state, function_name, *value).map(Some),
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
        _ => Ok(None),
    }
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
    let (source, tag) = function_handler_template(kind);
    let builder = crate::loader::chunk_cache::load_chunk(state, source, tag)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[target])
}

fn function_handler_template(kind: FunctionHandlerKind) -> (&'static str, &'static str) {
    match kind {
        FunctionHandlerKind::NoArgs => no_args_handler_template(),
        FunctionHandlerKind::SelfId => self_id_handler_template(),
        FunctionHandlerKind::EventVarargs => event_varargs_handler_template(),
        FunctionHandlerKind::Button => button_handler_template(),
        FunctionHandlerKind::Elapsed => elapsed_handler_template(),
    }
}

fn no_args_handler_template() -> (&'static str, &'static str) {
    (
        r#"
            local fn = ...
            return function(self, ...)
                return fn()
            end
        "#,
        "template-inline-function-noargs",
    )
}

fn self_id_handler_template() -> (&'static str, &'static str) {
    (
        r#"
            local fn = ...
            return function(self, ...)
                return fn(self:GetID())
            end
        "#,
        "template-inline-function-self-id",
    )
}

fn event_varargs_handler_template() -> (&'static str, &'static str) {
    (
        r#"
            local fn = ...
            return function(self, event, ...)
                return fn(self, event, ...)
            end
        "#,
        "template-inline-function-event-varargs",
    )
}

fn button_handler_template() -> (&'static str, &'static str) {
    (
        r#"
            local fn = ...
            return function(self, button, ...)
                return fn(self, button, ...)
            end
        "#,
        "template-inline-function-button",
    )
}

fn elapsed_handler_template() -> (&'static str, &'static str) {
    (
        r#"
            local fn = ...
            return function(self, elapsed, ...)
                return fn(self, elapsed, ...)
            end
        "#,
        "template-inline-function-elapsed",
    )
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
    let builder = crate::loader::chunk_cache::load_chunk(state, source, tag)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(depth as f64)],
    )
}

fn ancestor_function_handler_template(mode: AncestorArgMode) -> (&'static str, &'static str) {
    match mode {
        AncestorArgMode::Target => {
            ancestor_function_template("fn(target)", "template-inline-function-ancestor")
        }
        AncestorArgMode::Id => {
            ancestor_function_template("fn(target:GetID())", "template-inline-function-ancestor-id")
        }
    }
}

fn ancestor_function_template(
    return_expr: &'static str,
    tag: &'static str,
) -> (&'static str, &'static str) {
    (ancestor_function_source(return_expr), tag)
}

fn ancestor_function_source(return_expr: &'static str) -> &'static str {
    match return_expr {
        "fn(target)" => ancestor_target_source(),
        "fn(target:GetID())" => ancestor_id_source(),
        _ => unreachable!("unsupported ancestor function return expression"),
    }
}

fn ancestor_target_source() -> &'static str {
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
    "#
}

fn ancestor_id_source() -> &'static str {
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
    "#
}
