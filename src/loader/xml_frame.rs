//! Frame creation from XML definitions.

use crate::lua_api::LoaderEnv;
use std::time::Instant;

use super::LoadTiming;
use super::button::{apply_button_text, apply_button_textures};
use super::error::LoadError;
use super::helpers::rand_id;
use super::precompiled;
use super::xml_frame_codegen::build_frame_lua_code;
use super::xml_frame_extras::{apply_animation_groups, apply_bar_texture, init_action_bar_tables};
use super::xml_lifecycle::LifecycleScripts;
use super::xml_lifecycle::fire_lifecycle_scripts;

/// Create a frame from XML definition.
/// Returns the name of the created frame (or None if skipped).
///
/// `intrinsic_base` is set when the XML element is an intrinsic type (e.g.
/// `<ContainedAlertFrame>`) whose registered template should be implicitly
/// inherited before any explicit `inherits` attribute.
pub fn create_frame_from_xml(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    widget_type: &str,
    parent_override: Option<&str>,
    intrinsic_base: Option<&str>,
    timing: &mut LoadTiming,
) -> Result<Option<String>, LoadError> {
    if let Some(early) = register_virtual_or_intrinsic(env, frame, widget_type, parent_override) {
        return Ok(early);
    }

    let Some(prepared) = prepare_frame_creation(env, frame, parent_override, intrinsic_base) else {
        return Ok(None);
    };
    let build_start = Instant::now();
    let lua_code = build_frame_lua_code(
        widget_type,
        &prepared.name,
        prepared.explicit_parent.as_deref(),
        &prepared.inherits,
        frame,
        &prepared.parent,
    );
    timing.frame_code_build_time += build_start.elapsed();
    let frame_id = setup_frame(
        env,
        timing,
        SetupFrame {
            lua_code: &lua_code,
            name: &prepared.name,
            initial_hidden: prepared.initial_hidden,
            frame,
            inherits: &prepared.inherits,
            parent: &prepared.parent,
            intrinsic_base,
        },
    )?;
    finalize_frame(
        env,
        frame,
        frame_id,
        &prepared.name,
        &prepared.inherits,
        timing,
    )?;
    Ok(Some(prepared.name))
}

struct PreparedFrameCreation {
    name: String,
    explicit_parent: Option<String>,
    parent: String,
    inherits: String,
    initial_hidden: bool,
}

fn prepare_frame_creation(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    parent_override: Option<&str>,
    intrinsic_base: Option<&str>,
) -> Option<PreparedFrameCreation> {
    let creator_name = current_loading_addon_name(env);
    let name = resolve_frame_name(frame, parent_override, creator_name.as_deref())?;
    let inherited_parent_buf = resolve_parent(frame, parent_override);
    let explicit_parent = parent_override
        .or(frame.parent.as_deref())
        .or(inherited_parent_buf.as_deref())
        .map(str::to_string);
    let parent = explicit_parent
        .clone()
        .unwrap_or_else(|| "UIParent".to_string());
    let inherits_buf = build_inherits_chain(frame, intrinsic_base);
    let inherits = inherits_buf
        .as_deref()
        .unwrap_or(frame.inherits.as_deref().unwrap_or(""))
        .to_string();
    let initial_hidden = resolve_xml_hidden(frame, &inherits);

    Some(PreparedFrameCreation {
        name,
        explicit_parent,
        parent,
        inherits,
        initial_hidden,
    })
}

fn current_loading_addon_name(env: &LoaderEnv<'_>) -> Option<String> {
    let s = env.state().borrow();
    s.loading_addon_index
        .and_then(|idx| s.addons.get(idx as usize))
        .map(|a| a.folder_name.clone())
}

/// Execute CreateFrame Lua, apply XML properties, and record setup timing.
fn setup_frame(
    env: &LoaderEnv<'_>,
    timing: &mut LoadTiming,
    setup: SetupFrame<'_>,
) -> Result<u64, LoadError> {
    let setup_start = Instant::now();
    let exec_start = Instant::now();
    exec_create_frame_code(env, setup.lua_code, setup.name, setup.initial_hidden)?;
    timing.frame_exec_lua_time += exec_start.elapsed();
    let frame_id = created_frame_id(env, setup.name)?;
    let props_start = Instant::now();
    apply_xml_properties_direct(env, frame_id, setup.frame, setup.inherits, setup.parent);
    apply_intrinsic_property(env, setup.intrinsic_base, setup.name);
    timing.frame_apply_props_time += props_start.elapsed();
    timing.xml_frame_setup_time += setup_start.elapsed();
    timing.frame_count += 1;
    Ok(frame_id)
}

struct SetupFrame<'a> {
    lua_code: &'a str,
    name: &'a str,
    initial_hidden: bool,
    frame: &'a crate::xml::FrameXml,
    inherits: &'a str,
    parent: &'a str,
    intrinsic_base: Option<&'a str>,
}

