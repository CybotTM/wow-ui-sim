//! Runtime template chain application: applies XML template inheritance to
//! frames created via `CreateFrame("Frame", name, parent, "TemplateName")`.

mod builders;
mod parser;
mod runtime;

use super::helpers::{append_parent_array_entry, apply_frame_mixins, resolve_global_path};
use crate::lua_api::methods::{borrow_state, create_string, frame_ref};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub(crate) fn apply_runtime_template_chain(
    state: &mut LuaState,
    frame_id: u64,
    inherits: Option<&str>,
    fire_on_load: bool,
) -> LuaResult<()> {
    apply_runtime_template_chain_impl(state, frame_id, inherits, fire_on_load, None)
}

pub(crate) fn apply_runtime_template_chain_with_frame_overrides(
    state: &mut LuaState,
    frame_id: u64,
    inherits: Option<&str>,
    fire_on_load: bool,
    frame: &crate::xml::FrameXml,
) -> LuaResult<()> {
    apply_runtime_template_chain_impl(state, frame_id, inherits, fire_on_load, Some(frame))
}

fn apply_runtime_template_chain_impl(
    state: &mut LuaState,
    frame_id: u64,
    inherits: Option<&str>,
    fire_on_load: bool,
    direct_frame: Option<&crate::xml::FrameXml>,
) -> LuaResult<()> {
    let Some(inherits) = inherits.filter(|value| !value.trim().is_empty()) else {
        if let Some(frame) = direct_frame {
            apply_template_key_values(state, frame_id, frame.all_key_values());
        }
        return Ok(());
    };
    let chain = crate::xml::get_template_chain(inherits);
    if chain.is_empty() {
        return Ok(());
    }

    let state_rc = sim_state_rc(state)?;
    let frame_name = frame_lookup_name(state, frame_id);
    apply_template_parent_links(state, frame_id, &chain)?;
    apply_chain_entries(state, frame_id, &chain)?;

    if let Some(frame) = direct_frame {
        // XML direct key-values must exist before template child OnLoad
        // handlers run, matching the loader chunk order.
        apply_template_key_values(state, frame_id, frame.all_key_values());
    }

    // The chain is base-to-derived. Install all parent-facing state first so
    // template child OnLoad/OnShow handlers can see derived key values and
    // mixin methods (for example ThreeSliceButtonTemplate children expect the
    // derived template's `atlasName` to already exist on the parent button).
    for entry in &*chain {
        runtime::create_template_child_frames(
            state,
            &state_rc,
            frame_id,
            &frame_name,
            &frame_name,
            &entry.frame,
        )?;
    }

    finalize_template_frame(
        state,
        &state_rc,
        frame_id,
        inherits,
        &frame_name,
        fire_on_load,
    )
}

// ---------------------------------------------------------------------------
// Chain application
// ---------------------------------------------------------------------------

fn apply_template_parent_links(
    state: &mut LuaState,
    frame_id: u64,
    chain: &[Arc<crate::xml::TemplateEntry>],
) -> LuaResult<()> {
    let template_parent_key = chain
        .iter()
        .rev()
        .find_map(|entry| entry.frame.parent_key.as_deref());
    let template_parent_array = chain
        .iter()
        .rev()
        .find_map(|entry| entry.frame.parent_array.as_deref());
    let parent_id = borrow_state(state)
        .ok()
        .and_then(|sim| sim.widgets.get(frame_id).and_then(|frame| frame.parent_id));
    let Some(parent_id) = parent_id else {
        return Ok(());
    };
    if let Some(parent_key) = template_parent_key {
        crate::lua_api::globals::template::assign_parent_key(
            state, parent_id, parent_key, frame_id,
        )?;
    }
    if let Some(parent_array) = template_parent_array {
        append_parent_array_entry(state, parent_id, parent_array, frame_id);
    }
    Ok(())
}

fn apply_chain_entries(
    state: &mut LuaState,
    frame_id: u64,
    chain: &[Arc<crate::xml::TemplateEntry>],
) -> LuaResult<()> {
    for entry in chain {
        runtime::ensure_runtime_button_texture_slots(state, frame_id, &entry.frame)?;
        apply_frame_mixins(state, frame_id, entry.frame.combined_mixin().as_deref());
        apply_template_key_values(state, frame_id, entry.frame.all_key_values());
        if let Some(scripts) = entry.frame.scripts() {
            apply_template_scripts(state, frame_id, scripts)?;
        }
    }
    Ok(())
}

