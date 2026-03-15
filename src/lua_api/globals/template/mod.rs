//! Template application from the XML template registry.
//!
//! This module provides functionality to apply XML templates from the registry
//! when CreateFrame is called with a template name.

pub(crate) mod direct;
mod elements;

use crate::loader::helpers::generate_set_point_code;
use crate::loader::helpers_anim::generate_animation_group_code;
use crate::lua_api::SimState;
use crate::xml::{FrameElement, FrameXml, LayerElement, TemplateEntry, get_template_chain};
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

/// Extract the FrameXml, widget type, and optional intrinsic name from a FrameElement.
fn frame_element_type(
    element: &FrameElement,
) -> Option<(&FrameXml, &'static str, Option<&'static str>)> {
    specialized_frame_element(element).or_else(|| frame_like_frame_element(element))
}

/// Specialized widget types with distinct type strings or intrinsic bases.
fn specialized_frame_element(
    element: &FrameElement,
) -> Option<(&FrameXml, &'static str, Option<&'static str>)> {
    match element {
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
        _ => None,
    }
}

/// Frame-like elements that all map to widget type "Frame".
fn frame_like_frame_element(
    element: &FrameElement,
) -> Option<(&FrameXml, &'static str, Option<&'static str>)> {
    match element {
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
        | FrameElement::Minimap(f)
        | FrameElement::MovieFrame(f)
        | FrameElement::WorldFrame(f) => Some((f, "Frame", None)),
        _ => None,
    }
}

/// Apply templates from the registry to a frame.
///
/// This generates Lua code to create child frames, textures, and fontstrings
/// defined in the template chain (including inherited templates).
pub fn apply_templates_from_registry(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_name: &str,
    template_names: &str,
) {
    let chain = get_template_chain(template_names);
    if chain.is_empty() {
        return;
    }

    for entry in &chain {
        apply_single_template(lua, state, frame_name, entry);
    }
    elements::apply_deferred_mask_atlases(lua, frame_name, &chain);
}

/// Fire all deferred child OnLoad scripts that were queued during template
/// application while `__suppress_create_frame_onload` was active.
pub fn fire_deferred_child_onloads(lua: &Lua) {
    let Ok(deferred) = lua.globals().get::<mlua::Table>("__deferred_child_onloads") else {
        return;
    };
    let names: Vec<String> = deferred
        .sequence_values::<String>()
        .filter_map(|r| r.ok())
        .collect();
    let _ = lua
        .globals()
        .set("__deferred_child_onloads", mlua::Value::Nil);
    for name in &names {
        fire_on_load(lua, name);
    }
}

/// Apply a single template entry to a frame.
fn apply_single_template(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_name: &str,
    entry: &TemplateEntry,
) {
    let template = &entry.frame;

    apply_mixin(lua, &template.combined_mixin(), frame_name);
    let frame_id = resolve_template_frame_id(state, frame_name);
    for key_values in template.all_key_values() {
        apply_key_values(lua, key_values, frame_name);
    }
    if let Some(fid) = frame_id {
        apply_direct_rust_properties(state, fid, template, frame_name);
    }

    // Apply layers (textures and fontstrings)
    // subst_parent = frame_name at the template root level
    apply_layers(lua, template, frame_name, frame_name);

    // Apply button textures (NormalTexture, PushedTexture, etc.)
    apply_button_textures(lua, template, frame_name, frame_name);

    // Apply StatusBar BarTexture
    if let Some(bar) = template.bar_texture() {
        elements::create_bar_texture_from_template(lua, bar, frame_name, frame_name);
    }

    // Apply Slider ThumbTexture
    if let Some(thumb) = template.thumb_texture() {
        elements::create_thumb_texture_from_template(lua, thumb, frame_name, frame_name);
    }

    // Apply ButtonText and EditBox FontString
    apply_button_text(lua, template, frame_name, frame_name);
    elements::apply_button_text_attribute(lua, template, frame_name);
    apply_editbox_fontstring(lua, template, frame_name, frame_name);
    apply_button_fonts(lua, template, frame_name);
    apply_animation_groups(lua, template, frame_name);

    // Create child frames defined in the template
    create_child_frames(lua, state, template, frame_name, frame_name);

    // Create ScrollChild children
    if let Some(scroll_child) = template.scroll_child() {
        create_scroll_child_frames(lua, state, &scroll_child.children, frame_name, frame_name);
    }

    // Apply scripts from template (after children, so OnLoad can reference them)
    if let Some(scripts) = template.scripts() {
        elements::apply_scripts_from_template(lua, scripts, frame_name);
    }
}

