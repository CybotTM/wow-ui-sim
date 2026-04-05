//! Frame creation from XML definitions.

use crate::lua_api::LoaderEnv;
use std::time::Instant;

use super::LoadTiming;
use super::button::{apply_button_text, apply_button_textures};
use super::error::LoadError;
use super::helpers::{escape_lua_string, generate_scripts_code, lua_global_ref, rand_id};
use super::precompiled;
use super::xml_frame_extras::{apply_animation_groups, apply_bar_texture, init_action_bar_tables};
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
    finalize_frame(env, frame, &prepared.name, &prepared.inherits, timing)?;
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
    exec_create_frame_code(env, setup.lua_code, setup.name, setup.initial_hidden)?;
    timing.frame_exec_lua_time += exec_start.elapsed();
    let props_start = Instant::now();
    apply_xml_properties_direct(env, setup.name, setup.frame, setup.inherits, setup.parent);
    apply_intrinsic_property(env, setup.intrinsic_base, setup.name);
    timing.frame_apply_props_time += props_start.elapsed();
    timing.xml_frame_setup_time += setup_start.elapsed();
    timing.frame_count += 1;
    Ok(())
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
    name: &str,
    inherits: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let finalize_start = Instant::now();
    create_children_and_finalize(env, frame, name, inherits, timing)?;
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
fn build_frame_lua_code(
    widget_type: &str,
    name: &str,
    explicit_parent: Option<&str>,
    inherits: &str,
    frame: &crate::xml::FrameXml,
    parent: &str,
) -> String {
    let mut lua_code = build_create_frame_code(widget_type, name, explicit_parent, inherits);
    append_parent_key_code(&mut lua_code, frame, parent);
    append_mixins_code(&mut lua_code, frame, inherits);
    append_key_values_code(&mut lua_code, frame, inherits);
    append_xml_attributes_code(&mut lua_code, frame);
    // SetID must be in the Lua chunk (not deferred to Rust direct-set) because
    // template child OnLoad handlers may call GetParent():GetID() during
    // fire_deferred_child_onloads, which runs before apply_xml_properties_direct.
    if let Some(id) = frame.xml_id {
        lua_code.push_str(&format!("\n        frame:SetID({})", id));
    }
    append_scripts_code(&mut lua_code, frame);
    lua_code
}

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
    if has_lifecycle_scripts(frame, inherits) {
        let lc_start = Instant::now();
        fire_lifecycle_scripts(env, name);
        timing.frame_lifecycle_time += lc_start.elapsed();
        timing.lifecycle_fire_count += 1;
    }
    Ok(())
}

