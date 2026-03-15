//! CreateFrame implementation for creating WoW frames from Lua.

use super::super::SimState;
use super::super::frame::{extract_frame_id, frame_ref, sync_child_to_lua};
use super::template::{apply_templates_from_registry, fire_deferred_child_onloads, fire_on_load};
use crate::loader::helpers::lua_global_ref;
use crate::widget::{Frame, WidgetType};
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Write a frame's owner_addon into the persistent `__frame_owners` registry table.
pub(crate) fn sync_frame_owner_to_lua(lua: &Lua, state: &Rc<RefCell<SimState>>, frame_id: u64) {
    let owner = state
        .borrow()
        .widgets
        .get(frame_id)
        .and_then(|f| f.owner_addon);
    if let Some(idx) = owner {
        if let Ok(t) = lua.named_registry_value::<mlua::Table>("__frame_owners") {
            let _ = t.raw_set(frame_id as i64, idx as i64);
        }
    }
}

/// Extract a frame ID from a Lua Value, handling forbidden proxy tables.
fn extract_frame_id_or_proxy(value: &Value) -> Option<u64> {
    match value {
        Value::UserData(_) => extract_frame_id(value),
        Value::Table(t) => {
            // Forbidden proxy: UserData stored at "__lud"
            if let Ok(inner) = t.raw_get::<Value>("__lud") {
                extract_frame_id(&inner)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Create the CreateFrame Lua function.
pub fn create_frame_function(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Function> {
    let state_clone = Rc::clone(&state);
    let create_frame = lua.create_function(move |lua, args: mlua::MultiValue| {
        let cfa = parse_create_frame_args(lua, &args, &state_clone)?;
        let widget_type = parse_widget_type(&cfa.frame_type)?;
        let old_same_name = cfa
            .name
            .as_ref()
            .and_then(|name| state_clone.borrow().widgets.get_id_by_name(name));
        let frame_id = register_new_frame(
            &state_clone,
            widget_type,
            cfa.name.clone(),
            cfa.parent_id,
            cfa.parent_explicit,
        );
        sync_frame_owner_to_lua(lua, &state_clone, frame_id);
        // Store original type name when it differs from the WidgetType enum name,
        // so GetObjectType() returns e.g. "ArchaeologyDigSiteFrame" instead of "Frame".
        if !widget_type.as_str().eq_ignore_ascii_case(&cfa.frame_type) {
            let obj_name = resolve_object_type_name(&cfa.frame_type);
            if let Some(f) = state_clone.borrow_mut().widgets.get_mut_visual(frame_id) {
                f.object_type_name = Some(obj_name);
            }
        }
        apply_frame_id_arg(&state_clone, frame_id, cfa.id);
        create_widget_type_defaults(lua, &mut state_clone.borrow_mut(), frame_id, widget_type);
        if cfa.frame_type == "ItemButton" {
            create_item_button_intrinsics(lua, &mut state_clone.borrow_mut(), frame_id);
        }
        let is_forbidden = state_clone
            .borrow()
            .widgets
            .get(frame_id)
            .map(|f| f.forbidden)
            .unwrap_or(false);
        let ud = create_frame_userdata(lua, frame_id, cfa.name.as_deref(), is_forbidden)?;
        if let Some(old_id) = old_same_name {
            migrate_lua_fields_to_new_frame(lua, old_id, frame_id)?;
        }
        store_widget_type_key(lua, &ud, widget_type, &cfa.frame_type)?;
        if matches!(widget_type, WidgetType::Button | WidgetType::CheckButton)
            && let Some(ref btn_name) = cfa.name
        {
            register_button_child_globals(lua, &state_clone, frame_id, btn_name)?;
        }
        if cfa.frame_type == "ItemButton" {
            apply_item_button_mixin(lua, frame_id);
        }
        let ref_name = cfa.name.unwrap_or_else(|| format!("__frame_{}", frame_id));
        apply_intrinsic_and_templates(
            lua,
            &state_clone,
            &cfa.frame_type,
            &ref_name,
            cfa.template.as_deref(),
            cfa.parent_id,
            frame_id,
        )?;
        Ok(ud)
    })?;
    Ok(create_frame)
}

fn resolve_object_type_name(frame_type: &str) -> String {
    match frame_type.to_ascii_lowercase().as_str() {
        "checkout" => "BlizzardCheckout".to_string(),
        _ => frame_type.to_string(),
    }
}

fn parse_widget_type(frame_type: &str) -> Result<WidgetType> {
    let wt = WidgetType::from_str(frame_type).ok_or_else(|| {
        crate::lua_api::script_helpers::lua_error_val(format!(
            "CreateFrame: Unknown frame type '{}'",
            frame_type
        ))
    })?;
    // Texture, FontString, and Line are Region types — not creatable via CreateFrame.
    // They must be created via frame:CreateTexture(), frame:CreateFontString(), frame:CreateLine().
    if matches!(
        wt,
        WidgetType::Texture | WidgetType::FontString | WidgetType::Line
    ) {
        return Err(crate::lua_api::script_helpers::lua_error_val(format!(
            "CreateFrame: Unknown frame type '{}'",
            frame_type
        )));
    }
    Ok(wt)
}

fn apply_frame_id_arg(state: &Rc<RefCell<SimState>>, frame_id: u64, id: Option<i32>) {
    if let Some(frame_lua_id) = id {
        if let Some(frame) = state.borrow_mut().widgets.get_mut_visual(frame_id) {
            frame.user_id = frame_lua_id;
        }
    }
}

fn apply_item_button_mixin(lua: &Lua, frame_id: u64) {
    let frame_key = format!("__frame_{}", frame_id);
    let code = format!(
        "do local f = {} if f and ItemButtonMixin then Mixin(f, ItemButtonMixin) end end",
        lua_global_ref(&frame_key)
    );
    let _ = lua.load(&code).exec();
}

/// Parsed CreateFrame arguments.
/// Apply intrinsic templates, user templates, and fire OnLoad.
fn apply_intrinsic_and_templates(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_type: &str,
    ref_name: &str,
    template: Option<&str>,
    parent_id: Option<u64>,
    frame_id: u64,
) -> mlua::Result<()> {
    if let Some(entry) = &crate::xml::get_template(frame_type) {
        let canonical = &entry.name;
        apply_templates_from_registry(lua, state, ref_name, canonical);
        let code = format!("{}.intrinsic = \"{}\"", lua_global_ref(ref_name), canonical);
        let _ = lua.load(&code).exec();
    }
    if let Some(tmpl) = template {
        apply_templates_from_registry(lua, state, ref_name, tmpl);
        if parent_id.is_some() {
            apply_parent_array_from_template(lua, tmpl, frame_id, ref_name);
        }
    }
    let suppress_depth: i32 = lua
        .globals()
        .get("__suppress_create_frame_onload")
        .unwrap_or(0);
    if suppress_depth <= 0 {
        fire_deferred_child_onloads(lua);
        fire_on_load(lua, ref_name);
    }
    Ok(())
}

struct CreateFrameArgs {
    frame_type: String,
    name: Option<String>,
    parent_id: Option<u64>,
    template: Option<String>,
    id: Option<i32>,
    /// Whether the parent was explicitly provided (vs defaulting to UIParent).
    parent_explicit: bool,
}

/// Parse the arguments to CreateFrame: (frameType, name, parent, template, id).
fn parse_create_frame_args(
    lua: &Lua,
    args: &mlua::MultiValue,
    state: &Rc<RefCell<SimState>>,
) -> Result<CreateFrameArgs> {
    let mut args_iter = args.iter();
    let frame_type = parse_frame_type_arg(lua, args_iter.next());
    let (name_raw, name_arg_invalid) = parse_name_arg(lua, args_iter.next());
    let (parent_id, parent_explicit, explicit_parent) =
        parse_parent_arg(&mut args_iter, name_arg_invalid)?;
    let template = coerce_string_arg(lua, args_iter.next());
    let id = parse_id_arg(args_iter.next());
    let name = name_raw.map(|n| substitute_parent_name(n, explicit_parent, state));
    Ok(CreateFrameArgs {
        frame_type,
        name,
        parent_id,
        template,
        id,
        parent_explicit,
    })
}

fn parse_frame_type_arg(lua: &Lua, v: Option<&Value>) -> String {
    v.and_then(|v| lua.coerce_string(v.clone()).ok().flatten())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Frame".to_string())
}

/// Returns `(name_raw, is_invalid)`. Invalid = non-coercible type (frame/userdata/function).
fn parse_name_arg(lua: &Lua, v: Option<&Value>) -> (Option<String>, bool) {
    let invalid = matches!(
        v,
        Some(Value::UserData(_) | Value::Table(_) | Value::Function(_))
    );
    let name = v
        .and_then(|v| lua.coerce_string(v.clone()).ok().flatten())
        .map(|s| s.to_string_lossy().to_string());
    (name, invalid)
}

/// Returns `(parent_id, parent_explicit, explicit_parent)`.
fn parse_parent_arg(
    args_iter: &mut std::collections::vec_deque::Iter<'_, Value>,
    name_arg_invalid: bool,
) -> Result<(Option<u64>, bool, Option<u64>)> {
    if name_arg_invalid {
        return Ok((None, false, None));
    }
    let parent_arg = args_iter.next();
    if matches!(parent_arg, Some(Value::String(_))) {
        return Err(crate::lua_api::script_helpers::lua_error_val(
            "Usage: CreateFrame(\"type\" [, \"name\"] [, parent] [, \"template\"] [, id])",
        ));
    }
    let explicit_parent = parent_arg.and_then(|v| extract_frame_id_or_proxy(v));
    let parent_explicit = explicit_parent.is_some();
    Ok((explicit_parent, parent_explicit, explicit_parent))
}

fn coerce_string_arg(lua: &Lua, v: Option<&Value>) -> Option<String> {
    v.and_then(|v| lua.coerce_string(v.clone()).ok().flatten())
        .map(|s| s.to_string_lossy().to_string())
}

fn parse_id_arg(v: Option<&Value>) -> Option<i32> {
    v.and_then(|v| match v {
        Value::Integer(n) => Some(*n as i32),
        Value::Number(n) => Some(*n as i32),
        _ => None,
    })
}

/// Replace the `$parent` prefix in a frame name with the actual ancestor name.
///
/// Matches wowless `ParentSub()` behavior:
/// - Pattern `^$[pP][aA][rR][eE][nN][tT]` — case-insensitive, start-of-string only
/// - Walk parent chain to find the first NAMED ancestor (skip unnamed/anonymous frames)
/// - Fallback: "Top" when no named ancestor exists
/// - Single replacement only (anchored to start of string)
pub(crate) fn apply_parent_sub(name: &str, parent_id: Option<u64>, state: &SimState) -> String {
    // Fast-path: check if name starts with "$parent" (case-insensitive, 7 chars)
    if name.len() < 7 {
        return name.to_string();
    }
    let prefix = &name[..7];
    if !prefix.eq_ignore_ascii_case("$parent") {
        return name.to_string();
    }

    // Walk parent chain to find first named ancestor
    let ancestor_name = find_named_ancestor(parent_id, state);
    let replacement = ancestor_name.as_deref().unwrap_or("Top");

    // Replace only the leading $parent prefix (chars 0..7), keep the rest
    format!("{}{}", replacement, &name[7..])
}

/// Walk the parent chain from `parent_id` and return the first frame with a non-empty name.
/// Skips UIParent — when the walk reaches UIParent, returns None so the caller uses "Top".
fn find_named_ancestor(start_id: Option<u64>, state: &SimState) -> Option<String> {
    let mut current_id = start_id;
    while let Some(id) = current_id {
        if let Some(frame) = state.widgets.get(id) {
            if let Some(ref n) = frame.name {
                if !n.is_empty() && n != "UIParent" {
                    return Some(n.clone());
                }
            }
            current_id = frame.parent_id;
        } else {
            break;
        }
    }
    None
}

/// Replace $parent/$Parent placeholders in a frame name with the actual parent name.
fn substitute_parent_name(
    name: String,
    parent_id: Option<u64>,
    state: &Rc<RefCell<SimState>>,
) -> String {
    apply_parent_sub(&name, parent_id, &state.borrow())
}

/// Register a new frame in the widget registry and set up parent-child relationship.
/// If a named frame already exists, orphan the old one (remove from parent's children and hide).
/// When `parent_explicit` is false (UIParent default), frame_level stays at 0.
/// Set owner_addon and forbidden flag on a new frame.
/// Panics if no owner can be determined — every CreateFrame must have a creator.
fn attribute_frame_owner(frame: &mut Frame, state: &Rc<RefCell<SimState>>, parent_id: Option<u64>) {
    let s = state.borrow();
    frame.owner_addon = s
        .loading_addon_index
        .or(s.executing_addon_index)
        .or_else(|| parent_id.and_then(|pid| s.widgets.get(pid).and_then(|p| p.owner_addon)));
    frame.forbidden = s.loading_forbidden;
    if frame.owner_addon.is_none() {
        eprintln!(
            "{} [WARN] CreateFrame {:?} ({:?}): no owner addon (runtime creation without parent)",
            crate::logging::elapsed_prefix(s.start_time),
            frame.name,
            frame.widget_type
        );
    }
}

fn register_new_frame(
    state: &Rc<RefCell<SimState>>,
    widget_type: WidgetType,
    name: Option<String>,
    parent_id: Option<u64>,
    parent_explicit: bool,
) -> u64 {
    let mut frame = Frame::new(widget_type, name.clone(), parent_id);
    let initial_hidden = state
        .borrow_mut()
        .create_frame_initial_hidden
        .take()
        .unwrap_or(false);
    if initial_hidden {
        frame.visible = false;
        frame.effective_alpha = 0.0;
    }

    attribute_frame_owner(&mut frame, state, parent_id);

    let frame_id = frame.id;

    let mut state = state.borrow_mut();

    // If a frame with this name already exists, orphan it (WoW behavior: old frame
    // becomes unreachable via global, but still exists in the registry).
    let old_same_name = name.as_ref().and_then(|n| state.widgets.get_id_by_name(n));
    if let Some(old_id) = old_same_name {
        orphan_old_frame(&mut state.widgets, old_id);
    }

    state.widgets.register(frame);

    // Migrate children AFTER register so the new frame exists in the registry.
    if let Some(old_id) = old_same_name {
        migrate_children_to_new_frame(&mut state.widgets, old_id, frame_id);
    }

    if let Some(pid) = parent_id {
        state.widgets.add_child(pid, frame_id);

        // Inherit strata, level, effective_alpha, and effective_scale from parent.
        // When parent was defaulted to UIParent (not explicitly specified),
        // skip frame_level inheritance — WoW keeps level at 0 in that case.
        let parent_props = state.widgets.get(pid).map(|p| {
            (
                p.frame_strata,
                p.frame_level,
                p.effective_alpha,
                p.effective_scale,
            )
        });
        if let Some((parent_strata, parent_level, parent_eff_alpha, parent_eff_scale)) =
            parent_props
            && let Some(f) = state.widgets.get_mut_visual(frame_id)
        {
            f.frame_strata = parent_strata;
            if parent_explicit {
                f.frame_level = parent_level + 1;
            }
            f.effective_alpha = if f.visible {
                parent_eff_alpha * f.alpha
            } else {
                0.0
            };
            f.effective_scale = parent_eff_scale * f.scale;
        }
    }

    // Track whether the parent was defaulted (not explicitly provided).
    // SetAllPoints() with no args uses this to decide whether to store nil
    // or the actual parent ID as relativeTo (matching wowless headless behavior).
    if !parent_explicit {
        if let Some(f) = state.widgets.get_mut_visual(frame_id) {
            f.default_parent = true;
        }
    }

    frame_id
}

/// Create a FrameRef UserData value for a frame and cache it in `_G`.
fn create_frame_userdata(
    lua: &Lua,
    frame_id: u64,
    name: Option<&str>,
    is_forbidden: bool,
) -> Result<Value> {
    let val = frame_ref(lua, frame_id)?;

    if let Some(n) = name {
        if is_forbidden {
            let proxy = create_forbidden_proxy(lua, val.clone())?;
            crate::lua_api::secure_env::set_in_both_envs(lua, n, proxy)?;
        } else {
            crate::lua_api::secure_env::set_in_both_envs(lua, n, val.clone())?;
        }
    }
    // __frame_{id} cache is handled by frame_ref(), but ensure it's set for named frames too
    let frame_key = format!("__frame_{}", frame_id);
    lua.globals().raw_set(frame_key.as_str(), val.clone())?;

    Ok(val)
}

fn migrate_lua_fields_to_new_frame(lua: &Lua, old_id: u64, new_id: u64) -> Result<()> {
    let old_val = frame_ref(lua, old_id)?;
    let new_val = frame_ref(lua, new_id)?;
    let (Value::UserData(old_ud), Value::UserData(new_ud)) = (old_val, new_val) else {
        return Ok(());
    };

    let old_fields: mlua::Table = old_ud.user_value()?;
    let new_fields: mlua::Table = new_ud.user_value()?;

    for pair in old_fields.clone().pairs::<Value, Value>() {
        let (key, value) = pair?;
        if new_fields.raw_get::<Value>(key.clone())?.is_nil() {
            new_fields.raw_set(key, value)?;
        }
    }

    Ok(())
}

/// Cache per-type `__index` table as `__ti` in fenv[1] for Lua-side method lookup.
fn store_widget_type_key(lua: &Lua, ud: &Value, wt: WidgetType, frame_type: &str) -> Result<()> {
    let type_key = if wt.as_str().eq_ignore_ascii_case(frame_type) {
        wt.as_str().to_owned()
    } else {
        resolve_object_type_name(frame_type)
    };
    if let Value::UserData(u) = ud {
        let fields: mlua::Table = u.user_value()?;
        fields.raw_set("__wt", lua.create_string(type_key.as_str())?)?;
    }
    Ok(())
}

/// Create a forbidden proxy table: `{ __lud = ud }` with shared `__forbidden_proxy_mt`.
fn create_forbidden_proxy(lua: &Lua, ud: Value) -> Result<Value> {
    let proxy = lua.create_table()?;
    proxy.raw_set("__lud", ud)?;
    let mt: mlua::Table = lua.named_registry_value("__forbidden_proxy_mt")?;
    proxy.set_metatable(Some(mt));
    Ok(Value::Table(proxy))
}

/// Register button's default texture children as globals.
///
/// In WoW, named buttons get globals like `ButtonNameNormalTexture`, etc.
/// Sets both the widget registry name and `_G` entry for each child.
fn register_button_child_globals(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    button_name: &str,
) -> Result<()> {
    let keys: Vec<(String, u64)> = {
        let st = state.borrow();
        let Some(btn) = st.widgets.get(frame_id) else {
            return Ok(());
        };
        [
            "NormalTexture",
            "PushedTexture",
            "HighlightTexture",
            "DisabledTexture",
            "Text",
        ]
        .iter()
        .filter_map(|key| btn.children_keys.get(*key).map(|&id| (key.to_string(), id)))
        .collect()
    };
    let mut st = state.borrow_mut();
    for (key, child_id) in keys {
        let global_name = format!("{}{}", button_name, key);
        st.widgets.set_name(child_id, global_name.clone());
        drop(st);
        if let Ok(val) = frame_ref(lua, child_id) {
            let _ = crate::lua_api::secure_env::set_in_both_envs(lua, &global_name, val);
        }
        st = state.borrow_mut();
    }
    Ok(())
}

/// Create default children for widget types that fundamentally need them.
/// This is separate from templates - these are intrinsic to the widget type.
fn create_widget_type_defaults(
    lua: &Lua,
    state: &mut SimState,
    frame_id: u64,
    widget_type: WidgetType,
) {
    match widget_type {
        WidgetType::Button | WidgetType::CheckButton => {
            create_button_defaults(state, frame_id);
        }
        WidgetType::GameTooltip => {
            create_tooltip_defaults(state, frame_id);
        }
        WidgetType::SimpleHTML => {
            state.simple_htmls.insert(
                frame_id,
                crate::lua_api::simple_html::SimpleHtmlData::default(),
            );
        }
        WidgetType::MessageFrame => {
            state.message_frames.insert(
                frame_id,
                crate::lua_api::message_frame::MessageFrameData::default(),
            );
        }
        WidgetType::Slider => {
            create_slider_defaults(lua, state, frame_id);
        }
        WidgetType::EditBox => {
            if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
                frame.mouse_enabled = true;
                frame.visible = false;
            }
        }
        _ => {}
    }
}

/// Initialize Button/CheckButton defaults (no child widgets created).
///
/// WoW creates button texture/text children lazily via SetNormalTexture,
/// SetText, etc. — not at button creation time. See `get_or_create_button_texture`
/// and `apply_set_button_texture` for lazy creation.
fn create_button_defaults(state: &mut SimState, frame_id: u64) {
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.mouse_enabled = true;
    }
}

/// Create default tooltip state and set TOOLTIP strata.
fn create_tooltip_defaults(state: &mut SimState, frame_id: u64) {
    state
        .tooltips
        .insert(frame_id, crate::lua_api::tooltip::TooltipData::default());
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.frame_strata = crate::widget::FrameStrata::Tooltip;
        frame.has_fixed_frame_strata = true;
    }
}

/// Create default fontstrings and thumb texture for Slider.
fn create_slider_defaults(lua: &Lua, state: &mut SimState, frame_id: u64) {
    let low_id = create_child_widget(state, WidgetType::FontString, frame_id);
    let high_id = create_child_widget(state, WidgetType::FontString, frame_id);
    let text_id = create_child_widget(state, WidgetType::FontString, frame_id);
    let thumb_id = create_child_widget(state, WidgetType::Texture, frame_id);

    if let Some(slider) = state.widgets.get_mut_visual(frame_id) {
        slider.children_keys.insert("Low".to_string(), low_id);
        slider.children_keys.insert("High".to_string(), high_id);
        slider.children_keys.insert("Text".to_string(), text_id);
        slider
            .children_keys
            .insert("ThumbTexture".to_string(), thumb_id);
    }
    let _ = sync_child_to_lua(lua, frame_id, "Low", low_id);
    let _ = sync_child_to_lua(lua, frame_id, "High", high_id);
    let _ = sync_child_to_lua(lua, frame_id, "Text", text_id);
    let _ = sync_child_to_lua(lua, frame_id, "ThumbTexture", thumb_id);
}

/// Add TOPLEFT+BOTTOMRIGHT anchors to fill the parent (equivalent to SetAllPoints).
fn add_fill_parent_anchors(frame: &mut Frame, parent_id: u64) {
    use crate::widget::{Anchor, AnchorPoint};
    frame.anchors.push(Anchor {
        point: AnchorPoint::TopLeft,
        relative_to: None,
        relative_to_id: Some(parent_id as usize),
        relative_point: AnchorPoint::TopLeft,
        x_offset: 0.0,
        y_offset: 0.0,
    });
    frame.anchors.push(Anchor {
        point: AnchorPoint::BottomRight,
        relative_to: None,
        relative_to_id: Some(parent_id as usize),
        relative_point: AnchorPoint::BottomRight,
        x_offset: 0.0,
        y_offset: 0.0,
    });
}

/// Remove old frame from its parent's children and hide it.
fn orphan_old_frame(widgets: &mut crate::widget::WidgetRegistry, old_id: u64) {
    if let Some(old_frame) = widgets.get(old_id)
        && let Some(old_parent_id) = old_frame.parent_id
        && let Some(old_parent) = widgets.get_mut_visual(old_parent_id)
    {
        old_parent.children.retain(|&c| c != old_id);
    }
    if let Some(old_frame) = widgets.get_mut_visual(old_id) {
        old_frame.visible = false;
    }
}

/// Move all children from an old frame to a new replacement frame.
///
/// When a named frame is re-created (e.g. UIParent defined in XML replaces the
/// pre-built one), frames that were parented to the old version need to be
/// reparented to the new one so they remain in the live visibility tree.
fn migrate_children_to_new_frame(
    widgets: &mut crate::widget::WidgetRegistry,
    old_id: u64,
    new_id: u64,
) {
    let children: Vec<u64> = widgets
        .get(old_id)
        .map(|f| f.children.clone())
        .unwrap_or_default();
    for &child_id in &children {
        if let Some(child) = widgets.get_mut_visual(child_id) {
            child.parent_id = Some(new_id);
        }
    }
    // Move children_keys too (e.g. NineSlice for tooltips)
    let keys: std::collections::HashMap<String, u64> = widgets
        .get(old_id)
        .map(|f| f.children_keys.clone())
        .unwrap_or_default();

    // Preserve the old frame's explicit size on the new frame if the new frame has no
    // size yet. This covers frames like UIParent which are pre-seeded with screen
    // dimensions before XML loads — the XML re-creates them with setAllPoints but no
    // explicit <Size>, so the new frame would start at 0x0 without this copy.
    let (old_width, old_height) = widgets
        .get(old_id)
        .map(|f| (f.width, f.height))
        .unwrap_or((0.0, 0.0));

    if let Some(new_frame) = widgets.get_mut_visual(new_id) {
        new_frame.children.extend(&children);
        for (k, v) in keys {
            new_frame.children_keys.entry(k).or_insert(v);
        }
        if new_frame.width == 0.0 && old_width > 0.0 {
            new_frame.width = old_width;
        }
        if new_frame.height == 0.0 && old_height > 0.0 {
            new_frame.height = old_height;
        }
    }
    if let Some(old_frame) = widgets.get_mut_visual(old_id) {
        old_frame.children.clear();
        old_frame.children_keys.clear();
    }
}

/// Create a child widget of the given type, register it, and add it as a child. Returns the ID.
fn create_child_widget(state: &mut SimState, widget_type: WidgetType, parent_id: u64) -> u64 {
    let child = Frame::new(widget_type, None, Some(parent_id));
    let child_id = child.id;
    state.widgets.register(child);
    state.widgets.add_child(parent_id, child_id);
    // Inherit strata and level from parent
    let parent_props = state
        .widgets
        .get(parent_id)
        .map(|p| (p.frame_strata, p.frame_level));
    if let Some((parent_strata, parent_level)) = parent_props {
        if let Some(f) = state.widgets.get_mut_visual(child_id) {
            f.frame_strata = parent_strata;
            f.frame_level = parent_level + 1;
        }
    }
    child_id
}

/// Create intrinsic children for ItemButton (from WoW's intrinsic="true" template).
/// ItemButton defines: icon (Texture), Count (FontString), Stock (FontString),
/// searchOverlay, ItemContextOverlay, IconBorder, IconOverlay, IconOverlay2 (Textures).
fn create_item_button_intrinsics(lua: &Lua, state: &mut SimState, frame_id: u64) {
    let icon_id = create_item_button_icon(state, frame_id);
    let count_id = create_item_button_count(state, frame_id);
    let stock_id = create_hidden_artwork_fontstring(state, frame_id);
    let icon_border_id = create_hidden_overlay(state, frame_id);
    let icon_overlay_id = create_hidden_overlay(state, frame_id);
    let icon_overlay2_id = create_hidden_overlay(state, frame_id);
    let search_overlay_id = create_fill_parent_overlay(state, frame_id);
    let context_overlay_id = create_hidden_overlay(state, frame_id);
    register_item_button_children(
        lua,
        state,
        frame_id,
        icon_id,
        count_id,
        stock_id,
        icon_border_id,
        icon_overlay_id,
        icon_overlay2_id,
        search_overlay_id,
        context_overlay_id,
    );
}

fn create_item_button_icon(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::Texture, frame_id);
    if let Some(tex) = state.widgets.get_mut_visual(id) {
        tex.draw_layer = crate::widget::DrawLayer::Border;
        add_fill_parent_anchors(tex, frame_id);
    }
    id
}

fn create_item_button_count(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::FontString, frame_id);
    if let Some(fs) = state.widgets.get_mut_visual(id) {
        fs.draw_layer = crate::widget::DrawLayer::Artwork;
        fs.visible = false;
        fs.justify_h = crate::widget::TextJustify::Right;
        fs.anchors.push(crate::widget::Anchor {
            point: crate::widget::AnchorPoint::BottomRight,
            relative_to: None,
            relative_to_id: Some(frame_id as usize),
            relative_point: crate::widget::AnchorPoint::BottomRight,
            x_offset: -5.0,
            y_offset: -2.0,
        });
    }
    id
}