/// Look up frame_id for direct Rust property setting.
fn resolve_template_frame_id(state: &Rc<RefCell<SimState>>, frame_name: &str) -> Option<u64> {
    state
        .borrow()
        .widgets
        .get_id_by_name(frame_name)
        .or_else(|| {
            frame_name
                .strip_prefix("__frame_")
                .and_then(|s| s.parse::<u64>().ok())
        })
}

/// Apply direct Rust properties from template (size, anchors, hidden, frame level).
fn apply_direct_rust_properties(
    state: &Rc<RefCell<SimState>>,
    fid: u64,
    template: &FrameXml,
    frame_name: &str,
) {
    direct::set_size(state, fid, template);
    direct::set_anchors(state, fid, template, frame_name);
    direct::set_all_points(state, fid, template);
    direct::set_clips_children(state, fid, template);
    direct::set_hidden(state, fid, template);
    if template.protected == Some(true) {
        if let Some(frame) = state.borrow_mut().widgets.get_mut_visual(fid) {
            frame.is_protected = true;
        }
    }
    if let Some(level) = template.frame_level {
        let mut s = state.borrow_mut();
        if let Some(frame) = s.widgets.get_mut_visual(fid) {
            frame.frame_level_offset = Some(level);
        }
    }
}

/// Apply key values from a template to a frame.
fn apply_key_values(lua: &Lua, key_values: &crate::xml::KeyValuesXml, frame_name: &str) {
    let frame_ref = lua_global_ref(frame_name);
    for kv in &key_values.values {
        let value = format_key_value(&kv.value, kv.value_type.as_deref());
        let code = format!(
            "do local f = {} if f then f.{} = {} end end",
            frame_ref, kv.key, value
        );
        let _ = lua.load(&code).exec();
    }
}

/// Format a key value for Lua assignment.
fn format_key_value(value: &str, value_type: Option<&str>) -> String {
    match value_type {
        Some("number") => value.to_string(),
        Some("boolean") => value.to_lowercase(),
        Some("global") => value.to_string(),
        _ => format!("\"{}\"", escape_lua_string(value)),
    }
}

/// Apply layers (textures and fontstrings) from a template.
///
/// `subst_parent` is the name used for `$parent` substitution in child names.
/// For anonymous frames, this propagates from the nearest named ancestor.
fn apply_layers(lua: &Lua, template: &FrameXml, frame_name: &str, subst_parent: &str) {
    for layers in template.layers() {
        for layer in &layers.layers {
            let draw_layer = layer.level.as_deref().unwrap_or("ARTWORK");
            // Process elements in document order so that FontStrings referenced
            // by subsequent Texture anchors (e.g. $parent.SpecName) are already
            // created when the Texture's SetPoint runs.
            for element in &layer.elements {
                match element {
                    LayerElement::Texture(t) => {
                        elements::create_texture_from_template(
                            lua,
                            t,
                            frame_name,
                            subst_parent,
                            draw_layer,
                            false,
                            false,
                        );
                    }
                    LayerElement::Line(t) => {
                        elements::create_texture_from_template(
                            lua,
                            t,
                            frame_name,
                            subst_parent,
                            draw_layer,
                            false,
                            true,
                        );
                    }
                    LayerElement::MaskTexture(t) => {
                        elements::create_texture_from_template(
                            lua,
                            t,
                            frame_name,
                            subst_parent,
                            draw_layer,
                            true,
                            false,
                        );
                    }
                    LayerElement::FontString(f) => {
                        elements::create_fontstring_from_template(
                            lua,
                            f,
                            frame_name,
                            subst_parent,
                            draw_layer,
                        );
                    }
                }
            }
        }
    }
}

