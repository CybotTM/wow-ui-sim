//! Runtime template chain application: applies XML template inheritance to
//! frames created via `CreateFrame("Frame", name, parent, "TemplateName")`.

use super::helpers::{append_parent_array_entry, apply_frame_mixins, resolve_global_path};
use crate::lua_api::LoaderEnv;
use crate::lua_api::methods::{
    borrow_lua, borrow_state, borrow_state_mut, create_string, frame_ref, state_handle, table_set,
};
use crate::lua_api::script_helpers::{get_script, set_script};
use crate::widget::WidgetType;
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
        create_template_child_frames(
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
        ensure_runtime_button_texture_slots(state, frame_id, &entry.frame)?;
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
    apply_runtime_template_loader_effects(
        state,
        frame_name,
        frame_name,
        &crate::xml::FrameXml::default(),
        Some(inherits),
    )?;
    apply_runtime_template_direct_properties(state_rc, frame_id, inherits, frame_name);
    if fire_on_load {
        fire_frame_on_load(state, frame_id)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Child frame creation
// ---------------------------------------------------------------------------

pub(super) fn create_template_child_frames(
    state: &mut LuaState,
    state_rc: &Rc<RefCell<crate::lua_api::SimState>>,
    parent_id: u64,
    parent_name: &str,
    subst_parent: &str,
    frame: &crate::xml::FrameXml,
) -> LuaResult<()> {
    frame.try_for_each_frame_element(|child_frame, child_tag| {
        create_template_child_frame(
            state,
            state_rc,
            parent_id,
            parent_name,
            subst_parent,
            child_frame,
            child_tag,
        )?;
        Ok::<(), rilua::LuaError>(())
    })?;

    let Some(scroll_child) = frame.scroll_child() else {
        return Ok(());
    };

    let mut registered_scroll_child = false;
    for child in &scroll_child.children {
        let Some((child_frame, child_tag)) = child.as_frame_data() else {
            continue;
        };
        let child_id = create_template_child_frame(
            state,
            state_rc,
            parent_id,
            parent_name,
            subst_parent,
            child_frame,
            child_tag,
        )?;
        if !registered_scroll_child && let Some(child_id) = child_id {
            let mut sim = borrow_state_mut(state)?;
            crate::lua_api::frame::methods::widget_scroll::assign_scroll_child(
                &mut sim, parent_id, child_id, false,
            );
            registered_scroll_child = true;
        }
    }

    Ok(())
}

fn create_template_child_frame(
    state: &mut LuaState,
    state_rc: &Rc<RefCell<crate::lua_api::SimState>>,
    parent_id: u64,
    _parent_name: &str,
    subst_parent: &str,
    child_frame: &crate::xml::FrameXml,
    child_tag: &'static str,
) -> LuaResult<Option<u64>> {
    let Some((frame, widget_type_name, intrinsic)) = template_child_type(child_frame, child_tag)
    else {
        return Ok(None);
    };
    let child_name = template_child_name(frame.name.as_deref(), subst_parent);
    let child_id =
        instantiate_template_child(state, parent_id, frame, widget_type_name, &child_name)?;
    assign_child_parent_refs(state, parent_id, child_id, frame);
    apply_child_template_properties(state, child_id, frame, intrinsic)?;

    let child_subst = if frame.name.is_some() {
        child_name.as_str()
    } else {
        subst_parent
    };
    create_template_child_frames(state, state_rc, child_id, &child_name, child_subst, frame)?;

    let inherited_chain = build_child_inherits(intrinsic, frame.inherits.as_deref());
    apply_runtime_child_direct_properties(state_rc, child_id, frame, &child_name);
    ensure_runtime_button_texture_slots(state, child_id, frame)?;
    apply_runtime_template_loader_effects(
        state,
        &child_name,
        child_subst,
        frame,
        inherited_chain.as_deref(),
    )?;
    fire_frame_on_load(state, child_id)?;
    Ok(Some(child_id))
}

fn instantiate_template_child(
    state: &mut LuaState,
    parent_id: u64,
    frame: &crate::xml::FrameXml,
    widget_type_name: &str,
    child_name: &str,
) -> LuaResult<u64> {
    crate::lua_api::globals::create_frame::create_frame_instance(
        state,
        WidgetType::from_str(widget_type_name).ok_or_else(|| {
            rilua::runtime_error(format!("unknown widget type '{widget_type_name}'"))
        })?,
        widget_type_name,
        Some(child_name.to_owned()),
        Some(parent_id),
        true,
        frame.xml_id,
    )
}

fn assign_child_parent_refs(
    state: &mut LuaState,
    parent_id: u64,
    child_id: u64,
    frame: &crate::xml::FrameXml,
) {
    if let Some(parent_key) = resolve_inherited_string(frame, |t| t.parent_key.as_ref()) {
        let _ = crate::lua_api::globals::template::assign_parent_key(
            state,
            parent_id,
            &parent_key,
            child_id,
        );
    }
    if let Some(parent_array) = resolve_inherited_string(frame, |t| t.parent_array.as_ref()) {
        append_parent_array_entry(state, parent_id, &parent_array, child_id);
    }
}

fn apply_child_template_properties(
    state: &mut LuaState,
    child_id: u64,
    frame: &crate::xml::FrameXml,
    intrinsic: Option<&str>,
) -> LuaResult<()> {
    let inherited_chain = build_child_inherits(intrinsic, frame.inherits.as_deref());
    if let Some(chain) = inherited_chain.as_deref() {
        apply_runtime_template_chain(state, child_id, Some(chain), false)?;
    }
    if let Some(intrinsic) = intrinsic {
        crate::lua_api::globals::template::set_intrinsic(state, child_id, intrinsic);
    }
    apply_frame_mixins(state, child_id, frame.combined_mixin().as_deref());
    apply_template_key_values(state, child_id, frame.all_key_values());
    if let Some(scripts) = frame.scripts() {
        apply_template_scripts(state, child_id, scripts)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Loader effects
// ---------------------------------------------------------------------------

pub(super) fn apply_runtime_template_loader_effects(
    state: &mut LuaState,
    frame_name: &str,
    name_parent: &str,
    frame: &crate::xml::FrameXml,
    inherits: Option<&str>,
) -> LuaResult<()> {
    let loader_env = LoaderEnv::from_parts_active(borrow_lua(state)?, state_handle(state)?, state);
    let inherits = inherits.unwrap_or("");
    let mut timing = crate::loader::LoadTiming::default();
    apply_loader_chain_layers(&loader_env, inherits, frame_name, name_parent, &mut timing)?;
    apply_loader_frame_extras(
        &loader_env,
        frame,
        frame_name,
        name_parent,
        inherits,
        &mut timing,
    )
}

fn apply_loader_chain_layers(
    loader_env: &LoaderEnv,
    inherits: &str,
    frame_name: &str,
    name_parent: &str,
    timing: &mut crate::loader::LoadTiming,
) -> LuaResult<()> {
    for entry in &*crate::xml::get_template_chain(inherits) {
        crate::loader::xml_layer_batch::create_layer_children_batched_with_name_parent(
            loader_env,
            &entry.frame,
            frame_name,
            name_parent,
            timing,
        )
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    }
    Ok(())
}

fn apply_loader_frame_extras(
    loader_env: &LoaderEnv,
    frame: &crate::xml::FrameXml,
    frame_name: &str,
    name_parent: &str,
    inherits: &str,
    timing: &mut crate::loader::LoadTiming,
) -> LuaResult<()> {
    crate::loader::xml_layer_batch::create_layer_children_batched_with_name_parent(
        loader_env,
        frame,
        frame_name,
        name_parent,
        timing,
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::button::apply_button_textures(loader_env, frame, frame_name, inherits)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::button::apply_button_text(loader_env, frame, frame_name, inherits)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::xml_frame_extras::apply_animation_groups(
        loader_env, frame, frame_name, inherits,
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::xml_frame_extras::apply_bar_texture(loader_env, frame, frame_name, inherits)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::xml_frame_extras::init_action_bar_tables(loader_env, frame, frame_name);
    Ok(())
}

// ---------------------------------------------------------------------------
// Button texture slots
// ---------------------------------------------------------------------------

pub(super) fn ensure_runtime_button_texture_slots(
    state: &mut LuaState,
    frame_id: u64,
    frame: &crate::xml::FrameXml,
) -> LuaResult<()> {
    let is_button = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(frame_id)
            .map(|widget| {
                matches!(
                    widget.widget_type,
                    WidgetType::Button | WidgetType::CheckButton
                )
            })
            .unwrap_or(false)
    };
    if !is_button {
        return Ok(());
    }

    let slots = [
        ("NormalTexture", frame.normal_texture()),
        ("PushedTexture", frame.pushed_texture()),
        ("HighlightTexture", frame.highlight_texture()),
        ("DisabledTexture", frame.disabled_texture()),
    ];
    let mut sim = borrow_state_mut(state)?;
    for (key, texture) in slots {
        if texture.is_some() {
            crate::lua_api::frame::methods::methods_helpers::get_or_create_button_texture(
                &mut sim, frame_id, key,
            );
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Direct property application
// ---------------------------------------------------------------------------

fn apply_runtime_template_direct_properties(
    state: &Rc<RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    inherits: &str,
    frame_name: &str,
) {
    let frame = crate::xml::FrameXml::default();
    apply_runtime_child_direct_properties_with_inherits(
        state, frame_id, &frame, inherits, frame_name,
    );
}

fn apply_runtime_child_direct_properties(
    state: &Rc<RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    frame: &crate::xml::FrameXml,
    frame_name: &str,
) {
    let inherits = frame.inherits.as_deref().unwrap_or("");
    apply_runtime_child_direct_properties_with_inherits(
        state, frame_id, frame, inherits, frame_name,
    );
}

fn apply_runtime_child_direct_properties_with_inherits(
    state: &Rc<RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    frame: &crate::xml::FrameXml,
    inherits: &str,
    frame_name: &str,
) {
    crate::lua_api::globals::template::direct::apply_xml_size(state, frame_id, frame, inherits);
    crate::lua_api::globals::template::direct::apply_xml_anchors(
        state, frame_id, frame, inherits, frame_name,
    );
    crate::lua_api::globals::template::direct::apply_xml_hidden(state, frame_id, frame, inherits);
    crate::lua_api::globals::template::direct::apply_xml_propagate_mouse_input(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_clips_children(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_set_all_points(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_frame_level(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_frame_strata(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_protected(
        state, frame_id, frame, inherits,
    );
}

// ---------------------------------------------------------------------------
// OnLoad firing
// ---------------------------------------------------------------------------

pub(super) fn fire_frame_on_load(state: &mut LuaState, frame_id: u64) -> LuaResult<()> {
    let frame = frame_ref(state, frame_id)?;
    let intrinsic = crate::lua_api::methods::table_get_static(state, frame, "OnLoad_Intrinsic");
    call_handler_with_frame(state, intrinsic, frame)?;
    if let Some(on_load) = crate::lua_api::script_helpers::get_script(state, frame_id, "OnLoad") {
        call_handler_with_frame(state, on_load, frame)?;
    }
    Ok(())
}

fn call_handler_with_frame(state: &mut LuaState, handler: Val, frame: Val) -> LuaResult<()> {
    let Val::Function(_) = handler else {
        return Ok(());
    };
    match crate::lua_api::script_helpers::call_void_function_with_fallback_state(
        state,
        handler,
        &[frame],
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(rilua::runtime_error(err)),
    }
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
        install_fast_handler(state, frame_id, handler_name, install)?;
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
    Method(&'a str),
    ParentMethod(&'a str),
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
    FunctionWithGlobalArg {
        function_name: &'a str,
        arg_path: &'a str,
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
        let handler = if let Some(method_name) = script.method.as_deref() {
            FastHandlerRef::Method(method_name)
        } else if let Some(function_name) = script.function.as_deref() {
            FastHandlerRef::Function(function_name)
        } else if let Some(body) = script.body.as_deref() {
            parse_inline_fast_handler(handler_name, body)?
        } else {
            FastHandlerRef::NoOp
        };
        let install = match script.intrinsic_order.as_deref() {
            Some("precall" | "postcall") => FastScriptInstall::Intrinsic(handler),
            Some(_) => return None,
            None => match script.inherit.as_deref() {
                Some("append") => FastScriptInstall::Chain {
                    handler,
                    new_first: true,
                },
                Some("prepend") => FastScriptInstall::Chain {
                    handler,
                    new_first: false,
                },
                Some(_) => return None,
                None => FastScriptInstall::Set(handler),
            },
        };
        handlers.push((handler_name, install));
    }
    Some(())
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

fn build_method_handler(state: &mut LuaState, method_name: &str) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local method_name = ...
            return function(self, ...)
                return self[method_name](self, ...)
            end
        "#,
        "template-method-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name],
    )
}

fn build_parent_method_handler(state: &mut LuaState, method_name: &str) -> LuaResult<Val> {
    build_ancestor_method_handler(state, method_name, 1)
}

fn build_ancestor_method_handler(
    state: &mut LuaState,
    method_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local method_name, depth = ...
            return function(self, ...)
                local target = self
                for _ = 1, depth do
                    target = target and target:GetParent()
                end
                if not target then
                    return
                end
                return target[method_name](target, ...)
            end
        "#,
        "template-ancestor-method-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[method_name, Val::Num(depth as f64)],
    )
}

fn build_global_method_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local target, method_name = ...
            return function(self, ...)
                if not target then
                    return
                end
                return target[method_name](target, ...)
            end
        "#,
        "template-global-method-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name],
    )
}

fn build_global_method_with_self_string_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local target, method_name, arg = ...
            return function(self, ...)
                if not target then
                    return
                end
                return target[method_name](target, self, arg)
            end
        "#,
        "template-global-method-self-string-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let arg = create_string(state, arg);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name, arg],
    )
}

fn build_global_method_then_assign_handler(
    state: &mut LuaState,
    target_path: &str,
    method_name: &str,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local target, method_name, field_name, assigned_value = ...
            return function(self, ...)
                if target then
                    target[method_name](target)
                end
                self[field_name] = assigned_value
            end
        "#,
        "template-global-method-assign-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, target_path);
    let method_name = create_string(state, method_name);
    let field_name = create_string(state, field);
    let assigned_value = fast_literal_value(state, value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, method_name, field_name, assigned_value],
    )
}

fn build_register_for_clicks_handler(
    state: &mut LuaState,
    first: &str,
    second: Option<&str>,
    third: Option<&str>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local first, second, third = ...
            return function(self, ...)
                if third ~= nil then
                    self:RegisterForClicks(first, second, third)
                elseif second ~= nil then
                    self:RegisterForClicks(first, second)
                else
                    self:RegisterForClicks(first)
                end
            end
        "#,
        "template-register-for-clicks",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let first = create_string(state, first);
    let second = second
        .map(|value| create_string(state, value))
        .unwrap_or(Val::Nil);
    let third = third
        .map(|value| create_string(state, value))
        .unwrap_or(Val::Nil);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[first, second, third],
    )
}

fn build_register_for_drag_handler(state: &mut LuaState, button: &str) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local button = ...
            return function(self, ...)
                self:RegisterForDrag(button)
            end
        "#,
        "template-register-for-drag",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let button = create_string(state, button);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[button])
}

fn build_set_alpha_handler(state: &mut LuaState, alpha: f64) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local alpha = ...
            return function(self, ...)
                self:SetAlpha(alpha)
            end
        "#,
        "template-set-alpha",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[Val::Num(alpha)],
    )
}

fn build_fast_handler(
    state: &mut LuaState,
    handler_ref: FastHandlerRef<'_>,
) -> LuaResult<Option<Val>> {
    match handler_ref {
        FastHandlerRef::NoOp => Ok(None),
        FastHandlerRef::Sequence2(parts) => {
            let (first_ref, second_ref) = &*parts;
            let first = build_fast_handler(state, first_ref.clone())?;
            let second = build_fast_handler(state, second_ref.clone())?;
            match (first, second) {
                (Some(first), Some(second)) => {
                    build_chained_handler(state, first, second, "inline-sequence", false).map(Some)
                }
                (Some(first), None) => Ok(Some(first)),
                (None, Some(second)) => Ok(Some(second)),
                (None, None) => Ok(None),
            }
        }
        FastHandlerRef::Method(method_name) => build_method_handler(state, method_name).map(Some),
        FastHandlerRef::ParentMethod(method_name) => {
            build_parent_method_handler(state, method_name).map(Some)
        }
        FastHandlerRef::GrandparentMethod(method_name) => {
            build_ancestor_method_handler(state, method_name, 2).map(Some)
        }
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
        FastHandlerRef::GlobalMethodThenAssignLiteral {
            target_path,
            method_name,
            field,
            value,
        } => build_global_method_then_assign_handler(state, target_path, method_name, field, value)
            .map(Some),
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
        FastHandlerRef::FunctionWithGlobalArg {
            function_name,
            arg_path,
        } => build_function_handler_with_global_arg(state, function_name, arg_path).map(Some),
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
        FastHandlerRef::RegisterForClicks {
            first,
            second,
            third,
        } => build_register_for_clicks_handler(state, first, second, third).map(Some),
        FastHandlerRef::RegisterForDrag(button) => {
            build_register_for_drag_handler(state, button).map(Some)
        }
        FastHandlerRef::SetAlpha(alpha) => build_set_alpha_handler(state, alpha).map(Some),
        FastHandlerRef::SetFrameLevelFromParent(delta) => {
            build_set_frame_level_from_parent_handler(state, delta).map(Some)
        }
        FastHandlerRef::AssignAncestorRef { field, depth } => {
            build_ancestor_assignment_handler(state, field, depth).map(Some)
        }
        FastHandlerRef::AssignLiteral { field, value } => {
            build_assignment_handler(state, field, value).map(Some)
        }
        FastHandlerRef::AssignParentField { field, value } => {
            build_parent_assignment_handler(state, field, value).map(Some)
        }
    }
}

fn install_fast_handler(
    state: &mut LuaState,
    frame_id: u64,
    handler_name: &'static str,
    install: FastScriptInstall<'_>,
) -> LuaResult<()> {
    match install {
        FastScriptInstall::Set(handler_ref) => {
            if let Some(handler) = build_fast_handler(state, handler_ref)? {
                set_script(state, frame_id, handler_name, handler);
            }
        }
        FastScriptInstall::Intrinsic(handler_ref) => {
            let Some(handler) = build_fast_handler(state, handler_ref)? else {
                return Ok(());
            };
            let frame = frame_ref(state, frame_id)?;
            let intrinsic_name = format!("{handler_name}_Intrinsic");
            table_set(state, frame, &intrinsic_name, handler);
        }
        FastScriptInstall::Chain { handler, new_first } => {
            let Some(new_handler) = build_fast_handler(state, handler)? else {
                return Ok(());
            };
            let Some(old_handler) = get_script(state, frame_id, handler_name) else {
                set_script(state, frame_id, handler_name, new_handler);
                return Ok(());
            };
            let chained =
                build_chained_handler(state, old_handler, new_handler, handler_name, new_first)?;
            set_script(state, frame_id, handler_name, chained);
        }
    }
    Ok(())
}

fn build_chained_handler(
    state: &mut LuaState,
    old_handler: Val,
    new_handler: Val,
    handler_name: &str,
    new_first: bool,
) -> LuaResult<Val> {
    let (first, second) = if new_first {
        (new_handler, old_handler)
    } else {
        (old_handler, new_handler)
    };
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local handler_name, first, second = ...
            local report = debug.getregistry()["__report_script_error"]
            return function(self, ...)
                if securecall then
                    securecall(first, self, ...)
                    securecall(second, self, ...)
                else
                    local ok1, err1 = pcall(first, self, ...)
                    local ok2, err2 = pcall(second, self, ...)
                    if not ok1 then
                        local name = self.GetName and self:GetName() or "?"
                        report("[script:" .. handler_name .. "] " .. name .. ": " .. tostring(err1))
                    end
                    if not ok2 then
                        local name = self.GetName and self:GetName() or "?"
                        report("[script:" .. handler_name .. "] " .. name .. ": " .. tostring(err2))
                    end
                end
            end
        "#,
        "template-chained-handler",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let handler_name = create_string(state, handler_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[handler_name, first, second],
    )
}

#[derive(Copy, Clone)]
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
    let (source, tag) = match kind {
        FunctionHandlerKind::NoArgs => (
            r#"
                local fn = ...
                return function(self, ...)
                    return fn()
                end
            "#,
            "template-inline-function-noargs",
        ),
        FunctionHandlerKind::SelfId => (
            r#"
                local fn = ...
                return function(self, ...)
                    return fn(self:GetID())
                end
            "#,
            "template-inline-function-self-id",
        ),
        FunctionHandlerKind::EventVarargs => (
            r#"
                local fn = ...
                return function(self, event, ...)
                    return fn(self, event, ...)
                end
            "#,
            "template-inline-function-event",
        ),
        FunctionHandlerKind::Button => (
            r#"
                local fn = ...
                return function(self, button, ...)
                    return fn(self, button, ...)
                end
            "#,
            "template-inline-function-button",
        ),
        FunctionHandlerKind::Elapsed => (
            r#"
                local fn = ...
                return function(self, elapsed, ...)
                    return fn(self, elapsed, ...)
                end
            "#,
            "template-inline-function-elapsed",
        ),
    };
    let builder = crate::loader::chunk_cache::load_chunk(state, source, tag)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(state, Val::Function(builder.gc_ref()), &[target])
}

fn build_function_handler_with_string_arg(
    state: &mut LuaState,
    function_name: &str,
    arg: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, arg = ...
            return function(self, ...)
                return fn(self, arg)
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

fn build_function_handler_with_global_arg(
    state: &mut LuaState,
    function_name: &str,
    arg_path: &str,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local fn, arg = ...
            return function(self, ...)
                return fn(arg)
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

fn build_ancestor_function_handler(
    state: &mut LuaState,
    function_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
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
        "#,
        "template-inline-function-ancestor",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(depth as f64)],
    )
}

fn build_set_frame_level_from_parent_handler(state: &mut LuaState, delta: i32) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local delta = ...
            return function(self, ...)
                local parent = self:GetParent()
                if not parent then
                    return
                end
                self:SetFrameLevel(parent:GetFrameLevel() + delta)
            end
        "#,
        "template-inline-set-frame-level",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[Val::Num(delta as f64)],
    )
}

