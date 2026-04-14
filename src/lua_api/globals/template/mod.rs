//! Template application from the XML template registry.
//!
//! This module provides functionality to apply XML templates from the registry
//! when CreateFrame is called with a template name.

mod children;
pub(crate) mod direct;
mod elements;
mod elements_text;

use crate::loader::chunk_cache;
use crate::loader::helpers_anim::generate_animation_group_code;
use crate::loader::precompiled;
use crate::lua_api::SimState;
use crate::lua_api::frame::FrameRef;
use crate::xml::{
    FrameElement, FrameXml, LayerElement, LayerXml, TemplateEntry, TextureXml, get_template_chain,
};
use mlua::{Lua, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
pub(crate) mod test_counters {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static LUA_TEMPLATE_TEXTURE_CREATES: AtomicUsize = AtomicUsize::new(0);
    static LUA_TEMPLATE_FONTSTRING_CREATES: AtomicUsize = AtomicUsize::new(0);
    static LUA_TEMPLATE_BUTTON_TEXTURE_CREATES: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Snapshot {
        pub(crate) texture_creates: usize,
        pub(crate) fontstring_creates: usize,
        pub(crate) button_texture_creates: usize,
    }

    pub(crate) fn reset() {
        LUA_TEMPLATE_TEXTURE_CREATES.store(0, Ordering::SeqCst);
        LUA_TEMPLATE_FONTSTRING_CREATES.store(0, Ordering::SeqCst);
        LUA_TEMPLATE_BUTTON_TEXTURE_CREATES.store(0, Ordering::SeqCst);
    }

    pub(crate) fn snapshot() -> Snapshot {
        Snapshot {
            texture_creates: LUA_TEMPLATE_TEXTURE_CREATES.load(Ordering::SeqCst),
            fontstring_creates: LUA_TEMPLATE_FONTSTRING_CREATES.load(Ordering::SeqCst),
            button_texture_creates: LUA_TEMPLATE_BUTTON_TEXTURE_CREATES.load(Ordering::SeqCst),
        }
    }

    pub(crate) fn record_texture_create() {
        LUA_TEMPLATE_TEXTURE_CREATES.fetch_add(1, Ordering::SeqCst);
    }

    pub(crate) fn record_fontstring_create() {
        LUA_TEMPLATE_FONTSTRING_CREATES.fetch_add(1, Ordering::SeqCst);
    }

    pub(crate) fn record_button_texture_create() {
        LUA_TEMPLATE_BUTTON_TEXTURE_CREATES.fetch_add(1, Ordering::SeqCst);
    }
}

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

/// Frame-like elements that share the XML mapping used during regular loading.
fn frame_like_frame_element(
    element: &FrameElement,
) -> Option<(&FrameXml, &'static str, Option<&'static str>)> {
    if !matches!(
        element,
        FrameElement::EventFrame(_)
            | FrameElement::TaxiRouteFrame(_)
            | FrameElement::ModelFFX(_)
            | FrameElement::UiCamera(_)
            | FrameElement::UnitPositionFrame(_)
            | FrameElement::OffScreenFrame(_)
            | FrameElement::Checkout(_)
            | FrameElement::FogOfWarFrame(_)
            | FrameElement::QuestPOIFrame(_)
            | FrameElement::ArchaeologyDigSiteFrame(_)
            | FrameElement::ScenarioPOIFrame(_)
            | FrameElement::UIThemeContainerFrame(_)
            | FrameElement::MapScene(_)
            | FrameElement::Line(_)
            | FrameElement::Browser(_)
            | FrameElement::Minimap(_)
            | FrameElement::MovieFrame(_)
            | FrameElement::WorldFrame(_)
    ) {
        return None;
    }

    let (frame, tag) = element.as_frame_data()?;
    let (widget_type, intrinsic) = crate::xml::widget_type_for_tag(tag)?;
    Some((frame, widget_type, intrinsic))
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
    for entry in &chain {
        if let Some(scripts) = entry.frame.scripts() {
            elements::apply_missing_scripts_from_template(lua, scripts, frame_name);
        }
    }
    elements::apply_deferred_mask_atlases(lua, frame_name, &chain);
}

/// Fire all deferred child OnLoad scripts that were queued during template
/// application while `__suppress_create_frame_onload` was active.
pub fn fire_deferred_child_onloads(lua: &Lua) -> usize {
    let Ok(deferred) = lua.globals().get::<mlua::Table>("__deferred_child_onloads") else {
        return 0;
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
    names.len()
}

/// Apply a single template entry to a frame.
fn apply_single_template(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_name: &str,
    entry: &TemplateEntry,
) {
    let template = &entry.frame;
    let use_direct_region_creation = entry.name == "ActionButtonTemplate";

    apply_template_setup(lua, state, template, frame_name);
    apply_template_regions(lua, state, template, frame_name, use_direct_region_creation);
    create_template_children(lua, state, template, frame_name, &entry.name);
    apply_template_scripts(lua, template, frame_name);
}

fn apply_template_setup(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    template: &FrameXml,
    frame_name: &str,
) {
    apply_mixin(lua, &template.combined_mixin(), frame_name);
    for key_values in template.all_key_values() {
        apply_key_values(lua, key_values, frame_name);
    }
    if let Some(frame_id) = resolve_template_frame_id(state, frame_name) {
        apply_direct_rust_properties(state, frame_id, template, frame_name);
    }
}

fn apply_template_regions(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    template: &FrameXml,
    frame_name: &str,
    use_direct_region_creation: bool,
) {
    apply_layers(
        lua,
        state,
        template,
        frame_name,
        frame_name,
        use_direct_region_creation,
    );
    apply_button_texture_set(
        lua,
        state,
        template,
        frame_name,
        frame_name,
        use_direct_region_creation,
    );
    if let Some(bar) = template.bar_texture() {
        elements_text::create_bar_texture_from_template(lua, bar, frame_name, frame_name);
    }
    if let Some(thumb) = template.thumb_texture() {
        elements_text::create_thumb_texture_from_template(lua, thumb, frame_name, frame_name);
    }
    apply_button_text(lua, template, frame_name, frame_name);
    elements_text::apply_button_text_attribute(lua, template, frame_name);
    apply_editbox_fontstring(lua, template, frame_name, frame_name);
    apply_button_fonts(lua, template, frame_name);
    apply_animation_groups(lua, template, frame_name);
}

fn create_template_children(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    template: &FrameXml,
    frame_name: &str,
    template_name: &str,
) {
    let use_direct_child_creation = children::use_direct_runtime_child_creation(template_name);
    children::create_child_frames(
        lua,
        state,
        template,
        frame_name,
        frame_name,
        use_direct_child_creation,
    );
    if let Some(scroll_child) = template.scroll_child() {
        children::create_scroll_child_frames(
            lua,
            state,
            &scroll_child.children,
            frame_name,
            frame_name,
            use_direct_child_creation,
        );
    }
}

fn apply_template_scripts(lua: &Lua, template: &FrameXml, frame_name: &str) {
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
    if key_values.values.is_empty() {
        return;
    }
    if apply_key_values_direct(lua, key_values, frame_name).is_err() {
        apply_key_values_via_chunk(lua, key_values, frame_name);
    }
}

fn apply_key_values_direct(
    lua: &Lua,
    key_values: &crate::xml::KeyValuesXml,
    frame_name: &str,
) -> mlua::Result<()> {
    let Some(fields) = frame_fields_by_name(lua, frame_name)? else {
        return Ok(());
    };
    for kv in &key_values.values {
        let value = key_value_to_lua_value(lua, &kv.value, kv.value_type.as_deref())?;
        fields.raw_set(kv.key.as_str(), value)?;
    }
    Ok(())
}

fn apply_key_values_via_chunk(lua: &Lua, key_values: &crate::xml::KeyValuesXml, frame_name: &str) {
    let frame_ref = lua_global_ref(frame_name);
    let mut code = format!("do local f = {} if f then ", frame_ref);
    for kv in &key_values.values {
        let value = format_key_value(&kv.value, kv.value_type.as_deref());
        code.push_str(&format!("f.{} = {} ", kv.key, value));
    }
    code.push_str("end end");
    let _ = chunk_cache::exec(lua, &code, "template-key-values");
}

fn key_value_to_lua_value(lua: &Lua, value: &str, value_type: Option<&str>) -> mlua::Result<Value> {
    match value_type {
        Some("number") => parse_number_value(value),
        Some("boolean") => Ok(Value::Boolean(value.eq_ignore_ascii_case("true"))),
        Some("global") => resolve_global_path_value(lua, value),
        _ => Ok(Value::String(lua.create_string(value)?)),
    }
}

fn parse_number_value(value: &str) -> mlua::Result<Value> {
    if let Ok(integer) = value.parse::<i64>() {
        return Ok(Value::Integer(integer));
    }
    if let Ok(number) = value.parse::<f64>() {
        return Ok(Value::Number(number));
    }
    Err(mlua::Error::RuntimeError(format!(
        "invalid numeric key value '{}'",
        value
    )))
}

/// Format a key value for Lua assignment fallback.
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
fn apply_layers(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    template: &FrameXml,
    frame_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) {
    let region_ctx = RegionCreateContext {
        lua,
        state,
        frame_name,
        subst_parent,
    };
    for layers in template.layers() {
        for layer in &layers.layers {
            apply_layer_block(region_ctx, layer, use_direct_creation);
        }
    }
}

#[derive(Clone, Copy)]
struct RegionCreateContext<'a> {
    lua: &'a Lua,
    state: &'a Rc<RefCell<SimState>>,
    frame_name: &'a str,
    subst_parent: &'a str,
}

#[derive(Clone, Copy)]
struct LayerCreateContext<'a> {
    region: RegionCreateContext<'a>,
    draw_layer: &'a str,
    use_direct_creation: bool,
}

