//! Miscellaneous widget methods: ColorSelect, drag/move/resize, SimpleHTML, and stubs.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;
use crate::widget::AttributeValue;
use mlua::Value;

pub fn add_colorselect_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_colorselect_rgb_methods(methods);
    add_colorselect_hsv_methods(methods);
    add_colorselect_alpha_texture_stubs(methods);
}

pub fn add_drag_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_drag_move_methods(methods);
    add_drag_movable_resizable_methods(methods);
    add_drag_clamp_methods(methods);
    add_drag_resize_methods(methods);
}

pub fn add_simplehtml_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_simplehtml_hyperlink_methods(methods);
    add_simplehtml_content_methods(methods);
    methods.add_method("GetIndentedWordWrap", |_, _, ()| Ok(false));
}

pub fn add_misc_widget_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_misc_stubs_simple(methods);
    add_misc_stubs_mixin(methods);
}

// --- SimpleHTML ---

fn add_simplehtml_hyperlink_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHyperlinkFormat", |lua, this, format: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(data) = state.simple_htmls.get_mut(&this.0) {
            data.hyperlink_format = format;
        }
        Ok(())
    });

    methods.add_method("GetHyperlinkFormat", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let format = state.simple_htmls.get(&this.0)
            .map(|d| d.hyperlink_format.clone())
            .unwrap_or_else(|| "|H%s|h%s|h".to_string());
        Ok(format)
    });

    methods.add_method("SetHyperlinksEnabled", |lua, this, enabled: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(data) = state.simple_htmls.get_mut(&this.0) {
            data.hyperlinks_enabled = enabled;
        }
        Ok(())
    });

    methods.add_method("GetHyperlinksEnabled", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let enabled = state.simple_htmls.get(&this.0)
            .map(|d| d.hyperlinks_enabled)
            .unwrap_or(true);
        Ok(enabled)
    });
}

fn add_simplehtml_content_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetContentHeight", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = match state.widgets.get(this.0) {
            Some(f) => f,
            None => return Ok(0.0_f64),
        };
        let text = match &frame.text {
            Some(t) if !t.is_empty() => t,
            _ => return Ok(0.0_f64),
        };
        let font_size = frame.font_size.max(12.0) as f64;
        let line_height = font_size * 1.2;
        let width = frame.width.max(200.0) as f64;
        let chars_per_line = (width / (font_size * 0.6)).max(1.0);
        let estimated_lines = (text.len() as f64 / chars_per_line).ceil().max(1.0);
        Ok(estimated_lines * line_height)
    });

    methods.add_method("GetTextData", |lua, _this, ()| {
        Ok(Value::Table(lua.create_table()?))
    });
}

// --- ColorSelect ---

fn add_colorselect_rgb_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetColorRGB", |lua, this, (r, g, b): (f64, f64, f64)| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.attributes.insert("colorR".to_string(), AttributeValue::Number(r));
            frame.attributes.insert("colorG".to_string(), AttributeValue::Number(g));
            frame.attributes.insert("colorB".to_string(), AttributeValue::Number(b));
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
            frame.attributes.insert("colorR".to_string(), AttributeValue::Number(r));
            frame.attributes.insert("colorG".to_string(), AttributeValue::Number(g));
            frame.attributes.insert("colorB".to_string(), AttributeValue::Number(b));
            frame.attributes.insert("colorH".to_string(), AttributeValue::Number(h % 360.0));
            frame.attributes.insert("colorS".to_string(), AttributeValue::Number(s));
            frame.attributes.insert("colorV".to_string(), AttributeValue::Number(v));
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

