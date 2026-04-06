//! CreateFrame implementation for creating WoW frames from Lua.

mod widget_defaults;

use super::super::SimState;
use super::super::frame::{extract_frame_id, frame_ref};
use super::create_frame_util::{
    apply_parent_array_from_template, migrate_children_to_new_frame, orphan_old_frame,
    register_button_child_globals,
};
use super::template::{apply_templates_from_registry, fire_deferred_child_onloads, fire_on_load};
use crate::loader::helpers::lua_global_ref;
use crate::widget::{Frame, WidgetRegistry, WidgetType};
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;
use widget_defaults::{create_item_button_intrinsics, create_widget_type_defaults};

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
        let frame_id = register_frame_from_args(&state_clone, widget_type, &cfa);
        configure_frame_metadata(
            lua,
            &state_clone,
            frame_id,
            widget_type,
            &cfa.frame_type,
            cfa.id,
        );
        let is_forbidden = frame_is_forbidden(&state_clone, frame_id);
        let ud = create_frame_userdata(lua, frame_id, cfa.name.as_deref(), is_forbidden)?;
        if let Some(old_id) = old_same_name {
            migrate_lua_fields_to_new_frame(lua, old_id, frame_id)?;
        }
        finalize_registered_frame(lua, &state_clone, frame_id, widget_type, &cfa, &ud)?;
        Ok(ud)
    })?;
    Ok(create_frame)
}

fn register_frame_from_args(
    state: &Rc<RefCell<SimState>>,
    widget_type: WidgetType,
    cfa: &CreateFrameArgs,
) -> u64 {
    register_new_frame(
        state,
        widget_type,
        cfa.name.clone(),
        cfa.parent_id,
        cfa.parent_explicit,
    )
}

fn configure_frame_metadata(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    widget_type: WidgetType,
    frame_type: &str,
    frame_lua_id: Option<i32>,
) {
    sync_frame_owner_to_lua(lua, state, frame_id);
    apply_object_type_name(state, frame_id, widget_type, frame_type);
    apply_frame_id_arg(state, frame_id, frame_lua_id);
    create_frame_widget_defaults(lua, state, frame_id, frame_type, widget_type);
}

fn apply_object_type_name(
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    widget_type: WidgetType,
    frame_type: &str,
) {
    if widget_type.as_str().eq_ignore_ascii_case(frame_type) {
        return;
    }

    let object_type_name = resolve_object_type_name(frame_type);
    if let Some(frame) = state.borrow_mut().widgets.get_mut_visual(frame_id) {
        frame.object_type_name = Some(object_type_name);
    }
}

fn create_frame_widget_defaults(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    frame_type: &str,
    widget_type: WidgetType,
) {
    create_widget_type_defaults(lua, &mut state.borrow_mut(), frame_id, widget_type);
    if frame_type == "ItemButton" {
        create_item_button_intrinsics(lua, &mut state.borrow_mut(), frame_id);
    }
}

fn frame_is_forbidden(state: &Rc<RefCell<SimState>>, frame_id: u64) -> bool {
    state
        .borrow()
        .widgets
        .get(frame_id)
        .map(|frame| frame.forbidden)
        .unwrap_or(false)
}

fn finalize_registered_frame(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    widget_type: WidgetType,
    cfa: &CreateFrameArgs,
    ud: &Value,
) -> mlua::Result<()> {
    store_widget_type_key(lua, ud, widget_type, &cfa.frame_type)?;
    register_named_button_children(lua, state, frame_id, widget_type, cfa.name.as_deref())?;
    if cfa.frame_type == "ItemButton" {
        apply_item_button_mixin(lua, frame_id);
    }

    let ref_name = cfa
        .name
        .clone()
        .unwrap_or_else(|| format!("__frame_{}", frame_id));
    apply_intrinsic_and_templates(
        lua,
        state,
        &cfa.frame_type,
        &ref_name,
        cfa.template.as_deref(),
        cfa.parent_id,
        frame_id,
    )
}

fn register_named_button_children(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    widget_type: WidgetType,
    frame_name: Option<&str>,
) -> mlua::Result<()> {
    if !matches!(widget_type, WidgetType::Button | WidgetType::CheckButton) {
        return Ok(());
    }

    let Some(button_name) = frame_name else {
        return Ok(());
    };
    register_button_child_globals(lua, state, frame_id, button_name)
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
        parse_parent_arg(&mut args_iter, name_arg_invalid, state)?;
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
    state: &Rc<RefCell<SimState>>,
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
    let parent_id = explicit_parent.or_else(|| default_parent_id(state));
    Ok((parent_id, parent_explicit, explicit_parent))
}

