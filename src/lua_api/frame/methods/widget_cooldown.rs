//! Cooldown widget methods: SetCooldown, swipe/edge/bling display, pause/resume.

use super::super::handle::FrameRef;
use super::widget_tooltip::val_to_f32;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state, sync_child_to_lua};
use crate::widget::{AttributeValue, Color, Frame, WidgetType};
use mlua::ObjectLike;
use mlua::Value;

pub fn add_cooldown_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_cooldown_set_methods(methods);
    add_cooldown_get_methods(methods);
    add_cooldown_display_methods(methods);
    add_cooldown_bool_display_methods(methods);
    add_cooldown_texture_methods(methods);
    add_cooldown_state_methods(methods);
    add_cooldown_remaining_stubs(methods);
}

fn add_cooldown_remaining_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetUseAuraDisplayTime", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_use_aura_display_time)
            .unwrap_or(false))
    });
}

fn parse_f64_arg(val: Option<Value>) -> f64 {
    match val {
        Some(Value::Number(n)) => n,
        Some(Value::Integer(n)) => n as f64,
        _ => 0.0,
    }
}

fn normalize_mod_rate(mod_rate: f64) -> f64 {
    if mod_rate <= 0.0 { 1.0 } else { mod_rate }
}

fn set_cooldown_state(frame: &mut Frame, start: f64, duration: f64, mod_rate: f64) {
    frame.cooldown_start = start;
    frame.cooldown_duration = duration;
    frame.cooldown_display_duration_ms = duration.max(0.0) * 1000.0;
    frame.cooldown_mod_rate = normalize_mod_rate(mod_rate);
}

fn clear_cooldown_timing(frame: &mut Frame) {
    frame.cooldown_start = 0.0;
    frame.cooldown_duration = 0.0;
    frame.cooldown_display_duration_ms = 0.0;
    frame.cooldown_mod_rate = 1.0;
}

fn parse_optional_texture_path(value: Option<Value>) -> Option<String> {
    match value {
        Some(Value::String(path)) => Some(path.to_string_lossy().to_string()),
        _ => None,
    }
}

fn parse_vector2(value: &Value) -> Option<(f32, f32)> {
    match value {
        Value::Table(table) => Some((table.get::<f32>("x").ok()?, table.get::<f32>("y").ok()?)),
        Value::UserData(ud) => Some((ud.get::<f32>("x").ok()?, ud.get::<f32>("y").ok()?)),
        _ => None,
    }
}

fn duration_method_number(value: &Value, method: &str) -> Option<f64> {
    match value {
        Value::Table(table) => table.call_method::<f64>(method, ()).ok(),
        Value::UserData(ud) => {
            let method_fn: mlua::Function = ud.get(method).ok()?;
            method_fn.call::<f64>(ud.clone()).ok()
        }
        _ => None,
    }
}

fn duration_method_bool(value: &Value, method: &str) -> Option<bool> {
    match value {
        Value::Table(table) => table.call_method::<bool>(method, ()).ok(),
        Value::UserData(ud) => {
            let method_fn: mlua::Function = ud.get(method).ok()?;
            method_fn.call::<bool>(ud.clone()).ok()
        }
        _ => None,
    }
}

fn ensure_countdown_font_string(
    lua: &mlua::Lua,
    cooldown_id: u64,
    requested_font: Option<&str>,
) -> mlua::Result<u64> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();

    if let Some(existing_id) = state
        .widgets
        .get(cooldown_id)
        .and_then(|frame| frame.cooldown_countdown_font_string_id)
        && state
            .widgets
            .get(existing_id)
            .is_some_and(|child| child.parent_id == Some(cooldown_id))
    {
        if let Some(font_name) = requested_font
            && let Some(child) = state.widgets.get_mut_visual(existing_id)
        {
            child.font = Some(font_name.to_string());
        }
        return Ok(existing_id);
    }

    let mut countdown = Frame::new(WidgetType::FontString, None, Some(cooldown_id));
    super::methods_helpers::set_all_points_anchors_pub(&mut countdown, cooldown_id);
    countdown.draw_layer = crate::widget::DrawLayer::Overlay;
    countdown.parent_key = Some("Countdown".to_string());
    if let Some(font_name) = requested_font {
        countdown.font = Some(font_name.to_string());
    }
    if let Some(parent) = state.widgets.get(cooldown_id) {
        countdown.frame_strata = parent.frame_strata;
        countdown.frame_level = parent.frame_level + 1;
        countdown.layout_rect = parent.layout_rect;
    }

    let child_id = countdown.id;
    state.widgets.register(countdown);
    state.widgets.add_child(cooldown_id, child_id);
    if let Some(frame) = state.widgets.get_mut_visual(cooldown_id) {
        frame
            .children_keys
            .insert("Countdown".to_string(), child_id);
        frame.cooldown_countdown_font_string_id = Some(child_id);
    }
    state.widgets.mark_rect_dirty(cooldown_id);
    drop(state);

    let _ = sync_child_to_lua(lua, cooldown_id, "Countdown", child_id);
    Ok(child_id)
}