fn build_ancestor_id_function_handler(
    state: &mut LuaState,
    function_name: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
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
        "#,
        "template-inline-function-ancestor-id",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let target = resolve_global_path(state, function_name);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[target, Val::Num(depth as f64)],
    )
}

fn build_ancestor_assignment_handler(
    state: &mut LuaState,
    field: &str,
    depth: usize,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, depth = ...
            return function(self, ...)
                local target = self
                for _ = 1, depth do
                    target = target and target:GetParent()
                end
                if not target then
                    return
                end
                self[field_name] = target
            end
        "#,
        "template-inline-assignment-ancestor",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let field_name = create_string(state, field);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, Val::Num(depth as f64)],
    )
}

fn parse_inline_fast_handler<'a>(
    _handler_name: &'static str,
    body: &'a str,
) -> Option<FastHandlerRef<'a>> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Some(FastHandlerRef::NoOp);
    }
    let stmt = trimmed.strip_suffix(';').unwrap_or(trimmed).trim();
    if stmt.is_empty() {
        return Some(FastHandlerRef::NoOp);
    }
    if let Some(sequence) = parse_inline_sequence(stmt) {
        return Some(sequence);
    }
    parse_inline_single_fast_handler(stmt)
}

fn parse_inline_single_fast_handler<'a>(stmt: &'a str) -> Option<FastHandlerRef<'a>> {
    if let Some(method_name) = parse_inline_self_method(stmt) {
        return Some(FastHandlerRef::Method(method_name));
    }
    if let Some(method_name) = parse_inline_parent_method(stmt) {
        return Some(FastHandlerRef::ParentMethod(method_name));
    }
    if let Some(method_name) = parse_inline_grandparent_method(stmt) {
        return Some(FastHandlerRef::GrandparentMethod(method_name));
    }
    if let Some((target_path, method_name, field, value)) =
        parse_inline_global_method_then_assign(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodThenAssignLiteral {
            target_path,
            method_name,
            field,
            value,
        });
    }
    if let Some((target_path, method_name)) = parse_inline_global_method(stmt) {
        return Some(FastHandlerRef::GlobalMethod {
            target_path,
            method_name,
        });
    }
    if let Some((target_path, method_name, arg)) =
        parse_inline_global_method_with_self_string_arg(stmt)
    {
        return Some(FastHandlerRef::GlobalMethodWithSelfStringArg {
            target_path,
            method_name,
            arg,
        });
    }
    if let Some((first, second, third)) = parse_inline_register_for_clicks(stmt) {
        return Some(FastHandlerRef::RegisterForClicks {
            first,
            second,
            third,
        });
    }
    if let Some(button) = parse_inline_register_for_drag(stmt) {
        return Some(FastHandlerRef::RegisterForDrag(button));
    }
    if let Some(alpha) = parse_inline_set_alpha(stmt) {
        return Some(FastHandlerRef::SetAlpha(alpha));
    }
    if let Some(delta) = parse_inline_set_frame_level_from_parent(stmt) {
        return Some(FastHandlerRef::SetFrameLevelFromParent(delta));
    }
    if let Some(assign) = parse_inline_ancestor_assignment(stmt) {
        return Some(assign);
    }
    if let Some(assign) = parse_inline_assignment(stmt) {
        return Some(assign);
    }
    if let Some(assign) = parse_inline_parent_assignment(stmt) {
        return Some(assign);
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::Function(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("()")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionNoArgs(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetID())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithSelfIdArg(function_name));
    }
    if let Some((function_name, arg)) = parse_inline_function_with_self_string_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithSelfStringArg { function_name, arg });
    }
    if let Some((function_name, arg_path)) = parse_inline_function_with_global_arg(stmt) {
        return Some(FastHandlerRef::FunctionWithGlobalArg {
            function_name,
            arg_path,
        });
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetParent())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithParentArg(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetParent():GetParent())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithGrandparentArg(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self:GetParent():GetID())")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithParentIdArg(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self, event, ...)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithEventVarargs(function_name));
    }
    if let Some(function_name) = stmt
        .strip_suffix("(self, button)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
    {
        return Some(FastHandlerRef::FunctionWithButton(function_name));
    }
    stmt.strip_suffix("(self, elapsed)")
        .map(str::trim)
        .filter(|name| is_fast_handler_path(name))
        .map(FastHandlerRef::FunctionWithElapsed)
}

