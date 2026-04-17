//! Frame finalization: child creation, layers, animations, lifecycle scripts.

use std::time::Instant;

use crate::loader::LoadTiming;
use crate::loader::button::{apply_button_text, apply_button_textures};
use crate::loader::error::LoadError;
use crate::loader::xml_frame_extras::{
    apply_animation_groups, apply_bar_texture, init_action_bar_tables,
};
use crate::loader::xml_lifecycle::{LifecycleScripts, fire_lifecycle_scripts};
use crate::lua_api::LoaderEnv;

use super::create_frame_from_xml;

/// Create children, layers, animations, and fire lifecycle scripts with timing.
pub(super) fn finalize_frame(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    frame_id: u64,
    name: &str,
    inherits: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let finalize_start = Instant::now();
    create_children_and_finalize(env, frame, frame_id, name, inherits, timing)?;
    timing.xml_frame_finalize_time += finalize_start.elapsed();
    Ok(())
}

/// Create child frames, layer children, animations, and apply button/bar textures.
fn create_children_and_finalize(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    frame_id: u64,
    name: &str,
    inherits: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    create_child_frames(env, frame, name, inherits, timing)?;
    let layer_start = Instant::now();
    create_layer_children(env, frame, name, inherits, timing)?;
    timing.frame_layer_children_time += layer_start.elapsed();
    let anim_start = Instant::now();
    apply_animation_groups(env, frame, name, inherits)?;
    timing.frame_anim_time += anim_start.elapsed();
    let btn_start = Instant::now();
    apply_button_textures(env, frame, name, inherits)?;
    apply_button_text(env, frame, name, inherits)?;
    apply_bar_texture(env, frame, name, inherits)?;
    init_action_bar_tables(env, frame, name);
    timing.frame_button_time += btn_start.elapsed();
    let lifecycle = lifecycle_scripts(frame, inherits);
    if lifecycle.any() {
        let lc_start = Instant::now();
        fire_lifecycle_scripts(env, frame_id, name, lifecycle);
        timing.frame_lifecycle_time += lc_start.elapsed();
        timing.lifecycle_fire_count += 1;
    }
    Ok(())
}

fn lifecycle_scripts(frame: &crate::xml::FrameXml, inherits: &str) -> LifecycleScripts {
    let mut lifecycle = lifecycle_scripts_for_frame(frame);
    if inherits.is_empty() || (lifecycle.on_load && lifecycle.on_show) {
        return lifecycle;
    }

    let (inherited_on_load, inherited_on_show) = crate::xml::get_template_lifecycle_flags(inherits);
    lifecycle.on_load |= inherited_on_load;
    lifecycle.on_show |= inherited_on_show;

    lifecycle
}

fn lifecycle_scripts_for_frame(frame: &crate::xml::FrameXml) -> LifecycleScripts {
    let Some(scripts) = frame.scripts() else {
        return LifecycleScripts::default();
    };
    LifecycleScripts {
        on_load: !scripts.on_load.is_empty(),
        on_show: !scripts.on_show.is_empty(),
    }
}

/// Create textures and fontstrings from the frame's Layers.
fn create_layer_children(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    name: &str,
    _inherits: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    crate::loader::xml_layer_batch::create_layer_children_batched(env, frame, name, timing)
}

/// Map a FrameElement to its (FrameXml, widget_type, intrinsic_name) triple.
///
/// FrameElement-specific overrides vs the shared `widget_type_for_tag`:
/// - `DropDownToggleButton` / `EventButton` get intrinsic names (shared treats them as unknown)
pub(crate) fn frame_element_to_type<'a>(
    child_frame: &'a crate::xml::FrameXml,
    child_tag: &'static str,
) -> Option<(&'a crate::xml::FrameXml, &'static str, Option<&'static str>)> {
    let (wt, intrinsic) = match child_tag {
        "DropDownToggleButton" => ("Button", Some("DropDownToggleButton")),
        "EventButton" => ("Button", Some("EventButton")),
        _ => crate::xml::widget_type_for_tag(child_tag)?,
    };
    Some((child_frame, wt, intrinsic))
}

/// Recursively create child frames and assign parentKey references.
fn create_child_frames(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    name: &str,
    _inherits: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    frame.try_for_each_frame_element(|child_frame, child_tag| {
        create_single_child_frame(env, child_frame, child_tag, name, timing)
    })?;
    // ScrollChild children are parented to the ScrollFrame just like regular children,
    // but the first ScrollChild element also becomes the ScrollFrame's scroll child.
    if let Some(scroll_child) = frame.scroll_child() {
        create_scroll_child_elements(env, &scroll_child.children, name, timing)?;
    }
    Ok(())
}

fn create_scroll_child_elements(
    env: &LoaderEnv<'_>,
    elements: &[crate::xml::FrameElement],
    parent_name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let mut registered_scroll_child = false;
    for child in elements {
        let Some((child_frame, child_tag)) = child.as_frame_data() else {
            continue;
        };
        let (child_frame, child_type, intrinsic) =
            match frame_element_to_type(child_frame, child_tag) {
                Some(triple) => triple,
                None => continue,
            };
        let child_name = create_frame_from_xml(
            env,
            child_frame,
            child_type,
            Some(parent_name),
            intrinsic,
            timing,
        )?;

        if let (Some(actual_child_name), Some(parent_key)) =
            (child_name.as_deref(), &child_frame.parent_key)
        {
            assign_parent_key(env, parent_name, parent_key, actual_child_name);
        }

        if !registered_scroll_child {
            if let Some(actual_child_name) = child_name.as_deref() {
                register_scroll_child(env, parent_name, actual_child_name);
                registered_scroll_child = true;
            }
        }
    }
    Ok(())
}

/// Create a single child frame from a FrameElement and assign parentKey.
fn create_single_child_frame(
    env: &LoaderEnv<'_>,
    child_frame: &crate::xml::FrameXml,
    child_tag: &'static str,
    parent_name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let (child_frame, child_type, intrinsic) = match frame_element_to_type(child_frame, child_tag) {
        Some(triple) => triple,
        None => return Ok(()),
    };
    let child_name = create_frame_from_xml(
        env,
        child_frame,
        child_type,
        Some(parent_name),
        intrinsic,
        timing,
    )?;
    if let (Some(actual_child_name), Some(parent_key)) = (child_name, &child_frame.parent_key) {
        assign_parent_key(env, parent_name, parent_key, actual_child_name.as_str());
    }
    Ok(())
}

fn assign_parent_key(env: &LoaderEnv<'_>, parent_name: &str, parent_key: &str, child_name: &str) {
    let ids = {
        let state = env.state().borrow();
        (
            state.widgets.get_id_by_name(parent_name),
            state.widgets.get_id_by_name(child_name),
        )
    };
    let (Some(parent_id), Some(child_id)) = ids else {
        return;
    };

    let _ = env.with_state(|state| {
        let _ = crate::lua_api::globals::template::assign_parent_key(
            state, parent_id, parent_key, child_id,
        );
        Ok::<(), crate::Error>(())
    });
}

fn register_scroll_child(env: &LoaderEnv<'_>, parent_name: &str, child_name: &str) {
    let mut state = env.state().borrow_mut();
    let Some(parent_id) = state.widgets.get_id_by_name(parent_name) else {
        return;
    };
    let Some(child_id) = state.widgets.get_id_by_name(child_name) else {
        return;
    };
    crate::lua_api::frame::methods::widget_scroll::assign_scroll_child(
        &mut state, parent_id, child_id, false,
    );
}
