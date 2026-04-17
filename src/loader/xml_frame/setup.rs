//! Frame setup: CreateFrame execution, XML property application, error recovery.

use std::time::Instant;

use crate::loader::LoadTiming;
use crate::loader::error::LoadError;
use crate::lua_api::LoaderEnv;

pub(super) struct SetupFrame<'a> {
    pub(super) lua_code: &'a str,
    pub(super) name: &'a str,
    pub(super) initial_hidden: bool,
    pub(super) frame: &'a crate::xml::FrameXml,
    pub(super) inherits: &'a str,
    pub(super) parent: &'a str,
    pub(super) intrinsic_base: Option<&'a str>,
}

/// Execute CreateFrame Lua, apply XML properties, and record setup timing.
pub(super) fn setup_frame(
    env: &LoaderEnv<'_>,
    timing: &mut LoadTiming,
    setup: SetupFrame<'_>,
) -> Result<(), LoadError> {
    let setup_start = Instant::now();
    let exec_start = Instant::now();
    match exec_create_frame_code(env, setup.lua_code, setup.name, setup.initial_hidden) {
        Ok(()) => {}
        Err(error)
            if recover_frame_after_loader_vm_error(
                env,
                setup.name,
                setup.frame,
                setup.parent,
                &error,
            )? => {}
        Err(error) => return Err(error),
    }
    timing.frame_exec_lua_time += exec_start.elapsed();
    let props_start = Instant::now();
    let frame_id = created_frame_id(env, setup.name)?;
    apply_xml_properties_direct(env, frame_id, setup.frame, setup.inherits, setup.parent);
    apply_intrinsic_property(env, setup.intrinsic_base, frame_id);
    timing.frame_apply_props_time += props_start.elapsed();
    timing.xml_frame_setup_time += setup_start.elapsed();
    timing.frame_count += 1;
    Ok(())
}

pub(super) fn created_frame_id(env: &LoaderEnv<'_>, name: &str) -> Result<u64, LoadError> {
    env.state()
        .borrow()
        .widgets
        .get_id_by_name(name)
        .ok_or_else(|| LoadError::Lua(format!("Failed to locate created frame {name}")))
}

fn recover_frame_after_loader_vm_error(
    env: &LoaderEnv<'_>,
    name: &str,
    frame: &crate::xml::FrameXml,
    parent: &str,
    error: &LoadError,
) -> Result<bool, LoadError> {
    let error_text = error.to_string();
    if !error_text.contains("expected Lua closure in execute") {
        return Ok(false);
    }
    let frame_exists = env.state().borrow().widgets.get_id_by_name(name).is_some();
    if !frame_exists {
        return Ok(false);
    }
    let parent_key = frame.parent_key.as_deref();
    let parent_array = frame.parent_array.as_deref();
    if parent_key.is_none() && parent_array.is_none() {
        return Ok(false);
    }
    let repair = build_parent_link_repair_script(parent, name, parent_key, parent_array);
    env.exec(&repair).map_err(|repair_error| {
        LoadError::Lua(format!(
            "Recovered frame {name} exists but failed to repair parent links after loader VM error: {repair_error}"
        ))
    })?;
    Ok(true)
}

/// Build a Lua snippet that re-links `child` into `parent[parent_key]` and/or
/// `parent[parent_array]` after a loader VM error left the frame disconnected.
fn build_parent_link_repair_script(
    parent: &str,
    name: &str,
    parent_key: Option<&str>,
    parent_array: Option<&str>,
) -> String {
    let parent_ref = crate::loader::helpers::lua_global_ref(parent);
    let child_ref = crate::loader::helpers::lua_global_ref(name);
    let mut repair = format!("local parent = {parent_ref}\nlocal child = {child_ref}\n");
    repair.push_str("if parent and child then\n");
    if let Some(parent_key) = parent_key {
        repair.push_str(&format!("  parent[{parent_key:?}] = child\n"));
    }
    if let Some(parent_array) = parent_array {
        repair.push_str(&format!(
            "  parent[{parent_array:?}] = parent[{parent_array:?}] or {{}}\n"
        ));
        repair.push_str(&format!(
            "  table.insert(parent[{parent_array:?}], child)\n"
        ));
    }
    repair.push_str("end\n");
    repair
}

/// Execute CreateFrame Lua with OnLoad suppression (depth-counted for recursion).
fn exec_create_frame_code(
    env: &LoaderEnv<'_>,
    lua_code: &str,
    name: &str,
    initial_hidden: bool,
) -> Result<(), LoadError> {
    {
        let mut state = env.state().borrow_mut();
        state.create_frame_initial_hidden = Some(initial_hidden);
        state.suppress_runtime_on_load_depth += 1;
    }
    let exec_result = env
        .exec(lua_code)
        .map_err(|e| LoadError::Lua(format!("Failed to create frame {}: {}", name, e)));
    {
        let mut state = env.state().borrow_mut();
        state.create_frame_initial_hidden = None;
        state.suppress_runtime_on_load_depth =
            state.suppress_runtime_on_load_depth.saturating_sub(1);
    }
    exec_result
}

/// Set declarative frame properties directly in Rust after the Lua CreateFrame chunk.
fn apply_xml_properties_direct(
    env: &LoaderEnv<'_>,
    frame_id: u64,
    frame: &crate::xml::FrameXml,
    inherits: &str,
    parent: &str,
) {
    use crate::lua_api::globals::template::direct;
    let state = env.state();
    direct::apply_xml_size(state, frame_id, frame, inherits);
    direct::apply_xml_anchors(state, frame_id, frame, inherits, parent);
    direct::apply_xml_frame_strata(state, frame_id, frame, inherits);
    direct::apply_xml_frame_level(state, frame_id, frame, inherits);
    direct::apply_xml_hidden(state, frame_id, frame, inherits);
    direct::apply_xml_toplevel(state, frame_id, frame, inherits);
    direct::apply_xml_alpha(state, frame_id, frame, inherits);
    direct::apply_xml_enable_mouse(state, frame_id, frame, inherits);
    direct::apply_xml_clips_children(state, frame_id, frame, inherits);
    direct::apply_xml_hit_rect_insets(state, frame_id, frame);
    direct::apply_xml_clamped_to_screen(state, frame_id, frame, inherits);
    direct::apply_xml_set_all_points(state, frame_id, frame, inherits);
    direct::apply_xml_protected(state, frame_id, frame, inherits);
    direct::apply_xml_id(state, frame_id, frame);
    apply_xml_scripts(env, frame_id, frame);
}

/// Set the `intrinsic` property on intrinsic frames (e.g. frame.intrinsic = "DropdownButton").
fn apply_intrinsic_property(env: &LoaderEnv<'_>, intrinsic_base: Option<&str>, frame_id: u64) {
    if let Some(base) = intrinsic_base {
        let _ = env.with_state(|state| {
            crate::lua_api::globals::template::set_intrinsic(state, frame_id, base);
            Ok::<(), crate::Error>(())
        });
    }
}

fn apply_xml_scripts(env: &LoaderEnv<'_>, frame_id: u64, frame: &crate::xml::FrameXml) {
    let Some(scripts) = frame.scripts() else {
        return;
    };
    let _ = env.with_state(|state| {
        crate::lua_api::globals::rilua_create_frame::apply_template_scripts(state, frame_id, scripts)
            .map_err(|error| crate::Error::Other(error.to_string()))?;
        Ok::<(), crate::Error>(())
    });
}
