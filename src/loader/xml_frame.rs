//! Frame creation from XML definitions.

use crate::lua_api::LoaderEnv;
use std::time::Instant;

use super::LoadTiming;
use super::button::{apply_button_text, apply_button_textures};
use super::error::LoadError;
use super::helpers::rand_id;
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
    if let Some(early) =
        register_virtual_or_intrinsic(env, frame, widget_type, parent_override, intrinsic_base)
    {
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
    setup_frame(
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
    let frame_id = created_frame_id(env, &prepared.name)?;
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

    let parent_ref = super::helpers::lua_global_ref(parent);
    let child_ref = super::helpers::lua_global_ref(name);
    let mut repair = String::new();
    repair.push_str(&format!(
        "local parent = {parent_ref}\nlocal child = {child_ref}\n"
    ));
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
    env.exec(&repair).map_err(|repair_error| {
        LoadError::Lua(format!(
            "Recovered frame {name} exists but failed to repair parent links after loader VM error: {repair_error}"
        ))
    })?;
    Ok(true)
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
    intrinsic_base: Option<&str>,
) -> Option<Option<String>> {
    if frame.is_virtual != Some(true) && frame.intrinsic != Some(true) {
        return None;
    }
    if let Some(ref name) = frame.name {
        // Prepend intrinsic base to inherits so the template chain includes
        // the intrinsic mixin (e.g. DropdownButton → DropdownButtonMixin).
        let mut registered = frame.clone();
        if let Some(base) = intrinsic_base {
            registered.inherits = Some(match &registered.inherits {
                Some(existing) if !existing.is_empty() => format!("{base}, {existing}"),
                _ => base.to_string(),
            });
        }
        crate::xml::register_template(name, widget_type, registered);
    }
    if let Some(ref sm) = frame.secure_mixin {
        apply_secure_mixins(env, sm);
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
fn apply_intrinsic_property(env: &LoaderEnv<'_>, intrinsic_base: Option<&str>, frame_id: u64) {
    if let Some(base) = intrinsic_base {
        let _ = env.with_state(|state| {
            crate::lua_api::globals::template::set_intrinsic(state, frame_id, base);
            Ok::<(), crate::Error>(())
        });
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

    for entry in &*crate::xml::get_template_chain(inherits) {
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

fn resolve_xml_hidden(frame: &crate::xml::FrameXml, inherits: &str) -> bool {
    let mut hidden = frame.hidden;
    if hidden.is_none() && !inherits.is_empty() {
        for entry in &*crate::xml::get_template_chain(inherits) {
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
    _inherits: &str,
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
    _inherits: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    for child in frame.all_frame_elements() {
        create_single_child_frame(env, &child, name, timing)?;
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
fn apply_secure_mixins(env: &LoaderEnv<'_>, secure_mixin_attr: &str) {
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
    let names_table = names
        .iter()
        .map(|name| format!("{name:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let script = format!("local names = {{{names_table}}}\n{transform}");
    let _ = env.exec(&script);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::{FrameElement, FrameXml};

    fn default_frame() -> FrameXml {
        FrameXml::default()
    }

    /// Helper: call frame_element_to_type and return (widget_type, intrinsic).
    fn resolve(elem: &FrameElement) -> Option<(&'static str, Option<&'static str>)> {
        frame_element_to_type(elem).map(|(_, wt, intr)| (wt, intr))
    }

    #[test]
    fn specialized_widget_types() {
        let f = default_frame();
        assert_eq!(
            resolve(&FrameElement::Frame(f.clone())),
            Some(("Frame", None))
        );
        assert_eq!(
            resolve(&FrameElement::Button(f.clone())),
            Some(("Button", None))
        );
        assert_eq!(
            resolve(&FrameElement::ItemButton(f.clone())),
            Some(("Button", Some("ItemButton")))
        );
        assert_eq!(
            resolve(&FrameElement::CheckButton(f.clone())),
            Some(("CheckButton", None))
        );
        assert_eq!(
            resolve(&FrameElement::EditBox(f.clone())),
            Some(("EditBox", None))
        );
        assert_eq!(
            resolve(&FrameElement::EventEditBox(f.clone())),
            Some(("EditBox", None))
        );
        assert_eq!(
            resolve(&FrameElement::ScrollFrame(f.clone())),
            Some(("ScrollFrame", None))
        );
        assert_eq!(
            resolve(&FrameElement::EventScrollFrame(f.clone())),
            Some(("ScrollFrame", None))
        );
        assert_eq!(
            resolve(&FrameElement::Slider(f.clone())),
            Some(("Slider", None))
        );
        assert_eq!(
            resolve(&FrameElement::StatusBar(f.clone())),
            Some(("StatusBar", None))
        );
        assert_eq!(
            resolve(&FrameElement::Cooldown(f.clone())),
            Some(("Cooldown", None))
        );
        assert_eq!(
            resolve(&FrameElement::GameTooltip(f.clone())),
            Some(("GameTooltip", None))
        );
        assert_eq!(
            resolve(&FrameElement::ColorSelect(f.clone())),
            Some(("ColorSelect", None))
        );
        assert_eq!(
            resolve(&FrameElement::Model(f.clone())),
            Some(("Model", None))
        );
        assert_eq!(
            resolve(&FrameElement::ModelScene(f.clone())),
            Some(("ModelScene", None))
        );
        assert_eq!(
            resolve(&FrameElement::SimpleHTML(f.clone())),
            Some(("SimpleHTML", None))
        );
        assert_eq!(
            resolve(&FrameElement::Minimap(f.clone())),
            Some(("Minimap", None))
        );
        assert_eq!(
            resolve(&FrameElement::MessageFrame(f.clone())),
            Some(("MessageFrame", None))
        );
    }

    #[test]
    fn player_model_variants_all_map_to_player_model() {
        let f = default_frame();
        assert_eq!(
            resolve(&FrameElement::PlayerModel(f.clone())),
            Some(("PlayerModel", None))
        );
        assert_eq!(
            resolve(&FrameElement::CinematicModel(f.clone())),
            Some(("PlayerModel", None))
        );
        assert_eq!(
            resolve(&FrameElement::TabardModel(f.clone())),
            Some(("PlayerModel", None))
        );
        assert_eq!(
            resolve(&FrameElement::DressUpModel(f.clone())),
            Some(("PlayerModel", None))
        );
    }

    #[test]
    fn button_intrinsic_variants() {
        let f = default_frame();
        assert_eq!(
            resolve(&FrameElement::DropdownButton(f.clone())),
            Some(("Button", Some("DropdownButton")))
        );
        assert_eq!(
            resolve(&FrameElement::DropDownToggleButton(f.clone())),
            Some(("Button", Some("DropDownToggleButton")))
        );
        assert_eq!(
            resolve(&FrameElement::EventButton(f.clone())),
            Some(("Button", Some("EventButton")))
        );
        assert_eq!(
            resolve(&FrameElement::ContainedAlertFrame(f.clone())),
            Some(("Button", Some("ContainedAlertFrame")))
        );
    }

    #[test]
    fn scrolling_message_frame_has_intrinsic() {
        let f = default_frame();
        assert_eq!(
            resolve(&FrameElement::ScrollingMessageFrame(f)),
            Some(("MessageFrame", Some("ScrollingMessageFrame")))
        );
    }

    #[test]
    fn frame_like_elements_preserve_supported_alias_types() {
        let f = default_frame();
        let preserved_aliases = [
            FrameElement::EventFrame(f.clone()),
            FrameElement::UnitPositionFrame(f.clone()),
            FrameElement::OffScreenFrame(f.clone()),
            FrameElement::Checkout(f.clone()),
            FrameElement::FogOfWarFrame(f.clone()),
            FrameElement::QuestPOIFrame(f.clone()),
            FrameElement::ArchaeologyDigSiteFrame(f.clone()),
            FrameElement::ScenarioPOIFrame(f.clone()),
            FrameElement::Browser(f.clone()),
            FrameElement::MovieFrame(f.clone()),
        ];
        for elem in &preserved_aliases {
            let (_, tag) = elem.as_frame_data().unwrap();
            assert_eq!(
                resolve(elem),
                Some((tag, None)),
                "Expected preserved type for {:?}",
                std::mem::discriminant(elem)
            );
        }
    }

    #[test]
    fn unsupported_frame_like_elements_still_map_to_frame() {
        let f = default_frame();
        let frame_likes = [
            FrameElement::TaxiRouteFrame(f.clone()),
            FrameElement::ModelFFX(f.clone()),
            FrameElement::UiCamera(f.clone()),
            FrameElement::UIThemeContainerFrame(f.clone()),
            FrameElement::MapScene(f.clone()),
            FrameElement::Line(f.clone()),
            FrameElement::WorldFrame(f.clone()),
        ];
        for elem in &frame_likes {
            assert_eq!(
                resolve(elem),
                Some(("Frame", None)),
                "Expected Frame for {:?}",
                std::mem::discriminant(elem)
            );
        }
    }

    #[test]
    fn scoped_modifier_returns_none() {
        use crate::xml::ScopedModifierXml;
        let sm = ScopedModifierXml {
            forbidden: None,
            elements: vec![],
        };
        assert_eq!(resolve(&FrameElement::ScopedModifier(sm)), None);
    }
}