fn apply_countdown_font_name(
    lua: &mlua::Lua,
    countdown: &mut Frame,
    font_name: &str,
) -> mlua::Result<()> {
    let globals = lua.globals();
    match globals.get::<Value>(font_name)? {
        Value::Table(tbl) => {
            if let Ok(path) = tbl.get::<String>("__font") {
                countdown.font = Some(path);
            } else {
                countdown.font = Some(font_name.to_string());
            }
            if let Ok(height) = tbl.get::<f64>("__height") {
                countdown.font_size = height as f32;
            }
            if let Ok(outline) = tbl.get::<String>("__outline") {
                countdown.font_outline = crate::widget::TextOutline::from_wow_str(&outline);
            }
        }
        _ => countdown.font = Some(font_name.to_string()),
    }
    Ok(())
}

fn add_cooldown_set_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCooldown", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let start = parse_f64_arg(it.next());
        let duration = parse_f64_arg(it.next());
        let mod_rate = normalize_mod_rate(parse_f64_arg(it.next()));
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            set_cooldown_state(frame, start, duration, mod_rate);
        }
        Ok(())
    });

    methods.add_method("SetCooldownUNIX", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let start = parse_f64_arg(it.next());
        let duration = parse_f64_arg(it.next());
        let mod_rate = normalize_mod_rate(parse_f64_arg(it.next()));
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            set_cooldown_state(frame, start, duration, mod_rate);
        }
        Ok(())
    });

    methods.add_method(
        "SetCooldownFromExpirationTime",
        |lua, this, args: mlua::MultiValue| {
            let mut it = args.into_iter();
            let expiration_time = parse_f64_arg(it.next());
            let duration = parse_f64_arg(it.next());
            let mod_rate = normalize_mod_rate(parse_f64_arg(it.next()));
            let start = expiration_time - duration;
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                set_cooldown_state(frame, start, duration, mod_rate);
            }
            Ok(())
        },
    );

    methods.add_method(
        "SetCooldownFromDurationObject",
        |lua, this, args: mlua::MultiValue| {
            let mut it = args.into_iter();
            let duration_value = it.next().unwrap_or(Value::Nil);
            let clear_if_zero = match it.next() {
                Some(Value::Boolean(flag)) => flag,
                _ => true,
            };

            let is_zero = duration_method_bool(&duration_value, "IsZero").unwrap_or(false);
            let start = duration_method_number(&duration_value, "GetStartTime").unwrap_or(0.0);
            let duration =
                duration_method_number(&duration_value, "GetTotalDuration").unwrap_or(0.0);
            let mod_rate = normalize_mod_rate(
                duration_method_number(&duration_value, "GetModRate").unwrap_or(1.0),
            );

            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                if clear_if_zero && (is_zero || duration == 0.0) {
                    clear_cooldown_timing(frame);
                } else {
                    set_cooldown_state(frame, start, duration, mod_rate);
                }
            }
            Ok(())
        },
    );
}

fn add_cooldown_get_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCooldownTimes", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok((frame.cooldown_start, frame.cooldown_duration));
        }
        Ok((0.0_f64, 0.0_f64))
    });

    methods.add_method("GetCooldownDuration", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_duration)
            .unwrap_or(0.0))
    });

    methods.add_method("GetCooldownDisplayDuration", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_display_duration_ms)
            .unwrap_or(0.0))
    });

    methods.add_method("GetCountdownFontString", |lua, this, ()| {
        let countdown_id = ensure_countdown_font_string(lua, this.0, None)?;
        frame_ref(lua, countdown_id)
    });

    methods.add_method("GetDrawBling", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_draw_bling)
            .unwrap_or(true))
    });

    methods.add_method("GetDrawEdge", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_draw_edge)
            .unwrap_or(false))
    });

    methods.add_method("GetDrawSwipe", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_draw_swipe)
            .unwrap_or(true))
    });

    methods.add_method("GetEdgeScale", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_edge_scale)
            .unwrap_or(1.0))
    });

    methods.add_method("GetHideCountdownNumbers", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_hide_countdown)
            .unwrap_or(false))
    });

    methods.add_method("GetMinimumCountdownDuration", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_min_countdown_duration_ms)
            .unwrap_or(0.0))
    });
}

fn add_cooldown_display_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSwipeColor", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let r = val_to_f32(it.next(), 0.0);
        let g = val_to_f32(it.next(), 0.0);
        let b = val_to_f32(it.next(), 0.0);
        let a = val_to_f32(it.next(), 0.8);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.attributes.insert(
                "__swipe_color".to_string(),
                AttributeValue::String(format!("{},{},{},{}", r, g, b, a)),
            );
        }
        Ok(())
    });

    methods.add_method("SetHideCountdownNumbers", |lua, this, hide: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_hide_countdown = hide;
        }
        Ok(())
    });

    methods.add_method("SetEdgeColor", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let color = Color::new(
            val_to_f32(it.next(), 1.0),
            val_to_f32(it.next(), 1.0),
            val_to_f32(it.next(), 1.0),
            val_to_f32(it.next(), 1.0),
        );
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_edge_color = color;
        }
        Ok(())
    });

    methods.add_method(
        "SetMinimumCountdownDuration",
        |lua, this, milliseconds: Value| {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                frame.cooldown_min_countdown_duration_ms = parse_f64_arg(Some(milliseconds));
            }
            Ok(())
        },
    );

    methods.add_method("SetTexCoordRange", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let low = it.next().unwrap_or(Value::Nil);
        let high = it.next().unwrap_or(Value::Nil);
        let Some((low_x, low_y)) = parse_vector2(&low) else {
            return Ok(());
        };
        let Some((high_x, high_y)) = parse_vector2(&high) else {
            return Ok(());
        };

        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_tex_coord_range = Some((low_x, low_y, high_x, high_y));
        }
        Ok(())
    });
}

