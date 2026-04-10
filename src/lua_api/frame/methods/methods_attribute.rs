//! Attribute methods: GetAttribute, SetAttribute, frame references, etc.

use super::super::handle::{FrameRef, frame_ref};
use super::combat_lockdown;
use crate::lua_api::frame::handle::get_sim_state;
use crate::lua_api::script_helpers::lua_error;
use crate::widget::AttributeValue;
use mlua::Value;

/// Add attribute methods to the shared methods table.
pub fn add_attribute_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_set_attribute_methods(methods);
    add_execute_attribute(methods);
    add_frame_ref_methods(methods);
    add_security_and_input_stubs(methods);
}

fn add_get_set_attribute_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetAttribute", |lua, this, args: mlua::MultiValue| {
        let keys = validate_and_build_keys(lua, &args)?;
        get_attribute_value(lua, this.0, &keys)
    });

    methods.add_method(
        "SetAttribute",
        |lua, this, (name, value): (String, Value)| {
            let id = this.0;
            set_attribute_value(lua, id, &name, &value)?;
            fire_on_attribute_changed(lua, id, &name, value)?;
            Ok(())
        },
    );

    methods.add_method(
        "SetAttributeNoHandler",
        |lua, this, (name, value): (String, Value)| {
            set_attribute_value(lua, this.0, &name, &value)?;
            Ok(())
        },
    );

    methods.add_method("ClearAttributes", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.attributes.clear();
        }
        Ok(())
    });
}

/// Check if a Lua value is "truthy" (not nil and not false).
fn is_truthy(v: &Value) -> bool {
    !matches!(v, Value::Nil | Value::Boolean(false))
}

/// Validate GetAttribute arguments and build lookup keys.
fn validate_and_build_keys(lua: &mlua::Lua, args: &mlua::MultiValue) -> mlua::Result<Vec<String>> {
    let arg1 = args.get(0).unwrap_or(&Value::Nil);
    let arg2 = args.get(1).unwrap_or(&Value::Nil);
    let vararg_count = args.len().saturating_sub(2);

    if is_truthy(arg1) && !is_truthy(arg2) {
        Ok(build_attribute_keys(args))
    } else if !is_truthy(arg2) || vararg_count == 0 {
        let taint = crate::lua_api::script_helpers::get_stack_taint(lua);
        let msg = format!(
            "Arguments: (\"name\"){}",
            taint
                .map(|t| format!("\nLua Taint: {t}"))
                .unwrap_or_default()
        );
        Err(lua_error(lua, msg))
    } else {
        Ok(build_attribute_keys(args))
    }
}

/// Build the list of attribute keys to try, in WoW's fallback order.
fn build_attribute_keys(args: &mlua::MultiValue) -> Vec<String> {
    let strings: Vec<String> = args
        .iter()
        .filter_map(|v| match v {
            Value::String(s) => s.to_str().ok().map(|s| s.to_string()),
            _ => None,
        })
        .collect();

    match strings.len() {
        0 => vec![String::new()],
        1 => vec![strings[0].clone()],
        _ => {
            let prefix = &strings[0];
            let name = &strings[1];
            let suffix = if strings.len() > 2 {
                strings[2].as_str()
            } else {
                ""
            };
            vec![
                format!("{}{}{}", prefix, name, suffix),
                format!("*{}{}", name, suffix),
                format!("{}{}*", prefix, name),
                format!("*{}*", name),
                name.clone(),
            ]
        }
    }
}

/// Look up an attribute, trying each key in order until one is found.
fn get_attribute_value(lua: &mlua::Lua, id: u64, keys: &[String]) -> mlua::Result<Value> {
    let table_attrs: Option<mlua::Table> = lua.globals().get("__frame_table_attributes").ok();
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let frame = state.widgets.get(id);

    for key in keys {
        if let Some(attrs) = &table_attrs {
            let lua_key = format!("{}_{}", id, key);
            let table_val: Value = attrs.get(lua_key.as_str()).unwrap_or(Value::Nil);
            if !matches!(table_val, Value::Nil) {
                return Ok(table_val);
            }
        }
        if let Some(f) = frame {
            if let Some(attr) = f.attributes.get(key.as_str()) {
                return attribute_to_value(lua, attr);
            }
        }
    }
    Ok(Value::Nil)
}

/// Convert an AttributeValue to a Lua Value.
fn attribute_to_value(lua: &mlua::Lua, attr: &AttributeValue) -> mlua::Result<Value> {
    match attr {
        AttributeValue::String(s) => Ok(Value::String(lua.create_string(s)?)),
        AttributeValue::Number(n) => Ok(Value::Number(*n)),
        AttributeValue::Boolean(b) => Ok(Value::Boolean(*b)),
        AttributeValue::Nil => Ok(Value::Nil),
    }
}

/// Store the attribute value in Lua (tables) or Rust (simple types).
fn set_attribute_value(lua: &mlua::Lua, id: u64, name: &str, value: &Value) -> mlua::Result<()> {
    if matches!(
        value,
        Value::Table(_) | Value::UserData(_) | Value::Function(_)
    ) {
        store_table_attribute(lua, id, name, value)?;
    } else {
        store_simple_attribute(lua, id, name, value)?;
    }
    Ok(())
}

/// Store a complex Lua value (table/userdata/function) in the Lua-side attribute table.
fn store_table_attribute(lua: &mlua::Lua, id: u64, name: &str, value: &Value) -> mlua::Result<()> {
    let table_attrs: mlua::Table = lua
        .globals()
        .get("__frame_table_attributes")
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            lua.globals()
                .set("__frame_table_attributes", t.clone())
                .ok();
            t
        });
    let key = format!("{}_{}", id, name);
    table_attrs.set(key, value.clone())?;
    Ok(())
}

