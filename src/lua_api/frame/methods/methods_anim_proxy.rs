//! Animation proxy methods on FrameRef.
//!
//! When an AnimationGroup or Animation is represented as a FrameRef,
//! these methods delegate to the underlying AnimGroupState/AnimState
//! via the `anim_frame_to_group` and `anim_frame_to_anim` mappings.

use super::super::handle::FrameRef;
use crate::lua_api::SimState;
use crate::lua_api::animation::{AnimState, AnimationType, LoopType, Smoothing};
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use mlua::{MultiValue, Value};

/// Register all animation proxy methods on FrameRef.
pub fn add_anim_proxy_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_anim_group_proxy_methods(methods);
    add_anim_proxy_methods_inner(methods);
    add_anim_get_animations(methods);
    add_anim_misc(methods);
}

fn add_anim_group_proxy_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_group_looping_methods(methods);
    add_group_play_core(methods);
    add_group_play_stop(methods);
    add_group_play_synced(methods);
    add_group_play_extras(methods);
    add_group_state_methods(methods);
    add_group_state_extras(methods);
    add_group_timing_methods(methods);
    add_group_alpha_methods(methods);
}

/// Parse Play/Restart arguments: (reverse: bool, offset: f64).
fn parse_proxy_play_args(args: MultiValue) -> (bool, f64) {
    let args: Vec<Value> = args.into_iter().collect();
    let reverse = matches!(args.first(), Some(Value::Boolean(true)));
    let offset = args
        .get(1)
        .and_then(|v| match v {
            Value::Number(n) => Some(*n),
            Value::Integer(n) => Some(*n as f64),
            _ => None,
        })
        .unwrap_or(0.0);
    (reverse, offset)
}

fn add_group_looping_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetLooping", |lua, this, looping: Option<String>| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            if let Some(group) = state.animation_groups.get_mut(&gid) {
                group.looping = LoopType::from_str(looping.as_deref().unwrap_or("NONE"));
            }
        }
        Ok(())
    });

    methods.add_method("GetLooping", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let s = state
            .anim_frame_to_group
            .get(&this.0)
            .and_then(|gid| state.animation_groups.get(gid))
            .map_or("NONE", |g| g.looping.as_str());
        Ok(Value::String(lua.create_string(s)?))
    });

    methods.add_method("GetLoopState", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let s = state
            .anim_frame_to_group
            .get(&this.0)
            .and_then(|gid| state.animation_groups.get(gid))
            .map_or("NONE", |g| g.looping.as_str());
        Ok(Value::String(lua.create_string(s)?))
    });
}

fn add_group_play_core<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Play", |lua, this, args: MultiValue| {
        let (reverse, offset) = parse_proxy_play_args(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            let already = state
                .animation_groups
                .get(&gid)
                .is_some_and(|g| g.playing && !g.done);
            if !already {
                crate::lua_api::animation::group_handle::start_group_playback_at(
                    &mut state, gid, reverse, offset,
                );
            }
        }
        Ok(())
    });

    methods.add_method("Restart", |lua, this, args: MultiValue| {
        let (reverse, offset) = parse_proxy_play_args(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            crate::lua_api::animation::group_handle::start_group_playback_at(
                &mut state, gid, reverse, offset,
            );
        }
        Ok(())
    });
}

fn add_group_play_stop<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Stop", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            crate::lua_api::animation::group_handle::stop_group(&mut state, gid);
        }
        Ok(())
    });

    methods.add_method("Pause", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            if state.animation_groups.get(&gid).is_some_and(|g| g.playing) {
                if let Some(g) = state.animation_groups.get_mut(&gid) {
                    g.paused = true;
                    g.playing = false;
                }
            }
            crate::lua_api::animation::tick::apply_flipbook_for_group(&mut state, gid);
        }
        Ok(())
    });

    methods.add_method("Finish", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            if let Some(g) = state.animation_groups.get_mut(&gid) {
                g.pending_finish = true;
            }
        }
        Ok(())
    });
}

fn add_group_play_synced<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("PlaySynced", |lua, this, args: MultiValue| {
        let reverse = matches!(args.iter().next(), Some(Value::Boolean(true)));
        let state_rc = get_sim_state(lua);
        let offset =
            crate::lua_api::animation::group_handle::compute_sync_offset(lua, this.0, &state_rc)?;
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            let already = state
                .animation_groups
                .get(&gid)
                .is_some_and(|g| g.playing && !g.done);
            if !already {
                crate::lua_api::animation::group_handle::start_group_playback_at(
                    &mut state, gid, reverse, offset,
                );
            }
        }
        Ok(())
    });
}

