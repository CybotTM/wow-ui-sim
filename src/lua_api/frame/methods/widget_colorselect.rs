//! ColorSelect widget methods.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;
use crate::widget::AttributeValue;
use mlua::Value;

pub fn add_colorselect_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_colorselect_rgb_methods(methods);
    add_colorselect_hsv_methods(methods);
    add_colorselect_alpha_texture_stubs(methods);
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

fn add_colorselect_alpha_texture_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "ClearColorWheelTexture",
        |_, _, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method("GetColorAlpha", |_, _, ()| Ok(0.0_f64));
    methods.add_method("GetColorAlphaTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorAlphaThumbTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorValueTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorValueThumbTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorWheelTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorWheelThumbTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("SetColorAlpha", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetColorAlphaTexture", |_, _, _: mlua::Variadic<Value>| {
        Ok(())
    });
    methods.add_method(
        "SetColorAlphaThumbTexture",
        |_, _, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method("SetColorValueTexture", |_, _, _: mlua::Variadic<Value>| {
        Ok(())
    });
    methods.add_method(
        "SetColorValueThumbTexture",
        |_, _, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method("SetColorWheelTexture", |_, _, _: mlua::Variadic<Value>| {
        Ok(())
    });
    methods.add_method(
        "SetColorWheelThumbTexture",
        |_, _, _: mlua::Variadic<Value>| Ok(()),
    );
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