/// Apply button textures (NormalTexture, PushedTexture, etc.) from a template.
fn apply_button_textures(lua: &Lua, template: &FrameXml, frame_name: &str, subst_parent: &str) {
    let texture_specs: &[(&str, &str, Option<&crate::xml::TextureXml>)] = &[
        ("Normal", "SetNormalTexture", template.normal_texture()),
        ("Pushed", "SetPushedTexture", template.pushed_texture()),
        (
            "Disabled",
            "SetDisabledTexture",
            template.disabled_texture(),
        ),
        (
            "Highlight",
            "SetHighlightTexture",
            template.highlight_texture(),
        ),
        ("Checked", "SetCheckedTexture", template.checked_texture()),
        (
            "DisabledChecked",
            "SetDisabledCheckedTexture",
            template.disabled_checked_texture(),
        ),
    ];
    for &(parent_key, setter, tex_opt) in texture_specs {
        if let Some(tex) = tex_opt {
            elements::create_button_texture_from_template(
                lua,
                tex,
                frame_name,
                subst_parent,
                parent_key,
                setter,
            );
        }
    }
}

/// Apply mixin to a frame.
fn apply_mixin(lua: &Lua, mixin: &Option<String>, frame_name: &str) {
    let Some(mixin) = mixin else { return };
    let mut parts = Vec::new();
    for name in mixin.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        parts.push(format!(
            "do local m = {name} or (__secureenv and rawget(__secureenv, \"{name}\")) \
             if m then Mixin(f, m) end end"
        ));
    }
    if parts.is_empty() {
        return;
    }
    let post_init = build_mixin_post_init(mixin);
    let code = format!(
        "do local f = {} if f then {} {} end end",
        lua_global_ref(frame_name),
        parts.join(" "),
        post_init,
    );
    let _ = lua.load(&code).exec();
}

/// Build post-initialization code for known mixins that need pre-seeded fields.
fn build_mixin_post_init(mixin: &str) -> String {
    let mut post_init = String::new();
    for name in mixin.split(',').map(str::trim) {
        match name {
            "ActionBarMixin" => {
                post_init.push_str("f.actionButtons = f.actionButtons or {} ");
                post_init.push_str("f.shownButtonContainers = f.shownButtonContainers or {} ");
            }
            "EditModeSystemMixin" => {
                for alias in [
                    "SetScale",
                    "SetPoint",
                    "ClearAllPoints",
                    "SetShown",
                    "Show",
                    "Hide",
                    "IsShown",
                ] {
                    post_init.push_str(&format!("f.{alias}Base = f.{alias} "));
                }
            }
            "EventFrameMixin" | "CallbackRegistryMixin" => {
                post_init.push_str("if f.OnLoad_Intrinsic then pcall(f.OnLoad_Intrinsic, f) end ");
            }
            _ => {}
        }
    }
    post_init
}

/// Push the suppress-OnLoad depth counter (prevents premature OnLoad in nested CreateFrame).
fn push_suppress(lua: &Lua) {
    let depth: i32 = lua
        .globals()
        .get("__suppress_create_frame_onload")
        .unwrap_or(0);
    let _ = lua
        .globals()
        .set("__suppress_create_frame_onload", depth + 1);
}

/// Pop the suppress-OnLoad depth counter.
fn pop_suppress(lua: &Lua) {
    let depth: i32 = lua
        .globals()
        .get("__suppress_create_frame_onload")
        .unwrap_or(0);
    let _ = lua
        .globals()
        .set("__suppress_create_frame_onload", depth - 1);
}