fn has_lifecycle_scripts(frame: &crate::xml::FrameXml, inherits: &str) -> bool {
    if frame
        .scripts()
        .is_some_and(|scripts| !scripts.on_load.is_empty() || !scripts.on_show.is_empty())
    {
        return true;
    }

    if inherits.is_empty() {
        return false;
    }

    crate::xml::get_template_chain(inherits)
        .iter()
        .any(|entry| {
            entry
                .frame
                .scripts()
                .is_some_and(|scripts| !scripts.on_load.is_empty() || !scripts.on_show.is_empty())
        })
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
    name: &str,
    frame: &crate::xml::FrameXml,
    inherits: &str,
    parent: &str,
) {
    use crate::lua_api::globals::template::direct;
    let state = env.state();
    let fid = state.borrow().widgets.get_id_by_name(name);
    let Some(fid) = fid else { return };
    direct::apply_xml_size(state, fid, frame, inherits);
    direct::apply_xml_anchors(state, fid, frame, inherits, parent);
    direct::apply_xml_frame_strata(state, fid, frame, inherits);
    direct::apply_xml_frame_level(state, fid, frame, inherits);
    direct::apply_xml_hidden(state, fid, frame, inherits);
    direct::apply_xml_toplevel(state, fid, frame, inherits);
    direct::apply_xml_alpha(state, fid, frame, inherits);
    direct::apply_xml_enable_mouse(state, fid, frame, inherits);
    direct::apply_xml_clips_children(state, fid, frame, inherits);
    direct::apply_xml_hit_rect_insets(state, fid, frame);
    direct::apply_xml_clamped_to_screen(state, fid, frame, inherits);
    direct::apply_xml_set_all_points(state, fid, frame, inherits);
    direct::apply_xml_protected(state, fid, frame, inherits);
    direct::apply_xml_id(state, fid, frame);
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

/// Build the initial `CreateFrame(...)` Lua code.
fn build_create_frame_code(
    widget_type: &str,
    name: &str,
    parent: Option<&str>,
    inherits: &str,
) -> String {
    let inherits_arg = if inherits.is_empty() {
        "nil".to_string()
    } else {
        format!("\"{}\"", inherits)
    };
    // Engine-root frames (e.g. UIParent) are pre-created without a parent.
    // When XML defines them, name == default parent, which would self-parent.
    // Reuse the existing engine frame instead.
    if let Some(p) = parent {
        if name == p {
            return format!(
                r#"
        local frame = _G["{name}"]
        "#,
            );
        }
    }
    let parent_arg = match parent {
        Some(p) => format!("{} or UIParent", lua_global_ref(p)),
        // Lua CreateFrame defaults nil parent to UIParent, so pass UIParent
        // here and orphan the frame with SetParent(nil) afterwards.
        None => "UIParent".to_string(),
    };
    let orphan_code = if parent.is_none() {
        // In WoW, top-level XML frames without a parent attribute are created
        // as orphans (no parent). Our Lua CreateFrame always defaults to
        // UIParent, so we create with UIParent then immediately orphan.
        "\n        frame:SetParent(nil)"
    } else {
        ""
    };
    format!(
        r#"
        local frame = CreateFrame("{widget_type}", "{name}", {parent_arg}, {inherits_arg}){orphan_code}
        "#,
    )
}

/// Append parentKey assignment so sibling frames can reference this frame.
///
/// Handles `$parent` prefix in parentKey (e.g. `$parent.CloseButton`)
/// which navigates up from the direct parent before setting the key.
fn append_parent_key_code(lua_code: &mut String, frame: &crate::xml::FrameXml, parent: &str) {
    if let Some(parent_key) = &frame.parent_key {
        let parent_ref = lua_global_ref(parent);
        if let Some(key) = parent_key.strip_prefix("$parent.") {
            lua_code.push_str(&format!(
                r#"
        do local __pk = {}:GetParent(); if __pk then __pk.{} = frame end end
        "#,
                parent_ref, key
            ));
        } else {
            lua_code.push_str(&format!(
                r#"
        {}.{} = frame
        "#,
                parent_ref, parent_key
            ));
        }
    }
    append_parent_array_code(lua_code, frame, parent);
}

/// Append parentArray insertion when the attribute is directly on this frame.
///
/// Template-inherited parentArray is handled by `apply_parent_array_from_template`
/// inside `CreateFrame`, so we only handle the direct-attribute case here.
fn append_parent_array_code(lua_code: &mut String, frame: &crate::xml::FrameXml, parent: &str) {
    if let Some(parent_array) = &frame.parent_array {
        let parent_ref = lua_global_ref(parent);
        lua_code.push_str(&format!(
            "\n        {parent_ref}.{parent_array} = {parent_ref}.{parent_array} or {{}}\n        \
             table.insert({parent_ref}.{parent_array}, frame)\n        ",
        ));
    }
}

/// Collect mixins from inherited templates and the frame itself, then append Mixin() calls.
fn append_mixins_code(lua_code: &mut String, frame: &crate::xml::FrameXml, inherits: &str) {
    let mut all_mixins: Vec<String> = Vec::new();

    // Collect from inherited templates (base mixins first)
    if !inherits.is_empty() {
        for template_entry in &crate::xml::get_template_chain(inherits) {
            collect_mixins_from_attr(
                &mut all_mixins,
                template_entry.frame.combined_mixin().as_deref(),
            );
        }
    }

    // Direct mixins (override templates)
    collect_mixins_from_attr(&mut all_mixins, frame.combined_mixin().as_deref());

    for m in &all_mixins {
        lua_code.push_str(&format!(
            "\n        do local m = {m} or (__secureenv and rawget(__secureenv, \"{m}\")) \
             if m then Mixin(frame, m) end end"
        ));
    }
}

/// Parse a comma-separated mixin attribute and append unique entries.
fn collect_mixins_from_attr(all_mixins: &mut Vec<String>, mixin_attr: Option<&str>) {
    if let Some(mixin) = mixin_attr {
        for m in mixin.split(',').map(|s| s.trim()) {
            if !m.is_empty() && !all_mixins.contains(&m.to_string()) {
                all_mixins.push(m.to_string());
            }
        }
    }
}

/// Append KeyValue assignments from templates and the frame itself.
fn append_key_values_code(lua_code: &mut String, frame: &crate::xml::FrameXml, inherits: &str) {
    if !inherits.is_empty() {
        for template_entry in &crate::xml::get_template_chain(inherits) {
            for kv in template_entry.frame.all_key_values() {
                append_key_values_from_xml(lua_code, Some(kv));
            }
        }
    }
    for kv in frame.all_key_values() {
        append_key_values_from_xml(lua_code, Some(kv));
    }
}

/// Append `frame.key = value` assignments for a KeyValues block.
fn append_key_values_from_xml(
    lua_code: &mut String,
    key_values: Option<&crate::xml::KeyValuesXml>,
) {
    if let Some(key_values) = key_values {
        for kv in &key_values.values {
            let value = format_key_value_lua(&kv.value, kv.value_type.as_deref());
            lua_code.push_str(&format!(
                r#"
        frame.{} = {}
        "#,
                kv.key, value
            ));
        }
    }
}

/// Format a KeyValue's value as a Lua expression based on its type.
fn format_key_value_lua(value: &str, value_type: Option<&str>) -> String {
    match value_type {
        Some("number") => value.to_string(),
        Some("boolean") => value.to_lowercase(),
        Some("global") if !value.is_empty() => value.to_string(),
        Some("global") => "nil".to_string(),
        _ => format!("\"{}\"", escape_lua_string(value)),
    }
}

/// Append SetAttribute calls for `<Attributes>` XML elements.
fn append_xml_attributes_code(lua_code: &mut String, frame: &crate::xml::FrameXml) {
    if let Some(attrs) = frame.xml_attributes() {
        for attr in &attrs.entries {
            let value = match attr.attr_type.as_deref() {
                Some("number") => attr.value.as_deref().unwrap_or("0").to_string(),
                Some("boolean") => attr.value.as_deref().unwrap_or("false").to_lowercase(),
                Some("nil") => "nil".to_string(),
                _ => format!(
                    "\"{}\"",
                    escape_lua_string(attr.value.as_deref().unwrap_or(""))
                ),
            };
            lua_code.push_str(&format!(
                "\n        frame:SetAttribute(\"{}\", {})",
                escape_lua_string(&attr.name),
                value
            ));
        }
    }
}

/// Append script handler registrations from the frame's Scripts element.
fn append_scripts_code(lua_code: &mut String, frame: &crate::xml::FrameXml) {
    if let Some(scripts) = frame.scripts() {
        lua_code.push_str(&generate_scripts_code(scripts));
    }
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
fn frame_element_to_type(
    child: &crate::xml::FrameElement,
) -> Option<(&crate::xml::FrameXml, &'static str, Option<&'static str>)> {
    specialized_element_type(child).or_else(|| frame_like_element_type(child))
}

/// Specialized widget types with distinct type strings or intrinsic bases.
fn specialized_element_type(
    child: &crate::xml::FrameElement,
) -> Option<(&crate::xml::FrameXml, &'static str, Option<&'static str>)> {
    use crate::xml::FrameElement;
    match child {
        FrameElement::Frame(f) => Some((f, "Frame", None)),
        FrameElement::Button(f) => Some((f, "Button", None)),
        FrameElement::DropdownButton(f) => Some((f, "Button", Some("DropdownButton"))),
        FrameElement::DropDownToggleButton(f) => Some((f, "Button", Some("DropDownToggleButton"))),
        FrameElement::EventButton(f) => Some((f, "Button", Some("EventButton"))),
        FrameElement::ContainedAlertFrame(f) => Some((f, "Button", Some("ContainedAlertFrame"))),
        FrameElement::ItemButton(f) => Some((f, "ItemButton", None)),
        FrameElement::CheckButton(f) => Some((f, "CheckButton", None)),
        FrameElement::EditBox(f) | FrameElement::EventEditBox(f) => Some((f, "EditBox", None)),
        FrameElement::ScrollFrame(f) | FrameElement::EventScrollFrame(f) => {
            Some((f, "ScrollFrame", None))
        }
        FrameElement::Slider(f) => Some((f, "Slider", None)),
        FrameElement::StatusBar(f) => Some((f, "StatusBar", None)),
        FrameElement::Cooldown(f) => Some((f, "Cooldown", None)),
        FrameElement::GameTooltip(f) => Some((f, "GameTooltip", None)),
        FrameElement::ColorSelect(f) => Some((f, "ColorSelect", None)),
        FrameElement::Model(f) => Some((f, "Model", None)),
        FrameElement::ModelScene(f) => Some((f, "ModelScene", None)),
        FrameElement::PlayerModel(f)
        | FrameElement::CinematicModel(f)
        | FrameElement::TabardModel(f)
        | FrameElement::DressUpModel(f) => Some((f, "PlayerModel", None)),
        FrameElement::MessageFrame(f) => Some((f, "MessageFrame", None)),
        FrameElement::ScrollingMessageFrame(f) => {
            Some((f, "MessageFrame", Some("ScrollingMessageFrame")))
        }
        FrameElement::SimpleHTML(f) => Some((f, "SimpleHTML", None)),
        FrameElement::Minimap(f) => Some((f, "Minimap", None)),
        _ => None,
    }
}

/// Frame-like elements that all map to widget type "Frame".
fn frame_like_element_type(
    child: &crate::xml::FrameElement,
) -> Option<(&crate::xml::FrameXml, &'static str, Option<&'static str>)> {
    use crate::xml::FrameElement;
    match child {
        FrameElement::EventFrame(f)
        | FrameElement::TaxiRouteFrame(f)
        | FrameElement::ModelFFX(f)
        | FrameElement::UiCamera(f)
        | FrameElement::UnitPositionFrame(f)
        | FrameElement::OffScreenFrame(f)
        | FrameElement::Checkout(f)
        | FrameElement::FogOfWarFrame(f)
        | FrameElement::QuestPOIFrame(f)
        | FrameElement::ArchaeologyDigSiteFrame(f)
        | FrameElement::ScenarioPOIFrame(f)
        | FrameElement::UIThemeContainerFrame(f)
        | FrameElement::MapScene(f)
        | FrameElement::Line(f)
        | FrameElement::Browser(f)
        | FrameElement::MovieFrame(f)
        | FrameElement::WorldFrame(f) => Some((f, "Frame", None)),
        _ => None,
    }
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
    // ScrollChild children are parented to the ScrollFrame just like regular children
    if let Some(scroll_child) = frame.scroll_child() {
        create_frame_elements(env, &scroll_child.children, name, timing)?;
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

/// Create frames from a list of FrameElement, assigning parentKey references.
fn create_frame_elements(
    env: &LoaderEnv<'_>,
    elements: &[crate::xml::FrameElement],
    parent_name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
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

        // Assign parentKey so the parent can reference the child.
        // The Lua assignment triggers __newindex which syncs to Rust children_keys.
        if let (Some(actual_child_name), Some(parent_key)) = (child_name, &child_frame.parent_key) {
            let fns = precompiled::get(env.lua());
            fns.assign_parent_key
                .call::<()>((parent_name, parent_key.as_str(), actual_child_name.as_str()))
                .ok();
        }
    }
    Ok(())
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
        assert_eq!(resolve(&FrameElement::Frame(f.clone())), Some(("Frame", None)));
        assert_eq!(resolve(&FrameElement::Button(f.clone())), Some(("Button", None)));
        assert_eq!(resolve(&FrameElement::ItemButton(f.clone())), Some(("ItemButton", None)));
        assert_eq!(resolve(&FrameElement::CheckButton(f.clone())), Some(("CheckButton", None)));
        assert_eq!(resolve(&FrameElement::EditBox(f.clone())), Some(("EditBox", None)));
        assert_eq!(resolve(&FrameElement::EventEditBox(f.clone())), Some(("EditBox", None)));
        assert_eq!(resolve(&FrameElement::ScrollFrame(f.clone())), Some(("ScrollFrame", None)));
        assert_eq!(resolve(&FrameElement::EventScrollFrame(f.clone())), Some(("ScrollFrame", None)));
        assert_eq!(resolve(&FrameElement::Slider(f.clone())), Some(("Slider", None)));
        assert_eq!(resolve(&FrameElement::StatusBar(f.clone())), Some(("StatusBar", None)));
        assert_eq!(resolve(&FrameElement::Cooldown(f.clone())), Some(("Cooldown", None)));
        assert_eq!(resolve(&FrameElement::GameTooltip(f.clone())), Some(("GameTooltip", None)));
        assert_eq!(resolve(&FrameElement::ColorSelect(f.clone())), Some(("ColorSelect", None)));
        assert_eq!(resolve(&FrameElement::Model(f.clone())), Some(("Model", None)));
        assert_eq!(resolve(&FrameElement::ModelScene(f.clone())), Some(("ModelScene", None)));
        assert_eq!(resolve(&FrameElement::SimpleHTML(f.clone())), Some(("SimpleHTML", None)));
        assert_eq!(resolve(&FrameElement::Minimap(f.clone())), Some(("Minimap", None)));
        assert_eq!(resolve(&FrameElement::MessageFrame(f.clone())), Some(("MessageFrame", None)));
    }

    #[test]
    fn player_model_variants_all_map_to_player_model() {
        let f = default_frame();
        assert_eq!(resolve(&FrameElement::PlayerModel(f.clone())), Some(("PlayerModel", None)));
        assert_eq!(resolve(&FrameElement::CinematicModel(f.clone())), Some(("PlayerModel", None)));
        assert_eq!(resolve(&FrameElement::TabardModel(f.clone())), Some(("PlayerModel", None)));
        assert_eq!(resolve(&FrameElement::DressUpModel(f.clone())), Some(("PlayerModel", None)));
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
    fn frame_like_elements_map_to_frame() {
        let f = default_frame();
        let frame_likes = [
            FrameElement::EventFrame(f.clone()),
            FrameElement::TaxiRouteFrame(f.clone()),
            FrameElement::ModelFFX(f.clone()),
            FrameElement::UiCamera(f.clone()),
            FrameElement::UnitPositionFrame(f.clone()),
            FrameElement::OffScreenFrame(f.clone()),
            FrameElement::Checkout(f.clone()),
            FrameElement::FogOfWarFrame(f.clone()),
            FrameElement::QuestPOIFrame(f.clone()),
            FrameElement::ArchaeologyDigSiteFrame(f.clone()),
            FrameElement::ScenarioPOIFrame(f.clone()),
            FrameElement::UIThemeContainerFrame(f.clone()),
            FrameElement::MapScene(f.clone()),
            FrameElement::Line(f.clone()),
            FrameElement::Browser(f.clone()),
            FrameElement::MovieFrame(f.clone()),
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