/// Store a simple value (string/number/bool/nil) in the Rust-side attribute map.
fn store_simple_attribute(lua: &mlua::Lua, id: u64, name: &str, value: &Value) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut(id) {
        let attr = value_to_attribute(value);
        if matches!(attr, AttributeValue::Nil) && matches!(value, Value::Nil) {
            frame.attributes.remove(name);
            remove_from_table_attributes(lua, id, name);
        } else {
            frame.attributes.insert(name.to_string(), attr);
        }
    }
    Ok(())
}

fn value_to_attribute(value: &Value) -> AttributeValue {
    match value {
        Value::Nil => AttributeValue::Nil,
        Value::Boolean(b) => AttributeValue::Boolean(*b),
        Value::Integer(i) => AttributeValue::Number(*i as f64),
        Value::Number(n) => AttributeValue::Number(*n),
        Value::String(s) => {
            AttributeValue::String(s.to_str().map(|s| s.to_string()).unwrap_or_default())
        }
        _ => AttributeValue::Nil,
    }
}

fn remove_from_table_attributes(lua: &mlua::Lua, id: u64, name: &str) {
    if let Ok(table_attrs) = lua.globals().get::<mlua::Table>("__frame_table_attributes") {
        let key = format!("{}_{}", id, name);
        table_attrs.set(key, Value::Nil).ok();
    }
}

/// Fire OnAttributeChanged script handler if one exists.
fn fire_on_attribute_changed(
    lua: &mlua::Lua,
    id: u64,
    name: &str,
    value: Value,
) -> mlua::Result<()> {
    use crate::lua_api::script_helpers::{call_error_handler, get_script};
    if let Some(handler) = get_script(lua, id, "OnAttributeChanged") {
        let name_str = lua.create_string(name)?;
        if let Err(e) = handler.call::<()>((frame_ref(lua, id)?, name_str, value)) {
            call_error_handler(lua, &e.to_string());
        }
    }
    Ok(())
}

fn add_execute_attribute<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "ExecuteAttribute",
        |_lua, _this, _args: mlua::MultiValue| Ok(Value::Nil),
    );
}

fn add_frame_ref_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetFrameRef",
        |lua, this, (label, frame): (String, Value)| {
            let key = format!("frameref-{}", label);
            set_attribute_value(lua, this.0, &key, &frame)?;
            Ok(())
        },
    );
    methods.add_method("GetFrameRef", |lua, this, label: String| {
        let key = format!("frameref-{}", label);
        get_attribute_value(lua, this.0, &[key])
    });
}

fn add_security_and_input_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_security_stubs(methods);
    add_clip_children_methods(methods);
    add_hit_rect_methods(methods);
}

fn add_security_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_forbidden_methods(methods);
    add_security_capability_stubs(methods);
}

fn add_forbidden_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetForbidden", |lua, this, forbidden: Option<bool>| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "SetForbidden") {
                return Ok(());
            }
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(id) {
            frame.forbidden = forbidden.unwrap_or(true);
        }
        Ok(())
    });
    methods.add_method("IsForbidden", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.forbidden)
            .unwrap_or(false))
    });
}

fn add_security_capability_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CanChangeProtectedState", |_lua, _this, ()| Ok(true));
    methods.add_method(
        "SetPassThroughButtons",
        |lua, this, args: mlua::MultiValue| {
            let mut buttons = std::collections::HashSet::new();
            for value in args {
                if let mlua::Value::String(button) = value {
                    buttons.insert(button.to_str()?.to_ascii_lowercase());
                }
            }

            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut(this.0) {
                frame.pass_through_buttons = buttons;
            }
            Ok(())
        },
    );
    methods.add_method(
        "SetFlattensRenderLayers",
        |lua, this, flatten: Option<bool>| {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut(this.0) {
                frame.flattens_render_layers = flatten.unwrap_or(false);
            }
            Ok(())
        },
    );
    methods.add_method(
        "SetMotionScriptsWhileDisabled",
        |_lua, _this, _enabled: Option<bool>| Ok(()),
    );
    methods.add_method("GetMotionScriptsWhileDisabled", |_lua, _this, ()| Ok(false));
}

fn add_clip_children_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetClipsChildren", |lua, this, clips: Option<bool>| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.clips_children = clips.unwrap_or(false);
        }
        Ok(())
    });

    methods.add_method("DoesClipChildren", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.clips_children)
            .unwrap_or(false))
    });
}

fn add_hit_rect_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHitRectInsets", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            if combat_lockdown::check_and_fire(lua, &state_rc, id, "SetHitRectInsets") {
                return Ok(());
            }
        }
        let (l, r, t, b) = parse_hit_rect_insets(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(id) {
            frame.hit_rect_insets = (l, r, t, b);
        }
        Ok(())
    });

    methods.add_method("GetHitRectInsets", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            let (l, r, t, b) = frame.hit_rect_insets;
            return Ok((l as f64, r as f64, t as f64, b as f64));
        }
        Ok((0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64))
    });
}

/// Parse 4 numeric values from a MultiValue for hit rect insets.
fn parse_hit_rect_insets(args: mlua::MultiValue) -> (f32, f32, f32, f32) {
    let mut it = args.into_iter();
    let l = next_f32(&mut it);
    let r = next_f32(&mut it);
    let t = next_f32(&mut it);
    let b = next_f32(&mut it);
    (l, r, t, b)
}

fn next_f32(it: &mut impl Iterator<Item = Value>) -> f32 {
    match it.next() {
        Some(Value::Number(n)) => n as f32,
        Some(Value::Integer(n)) => n as f32,
        _ => 0.0,
    }
}