fn add_group_play_extras<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPlaying", |lua, this, playing: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            if playing {
                crate::lua_api::animation::group_handle::start_group_playback(
                    &mut state, gid, false,
                );
            } else {
                crate::lua_api::animation::group_handle::stop_group(&mut state, gid);
            }
        }
        Ok(())
    });

    methods.add_method("RemoveAnimations", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            if let Some(group) = state.animation_groups.get_mut(&gid) {
                group.animations.clear();
            }
            state.anim_frame_to_anim.retain(|_, &mut (g, _)| g != gid);
        }
        Ok(())
    });
}

fn add_group_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsPlaying", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).is_some_and(|g| g.playing));
        }
        if let Some(&(gid, _)) = state.anim_frame_to_anim.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).is_some_and(|g| g.playing));
        }
        Ok(false)
    });

    methods.add_method("IsPaused", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).is_some_and(|g| g.paused));
        }
        if let Some(&(gid, _)) = state.anim_frame_to_anim.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).is_some_and(|g| g.paused));
        }
        Ok(false)
    });

    methods.add_method("IsDone", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).is_none_or(|g| g.done));
        }
        if let Some(&(gid, _)) = state.anim_frame_to_anim.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).is_none_or(|g| g.done));
        }
        Ok(true)
    });
}

fn add_group_state_extras<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsPendingFinish", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state
                .animation_groups
                .get(&gid)
                .is_some_and(|g| g.pending_finish));
        }
        Ok(false)
    });

    methods.add_method("IsReverse", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).is_some_and(|g| g.reverse));
        }
        Ok(false)
    });

    methods.add_method("IsStopped", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&(gid, _)) = state.anim_frame_to_anim.get(&this.0) {
            return Ok(state
                .animation_groups
                .get(&gid)
                .is_none_or(|g| !g.playing && !g.paused));
        }
        Ok(true)
    });

    methods.add_method("IsDelaying", |_, _this, ()| Ok(false));
}

fn add_group_timing_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAnimationSpeedMultiplier", |lua, this, mult: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            if let Some(g) = state.animation_groups.get_mut(&gid) {
                g.speed_multiplier = mult;
            }
        }
        Ok(())
    });

    methods.add_method("GetAnimationSpeedMultiplier", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .anim_frame_to_group
            .get(&this.0)
            .and_then(|gid| state.animation_groups.get(gid))
            .map_or(1.0, |g| g.speed_multiplier))
    });
}

fn add_group_alpha_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetToFinalAlpha", |lua, this, val: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            if let Some(g) = state.animation_groups.get_mut(&gid) {
                g.set_to_final_alpha = val;
            }
        }
        Ok(())
    });

    methods.add_method("IsSetToFinalAlpha", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .anim_frame_to_group
            .get(&this.0)
            .and_then(|gid| state.animation_groups.get(gid))
            .is_some_and(|g| g.set_to_final_alpha))
    });

    methods.add_method("GetToFinalAlpha", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .anim_frame_to_group
            .get(&this.0)
            .and_then(|gid| state.animation_groups.get(gid))
            .is_some_and(|g| g.set_to_final_alpha))
    });
}

// ── Individual animation proxy methods ──────────────────────────────────────

fn add_anim_proxy_methods_inner<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_anim_duration_order(methods);
    add_anim_delay(methods);
    add_anim_smoothing(methods);
    add_anim_alpha_props(methods);
    add_anim_translation_props(methods);
    add_anim_scale_props(methods);
    add_anim_rotation_props(methods);
    add_anim_flipbook_props(methods);
    add_anim_progress(methods);
    add_anim_target(methods);
}

fn add_anim_duration_order<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDuration", |lua, this, dur: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.duration = dur;
        }
        Ok(())
    });
    methods.add_method("GetDuration", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state
                .animation_groups
                .get(&gid)
                .map_or(0.0, |g| g.total_duration()));
        }
        Ok(lookup_anim(&state, this.0).map_or(0.0, |a| a.duration))
    });
    methods.add_method("SetOrder", |lua, this, order: u32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.order = order;
        }
        Ok(())
    });
    methods.add_method("GetOrder", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(1u32, |a| a.order))
    });
}

