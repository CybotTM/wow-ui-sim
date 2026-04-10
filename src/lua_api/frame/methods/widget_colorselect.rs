//! ColorSelect widget methods.

use super::super::handle::FrameRef;
use super::widget_slider::get_or_create_child_texture;
use crate::lua_api::frame::handle::get_sim_state;
use crate::widget::AttributeValue;
use mlua::{Lua, Table, Value};

pub fn add_colorselect_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_colorselect_rgb_methods(methods);
    add_colorselect_hsv_methods(methods);
    add_colorselect_texture_methods(methods);
    add_colorselect_alpha_stubs(methods);
}

fn add_colorselect_rgb_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetColorRGB", |lua, this, (r, g, b): (f64, f64, f64)| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame
                .attributes
                .insert("colorR".to_string(), AttributeValue::Number(r));
            frame
                .attributes
                .insert("colorG".to_string(), AttributeValue::Number(g));
            frame
                .attributes
                .insert("colorB".to_string(), AttributeValue::Number(b));
        }
        Ok(())
    });

    methods.add_method("GetColorRGB", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            let r = get_attr_num(&frame.attributes, "colorR");
            let g = get_attr_num(&frame.attributes, "colorG");
            let b = get_attr_num(&frame.attributes, "colorB");
            return Ok((r, g, b));
        }
        Ok((1.0, 1.0, 1.0))
    });
}

fn get_attr_num(attrs: &std::collections::HashMap<String, AttributeValue>, key: &str) -> f64 {
    match attrs.get(key) {
        Some(AttributeValue::Number(n)) => *n,
        _ => 1.0,
    }
}

fn add_colorselect_hsv_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetColorHSV", |lua, this, (h, s, v): (f64, f64, f64)| {
        let (r, g, b) = hsv_to_rgb(h, s, v);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            set_color_rgb_attrs(frame, r, g, b);
            set_color_hsv_attrs(frame, h, s, v);
        }
        Ok(())
    });

    methods.add_method("GetColorHSV", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok(get_color_hsv_from_attrs(&frame.attributes));
        }
        Ok((0.0, 0.0, 1.0))
    });
}

fn set_color_rgb_attrs(frame: &mut crate::widget::Frame, r: f64, g: f64, b: f64) {
    frame
        .attributes
        .insert("colorR".to_string(), AttributeValue::Number(r));
    frame
        .attributes
        .insert("colorG".to_string(), AttributeValue::Number(g));
    frame
        .attributes
        .insert("colorB".to_string(), AttributeValue::Number(b));
}

fn set_color_hsv_attrs(frame: &mut crate::widget::Frame, h: f64, s: f64, v: f64) {
    frame
        .attributes
        .insert("colorH".to_string(), AttributeValue::Number(h % 360.0));
    frame
        .attributes
        .insert("colorS".to_string(), AttributeValue::Number(s));
    frame
        .attributes
        .insert("colorV".to_string(), AttributeValue::Number(v));
}

fn get_color_hsv_from_attrs(
    attrs: &std::collections::HashMap<String, AttributeValue>,
) -> (f64, f64, f64) {
    let get = |key: &str| match attrs.get(key) {
        Some(AttributeValue::Number(n)) => Some(*n),
        _ => None,
    };
    if let (Some(h), Some(s), Some(v)) = (get("colorH"), get("colorS"), get("colorV")) {
        return (h, s, v);
    }
    let r = get("colorR").unwrap_or(1.0);
    let g = get("colorG").unwrap_or(1.0);
    let b = get("colorB").unwrap_or(1.0);
    rgb_to_hsv(r, g, b)
}

const COLOR_ALPHA_TEXTURE_KEY: &str = "ColorAlphaTexture";
const COLOR_ALPHA_THUMB_TEXTURE_KEY: &str = "ColorAlphaThumbTexture";
const COLOR_VALUE_TEXTURE_KEY: &str = "ColorValueTexture";
const COLOR_VALUE_THUMB_TEXTURE_KEY: &str = "ColorValueThumbTexture";
const COLOR_WHEEL_TEXTURE_KEY: &str = "ColorWheelTexture";
const COLOR_WHEEL_THUMB_TEXTURE_KEY: &str = "ColorWheelThumbTexture";