fn parse_inline_sequence(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let parts = stmt
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let first = parse_inline_single_fast_handler(parts[0])?;
    let second = parse_inline_single_fast_handler(parts[1])?;
    Some(FastHandlerRef::Sequence2(Box::new((first, second))))
}

fn parse_inline_self_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:")
}

fn parse_inline_parent_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:GetParent():")
}

fn parse_inline_grandparent_method(stmt: &str) -> Option<&str> {
    parse_inline_method_call(stmt, "self:GetParent():GetParent():")
}

fn parse_inline_method_call<'a>(stmt: &'a str, prefix: &str) -> Option<&'a str> {
    let remainder = stmt.strip_prefix(prefix)?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let method_name = method_name.trim();
    (is_fast_identifier(method_name) && is_fast_passthrough_args(args)).then_some(method_name)
}

fn parse_inline_global_method(stmt: &str) -> Option<(&str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && is_fast_passthrough_args(args))
    .then_some((target_path, method_name))
}

fn parse_inline_global_method_with_self_string_arg(stmt: &str) -> Option<(&str, &str, &str)> {
    let (target_path, remainder) = stmt.rsplit_once(':')?;
    let (method_name, args) = remainder.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, raw_string_arg) = args.split_once(',')?;
    let target_path = target_path.trim();
    let method_name = method_name.trim();
    let arg = parse_single_string_literal(raw_string_arg.trim())?;
    (is_fast_handler_path(target_path)
        && is_fast_identifier(method_name)
        && self_arg.trim() == "self")
        .then_some((target_path, method_name, arg))
}