fn add_anim_delay<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetStartDelay", |lua, this, d: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.start_delay = d;
        }
        Ok(())
    });
    methods.add_method("GetStartDelay", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(0.0, |a| a.start_delay))
    });
    methods.add_method("SetEndDelay", |lua, this, d: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.end_delay = d;
        }
        Ok(())
    });
    methods.add_method("GetEndDelay", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(0.0, |a| a.end_delay))
    });
}

fn add_anim_smoothing<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSmoothing", |lua, this, smooth: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.smoothing = Smoothing::from_str(&smooth);
        }
        Ok(())
    });
    methods.add_method("GetSmoothing", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let s = lookup_anim(&state, this.0).map_or("NONE", |a| a.smoothing.as_str());
        Ok(Value::String(lua.create_string(s)?))
    });
}

fn add_anim_alpha_props<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFromAlpha", |lua, this, v: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.from_alpha = v;
        }
        Ok(())
    });
    methods.add_method("GetFromAlpha", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(0.0, |a| a.from_alpha))
    });
    methods.add_method("SetToAlpha", |lua, this, v: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.to_alpha = v;
        }
        Ok(())
    });
    methods.add_method("GetToAlpha", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(1.0, |a| a.to_alpha))
    });
    methods.add_method("SetChange", |lua, this, args: MultiValue| {
        let args: Vec<Value> = args.into_iter().collect();
        let val = crate::lua_api::animation::extract_number(&args, 0).unwrap_or(0.0);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            if a.anim_type == AnimationType::Alpha {
                a.to_alpha = a.from_alpha + val;
            }
        }
        Ok(())
    });
}

fn add_anim_translation_props<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetOffset", |lua, this, args: MultiValue| {
        let args: Vec<Value> = args.into_iter().collect();
        let x = crate::lua_api::animation::extract_number(&args, 0).unwrap_or(0.0);
        let y = crate::lua_api::animation::extract_number(&args, 1).unwrap_or(0.0);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.offset_x = x;
            a.offset_y = y;
        }
        Ok(())
    });
}

fn add_anim_scale_props<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScaleFrom", |lua, this, args: MultiValue| {
        let args: Vec<Value> = args.into_iter().collect();
        let x = crate::lua_api::animation::extract_number(&args, 0).unwrap_or(1.0);
        let y = crate::lua_api::animation::extract_number(&args, 1).unwrap_or(1.0);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.from_scale_x = x;
            a.from_scale_y = y;
        }
        Ok(())
    });
    methods.add_method("SetScaleTo", |lua, this, args: MultiValue| {
        let args: Vec<Value> = args.into_iter().collect();
        let x = crate::lua_api::animation::extract_number(&args, 0).unwrap_or(1.0);
        let y = crate::lua_api::animation::extract_number(&args, 1).unwrap_or(1.0);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.to_scale_x = x;
            a.to_scale_y = y;
        }
        Ok(())
    });
}

fn add_anim_rotation_props<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDegrees", |lua, this, deg: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.degrees = deg;
        }
        Ok(())
    });
    methods.add_method("SetOrigin", |lua, this, args: MultiValue| {
        let mut iter = args.into_iter();
        let point = match iter.next() {
            Some(Value::String(s)) => s.to_string_lossy().to_string(),
            _ => "CENTER".to_string(),
        };
        let ox = extract_num_iter(&mut iter);
        let oy = extract_num_iter(&mut iter);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.origin_point = point;
            a.origin_offset_x = ox;
            a.origin_offset_y = oy;
        }
        Ok(())
    });
    methods.add_method("GetOrigin", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(a) = lookup_anim(&state, this.0) {
            return Ok(MultiValue::from_vec(vec![
                Value::String(lua.create_string(&a.origin_point)?),
                Value::Number(a.origin_offset_x),
                Value::Number(a.origin_offset_y),
            ]));
        }
        Ok(MultiValue::from_vec(vec![
            Value::String(lua.create_string("CENTER")?),
            Value::Number(0.0),
            Value::Number(0.0),
        ]))
    });
}

fn add_anim_flipbook_props<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFlipBookRows", |lua, this, v: u32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.flip_book_rows = v;
        }
        Ok(())
    });
    methods.add_method("GetFlipBookRows", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(1u32, |a| a.flip_book_rows))
    });
    methods.add_method("SetFlipBookColumns", |lua, this, v: u32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.flip_book_columns = v;
        }
        Ok(())
    });
    methods.add_method("GetFlipBookColumns", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(1u32, |a| a.flip_book_columns))
    });
    methods.add_method("SetFlipBookFrames", |lua, this, v: u32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.flip_book_frames = v;
        }
        Ok(())
    });
    methods.add_method("GetFlipBookFrames", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(1u32, |a| a.flip_book_frames))
    });
}