/// Create children, layers, animations, and fire lifecycle scripts with timing.
fn finalize_frame(
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

/// Register virtual/intrinsic frames as templates. Returns Some(None) to skip instantiation
/// for top-level virtual frames, or None to continue with normal creation.
fn register_virtual_or_intrinsic(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    widget_type: &str,
    parent_override: Option<&str>,
) -> Option<Option<String>> {
    if frame.is_virtual != Some(true) && frame.intrinsic != Some(true) {
        return None;
    }
    if let Some(ref name) = frame.name {
        crate::xml::register_template(name, widget_type, frame.clone());
    }
    if let Some(ref sm) = frame.secure_mixin {
        apply_secure_mixins(env.lua(), sm);
    }
    if parent_override.is_none() {
        Some(None) // skip instantiation for top-level virtual frames
    } else {
        None // child virtual frames are still created
    }
}

/// Prepend intrinsic base template to the inherits chain.
fn build_inherits_chain(
    frame: &crate::xml::FrameXml,
    intrinsic_base: Option<&str>,
) -> Option<String> {
    let explicit = frame.inherits.as_deref().unwrap_or("");
    match intrinsic_base {
        Some(base) if !explicit.is_empty() => Some(format!("{}, {}", base, explicit)),
        Some(base) => Some(base.to_string()),
        None => None,
    }
}

/// Build the Lua code that creates a frame and sets Lua-only XML properties.
///
/// Declarative properties (size, anchors, strata, level, alpha, hidden, toplevel,
/// enableMouse, hitRectInsets, clampedToScreen, setAllPoints) are set directly
/// in Rust by `apply_xml_properties_direct()` after this Lua chunk executes.
/// Note: `id` is set here in Lua (not deferred to Rust) because template child
/// OnLoad handlers may reference parent IDs during fire_deferred_child_onloads.
/// Set the `intrinsic` property on intrinsic frames (e.g. frame.intrinsic = "DropdownButton").
fn apply_intrinsic_property(env: &LoaderEnv<'_>, intrinsic_base: Option<&str>, name: &str) {
    if let Some(base) = intrinsic_base {
        let fns = precompiled::get(env.lua());
        fns.set_intrinsic.call::<()>((name, base)).ok();
    }
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
    create_child_frames(env, frame, name, timing)?;
    let layer_start = Instant::now();
    create_layer_children(env, frame, name, timing)?;
    timing.frame_layer_children_time += layer_start.elapsed();
    let anim_start = Instant::now();
    apply_animation_groups(env, frame, name, inherits)?;
    timing.frame_anim_time += anim_start.elapsed();
    let btn_start = Instant::now();
    apply_button_textures(env, frame, name)?;
    apply_button_text(env, frame, name, inherits)?;
    apply_bar_texture(env, frame, name)?;
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

fn created_frame_id(env: &LoaderEnv<'_>, name: &str) -> Result<u64, LoadError> {
    env.state()
        .borrow()
        .widgets
        .get_id_by_name(name)
        .ok_or_else(|| LoadError::Lua(format!("Failed to locate created frame {name}")))
}

fn lifecycle_scripts(frame: &crate::xml::FrameXml, inherits: &str) -> LifecycleScripts {
    let mut lifecycle = lifecycle_scripts_for_frame(frame);
    if inherits.is_empty() || (lifecycle.on_load && lifecycle.on_show) {
        return lifecycle;
    }

    for entry in &crate::xml::get_template_chain(inherits) {
        let inherited = lifecycle_scripts_for_frame(&entry.frame);
        lifecycle.on_load |= inherited.on_load;
        lifecycle.on_show |= inherited.on_show;
        if lifecycle.on_load && lifecycle.on_show {
            break;
        }
    }

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

/// Execute CreateFrame Lua with OnLoad suppression (depth-counted for recursion).
fn exec_create_frame_code(
    env: &LoaderEnv<'_>,
    lua_code: &str,
    name: &str,
    initial_hidden: bool,
) -> Result<(), LoadError> {
    let fns = precompiled::get(env.lua());
    fns.suppress_push.call::<()>(()).ok();
    env.state().borrow_mut().create_frame_initial_hidden = Some(initial_hidden);
    let exec_result = env
        .exec(lua_code)
        .map_err(|e| LoadError::Lua(format!("Failed to create frame {}: {}", name, e)));
    env.state().borrow_mut().create_frame_initial_hidden = None;
    fns.suppress_pop.call::<()>(()).ok();
    exec_result?;
    crate::lua_api::globals::template::fire_deferred_child_onloads(env.lua());
    Ok(())
}

fn resolve_xml_hidden(frame: &crate::xml::FrameXml, inherits: &str) -> bool {
    let mut hidden = frame.hidden;
    if hidden.is_none() && !inherits.is_empty() {
        for entry in &crate::xml::get_template_chain(inherits) {
            if let Some(h) = entry.frame.hidden {
                hidden = Some(h);
                break;
            }
        }
    }
    hidden == Some(true)
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

/// Resolve the frame name, applying `$parent` substitution and generating anonymous names.
/// Returns `None` if the frame should be skipped (anonymous top-level frame).
fn resolve_frame_name(
    frame: &crate::xml::FrameXml,
    parent_override: Option<&str>,
    creator: Option<&str>,
) -> Option<String> {
    match &frame.name {
        Some(n) => {
            if let Some(parent_name) = parent_override {
                Some(n.replace("$parent", parent_name))
            } else {
                Some(n.clone())
            }
        }
        None => {
            if parent_override.is_some() {
                Some(format!("__{}_{}", creator.unwrap_or("anon"), rand_id()))
            } else {
                None // Anonymous top-level frames are templates
            }
        }
    }
}

/// Resolve the parent for a frame, checking inherited templates when the frame
/// itself has no explicit `parent` attribute (e.g. ClassPowerBarFrame defines
/// `parent="PlayerFrame"` which propagates to PaladinPowerBarFrame).
///
/// Returns `Some(parent_name)` from the template chain, or `None` if no
/// template provides a parent.  The caller should prefer `parent_override`
/// and `frame.parent` first.
fn resolve_parent(frame: &crate::xml::FrameXml, parent_override: Option<&str>) -> Option<String> {
    if parent_override.is_some() || frame.parent.is_some() {
        return None; // Already have an explicit parent, no need to search templates.
    }
    frame.inherits.as_deref().and_then(|inherits| {
        crate::xml::get_template_chain(inherits)
            .iter()
            .find_map(|entry| entry.frame.parent.clone())
    })
}

/// Create textures and fontstrings from the frame's Layers.
fn create_layer_children(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    super::xml_layer_batch::create_layer_children_batched(env, frame, name, timing)
}

/// Map a FrameElement to its (FrameXml, widget_type, intrinsic_name) triple.
///
/// FrameElement-specific overrides vs the shared `widget_type_for_tag`:
/// - `DropDownToggleButton` / `EventButton` get intrinsic names (shared treats them as unknown)
fn frame_element_to_type(
    child: &crate::xml::FrameElement,
) -> Option<(&crate::xml::FrameXml, &'static str, Option<&'static str>)> {
    let (f, tag) = child.as_frame_data()?;
    let (wt, intrinsic) = match tag {
        "DropDownToggleButton" => ("Button", Some("DropDownToggleButton")),
        "EventButton" => ("Button", Some("EventButton")),
        _ => crate::xml::widget_type_for_tag(tag)?,
    };
    Some((f, wt, intrinsic))
}

/// Recursively create child frames and assign parentKey references.
fn create_child_frames(
    env: &LoaderEnv<'_>,
    frame: &crate::xml::FrameXml,
    name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    // Use all_frame_elements() to handle multiple <Frames> sections in the XML
    // and standalone frame-type children outside <Frames> wrappers
    let elements = frame.all_frame_elements();
    for child in &elements {
        create_single_child_frame(env, child, name, timing)?;
    }
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
        let (child_frame, child_type, intrinsic) = match frame_element_to_type(child) {
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
            let fns = precompiled::get(env.lua());
            fns.assign_parent_key
                .call::<()>((parent_name, parent_key.as_str(), actual_child_name))
                .ok();
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
    child: &crate::xml::FrameElement,
    parent_name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let (child_frame, child_type, intrinsic) = match frame_element_to_type(child) {
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
        let fns = precompiled::get(env.lua());
        fns.assign_parent_key
            .call::<()>((parent_name, parent_key.as_str(), actual_child_name.as_str()))
            .ok();
    }
    Ok(())
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

/// Apply the secure mixin transformation to the named mixin tables.
///
/// In the real WoW client (and in wowless), when a frame has `secureMixin="Foo"`,
/// the mixin table `_G.Foo` has its methods moved into a hidden `__index` table,
/// leaving the mixin itself empty. `getmetatable(Foo)` returns a number (0).
///
/// This mirrors the wowless `securemixin` handler:
/// ```lua
/// local vv = {}
/// for k, v in pairs(mv) do vv[k] = v; mv[k] = nil end
/// setmetatable(mv, { __index = vv, __metatable = 0 })
/// ```
///
/// Additionally, we store the methods table in `__secureMixinMethods` (a registry table)
/// keyed by the mixin table reference, so that `Mixin()` can apply only the stable
/// methods (not user-added direct entries like test fixtures) when applying secure mixins
/// to new frame instances.
fn apply_secure_mixins(lua: &mlua::Lua, secure_mixin_attr: &str) {
    let transform = r#"
        local names = ...
        __secureMixinMethods = __secureMixinMethods or {}
        for _, name in ipairs(names) do
            local mv = _G[name] or (__secureenv and rawget(__secureenv, name))
            if mv and type(mv) == 'table' then
                local vv = {}
                for k, v in pairs(mv) do
                    vv[k] = v
                    mv[k] = nil
                end
                setmetatable(mv, { __index = vv, __metatable = 0 })
                __secureMixinMethods[mv] = vv
            end
        end
    "#;
    let names: Vec<String> = secure_mixin_attr
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if names.is_empty() {
        return;
    }
    let lua_names: mlua::Result<mlua::Table> = lua.create_sequence_from(names);
    if let Ok(tbl) = lua_names {
        let _ = lua.load(transform).call::<()>(tbl);
    }
}

#[cfg(test)]
#[path = "xml_frame/tests.rs"]
mod tests;