fn parse_inline_global_method_then_assign(
    stmt: &str,
) -> Option<(&str, &str, &str, FastLiteralValue<'_>)> {
    let (first, second) = stmt.split_once(';')?;
    let (target_path, method_name) = parse_inline_global_method(first.trim())?;
    let FastHandlerRef::AssignLiteral { field, value } = parse_inline_assignment(second.trim())?
    else {
        return None;
    };
    Some((target_path, method_name, field, value))
}

fn parse_inline_function_with_self_string_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let args = args.strip_suffix(')')?.trim();
    let (self_arg, raw_string_arg) = args.split_once(',')?;
    let function_name = function_name.trim();
    let arg = parse_single_string_literal(raw_string_arg.trim())?;
    (is_fast_handler_path(function_name) && self_arg.trim() == "self")
        .then_some((function_name, arg))
}

fn parse_inline_function_with_global_arg(stmt: &str) -> Option<(&str, &str)> {
    let (function_name, args) = stmt.split_once('(')?;
    let arg_path = args.strip_suffix(')')?.trim();
    let function_name = function_name.trim();
    (is_fast_handler_path(function_name) && is_fast_handler_path(arg_path))
        .then_some((function_name, arg_path))
}

fn parse_inline_register_for_clicks(stmt: &str) -> Option<(&str, Option<&str>, Option<&str>)> {
    let args = stmt
        .strip_prefix("self:RegisterForClicks(")?
        .strip_suffix(')')?
        .trim();
    let args = parse_string_literal_args(args)?;
    match args.as_slice() {
        [first] => Some((first, None, None)),
        [first, second] => Some((first, Some(second), None)),
        [first, second, third] => Some((first, Some(second), Some(third))),
        _ => None,
    }
}