fn add_anim_progress<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetProgress", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).map_or(0.0, |g| {
                let dur = g.total_duration();
                if dur <= 0.0 {
                    0.0
                } else {
                    (g.elapsed / dur).clamp(0.0, 1.0)
                }
            }));
        }
        Ok(lookup_anim(&state, this.0).map_or(0.0, |a| a.raw_progress()))
    });
    methods.add_method("GetSmoothProgress", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(lookup_anim(&state, this.0).map_or(0.0, |a| a.smooth_progress()))
    });
    methods.add_method("GetElapsed", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&gid) = state.anim_frame_to_group.get(&this.0) {
            return Ok(state.animation_groups.get(&gid).map_or(0.0, |g| g.elapsed));
        }
        Ok(lookup_anim(&state, this.0).map_or(0.0, |a| a.elapsed))
    });
}

fn add_anim_target<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetTarget", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let Some(&(gid, idx)) = state.anim_frame_to_anim.get(&this.0) else {
            return Ok(Value::Nil);
        };
        let Some(group) = state.animation_groups.get(&gid) else {
            return Ok(Value::Nil);
        };
        let owner_id = group.owner_frame_id;
        let child_key = group.animations.get(idx).and_then(|a| a.child_key.clone());
        let target_id = match &child_key {
            Some(key) => state
                .widgets
                .get(owner_id)
                .and_then(|o| o.children_keys.get(key.as_str()).copied()),
            None => Some(owner_id),
        };
        let Some(id) = target_id else {
            return Ok(Value::Nil);
        };
        drop(state);
        frame_ref(lua, id)
    });
    methods.add_method("SetTarget", |_, _this, target: Value| {
        if target.is_nil() || target == Value::NULL {
            return Err(mlua::Error::RuntimeError(
                "Usage: Animation:SetTarget(target)".into(),
            ));
        }
        Ok(())
    });
    methods.add_method("SetChildKey", |lua, this, key: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(a) = lookup_anim_mut(&mut state, this.0) {
            a.child_key = Some(key);
        }
        Ok(())
    });
    methods.add_method("SetTargetKey", |_, _this, _key: String| Ok(()));
    methods.add_method("SetTargetName", |_, _this, _name: String| Ok(()));
    methods.add_method("SetTargetParent", |_, _this, ()| Ok(()));
}

fn add_anim_get_animations<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetAnimations", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let Some(&gid) = state.anim_frame_to_group.get(&this.0) else {
            return Ok(MultiValue::new());
        };
        let anim_fids: Vec<u64> = state
            .anim_frame_to_anim
            .iter()
            .filter(|&(_, &(g, _))| g == gid)
            .map(|(&fid, _)| fid)
            .collect();
        drop(state);
        let mut values = Vec::with_capacity(anim_fids.len());
        for fid in anim_fids {
            values.push(frame_ref(lua, fid)?);
        }
        Ok(MultiValue::from_vec(values))
    });
}

fn add_anim_misc<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetRegionParent", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(&(gid, _)) = state.anim_frame_to_anim.get(&this.0) {
            if let Some(owner_id) = state.animation_groups.get(&gid).map(|g| g.owner_frame_id) {
                drop(state);
                return frame_ref(lua, owner_id);
            }
        }
        Ok(Value::Nil)
    });
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn lookup_anim(state: &SimState, frame_id: u64) -> Option<&AnimState> {
    let &(gid, idx) = state.anim_frame_to_anim.get(&frame_id)?;
    state.animation_groups.get(&gid)?.animations.get(idx)
}

fn lookup_anim_mut(state: &mut SimState, frame_id: u64) -> Option<&mut AnimState> {
    let &(gid, idx) = state.anim_frame_to_anim.get(&frame_id)?;
    state
        .animation_groups
        .get_mut(&gid)?
        .animations
        .get_mut(idx)
}

fn extract_num_iter(iter: &mut impl Iterator<Item = Value>) -> f64 {
    match iter.next() {
        Some(Value::Number(n)) => n,
        Some(Value::Integer(n)) => n as f64,
        _ => 0.0,
    }
}