fn get_color_hsv_from_attrs(attrs: &std::collections::HashMap<String, AttributeValue>) -> (f64, f64, f64) {
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
    methods.add_method("ClearColorWheelTexture", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("GetColorAlpha", |_, _, ()| Ok(0.0_f64));
    methods.add_method("GetColorAlphaTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorAlphaThumbTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorValueTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorValueThumbTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorWheelTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("GetColorWheelThumbTexture", |_, _, ()| Ok(Value::Nil));
    methods.add_method("SetColorAlpha", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetColorAlphaTexture", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetColorAlphaThumbTexture", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetColorValueTexture", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetColorValueThumbTexture", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetColorWheelTexture", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetColorWheelThumbTexture", |_, _, _: mlua::Variadic<Value>| Ok(()));
}

// --- Drag/Move/Resize ---

fn add_drag_move_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("StartMoving", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0)
                && frame.movable {
                    frame.is_moving = true;
                }
        Ok(())
    });
    methods.add_method("StopMovingOrSizing", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0) {
                frame.is_moving = false;
            }
        Ok(())
    });
    methods.add_method("SetMovable", |lua, this, movable: bool| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0) {
                frame.movable = movable;
            }
        Ok(())
    });
    methods.add_method("IsMovable", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0) {
                return Ok(frame.movable);
            }
        Ok(false)
    });
}

fn add_drag_movable_resizable_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetResizable", |lua, this, resizable: bool| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0) {
                frame.resizable = resizable;
            }
        Ok(())
    });
    methods.add_method("IsResizable", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0) {
                return Ok(frame.resizable);
            }
        Ok(false)
    });
}

fn add_drag_clamp_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetClampedToScreen", |lua, this, clamped: bool| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0) {
                frame.clamped_to_screen = clamped;
            }
        Ok(())
    });
    methods.add_method("IsClampedToScreen", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0) {
                return Ok(frame.clamped_to_screen);
            }
        Ok(false)
    });
    methods.add_method("SetClampRectInsets", |_, _this, _args: mlua::MultiValue| Ok(()));
}

fn add_drag_resize_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetResizeBounds", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetResizeBounds", |_, _this, ()| Ok((0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32)));
    methods.add_method("SetMinResize", |_, _this, (_w, _h): (f32, f32)| Ok(()));
    methods.add_method("SetMaxResize", |_, _this, (_w, _h): (f32, f32)| Ok(()));
    methods.add_method("StartSizing", |_, _this, _point: Option<String>| Ok(()));
    methods.add_method("RegisterForDrag", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetUserPlaced", |_, _this, _user_placed: bool| Ok(()));
    methods.add_method("IsUserPlaced", |_, _this, ()| Ok(false));
    methods.add_method("SetDontSavePosition", |_, _this, _dont_save: bool| Ok(()));
}

// --- Misc stubs ---

fn add_misc_stubs_simple<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetupMenu", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetAlertContainer", |_, _this, _container: Value| Ok(()));
    methods.add_method("SetColorFill", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetTextToFit", |lua, this, text: Option<String>| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) { frame.text = text; }
        Ok(())
    });
    methods.add_method("SetSelectionTranslator", |_, _this, _func: Value| Ok(()));
    methods.add_method("SetItemButtonScale", |_, _this, _scale: Value| Ok(()));
    methods.add_method("UpdateItemContextMatching", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("UpdateHeight", |_, _this, ()| Ok(()));
    methods.add_method("SetDefaultText", |_, _this, _text: Value| Ok(()));
    methods.add_method("SetVisuals", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("RegisterForWidgetSet", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("UnregisterForWidgetSet", |_, _this, _args: mlua::MultiValue| Ok(()));
}

fn add_misc_stubs_mixin<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetRotationIncrement", |lua, this, inc: Value| {
        let id = this.0;
        if let Some((func, frame_ud)) = super::methods_helpers::get_mixin_override(lua, id, "SetRotationIncrement") {
            return func.call::<()>((frame_ud, inc));
        }
        Ok(())
    });
}

// --- Color conversion helpers ---

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let h = h % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = if h < 60.0 { (c, x, 0.0)
    } else if h < 120.0 { (x, c, 0.0)
    } else if h < 180.0 { (0.0, c, x)
    } else if h < 240.0 { (0.0, x, c)
    } else if h < 300.0 { (x, 0.0, c)
    } else { (c, 0.0, x) };
    (r1 + m, g1 + m, b1 + m)
}

fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let v = max;
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let h = if delta == 0.0 { 0.0
    } else if max == r { 60.0 * (((g - b) / delta) % 6.0)
    } else if max == g { 60.0 * ((b - r) / delta + 2.0)
    } else { 60.0 * ((r - g) / delta + 4.0) };
    let h = if h < 0.0 { h + 360.0 } else { h };
    (h, s, v)
}