fn parse_inline_register_for_drag(stmt: &str) -> Option<&str> {
    let args = stmt
        .strip_prefix("self:RegisterForDrag(")?
        .strip_suffix(')')?
        .trim();
    let args = parse_string_literal_args(args)?;
    match args.as_slice() {
        [button] => Some(button),
        _ => None,
    }
}

fn parse_inline_set_alpha(stmt: &str) -> Option<f64> {
    stmt.strip_prefix("self:SetAlpha(")?
        .strip_suffix(')')?
        .trim()
        .parse::<f64>()
        .ok()
}

fn parse_inline_set_frame_level_from_parent(stmt: &str) -> Option<i32> {
    let remainder = stmt
        .strip_prefix("self:SetFrameLevel(self:GetParent():GetFrameLevel()")?
        .trim();
    if let Some(remainder) = remainder.strip_suffix(')') {
        let remainder = remainder.trim();
        if remainder.is_empty() {
            return Some(0);
        }
        if let Some(delta) = remainder.strip_prefix('+') {
            return delta.trim().parse::<i32>().ok();
        }
        if let Some(delta) = remainder.strip_prefix('-') {
            return delta.trim().parse::<i32>().ok().map(|delta| -delta);
        }
    }
    None
}

fn parse_inline_ancestor_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (field, depth) =
        if let Some((field, _)) = stmt.strip_prefix("self.")?.split_once("= self:GetParent()") {
            let field = field.trim();
            if let Some(suffix) = stmt.trim().strip_prefix(&format!("self.{field} = ")) {
                let depth = if suffix.trim() == "self:GetParent()" {
                    1
                } else if suffix.trim() == "self:GetParent():GetParent()" {
                    2
                } else {
                    return None;
                };
                (field, depth)
            } else {
                return None;
            }
        } else {
            return None;
        };
    if !is_fast_identifier(field) {
        return None;
    }
    Some(FastHandlerRef::AssignAncestorRef { field, depth })
}