fn add_colorselect_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "ClearColorWheelTexture",
        |lua, this, _: mlua::Variadic<Value>| {
            clear_colorselect_texture_slot(lua, this.0, COLOR_WHEEL_TEXTURE_KEY)
        },
    );

    for (method_name, slot_key) in [
        ("GetColorAlphaTexture", COLOR_ALPHA_TEXTURE_KEY),
        ("GetColorAlphaThumbTexture", COLOR_ALPHA_THUMB_TEXTURE_KEY),
        ("GetColorValueTexture", COLOR_VALUE_TEXTURE_KEY),
        ("GetColorValueThumbTexture", COLOR_VALUE_THUMB_TEXTURE_KEY),
        ("GetColorWheelTexture", COLOR_WHEEL_TEXTURE_KEY),
        ("GetColorWheelThumbTexture", COLOR_WHEEL_THUMB_TEXTURE_KEY),
    ] {
        methods.add_method(method_name, move |lua, this, ()| {
            get_colorselect_texture_slot(lua, this.0, slot_key)
        });
    }

    for (method_name, slot_key) in [
        ("SetColorAlphaTexture", COLOR_ALPHA_TEXTURE_KEY),
        ("SetColorAlphaThumbTexture", COLOR_ALPHA_THUMB_TEXTURE_KEY),
        ("SetColorValueTexture", COLOR_VALUE_TEXTURE_KEY),
        ("SetColorValueThumbTexture", COLOR_VALUE_THUMB_TEXTURE_KEY),
        ("SetColorWheelTexture", COLOR_WHEEL_TEXTURE_KEY),
        ("SetColorWheelThumbTexture", COLOR_WHEEL_THUMB_TEXTURE_KEY),
    ] {
        methods.add_method(
            method_name,
            move |lua, this, args: mlua::Variadic<Value>| {
                let texture = args.first().cloned().unwrap_or(Value::Nil);
                set_colorselect_texture_slot(lua, this.0, slot_key, texture)
            },
        );
    }
}

fn add_colorselect_alpha_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetColorAlpha", |_, _, ()| Ok(0.0_f64));
    methods.add_method("SetColorAlpha", |_, _, _: mlua::Variadic<Value>| Ok(()));
}

fn set_colorselect_texture_slot(
    lua: &Lua,
    frame_id: u64,
    slot_key: &str,
    texture: Value,
) -> mlua::Result<()> {
    match texture {
        Value::UserData(_) => store_colorselect_texture_slot(lua, frame_id, slot_key, texture),
        Value::Integer(_) | Value::Number(_) | Value::String(_) => {
            let existing_texture = get_colorselect_texture_slot(lua, frame_id, slot_key)?;
            let texture_ud = match existing_texture {
                Value::UserData(_) => existing_texture,
                _ => {
                    let child_texture = get_or_create_child_texture(lua, frame_id, slot_key)?;
                    store_colorselect_texture_slot(lua, frame_id, slot_key, child_texture.clone())?;
                    child_texture
                }
            };
            set_texture_value(lua, &texture_ud, &texture)
        }
        Value::Nil => clear_colorselect_texture_slot(lua, frame_id, slot_key),
        _ => Ok(()),
    }
}

fn get_colorselect_texture_slot(lua: &Lua, frame_id: u64, slot_key: &str) -> mlua::Result<Value> {
    let store = get_colorselect_texture_store(lua)?;
    match store.get::<Value>(frame_id)? {
        Value::Table(frame_store) => frame_store.get(slot_key),
        _ => Ok(Value::Nil),
    }
}

fn store_colorselect_texture_slot(
    lua: &Lua,
    frame_id: u64,
    slot_key: &str,
    texture: Value,
) -> mlua::Result<()> {
    let frame_store = get_or_create_colorselect_frame_store(lua, frame_id)?;
    frame_store.set(slot_key, texture)
}

fn clear_colorselect_texture_slot(lua: &Lua, frame_id: u64, slot_key: &str) -> mlua::Result<()> {
    let existing_texture = get_colorselect_texture_slot(lua, frame_id, slot_key)?;
    if matches!(existing_texture, Value::UserData(_)) {
        set_texture_value(lua, &existing_texture, &Value::Nil)?;
    }

    let store = get_colorselect_texture_store(lua)?;
    if let Value::Table(frame_store) = store.get::<Value>(frame_id)? {
        frame_store.set(slot_key, Value::Nil)?;
    }
    Ok(())
}

fn get_colorselect_texture_store(lua: &Lua) -> mlua::Result<Table> {
    lua.load(
        "_G.__colorselect_textures = _G.__colorselect_textures or {}; return _G.__colorselect_textures",
    )
    .eval()
}

fn get_or_create_colorselect_frame_store(lua: &Lua, frame_id: u64) -> mlua::Result<Table> {
    let store = get_colorselect_texture_store(lua)?;
    match store.get::<Value>(frame_id)? {
        Value::Table(frame_store) => Ok(frame_store),
        _ => {
            let frame_store = lua.create_table()?;
            store.set(frame_id, frame_store.clone())?;
            Ok(frame_store)
        }
    }
}

fn set_texture_value(lua: &Lua, texture_ud: &Value, texture: &Value) -> mlua::Result<()> {
    lua.load("local region, value = ...; region:SetTexture(value)")
        .call::<()>((texture_ud.clone(), texture.clone()))
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let h = h % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r1 + m, g1 + m, b1 + m)
}

fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let v = max;
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    let h = if h < 0.0 { h + 360.0 } else { h };
    (h, s, v)
}