#[derive(Clone, Copy)]
struct ButtonTextureContext<'a> {
    region: RegionCreateContext<'a>,
    use_direct_creation: bool,
}

fn apply_layer_block(region: RegionCreateContext<'_>, layer: &LayerXml, use_direct_creation: bool) {
    let draw_layer = layer.level.as_deref().unwrap_or("ARTWORK");
    let ctx = LayerCreateContext {
        region,
        draw_layer,
        use_direct_creation,
    };
    for element in &layer.elements {
        apply_layer_element(ctx, element);
    }
}

fn apply_layer_element(ctx: LayerCreateContext<'_>, element: &LayerElement) {
    match element {
        LayerElement::Texture(texture) => create_texture_layer(ctx, texture, false, false),
        LayerElement::Line(texture) => create_texture_layer(ctx, texture, false, true),
        LayerElement::MaskTexture(texture) => create_texture_layer(ctx, texture, true, false),
        LayerElement::FontString(fontstring) => create_fontstring_layer(ctx, fontstring),
    }
}

fn create_texture_layer(
    ctx: LayerCreateContext<'_>,
    texture: &TextureXml,
    is_mask: bool,
    is_line: bool,
) {
    let used_direct = ctx.use_direct_creation
        && elements::create_texture_from_template_direct(
            ctx.region.lua,
            ctx.region.state,
            texture,
            ctx.region.frame_name,
            ctx.region.subst_parent,
            ctx.draw_layer,
            is_mask,
            is_line,
        )
        .is_ok();
    if used_direct {
        return;
    }

    elements::create_texture_from_template(
        ctx.region.lua,
        texture,
        ctx.region.frame_name,
        ctx.region.subst_parent,
        ctx.draw_layer,
        is_mask,
        is_line,
    );
}