fn parse_inline_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (field, raw_value) = stmt.strip_prefix("self.")?.split_once('=')?;
    let field = field.trim();
    let raw_value = raw_value.trim();
    if !is_fast_identifier(field) {
        return None;
    }
    let value = if raw_value.eq("nil") {
        FastLiteralValue::Nil
    } else if raw_value.eq("true") {
        FastLiteralValue::Bool(true)
    } else if raw_value.eq("false") {
        FastLiteralValue::Bool(false)
    } else if let Ok(number) = raw_value.parse::<f64>() {
        FastLiteralValue::Number(number)
    } else if is_fast_handler_path(raw_value) {
        FastLiteralValue::Global(raw_value)
    } else {
        return None;
    };
    Some(FastHandlerRef::AssignLiteral { field, value })
}

fn parse_inline_parent_assignment(stmt: &str) -> Option<FastHandlerRef<'_>> {
    let (field, raw_value) = stmt.strip_prefix("self:GetParent().")?.split_once('=')?;
    let field = field.trim();
    let raw_value = raw_value.trim();
    if !is_fast_identifier(field) {
        return None;
    }
    let value = parse_fast_literal_value(raw_value)?;
    Some(FastHandlerRef::AssignParentField { field, value })
}

fn parse_string_literal_args(args: &str) -> Option<Vec<&str>> {
    if args.is_empty() {
        return Some(Vec::new());
    }
    let mut values = Vec::new();
    for part in args.split(',') {
        let part = part.trim();
        let value = part.strip_prefix('"')?.strip_suffix('"')?;
        values.push(value);
    }
    Some(values)
}