/// Queue a child frame name for deferred OnLoad firing.
fn defer_child_onload(lua: &Lua, name: &str) {
    let deferred: mlua::Table = lua
        .globals()
        .get("__deferred_child_onloads")
        .unwrap_or_else(|_| lua.create_table().unwrap());
    let len = deferred.raw_len();
    let _ = deferred.raw_set(len + 1, name);
    let _ = lua.globals().set("__deferred_child_onloads", deferred);
}

/// Fire OnLoad on a frame.
///
/// Only fires handlers registered via `SetScript` (from `<Scripts>` XML tags)
/// and `OnLoad_Intrinsic` (from intrinsic mixins like EventFrameMixin).
/// Does NOT call `frame.OnLoad` as a fallback — in WoW, the C++ engine only
/// calls registered script handlers, not mixin table fields. Mixin OnLoad
/// methods are invoked via `<Scripts><OnLoad method="OnLoad"/></Scripts>` which
/// generates a `SetScript("OnLoad", function(self) self:OnLoad() end)` call.
pub(crate) fn fire_on_load(lua: &Lua, frame_name: &str) {
    let frame_ref = lua_global_ref(frame_name);
    // Fire intrinsic OnLoad_Intrinsic first (e.g. EventFrameMixin) — in WoW,
    // intrinsic scripts always fire regardless of what user templates set.
    let code = format!(
        r#"
        local frame = {frame_ref}
        if frame then
            if type(frame.OnLoad_Intrinsic) == "function" then
                local ok, err = pcall(frame.OnLoad_Intrinsic, frame)
                if not ok then
                    return tostring(err)
                end
            end
            local handler = frame:GetScript("OnLoad")
            if handler then
                local ok, err = pcall(handler, frame)
                if not ok then
                    return tostring(err)
                end
            end
        end
        "#
    );
    match lua.load(&code).eval::<Option<String>>() {
        Ok(Some(err)) => eprintln!("[fire_on_load] {} error: {}", frame_name, err),
        Err(e) => eprintln!("[fire_on_load] {} eval error: {}", frame_name, e),
        _ => {}
    }
}

/// Create child frames from template XML.
fn create_child_frames(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    parent_name: &str,
    subst_parent: &str,
) {
    let elements = frame.all_frame_elements();
    for child in &elements {
        let Some((child_frame, child_type, intrinsic)) = frame_element_type(child) else {
            continue;
        };
        create_child_frame_from_template(
            lua,
            state,
            child_frame,
            child_type,
            intrinsic,
            parent_name,
            subst_parent,
        );
    }
}

