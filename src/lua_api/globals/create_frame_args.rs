//! Argument parsing for the CreateFrame Lua function.

use super::super::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Parsed CreateFrame arguments.
pub(super) struct CreateFrameArgs {
    pub frame_type: String,
    pub name: Option<String>,
    pub parent_id: Option<u64>,
    pub template: Option<String>,
    pub id: Option<i32>,
    /// Whether the parent was explicitly provided (vs defaulting to UIParent).
    pub parent_explicit: bool,
}

/// Parse the arguments to CreateFrame: (frameType, name, parent, template, id).
pub(super) fn parse_create_frame_args(
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

fn extract_frame_id_or_proxy(value: &Value) -> Option<u64> {
    super::super::frame::extract_frame_id(value)
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

/// Replace $parent/$Parent placeholders in a frame name with the actual parent name.
fn substitute_parent_name(
    name: String,
    parent_id: Option<u64>,
    state: &Rc<RefCell<SimState>>,
) -> String {
    apply_parent_sub(&name, parent_id, &state.borrow())
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
