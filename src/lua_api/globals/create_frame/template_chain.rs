//! Runtime template chain application: applies XML template inheritance to
//! frames created via `CreateFrame("Frame", name, parent, "TemplateName")`.

use super::helpers::{append_parent_array_entry, apply_frame_mixins, resolve_global_path};
use crate::lua_api::LoaderEnv;
use crate::lua_api::methods::{
    borrow_lua, borrow_state, borrow_state_mut, create_string, extract_frame_id, frame_ref,
    state_handle,
};
use crate::lua_api::script_helpers::set_script;
use crate::widget::WidgetType;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::cell::RefCell;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub(crate) fn apply_runtime_template_chain(
    state: &mut LuaState,
    frame_id: u64,
    inherits: Option<&str>,
    fire_on_load: bool,
) -> LuaResult<()> {
    let Some(inherits) = inherits.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };
    let chain = crate::xml::get_template_chain(inherits);
    if chain.is_empty() {
        return Ok(());
    }

    let state_rc = sim_state_rc(state)?;
    let frame_name = frame_lookup_name(state, frame_id);
    apply_template_parent_array(state, frame_id, &chain);
    apply_chain_entries(state, frame_id, &chain)?;

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

fn apply_template_parent_array(
    state: &mut LuaState,
    frame_id: u64,
    chain: &[crate::xml::TemplateEntry],
) {
    let template_parent_array = chain
        .iter()
        .find_map(|entry| entry.frame.parent_array.as_deref());
    let parent_id = borrow_state(state)
        .ok()
        .and_then(|sim| sim.widgets.get(frame_id).and_then(|frame| frame.parent_id));
    if let Some(parent_array) = template_parent_array
        && let Some(parent_id) = parent_id
    {
        append_parent_array_entry(state, parent_id, parent_array, frame_id);
    }
}

fn apply_chain_entries(
    state: &mut LuaState,
    frame_id: u64,
    chain: &[crate::xml::TemplateEntry],
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

fn apply_template_scripts(
    state: &mut LuaState,
    frame_id: u64,
    scripts: &crate::xml::ScriptsXml,
) -> LuaResult<()> {
    if apply_method_only_scripts_fast(state, frame_id, scripts)? {
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

fn apply_method_only_scripts_fast(
    state: &mut LuaState,
    frame_id: u64,
    scripts: &crate::xml::ScriptsXml,
) -> LuaResult<bool> {
    let Some(handlers) = collect_method_only_handlers(scripts) else {
        return Ok(false);
    };
    if handlers.is_empty() {
        return Ok(true);
    }

    for (handler_name, method_name) in handlers {
        let handler = build_method_handler(state, method_name)?;
        set_script(state, frame_id, handler_name, handler);
    }

    Ok(true)
}

fn collect_method_only_handlers(
    scripts: &crate::xml::ScriptsXml,
) -> Option<Vec<(&'static str, &str)>> {
    let mut handlers = Vec::new();
    collect_method_only_handler_group(&mut handlers, base_method_only_handlers(scripts))?;
    collect_method_only_handler_group(&mut handlers, pointer_method_only_handlers(scripts))?;
    collect_method_only_handler_group(&mut handlers, text_method_only_handlers(scripts))?;
    collect_method_only_handler_group(&mut handlers, state_method_only_handlers(scripts))?;
    Some(handlers)
}

type MethodOnlyScript<'a> = (&'static str, Option<&'a crate::xml::ScriptBodyXml>);

fn collect_method_only_handler_group<'a>(
    handlers: &mut Vec<(&'static str, &'a str)>,
    group: impl IntoIterator<Item = MethodOnlyScript<'a>>,
) -> Option<()> {
    for (handler_name, script) in group {
        let Some(script) = script else {
            continue;
        };
        if script.intrinsic_order.is_some() || script.inherit.is_some() || script.function.is_some()
        {
            return None;
        }
        let method_name = script.method.as_deref()?;
        if script
            .body
            .as_deref()
            .is_some_and(|body| !body.trim().is_empty())
        {
            return None;
        }
        handlers.push((handler_name, method_name));
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