/// Create a child frame from template XML.
///
/// Suppresses OnLoad during CreateFrame so it doesn't fire prematurely on a
/// bare frame (before mixin/scripts are applied). After all templates and
/// inline content are applied, defers this child's name to
/// `__deferred_child_onloads` for firing by `create_frame_function` or
/// `exec_create_frame_code`.
///
/// `subst_parent` is the name used for `$parent` substitution. For anonymous
/// frames (no name attribute), `$parent` propagates from the nearest named
/// ancestor rather than using the auto-generated frame name.
fn create_child_frame_from_template(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &crate::xml::FrameXml,
    widget_type: &str,
    intrinsic: Option<&str>,
    parent_name: &str,
    subst_parent: &str,
) {
    let is_named = frame.name.is_some();
    let child_name = frame
        .name
        .as_ref()
        .map(|n| n.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__tpl_{}", rand_id()));

    // For named children, their name becomes the new $parent for descendants.
    // For anonymous children, propagate the current subst_parent.
    let child_subst = if is_named { &child_name } else { subst_parent };

    // Suppress OnLoad in nested CreateFrame — the child has no handlers yet.
    push_suppress(lua);

    let code = build_create_child_code(frame, widget_type, parent_name, &child_name);
    if let Err(e) = lua.load(&code).exec() {
        eprintln!(
            "[template] Failed to create child '{}' (type={}) under '{}': {}",
            child_name, widget_type, parent_name, e
        );
        pop_suppress(lua);
        return;
    }
    // Apply intrinsic template (e.g. DropdownButton) and set the intrinsic property.
    if let Some(intrinsic_name) = intrinsic {
        apply_templates_from_registry(lua, state, &child_name, intrinsic_name);
        let code = format!(
            "{}.intrinsic = \"{}\"",
            lua_global_ref(&child_name),
            intrinsic_name
        );
        let _ = lua.load(&code).exec();
    }

    // Apply inherited templates AFTER CreateFrame but BEFORE inline content.
    // CreateFrame is called without the template arg so it doesn't fire OnLoad
    // prematurely. Templates set up mixin/scripts, then inline content adds
    // layers/children, and finally the deferred OnLoad fires with everything
    // in place.
    let inherits = frame.inherits.as_deref().unwrap_or("");
    if !inherits.is_empty() {
        apply_templates_from_registry(lua, state, &child_name, inherits);
    }

    apply_inline_frame_content(lua, state, frame, &child_name, child_subst);

    pop_suppress(lua);

    // Queue for deferred OnLoad (fired by create_frame_function or exec_create_frame_code)
    defer_child_onload(lua, &child_name);
}

/// Build Lua code to create a child frame with size, anchors, visibility, and parentKey.
///
/// The `inherits` template is NOT passed to CreateFrame here — it's applied
/// separately in `create_child_frame_from_template` so that inline XML content
/// (layers, children) is fully created before OnLoad fires.
fn build_create_child_code(
    frame: &crate::xml::FrameXml,
    widget_type: &str,
    parent_name: &str,
    child_name: &str,
) -> String {
    let mut code = format!(
        r#"
        local parent = {}
        if parent then
            local child = CreateFrame("{}", "{}", parent, nil)
        "#,
        lua_global_ref(parent_name),
        widget_type,
        escape_lua_string(child_name),
    );

    append_child_size_and_anchors(&mut code, frame, parent_name);
    append_child_parent_refs(&mut code, frame);
    code.push_str("        end\n");
    code
}

/// Append anchors, setAllPoints, and hidden to child frame code.
/// Size is NOT set here — it's applied later in apply_inline_frame_content
/// so that template defaults are set first, then inline size overrides.
fn append_child_size_and_anchors(code: &mut String, frame: &FrameXml, parent_name: &str) {
    if let Some(anchors) = frame.anchors() {
        code.push_str(&generate_set_point_code(
            anchors,
            "child",
            "parent",
            parent_name,
            "parent",
        ));
    }
    if frame.set_all_points == Some(true) {
        code.push_str("            child:SetAllPoints(true)\n");
    }
    let hidden = frame.hidden.or_else(|| {
        let inherits = frame.inherits.as_deref().unwrap_or("");
        if inherits.is_empty() {
            return None;
        }
        for entry in &get_template_chain(inherits) {
            if entry.frame.hidden.is_some() {
                return entry.frame.hidden;
            }
        }
        None
    });
    if hidden == Some(true) {
        code.push_str("            child:Hide()\n");
    }
}

/// Append parentKey and parentArray assignment (with template chain resolution).
fn append_child_parent_refs(code: &mut String, frame: &FrameXml) {
    if let Some(parent_key) = &resolve_inherited_field(frame, |f| f.parent_key.as_ref()) {
        let key = escape_lua_string(parent_key);
        code.push_str(&format!("            parent[\"{key}\"] = child\n"));
    }
    if let Some(parent_array) = &resolve_inherited_field(frame, |f| f.parent_array.as_ref()) {
        let arr = escape_lua_string(parent_array);
        code.push_str(&format!(
            "            parent[\"{arr}\"] = parent[\"{arr}\"] or {{}}\n\
             table.insert(parent[\"{arr}\"], child)\n"
        ));
    }
}

/// Resolve a field from a frame or its inherited template chain.
fn resolve_inherited_field(
    frame: &FrameXml,
    getter: impl Fn(&FrameXml) -> Option<&String>,
) -> Option<String> {
    if let Some(val) = getter(frame) {
        return Some(val.clone());
    }
    let inherits = frame.inherits.as_deref().unwrap_or("");
    if inherits.is_empty() {
        return None;
    }
    for entry in &get_template_chain(inherits) {
        if let Some(val) = getter(&entry.frame) {
            return Some(val.clone());
        }
    }
    None
}

/// Create child frames from a ScrollChild element.
fn create_scroll_child_frames(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    children: &[FrameElement],
    parent_name: &str,
    subst_parent: &str,
) {
    for child in children {
        let Some((child_frame, child_type, intrinsic)) = frame_element_type(child) else {
            continue;
        };
        create_child_frame_from_template(
            lua,
            state,
            child_frame,
            child_type,
            intrinsic,
            parent_name,
            subst_parent,
        );
    }
}

/// Apply inline content from a FrameXml to an already-created frame.
fn apply_inline_frame_content(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &crate::xml::FrameXml,
    frame_name: &str,
    subst_parent: &str,
) {
    apply_mixin(lua, &frame.combined_mixin(), frame_name);
    apply_inline_key_values(lua, frame, frame_name);
    // Re-apply inline size — templates may override the size set in build_create_child_code.
    let fid = state
        .borrow()
        .widgets
        .get_id_by_name(frame_name)
        .or_else(|| {
            frame_name
                .strip_prefix("__frame_")
                .and_then(|s| s.parse::<u64>().ok())
        });
    if let Some(fid) = fid {
        direct::set_size_partial(state, fid, frame);
    }
    apply_layers(lua, frame, frame_name, subst_parent);

    if let Some(thumb) = frame.thumb_texture() {
        elements::create_thumb_texture_from_template(lua, thumb, frame_name, subst_parent);
    }
    if let Some(bar) = frame.bar_texture() {
        elements::create_bar_texture_from_template(lua, bar, frame_name, subst_parent);
    }

    apply_inline_button_textures(lua, frame, frame_name, subst_parent);
    apply_button_text(lua, frame, frame_name, subst_parent);
    elements::apply_button_text_attribute(lua, frame, frame_name);
    apply_editbox_fontstring(lua, frame, frame_name, subst_parent);
    apply_animation_groups(lua, frame, frame_name);

    create_child_frames(lua, state, frame, frame_name, subst_parent);
    if let Some(scroll_child) = frame.scroll_child() {
        create_scroll_child_frames(lua, state, &scroll_child.children, frame_name, subst_parent);
    }

    if let Some(scripts) = frame.scripts() {
        elements::apply_scripts_from_template(lua, scripts, frame_name);
    }
}

/// Apply animation groups from a FrameXml to an already-created frame.
fn apply_animation_groups(lua: &Lua, frame: &FrameXml, frame_name: &str) {
    let Some(anims) = frame.animations() else {
        return;
    };
    let mut code = format!("local frame = {}\n", lua_global_ref(frame_name));
    for group in &anims.animations {
        if group.is_virtual == Some(true) {
            continue;
        }
        code.push_str(&generate_animation_group_code(group, "frame"));
    }
    let _ = lua.load(&code).exec();
}

/// Apply KeyValues from inline frame content (handles multiple `<KeyValues>` blocks).
fn apply_inline_key_values(lua: &Lua, frame: &crate::xml::FrameXml, frame_name: &str) {
    let frame_ref = lua_global_ref(frame_name);
    for key_values in frame.all_key_values() {
        for kv in &key_values.values {
            let value = format_key_value(&kv.value, kv.value_type.as_deref());
            let code = format!(
                "do local f = {} if f then f.{} = {} end end",
                frame_ref, kv.key, value
            );
            let _ = lua.load(&code).exec();
        }
    }
}

/// Apply button textures from inline frame content.
fn apply_inline_button_textures(
    lua: &Lua,
    frame: &crate::xml::FrameXml,
    frame_name: &str,
    subst_parent: &str,
) {
    let texture_specs: &[(&str, &str, Option<&crate::xml::TextureXml>)] = &[
        ("Normal", "SetNormalTexture", frame.normal_texture()),
        ("Pushed", "SetPushedTexture", frame.pushed_texture()),
        ("Disabled", "SetDisabledTexture", frame.disabled_texture()),
        (
            "Highlight",
            "SetHighlightTexture",
            frame.highlight_texture(),
        ),
        ("Checked", "SetCheckedTexture", frame.checked_texture()),
        (
            "DisabledChecked",
            "SetDisabledCheckedTexture",
            frame.disabled_checked_texture(),
        ),
    ];
    for &(parent_key, setter, tex_opt) in texture_specs {
        if let Some(tex) = tex_opt {
            elements::create_button_texture_from_template(
                lua,
                tex,
                frame_name,
                subst_parent,
                parent_key,
                setter,
            );
        }
    }
}

/// Create ButtonText fontstring from template.
fn apply_button_text(
    lua: &Lua,
    frame: &crate::xml::FrameXml,
    frame_name: &str,
    subst_parent: &str,
) {
    let Some(fs) = frame.button_text() else {
        return;
    };
    elements::create_fontstring_from_template(lua, fs, frame_name, subst_parent, "OVERLAY");
    // Only apply SetAllPoints when the ButtonText has no explicit anchors.
    // Templates like ChatTabTemplate define explicit anchors (e.g. CENTER 0 -5)
    // that would be wiped by SetAllPoints.
    let has_anchors = fs.anchors.as_ref().is_some_and(|a| !a.anchors.is_empty());
    let text_ref = if let Some(ref pk) = fs.parent_key {
        format!("p[\"{}\"]", escape_lua_string(pk))
    } else {
        "select(p:GetNumRegions(), p:GetRegions())".to_string()
    };
    let set_all_points = if has_anchors {
        ""
    } else {
        "if t then t:SetAllPoints(p) end "
    };
    let code = format!(
        "do local p = {} if p then \
         local t = {text_ref} \
         {set_all_points}\
         if not p.Text then p.Text = t end \
         end end",
        lua_global_ref(frame_name)
    );
    let _ = lua.load(&code).exec();
}

/// Apply NormalFont/HighlightFont/DisabledFont from template XML.
fn apply_button_fonts(lua: &Lua, frame: &crate::xml::FrameXml, frame_name: &str) {
    let frame_ref = lua_global_ref(frame_name);
    for (setter, font_ref) in frame.button_fonts() {
        let Some(font_ref) = font_ref else { continue };
        let Some(style) = font_ref.style.as_deref().or(font_ref.inherits.as_deref()) else {
            continue;
        };
        let code = format!(
            "do local f={frame_ref} local fo={style} if f and fo then f:{setter}(fo) \
             if f.Text and f.Text.SetFontObject then f.Text:SetFontObject(fo) end end end"
        );
        let _ = lua.load(&code).exec();
    }
}

/// Create EditBox FontString child from template.
fn apply_editbox_fontstring(
    lua: &Lua,
    frame: &crate::xml::FrameXml,
    frame_name: &str,
    subst_parent: &str,
) {
    let Some(fs) = frame.font_string_child() else {
        return;
    };
    elements::create_fontstring_from_template(lua, fs, frame_name, subst_parent, "OVERLAY");
}

/// Get size values from a SizeXml.
pub(super) fn get_size_values(size: &crate::xml::SizeXml) -> (Option<f32>, Option<f32>) {
    if size.x.is_some() || size.y.is_some() {
        (size.x, size.y)
    } else if let Some(abs) = &size.abs_dimension {
        (abs.x, abs.y)
    } else {
        (None, None)
    }
}

/// Escape a string for use in Lua code.
fn escape_lua_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// Return a Lua expression that references a frame by its global name.
///
/// Frame names like `$TankMarkerCheckButton` contain characters that aren't
/// valid in Lua identifiers, so we always use `_G["name"]` instead of bare names.
pub(super) fn lua_global_ref(name: &str) -> String {
    format!("_G[\"{}\"]", escape_lua_string(name))
}

/// Generate a unique ID (delegates to shared atomic counter).
fn rand_id() -> u64 {
    crate::loader::helpers::rand_id()
}