fn parse_single_string_literal(arg: &str) -> Option<&str> {
    arg.strip_prefix('"')?.strip_suffix('"')
}

fn is_fast_handler_path(path: &str) -> bool {
    path.split('.').all(is_fast_identifier)
}

fn is_fast_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(ch) if ch == '_' || ch.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn is_fast_passthrough_args(args: &str) -> bool {
    args.split(',')
        .map(str::trim)
        .all(|arg| arg.is_empty() || arg == "..." || is_fast_identifier(arg))
}

fn parse_fast_literal_value(raw_value: &str) -> Option<FastLiteralValue<'_>> {
    if raw_value.eq("nil") {
        Some(FastLiteralValue::Nil)
    } else if raw_value.eq("true") {
        Some(FastLiteralValue::Bool(true))
    } else if raw_value.eq("false") {
        Some(FastLiteralValue::Bool(false))
    } else if let Ok(number) = raw_value.parse::<f64>() {
        Some(FastLiteralValue::Number(number))
    } else if is_fast_handler_path(raw_value) {
        Some(FastLiteralValue::Global(raw_value))
    } else {
        None
    }
}

fn build_assignment_handler(
    state: &mut LuaState,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
        state,
        r#"
            local field_name, assigned_value = ...
            return function(self, ...)
                self[field_name] = assigned_value
            end
        "#,
        "template-inline-assignment",
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    let field_name = create_string(state, field);
    let assigned_value = fast_literal_value(state, value);
    crate::lua_api::methods::call_function_state(
        state,
        Val::Function(builder.gc_ref()),
        &[field_name, assigned_value],
    )
}

fn build_parent_assignment_handler(
    state: &mut LuaState,
    field: &str,
    value: FastLiteralValue<'_>,
) -> LuaResult<Val> {
    let builder = crate::loader::chunk_cache::load_chunk(
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
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
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