fn default_parent_id(state: &Rc<RefCell<SimState>>) -> Option<u64> {
    state.borrow().widgets.get_id_by_name("UIParent")
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
    let frame = build_new_frame(state, widget_type, name.clone(), parent_id);
    let frame_id = frame.id;
    let mut state = state.borrow_mut();
    register_frame_instance(&mut state, name.as_deref(), frame);
    apply_new_frame_parent_state(&mut state, frame_id, parent_id, parent_explicit);
    frame_id
}

fn build_new_frame(
    state: &Rc<RefCell<SimState>>,
    widget_type: WidgetType,
    name: Option<String>,
    parent_id: Option<u64>,
) -> Frame {
    let mut frame = Frame::new(widget_type, name, parent_id);
    if take_create_frame_initial_hidden(state) {
        frame.visible = false;
        frame.effective_alpha = 0.0;
    }
    if widget_type_defaults_mouse_enabled(widget_type) {
        frame.mouse_enabled = true;
    }
    attribute_frame_owner(&mut frame, state, parent_id);
    frame
}

/// WoW enables mouse interaction by default on interactive widget types.
fn widget_type_defaults_mouse_enabled(widget_type: WidgetType) -> bool {
    matches!(
        widget_type,
        WidgetType::Button | WidgetType::CheckButton | WidgetType::EditBox
    )
}

fn take_create_frame_initial_hidden(state: &Rc<RefCell<SimState>>) -> bool {
    state
        .borrow_mut()
        .create_frame_initial_hidden
        .take()
        .unwrap_or(false)
}

fn register_frame_instance(state: &mut SimState, name: Option<&str>, frame: Frame) {
    let frame_id = frame.id;
    let old_same_name = name.and_then(|name| state.widgets.get_id_by_name(name));
    orphan_same_name_frame(&mut state.widgets, old_same_name);
    state.widgets.register(frame);
    migrate_recreated_frame_children(&mut state.widgets, old_same_name, frame_id);
}

fn orphan_same_name_frame(widgets: &mut WidgetRegistry, old_same_name: Option<u64>) {
    if let Some(old_id) = old_same_name {
        // WoW behavior: old frame becomes unreachable via global,
        // but still exists in the registry.
        orphan_old_frame(widgets, old_id);
    }
}

fn migrate_recreated_frame_children(
    widgets: &mut WidgetRegistry,
    old_same_name: Option<u64>,
    frame_id: u64,
) {
    if let Some(old_id) = old_same_name {
        // Migrate children AFTER register so the new frame exists in the registry.
        migrate_children_to_new_frame(widgets, old_id, frame_id);
    }
}

fn apply_new_frame_parent_state(
    state: &mut SimState,
    frame_id: u64,
    parent_id: Option<u64>,
    parent_explicit: bool,
) {
    if let Some(pid) = parent_id {
        state.widgets.add_child(pid, frame_id);
        inherit_parent_frame_state(&mut state.widgets, frame_id, pid, parent_explicit);
    }
    if !parent_explicit {
        mark_default_parent(&mut state.widgets, frame_id);
    }
}

fn inherit_parent_frame_state(
    widgets: &mut WidgetRegistry,
    frame_id: u64,
    parent_id: u64,
    parent_explicit: bool,
) {
    let parent_props = widgets.get(parent_id).map(|parent| {
        (
            parent.frame_strata,
            parent.frame_level,
            parent.effective_alpha,
            parent.effective_scale,
        )
    });
    let Some((parent_strata, parent_level, parent_eff_alpha, parent_eff_scale)) = parent_props
    else {
        return;
    };
    let Some(frame) = widgets.get_mut_visual(frame_id) else {
        return;
    };
    frame.frame_strata = parent_strata;
    if parent_explicit {
        frame.frame_level = parent_level + 1;
    }
    frame.effective_alpha = if frame.visible {
        parent_eff_alpha * frame.alpha
    } else {
        0.0
    };
    frame.effective_scale = parent_eff_scale * frame.scale;
}

fn mark_default_parent(widgets: &mut WidgetRegistry, frame_id: u64) {
    // SetAllPoints() with no args uses this to decide whether to store nil
    // or the actual parent ID as relativeTo (matching wowless headless behavior).
    if let Some(frame) = widgets.get_mut_visual(frame_id) {
        frame.default_parent = true;
    }
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