fn add_cooldown_bool_display_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDrawSwipe", |lua, this, draw: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_draw_swipe = draw;
        }
        Ok(())
    });
    methods.add_method("SetDrawEdge", |lua, this, draw: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_draw_edge = draw;
        }
        Ok(())
    });
    methods.add_method("SetDrawBling", |lua, this, draw: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_draw_bling = draw;
        }
        Ok(())
    });
    methods.add_method("SetReverse", |lua, this, reverse: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_reverse = reverse;
        }
        Ok(())
    });
}

fn add_cooldown_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_reverse(methods);
    add_set_cooldown_duration(methods);
    add_set_edge_scale(methods);
    add_set_swipe_texture(methods);
    add_set_edge_texture(methods);
    add_set_bling_texture(methods);
    add_set_use_circular_edge(methods);
    add_set_countdown_abbrev_threshold(methods);
    add_set_countdown_font(methods);
    add_set_use_aura_display_time(methods);
}

fn add_get_reverse<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetReverse", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_reverse)
            .unwrap_or(false))
    });
}

fn add_set_cooldown_duration<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetCooldownDuration",
        |lua, this, args: mlua::MultiValue| {
            let mut it = args.into_iter();
            let duration = parse_f64_arg(it.next());
            let mod_rate = normalize_mod_rate(parse_f64_arg(it.next()));
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                set_cooldown_state(frame, frame.cooldown_start, duration, mod_rate);
            }
            Ok(())
        },
    );
}

fn add_set_edge_scale<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetEdgeScale", |lua, this, scale: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_edge_scale = parse_f64_arg(Some(scale));
        }
        Ok(())
    });
}

fn add_set_swipe_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSwipeTexture", |lua, this, args: mlua::MultiValue| {
        let path = parse_optional_texture_path(args.into_iter().next());
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_swipe_texture = path;
        }
        Ok(())
    });
}

fn add_set_edge_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetEdgeTexture", |lua, this, args: mlua::MultiValue| {
        let mut it = args.into_iter();
        let path = parse_optional_texture_path(it.next());
        let color = match (&path, it.next(), it.next(), it.next(), it.next()) {
            (_, Some(r), Some(g), Some(b), Some(a)) => Some(Color::new(
                val_to_f32(Some(r), 1.0),
                val_to_f32(Some(g), 1.0),
                val_to_f32(Some(b), 1.0),
                val_to_f32(Some(a), 1.0),
            )),
            _ => None,
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_edge_texture = path;
            if let Some(color) = color {
                frame.cooldown_edge_color = color;
            }
        }
        Ok(())
    });
}

fn add_set_bling_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetBlingTexture", |lua, this, args: mlua::MultiValue| {
        let path = parse_optional_texture_path(args.into_iter().next());
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_bling_texture = path;
        }
        Ok(())
    });
}

fn add_set_use_circular_edge<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetUseCircularEdge", |lua, this, enabled: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_use_circular_edge = enabled;
        }
        Ok(())
    });
}

fn add_set_countdown_abbrev_threshold<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetCountdownAbbrevThreshold",
        |lua, this, threshold: Value| {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                frame.cooldown_countdown_abbrev_threshold_seconds = parse_f64_arg(Some(threshold));
            }
            Ok(())
        },
    );
}

fn add_set_countdown_font<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCountdownFont", |lua, this, font: Value| {
        let Value::String(font) = font else {
            return Ok(());
        };
        let font_name = font.to_string_lossy().to_string();
        let countdown_id = ensure_countdown_font_string(lua, this.0, None)?;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(countdown) = state.widgets.get_mut_visual(countdown_id) {
            apply_countdown_font_name(lua, countdown, &font_name)?;
        }
        Ok(())
    });
}

fn add_set_use_aura_display_time<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetUseAuraDisplayTime", |lua, this, enabled: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_use_aura_display_time = match enabled {
                Value::Boolean(flag) => flag,
                other => parse_f64_arg(Some(other)) != 0.0,
            };
        }
        Ok(())
    });
}

fn add_cooldown_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Clear", |lua, this, _: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            clear_cooldown_timing(frame);
        }
        Ok(())
    });

    methods.add_method("Pause", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_paused = true;
        }
        Ok(())
    });

    methods.add_method("Resume", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.cooldown_paused = false;
        }
        Ok(())
    });

    methods.add_method("IsPaused", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.cooldown_paused)
            .unwrap_or(false))
    });
}
