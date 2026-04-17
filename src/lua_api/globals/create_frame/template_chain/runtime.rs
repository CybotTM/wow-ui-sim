use crate::lua_api::LoaderEnv;
use crate::lua_api::methods::{
    borrow_lua, borrow_state, borrow_state_mut, frame_ref, state_handle,
};
use crate::lua_api::script_helpers::get_script;
use crate::widget::WidgetType;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn create_template_child_frames(
    state: &mut LuaState,
    state_rc: &Rc<RefCell<crate::lua_api::SimState>>,
    parent_id: u64,
    parent_name: &str,
    subst_parent: &str,
    frame: &crate::xml::FrameXml,
) -> LuaResult<()> {
    create_direct_child_frames(state, state_rc, parent_id, parent_name, subst_parent, frame)?;
    create_scroll_child_frames(state, state_rc, parent_id, parent_name, subst_parent, frame)?;
    Ok(())
}

fn create_direct_child_frames(
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
    })
}

fn create_scroll_child_frames(
    state: &mut LuaState,
    state_rc: &Rc<RefCell<crate::lua_api::SimState>>,
    parent_id: u64,
    parent_name: &str,
    subst_parent: &str,
    frame: &crate::xml::FrameXml,
) -> LuaResult<()> {
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
    let Some((frame, widget_type_name, intrinsic)) =
        super::template_child_type(child_frame, child_tag)
    else {
        return Ok(None);
    };
    let child_name = super::template_child_name(frame.name.as_deref(), subst_parent);
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

    let inherited_chain = super::build_child_inherits(intrinsic, frame.inherits.as_deref());
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
    if let Some(parent_key) = super::resolve_inherited_string(frame, |t| t.parent_key.as_ref()) {
        let _ = crate::lua_api::globals::template::assign_parent_key(
            state,
            parent_id,
            &parent_key,
            child_id,
        );
    }
    if let Some(parent_array) = super::resolve_inherited_string(frame, |t| t.parent_array.as_ref())
    {
        crate::lua_api::globals::create_frame::append_parent_array_entry(
            state,
            parent_id,
            &parent_array,
            child_id,
        );
    }
}

fn apply_child_template_properties(
    state: &mut LuaState,
    child_id: u64,
    frame: &crate::xml::FrameXml,
    intrinsic: Option<&str>,
) -> LuaResult<()> {
    let inherited_chain = super::build_child_inherits(intrinsic, frame.inherits.as_deref());
    if let Some(chain) = inherited_chain.as_deref() {
        super::apply_runtime_template_chain(state, child_id, Some(chain), false)?;
    }
    if let Some(intrinsic) = intrinsic {
        crate::lua_api::globals::template::set_intrinsic(state, child_id, intrinsic);
    }
    crate::lua_api::globals::create_frame::apply_frame_mixins(
        state,
        child_id,
        frame.combined_mixin().as_deref(),
    );
    super::apply_template_key_values(state, child_id, frame.all_key_values());
    if let Some(scripts) = frame.scripts() {
        super::apply_template_scripts(state, child_id, scripts)?;
    }
    Ok(())
}

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

pub(super) fn apply_runtime_template_direct_properties(
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

pub(super) fn fire_frame_on_load(state: &mut LuaState, frame_id: u64) -> LuaResult<()> {
    let frame = frame_ref(state, frame_id)?;
    let intrinsic = crate::lua_api::methods::table_get_static(state, frame, "OnLoad_Intrinsic");
    call_handler_with_frame(state, intrinsic, frame)?;
    if let Some(on_load) = get_script(state, frame_id, "OnLoad") {
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