fn create_fontstring_layer(ctx: LayerCreateContext<'_>, fontstring: &crate::xml::FontStringXml) {
    let used_direct = ctx.use_direct_creation
        && elements_text::create_fontstring_from_template_direct(
            ctx.region.lua,
            ctx.region.state,
            fontstring,
            ctx.region.frame_name,
            ctx.region.subst_parent,
            ctx.draw_layer,
        )
        .is_ok();
    if used_direct {
        return;
    }

    elements_text::create_fontstring_from_template(
        ctx.region.lua,
        fontstring,
        ctx.region.frame_name,
        ctx.region.subst_parent,
        ctx.draw_layer,
    );
}

/// Apply button textures (NormalTexture, PushedTexture, etc.) from a frame/template.
pub(super) fn apply_button_texture_set(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    frame_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) {
    let ctx = ButtonTextureContext {
        region: RegionCreateContext {
            lua,
            state,
            frame_name,
            subst_parent,
        },
        use_direct_creation,
    };
    for (parent_key, setter, tex_opt) in button_texture_specs(frame) {
        let Some(texture) = tex_opt else {
            continue;
        };
        apply_button_texture_spec(ctx, texture, parent_key, setter);
    }
}

fn button_texture_specs(
    frame: &FrameXml,
) -> [(&'static str, &'static str, Option<&TextureXml>); 6] {
    [
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
    ]
}