fn create_hidden_artwork_fontstring(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::FontString, frame_id);
    if let Some(fs) = state.widgets.get_mut_visual(id) {
        fs.draw_layer = crate::widget::DrawLayer::Artwork;
        fs.visible = false;
    }
    id
}

fn create_fill_parent_overlay(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::Texture, frame_id);
    if let Some(tex) = state.widgets.get_mut_visual(id) {
        tex.draw_layer = crate::widget::DrawLayer::Overlay;
        tex.visible = false;
        add_fill_parent_anchors(tex, frame_id);
    }
    id
}

#[allow(clippy::too_many_arguments)]
fn register_item_button_children(
    lua: &Lua,
    state: &mut SimState,
    frame_id: u64,
    icon_id: u64,
    count_id: u64,
    stock_id: u64,
    icon_border_id: u64,
    icon_overlay_id: u64,
    icon_overlay2_id: u64,
    search_overlay_id: u64,
    context_overlay_id: u64,
) {
    if let Some(btn) = state.widgets.get_mut_visual(frame_id) {
        btn.children_keys.insert("icon".to_string(), icon_id);
        btn.children_keys.insert("Count".to_string(), count_id);
        btn.children_keys.insert("Stock".to_string(), stock_id);
        btn.children_keys
            .insert("IconBorder".to_string(), icon_border_id);
        btn.children_keys
            .insert("IconOverlay".to_string(), icon_overlay_id);
        btn.children_keys
            .insert("IconOverlay2".to_string(), icon_overlay2_id);
        btn.children_keys
            .insert("searchOverlay".to_string(), search_overlay_id);
        btn.children_keys
            .insert("ItemContextOverlay".to_string(), context_overlay_id);
    }
    let _ = sync_child_to_lua(lua, frame_id, "icon", icon_id);
    let _ = sync_child_to_lua(lua, frame_id, "Count", count_id);
    let _ = sync_child_to_lua(lua, frame_id, "Stock", stock_id);
    let _ = sync_child_to_lua(lua, frame_id, "IconBorder", icon_border_id);
    let _ = sync_child_to_lua(lua, frame_id, "IconOverlay", icon_overlay_id);
    let _ = sync_child_to_lua(lua, frame_id, "IconOverlay2", icon_overlay2_id);
    let _ = sync_child_to_lua(lua, frame_id, "searchOverlay", search_overlay_id);
    let _ = sync_child_to_lua(lua, frame_id, "ItemContextOverlay", context_overlay_id);
}

