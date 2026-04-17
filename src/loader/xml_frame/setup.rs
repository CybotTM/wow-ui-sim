//! Frame setup: CreateFrame execution, XML property application, error recovery.

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use crate::loader::LoadTiming;
use crate::loader::error::LoadError;
use crate::lua_api::LoaderEnv;

pub(super) struct SetupFrame<'a> {
    pub(super) widget_type: &'a str,
    pub(super) lua_code: &'a str,
    pub(super) name: &'a str,
    pub(super) explicit_parent: bool,
    pub(super) initial_hidden: bool,
    pub(super) frame: &'a crate::xml::FrameXml,
    pub(super) inherits: &'a str,
    pub(super) parent: &'a str,
    pub(super) intrinsic_base: Option<&'a str>,
}

#[derive(Default)]
struct FastCreateFrameProfile {
    fast_hits: u64,
    slow_fallbacks: u64,
    miss_reasons: BTreeMap<&'static str, u64>,
}

/// Execute CreateFrame Lua, apply XML properties, and record setup timing.
pub(super) fn setup_frame(
    env: &LoaderEnv<'_>,
    timing: &mut LoadTiming,
    setup: SetupFrame<'_>,
) -> Result<(), LoadError> {
    let setup_start = Instant::now();
    let exec_start = Instant::now();
    match exec_create_frame_code(env, &setup) {
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
fn exec_create_frame_code(env: &LoaderEnv<'_>, setup: &SetupFrame<'_>) -> Result<(), LoadError> {
    {
        let mut state = env.state().borrow_mut();
        state.create_frame_initial_hidden = Some(setup.initial_hidden);
        state.suppress_runtime_on_load_depth += 1;
    }
    let exec_result = if can_fast_create_frame(setup) {
        fast_create_frame(env, setup)
    } else {
        env.exec(setup.lua_code)
            .map_err(|e| LoadError::Lua(format!("Failed to create frame {}: {}", setup.name, e)))
    };
    {
        let mut state = env.state().borrow_mut();
        state.create_frame_initial_hidden = None;
        state.suppress_runtime_on_load_depth =
            state.suppress_runtime_on_load_depth.saturating_sub(1);
    }
    exec_result
}

fn can_fast_create_frame(setup: &SetupFrame<'_>) -> bool {
    if !fast_create_frame_profiling_enabled() {
        return setup.explicit_parent
            && setup.name != setup.parent
            && setup.frame.all_key_values().next().is_none()
            && setup.frame.xml_attributes().is_none()
            && setup.frame.scripts().is_none();
    }

    let mut miss_reasons = Vec::new();

    if !setup.explicit_parent {
        miss_reasons.push("no_explicit_parent");
    }
    if setup.name == setup.parent {
        miss_reasons.push("root_frame_reuse");
    }
    if setup.frame.all_key_values().next().is_some() {
        miss_reasons.push("key_values");
    }
    if setup.frame.xml_attributes().is_some() {
        miss_reasons.push("xml_attributes");
    }
    if setup.frame.scripts().is_some() {
        miss_reasons.push("scripts");
    }

    record_fast_create_frame_profile(&miss_reasons);
    miss_reasons.is_empty()
}

fn fast_create_frame(env: &LoaderEnv<'_>, setup: &SetupFrame<'_>) -> Result<(), LoadError> {
    env.with_state(|state| {
        let widget_type =
            crate::widget::WidgetType::from_str(setup.widget_type).ok_or_else(|| {
                crate::Error::Other(format!("unknown widget type '{}'", setup.widget_type))
            })?;
        let parent_id = crate::lua_api::methods::borrow_state(state)?
            .widgets
            .get_id_by_name(setup.parent)
            .ok_or_else(|| crate::Error::Other(format!("missing parent '{}'", setup.parent)))?;
        let frame_id = crate::lua_api::globals::create_frame::create_frame_instance(
            state,
            widget_type,
            setup.widget_type,
            Some(setup.name.to_string()),
            Some(parent_id),
            true,
            setup.frame.xml_id,
        )?;
        crate::lua_api::globals::create_frame::apply_runtime_template_chain(
            state,
            frame_id,
            (!setup.inherits.is_empty()).then_some(setup.inherits),
            false,
        )
        .map_err(|error| crate::Error::Other(error.to_string()))?;
        if let Some(parent_key) = setup.frame.parent_key.as_deref() {
            crate::lua_api::globals::template::assign_parent_key(
                state, parent_id, parent_key, frame_id,
            )
            .map_err(|error| crate::Error::Other(error.to_string()))?;
        }
        if let Some(parent_array) = setup.frame.parent_array.as_deref() {
            crate::lua_api::globals::create_frame::append_parent_array_entry(
                state,
                parent_id,
                parent_array,
                frame_id,
            );
        }
        crate::lua_api::globals::create_frame::apply_frame_mixins(
            state,
            frame_id,
            setup.frame.combined_mixin().as_deref(),
        );
        Ok::<(), crate::Error>(())
    })
    .map_err(|error| LoadError::Lua(format!("Failed to create frame {}: {}", setup.name, error)))
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

fn fast_create_frame_profiling_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("WOW_SIM_PROFILE_XML_FAST_PATH").is_ok())
}

fn fast_create_frame_profile() -> &'static Mutex<FastCreateFrameProfile> {
    static PROFILE: OnceLock<Mutex<FastCreateFrameProfile>> = OnceLock::new();
    PROFILE.get_or_init(|| Mutex::new(FastCreateFrameProfile::default()))
}

fn record_fast_create_frame_profile(miss_reasons: &[&'static str]) {
    if !fast_create_frame_profiling_enabled() {
        return;
    }
    let Ok(mut profile) = fast_create_frame_profile().lock() else {
        return;
    };
    if miss_reasons.is_empty() {
        profile.fast_hits += 1;
        return;
    }
    profile.slow_fallbacks += 1;
    for reason in miss_reasons {
        *profile.miss_reasons.entry(reason).or_default() += 1;
    }
}

pub(super) fn fast_create_frame_profile_report() -> Option<String> {
    if !fast_create_frame_profiling_enabled() {
        return None;
    }
    let Ok(profile) = fast_create_frame_profile().lock() else {
        return None;
    };
    let total = profile.fast_hits + profile.slow_fallbacks;
    if total == 0 {
        return Some("xml fast path: no frames recorded".to_string());
    }

    let mut reasons = profile
        .miss_reasons
        .iter()
        .map(|(reason, count)| (*reason, *count))
        .collect::<Vec<_>>();
    reasons.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));

    let top = reasons
        .into_iter()
        .take(8)
        .map(|(reason, count)| format!("{reason}={count}"))
        .collect::<Vec<_>>()
        .join(", ");

    Some(format!(
        "xml fast path: hits={} slow={} total={} misses: {}",
        profile.fast_hits,
        profile.slow_fallbacks,
        total,
        if top.is_empty() {
            "none".to_string()
        } else {
            top
        }
    ))
}