fn apply_button_texture_spec(
    ctx: ButtonTextureContext<'_>,
    texture: &TextureXml,
    parent_key: &str,
    setter: &str,
) {
    let used_direct = ctx.use_direct_creation
        && elements_text::create_button_texture_from_template_direct(
            ctx.region.lua,
            ctx.region.state,
            texture,
            ctx.region.frame_name,
            ctx.region.subst_parent,
            parent_key,
        )
        .is_ok();
    if used_direct {
        return;
    }

    elements_text::create_button_texture_from_template(
        ctx.region.lua,
        texture,
        ctx.region.frame_name,
        ctx.region.subst_parent,
        parent_key,
        setter,
    );
}

/// Apply mixin to a frame.
fn apply_mixin(lua: &Lua, mixin: &Option<String>, frame_name: &str) {
    let Some(mixin) = mixin else { return };
    if apply_mixin_direct(lua, mixin, frame_name).is_ok() {
        return;
    }
    apply_mixin_via_chunk(lua, mixin, frame_name);
}

fn apply_mixin_direct(lua: &Lua, mixin: &str, frame_name: &str) -> mlua::Result<()> {
    let Some(fields) = frame_fields_by_name(lua, frame_name)? else {
        return Ok(());
    };
    let secure_methods: Value = lua.globals().get("__secureMixinMethods")?;
    for mixin_table in resolve_mixin_tables(lua, mixin)? {
        // For secure mixins, methods are moved to __index and the table is empty.
        // Check __secureMixinMethods for the actual methods table, same as Mixin().
        let source = if let Value::Table(ref sm) = secure_methods {
            if let Value::Table(t) = sm.get::<Value>(mixin_table.clone())? {
                t
            } else {
                mixin_table
            }
        } else {
            mixin_table
        };
        for pair in source.pairs::<Value, Value>() {
            let (key, value) = pair?;
            fields.raw_set(key, value)?;
        }
    }
    let post_init = build_mixin_post_init(mixin);
    if !post_init.is_empty() {
        let code = format!(
            "do local f = {} if f then {} end end",
            lua_global_ref(frame_name),
            post_init,
        );
        let _ = chunk_cache::exec(lua, &code, "template-mod");
    }
    Ok(())
}

fn resolve_mixin_tables(lua: &Lua, mixin: &str) -> mlua::Result<Vec<mlua::Table>> {
    let mut mixins = Vec::new();
    for name in mixin.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Value::Table(table) = resolve_top_level_value(lua, name)? {
            mixins.push(table);
        }
    }
    Ok(mixins)
}

fn apply_mixin_via_chunk(lua: &Lua, mixin: &str, frame_name: &str) {
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
    let _ = chunk_cache::exec(lua, &code, "template-mod");
}

fn frame_fields_by_name(lua: &Lua, frame_name: &str) -> mlua::Result<Option<mlua::Table>> {
    let value: Value = lua.globals().raw_get(frame_name)?;
    frame_fields_from_value(value)
}

fn frame_fields_from_value(value: Value) -> mlua::Result<Option<mlua::Table>> {
    let userdata = match value {
        Value::UserData(ud) => Some(ud),
        Value::Table(t) => match t.raw_get::<Value>("__lud")? {
            Value::UserData(ud) => Some(ud),
            _ => None,
        },
        _ => None,
    };
    let Some(userdata) = userdata else {
        return Ok(None);
    };
    if userdata.is::<FrameRef>() {
        return userdata.user_value::<mlua::Table>().map(Some);
    }
    Ok(None)
}