/// Check the template chain for a `parentArray` attribute and insert the frame
/// into its parent's Lua array if found.
fn apply_parent_array_from_template(
    lua: &Lua,
    template_names: &str,
    _frame_id: u64,
    ref_name: &str,
) {
    let chain = crate::xml::get_template_chain(template_names);
    for entry in &chain {
        if let Some(parent_array) = &entry.frame.parent_array {
            let frame_ref = lua_global_ref(ref_name);
            let code = format!(
                "do local child = {frame_ref}\n\
                 if child then\n\
                     local parent = child:GetParent()\n\
                     if parent then\n\
                         parent[\"{parent_array}\"] = parent[\"{parent_array}\"] or {{}}\n\
                         table.insert(parent[\"{parent_array}\"], child)\n\
                     end\n\
                 end\nend",
            );
            let _ = lua.load(&code).exec();
            break;
        }
    }
}

/// Create a hidden overlay texture child (OVERLAY layer, hidden, centered on parent).
fn create_hidden_overlay(state: &mut SimState, parent_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::Texture, parent_id);
    if let Some(tex) = state.widgets.get_mut_visual(id) {
        tex.draw_layer = crate::widget::DrawLayer::Overlay;
        tex.visible = false;
        tex.anchors.push(crate::widget::Anchor {
            point: crate::widget::AnchorPoint::Center,
            relative_to: None,
            relative_to_id: Some(parent_id as usize),
            relative_point: crate::widget::AnchorPoint::Center,
            x_offset: 0.0,
            y_offset: 0.0,
        });
    }
    id
}