fn finalize_template_frame(
    state: &mut LuaState,
    state_rc: &Rc<RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    inherits: &str,
    frame_name: &str,
    fire_on_load: bool,
) -> LuaResult<()> {
    runtime::apply_runtime_template_loader_effects(
        state,
        frame_name,
        frame_name,
        &crate::xml::FrameXml::default(),
        Some(inherits),
    )?;
    runtime::apply_runtime_template_direct_properties(state_rc, frame_id, inherits, frame_name);
    if fire_on_load {
        runtime::fire_frame_on_load(state, frame_id)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Script helpers
// ---------------------------------------------------------------------------

fn apply_template_key_values<'a>(
    state: &mut LuaState,
    frame_id: u64,
    key_values: impl Iterator<Item = &'a crate::xml::KeyValuesXml>,
) {
    let frame = frame_ref(state, frame_id).ok();
    let Some(Val::Table(frame_ref)) = frame else {
        return;
    };

    for key_block in key_values {
        for entry in &key_block.values {
            let value = template_key_value(state, &entry.value, entry.value_type.as_deref());
            let key = create_string(state, &entry.key);
            if let Some(table) = state.gc.tables.get_mut(frame_ref) {
                let _ = table.raw_set(key, value, &state.gc.string_arena);
            }
            state.gc.barrier_back(frame_ref);
        }
    }
}

pub(crate) fn apply_template_scripts(
    state: &mut LuaState,
    frame_id: u64,
    scripts: &crate::xml::ScriptsXml,
) -> LuaResult<()> {
    if apply_fast_scripts(state, frame_id, scripts)? {
        return Ok(());
    }

    let script_code = crate::loader::helpers::generate_scripts_code(scripts);
    if script_code.trim().is_empty() {
        return Ok(());
    }

    let chunk = format!("local frame = ...\n{script_code}");
    let func = crate::loader::chunk_cache::load_chunk(state, &chunk, "template-scripts")
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let frame = frame_ref(state, frame_id)?;
    match crate::lua_api::script_helpers::call_void_function_with_fallback_state(
        state,
        Val::Function(func.gc_ref()),
        &[frame],
    ) {
        Ok(_) => {}
        Err(error) => return Err(rilua::runtime_error(error)),
    }
    Ok(())
}

pub(crate) fn scripts_support_fast_install(scripts: &crate::xml::ScriptsXml) -> bool {
    collect_fast_handlers(scripts).is_some()
}

pub(crate) fn first_fast_install_miss(scripts: &crate::xml::ScriptsXml) -> Option<String> {
    first_fast_handler_miss_in_group(base_method_only_handlers(scripts))
        .or_else(|| first_fast_handler_miss_in_group(pointer_method_only_handlers(scripts)))
        .or_else(|| first_fast_handler_miss_in_group(text_method_only_handlers(scripts)))
        .or_else(|| first_fast_handler_miss_in_group(state_method_only_handlers(scripts)))
}

fn apply_fast_scripts(
    state: &mut LuaState,
    frame_id: u64,
    scripts: &crate::xml::ScriptsXml,
) -> LuaResult<bool> {
    let Some(installs) = collect_fast_handlers(scripts) else {
        return Ok(false);
    };
    if installs.is_empty() {
        return Ok(true);
    }

    for (handler_name, install) in installs {
        builders::install_fast_handler(state, frame_id, handler_name, install)?;
    }

    Ok(true)
}

fn collect_fast_handlers(
    scripts: &crate::xml::ScriptsXml,
) -> Option<Vec<(&'static str, FastScriptInstall<'_>)>> {
    let mut handlers = Vec::new();
    collect_fast_handler_group(&mut handlers, base_method_only_handlers(scripts))?;
    collect_fast_handler_group(&mut handlers, pointer_method_only_handlers(scripts))?;
    collect_fast_handler_group(&mut handlers, text_method_only_handlers(scripts))?;
    collect_fast_handler_group(&mut handlers, state_method_only_handlers(scripts))?;
    Some(handlers)
}

type MethodOnlyScript<'a> = (&'static str, Option<&'a crate::xml::ScriptBodyXml>);
type FastHandler<'a> = (&'static str, Option<&'a crate::xml::ScriptBodyXml>);

#[derive(Clone)]
enum FastHandlerRef<'a> {
    NoOp,
    Sequence2(Box<(FastHandlerRef<'a>, FastHandlerRef<'a>)>),
    Sequence3(Box<(FastHandlerRef<'a>, FastHandlerRef<'a>, FastHandlerRef<'a>)>),
    Method(&'a str),
    MethodWithBoolArg {
        method_name: &'a str,
        value: bool,
    },
    MethodWithStringArg {
        method_name: &'a str,
        arg: &'a str,
    },
    SelfFieldMethod {
        field: &'a str,
        method_name: &'a str,
    },
    SelfFieldMethodWithStringArg {
        field: &'a str,
        method_name: &'a str,
        arg: &'a str,
    },
    SelfFieldMethodWithNumberArg {
        field: &'a str,
        method_name: &'a str,
        value: f64,
    },
    SelfFieldMethodWithGlobalArg {
        field: &'a str,
        method_name: &'a str,
        arg_path: &'a str,
    },
    SelfFieldMethodWithSelfFieldArg {
        field: &'a str,
        method_name: &'a str,
        arg_field: &'a str,
    },
    SelfFieldMethodWithStringNumberNumberArgs {
        field: &'a str,
        method_name: &'a str,
        first: &'a str,
        second: f64,
        third: f64,
    },
    ParentMethod(&'a str),
    ParentMethodWithStringArg {
        method_name: &'a str,
        arg: &'a str,
    },
    GrandparentMethod(&'a str),
    GlobalMethod {
        target_path: &'a str,
        method_name: &'a str,
    },
    GlobalMethodWithSelfStringArg {
        target_path: &'a str,
        method_name: &'a str,
        arg: &'a str,
    },
    GlobalMethodWithSelfIdArg {
        target_path: &'a str,
        method_name: &'a str,
    },
    GlobalMethodWithSelfFieldArg {
        target_path: &'a str,
        method_name: &'a str,
        field: &'a str,
    },
    GlobalTooltipSetOwnerThenSetText {
        target_path: &'a str,
        anchor: &'a str,
        text_path: &'a str,
        red_path: &'a str,
        green_path: &'a str,
        blue_path: &'a str,
        wrap: bool,
    },
    ConditionalTooltip {
        target_path: &'a str,
        field: &'a str,
        anchor: &'a str,
        red_path: &'a str,
        green_path: &'a str,
        blue_path: &'a str,
    },
    NamedGlobalMethodWithGlobalArg {
        suffix: &'a str,
        method_name: &'a str,
        arg_path: &'a str,
    },
    GlobalMethodThenAssignLiteral {
        target_path: &'a str,
        method_name: &'a str,
        field: &'a str,
        value: FastLiteralValue<'a>,
    },
    Function(&'a str),
    FunctionNoArgs(&'a str),
    FunctionWithSelfIdArg(&'a str),
    FunctionWithSelfStringArg {
        function_name: &'a str,
        arg: &'a str,
    },
    FunctionWithSelfNumberArg {
        function_name: &'a str,
        value: f64,
    },
    FunctionWithNumberArg {
        function_name: &'a str,
        value: f64,
    },
    FunctionWithGlobalArg {
        function_name: &'a str,
        arg_path: &'a str,
    },
    FunctionWithGlobalAndSelfArg {
        function_name: &'a str,
        global_arg_path: &'a str,
    },
    FunctionWithSelfAndParentFieldArg {
        function_name: &'a str,
        field: &'a str,
    },
    FunctionWithParentArg(&'a str),
    FunctionWithGrandparentArg(&'a str),
    FunctionWithParentIdArg(&'a str),
    FunctionWithEventVarargs(&'a str),
    FunctionWithButton(&'a str),
    FunctionWithElapsed(&'a str),
    RegisterForClicks {
        first: &'a str,
        second: Option<&'a str>,
        third: Option<&'a str>,
    },
    RegisterForDrag(&'a str),
    SetAlpha(f64),
    SetFrameLevelFromParent(i32),
    AssignAncestorRef {
        field: &'a str,
        depth: usize,
    },
    AssignLiteral {
        field: &'a str,
        value: FastLiteralValue<'a>,
    },
    AssignNestedLiteral {
        parent_field: &'a str,
        field: &'a str,
        value: FastLiteralValue<'a>,
    },
    AssignNestedGlobalPairTable {
        parent_field: &'a str,
        field: &'a str,
        first_path: &'a str,
        second_path: &'a str,
    },
    AssignParentField {
        field: &'a str,
        value: FastLiteralValue<'a>,
    },
}

#[derive(Copy, Clone)]
enum FastLiteralValue<'a> {
    Global(&'a str),
    Number(f64),
    Nil,
    Bool(bool),
}

#[derive(Clone)]
enum FastScriptInstall<'a> {
    Set(FastHandlerRef<'a>),
    Intrinsic(FastHandlerRef<'a>),
    Chain {
        handler: FastHandlerRef<'a>,
        new_first: bool,
    },
}

fn collect_fast_handler_group<'a>(
    handlers: &mut Vec<(&'static str, FastScriptInstall<'a>)>,
    group: impl IntoIterator<Item = FastHandler<'a>>,
) -> Option<()> {
    for (handler_name, script) in group {
        let Some(script) = script else {
            continue;
        };
        let install = fast_script_install(handler_name, script)?;
        handlers.push((handler_name, install));
    }
    Some(())
}

fn first_fast_handler_miss_in_group<'a>(
    group: impl IntoIterator<Item = FastHandler<'a>>,
) -> Option<String> {
    for (handler_name, script) in group {
        let Some(script) = script else {
            continue;
        };
        if fast_script_install(handler_name, script).is_none() {
            return Some(describe_fast_script_miss(handler_name, script));
        }
    }
    None
}

fn fast_script_install<'a>(
    handler_name: &'static str,
    script: &'a crate::xml::ScriptBodyXml,
) -> Option<FastScriptInstall<'a>> {
    let handler = if let Some(method_name) = script.method.as_deref() {
        FastHandlerRef::Method(method_name)
    } else if let Some(function_name) = script.function.as_deref() {
        FastHandlerRef::Function(function_name)
    } else if let Some(body) = script.body.as_deref() {
        parser::parse_inline_fast_handler(handler_name, body)?
    } else {
        FastHandlerRef::NoOp
    };
    match script.intrinsic_order.as_deref() {
        Some("precall" | "postcall") => Some(FastScriptInstall::Intrinsic(handler)),
        Some(_) => None,
        None => match script.inherit.as_deref() {
            Some("append") => Some(FastScriptInstall::Chain {
                handler,
                new_first: true,
            }),
            Some("prepend") => Some(FastScriptInstall::Chain {
                handler,
                new_first: false,
            }),
            Some(_) => None,
            None => Some(FastScriptInstall::Set(handler)),
        },
    }
}

fn describe_fast_script_miss(handler_name: &str, script: &crate::xml::ScriptBodyXml) -> String {
    let body = script.body.as_deref().unwrap_or("");
    let body = body.trim().replace('\n', " ");
    if let Some(intrinsic_order) = script.intrinsic_order.as_deref() {
        format!("{handler_name}|intrinsic={intrinsic_order}|{body}")
    } else if let Some(inherit) = script.inherit.as_deref() {
        format!("{handler_name}|inherit={inherit}|{body}")
    } else if let Some(method_name) = script.method.as_deref() {
        format!("{handler_name}|method={method_name}|{body}")
    } else if let Some(function_name) = script.function.as_deref() {
        format!("{handler_name}|function={function_name}|{body}")
    } else {
        format!("{handler_name}|{body}")
    }
}

fn base_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 8] {
    [
        ("OnLoad", scripts.on_load.last()),
        ("OnEvent", scripts.on_event.last()),
        ("OnUpdate", scripts.on_update.last()),
        ("OnClick", scripts.on_click.last()),
        ("PreClick", scripts.pre_click.last()),
        ("PostClick", scripts.post_click.last()),
        ("OnShow", scripts.on_show.last()),
        ("OnHide", scripts.on_hide.last()),
    ]
}

fn pointer_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 8] {
    [
        ("OnEnter", scripts.on_enter.last()),
        ("OnLeave", scripts.on_leave.last()),
        ("OnMouseDown", scripts.on_mouse_down.last()),
        ("OnMouseUp", scripts.on_mouse_up.last()),
        ("OnMouseWheel", scripts.on_mouse_wheel.last()),
        ("OnDragStart", scripts.on_drag_start.last()),
        ("OnDragStop", scripts.on_drag_stop.last()),
        ("OnReceiveDrag", scripts.on_receive_drag.last()),
    ]
}

fn text_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 10] {
    [
        ("OnEnterPressed", scripts.on_enter_pressed.last()),
        ("OnEscapePressed", scripts.on_escape_pressed.last()),
        ("OnTabPressed", scripts.on_tab_pressed.last()),
        ("OnSpacePressed", scripts.on_space_pressed.last()),
        ("OnTextChanged", scripts.on_text_changed.last()),
        ("OnTextSet", scripts.on_text_set.last()),
        ("OnChar", scripts.on_char.last()),
        ("OnEditFocusGained", scripts.on_edit_focus_gained.last()),
        ("OnEditFocusLost", scripts.on_edit_focus_lost.last()),
        (
            "OnInputLanguageChanged",
            scripts.on_input_language_changed.last(),
        ),
    ]
}

fn state_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 10] {
    [
        ("OnKeyDown", scripts.on_key_down.last()),
        ("OnKeyUp", scripts.on_key_up.last()),
        ("OnValueChanged", scripts.on_value_changed.last()),
        ("OnEnable", scripts.on_enable.last()),
        ("OnDisable", scripts.on_disable.last()),
        ("OnSizeChanged", scripts.on_size_changed.last()),
        ("OnAttributeChanged", scripts.on_attribute_changed.last()),
        ("OnHyperlinkClick", scripts.on_hyperlink_click.last()),
        ("OnHyperlinkEnter", scripts.on_hyperlink_enter.last()),
        ("OnHyperlinkLeave", scripts.on_hyperlink_leave.last()),
    ]
}

fn template_key_value(state: &mut LuaState, value: &str, value_type: Option<&str>) -> Val {
    match value_type {
        Some("number") => value.parse::<f64>().map(Val::Num).unwrap_or(Val::Nil),
        Some("boolean") => Val::Bool(value.eq_ignore_ascii_case("true")),
        Some("global") => resolve_global_path(state, value),
        // Auto-detect numbers when type is not specified (WoW behavior)
        None if value.parse::<f64>().is_ok() => Val::Num(value.parse().unwrap()),
        _ => create_string(state, value),
    }
}

// ---------------------------------------------------------------------------
// Small utility helpers
// ---------------------------------------------------------------------------

pub(super) fn template_child_name(name: Option<&str>, subst_parent: &str) -> String {
    name.map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__tpl_{}", crate::loader::helpers::rand_id()))
}

pub(super) fn build_child_inherits(
    intrinsic: Option<&str>,
    inherits: Option<&str>,
) -> Option<String> {
    match (intrinsic, inherits.filter(|value| !value.trim().is_empty())) {
        (Some(base), Some(inherits)) => Some(format!("{base}, {inherits}")),
        (Some(base), None) => Some(base.to_string()),
        (None, Some(inherits)) => Some(inherits.to_string()),
        (None, None) => None,
    }
}

pub(super) fn frame_lookup_name(state: &LuaState, frame_id: u64) -> String {
    borrow_state(state)
        .ok()
        .and_then(|sim| {
            sim.widgets
                .get(frame_id)
                .and_then(|frame| frame.name.clone())
        })
        .unwrap_or_else(|| format!("__frame_{frame_id}"))
}

pub(super) fn sim_state_rc(state: &LuaState) -> LuaResult<Rc<RefCell<crate::lua_api::SimState>>> {
    state
        .app_data::<crate::lua_api::env::WowLuaAppData>()
        .map(|app| app.sim_state.clone())
        .ok_or_else(|| rilua::runtime_error("missing WowLuaAppData"))
}

fn template_child_type<'a>(
    frame: &'a crate::xml::FrameXml,
    tag: &'static str,
) -> Option<(&'a crate::xml::FrameXml, &'static str, Option<&'static str>)> {
    match tag {
        "DropDownToggleButton" => Some((frame, "Button", Some("DropDownToggleButton"))),
        "EventButton" => Some((frame, "Button", Some("EventButton"))),
        _ => crate::xml::widget_type_for_tag(tag)
            .map(|(widget_type, intrinsic)| (frame, widget_type, intrinsic)),
    }
}

fn resolve_inherited_string(
    frame: &crate::xml::FrameXml,
    project: impl Fn(&crate::xml::FrameXml) -> Option<&String>,
) -> Option<String> {
    if let Some(value) = project(frame) {
        return Some(value.clone());
    }
    let inherits = frame.inherits.as_deref()?;
    crate::xml::get_template_chain(inherits)
        .iter()
        .find_map(|entry| project(&entry.frame).cloned())
}