fn resolve_global_path_value(lua: &Lua, path: &str) -> mlua::Result<Value> {
    let mut segments = path.split('.').filter(|segment| !segment.is_empty());
    let Some(first) = segments.next() else {
        return Ok(Value::Nil);
    };
    let mut current = resolve_top_level_value(lua, first)?;
    for segment in segments {
        current = match current {
            Value::Table(table) => table.get::<Value>(segment)?,
            _ => return Ok(Value::Nil),
        };
    }
    Ok(current)
}

fn resolve_top_level_value(lua: &Lua, name: &str) -> mlua::Result<Value> {
    let globals = lua.globals();
    let global_value = globals.get::<Value>(name)?;
    if !global_value.is_nil() {
        return Ok(global_value);
    }
    if let Ok(secureenv) = lua.named_registry_value::<mlua::Table>("__secureenv") {
        let secure_value = secureenv.get::<Value>(name)?;
        if !secure_value.is_nil() {
            return Ok(secure_value);
        }
    }
    Ok(Value::Nil)
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
                post_init.push_str("f.callbackTables = f.callbackTables or {} ");
                post_init.push_str("f.executingEvents = f.executingEvents or {} ");
            }
            "UIParentManagedFrameContainerMixin" => {
                post_init.push_str("f.showingFrames = f.showingFrames or {} ");
            }
            "TabSystemMixin" => {
                post_init.push_str("f.tabs = f.tabs or {} ");
            }
            _ => {}
        }
    }
    post_init
}

/// Push the suppress-OnLoad depth counter (prevents premature OnLoad in nested CreateFrame).
/// Uses a precompiled Lua function to avoid Rust→Lua globals.get + globals.set overhead.
fn push_suppress(lua: &Lua) {
    if let Some(fns) = precompiled::try_get(lua) {
        let _ = fns.suppress_push.call::<()>(());
    }
}

/// Pop the suppress-OnLoad depth counter.
/// Uses a precompiled Lua function to avoid Rust→Lua globals.get + globals.set overhead.
fn pop_suppress(lua: &Lua) {
    if let Some(fns) = precompiled::try_get(lua) {
        let _ = fns.suppress_pop.call::<()>(());
    }
}

/// Queue a child frame name for deferred OnLoad firing.
/// Uses a precompiled Lua function to avoid repeated globals.get/set and table creation.
fn defer_child_onload(lua: &Lua, name: &str) {
    if let Some(fns) = precompiled::try_get(lua) {
        let _ = fns.defer_onload.call::<()>(name);
    }
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
    if let Some(fns) = precompiled::try_get(lua) {
        if let Err(e) = fns.fire_onload.call::<()>(frame_name) {
            eprintln!("[fire_on_load] {} error: {}", frame_name, e);
        }
        return;
    }

    let frame_ref = lua_global_ref(frame_name);
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
    let _ = chunk_cache::exec(lua, &code, "template-mod");
}

/// Apply KeyValues from inline frame content (handles multiple `<KeyValues>` blocks).
fn apply_inline_key_values(lua: &Lua, frame: &crate::xml::FrameXml, frame_name: &str) {
    let frame_ref = lua_global_ref(frame_name);
    for key_values in frame.all_key_values() {
        if key_values.values.is_empty() {
            continue;
        }
        let mut code = format!("do local f = {} if f then ", frame_ref);
        for kv in &key_values.values {
            let value = format_key_value(&kv.value, kv.value_type.as_deref());
            code.push_str(&format!("f.{} = {} ", kv.key, value));
        }
        code.push_str("end end");
        let _ = chunk_cache::exec(lua, &code, "template-key-values");
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
    elements_text::create_fontstring_from_template(lua, fs, frame_name, subst_parent, "OVERLAY");
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
    let _ = chunk_cache::exec(lua, &code, "template-mod");
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
        let _ = chunk_cache::exec(lua, &code, "template-mod");
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
    elements_text::create_fontstring_from_template(lua, fs, frame_name, subst_parent, "OVERLAY");
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
    if let Some(id) = name
        .strip_prefix("__frame_")
        .and_then(|suffix| suffix.parse::<u64>().ok())
    {
        return format!("debug.getregistry().__frame_refs[{id}]");
    }
    format!("_G[\"{}\"]", escape_lua_string(name))
}

/// Generate a unique ID (delegates to shared atomic counter).
fn rand_id() -> u64 {
    crate::loader::helpers::rand_id()
}
