//! Model and ModelScene widget method stubs.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::{IntoLuaMulti, Value};

pub fn add_model_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_transform_methods(methods);
    add_model_appearance_methods(methods);
    add_model_scene_id_methods(methods);
    add_model_camera_stubs(methods);
    add_model_transform_extra_stubs(methods);
    add_model_rendering_extra_stubs(methods);
}

fn add_model_transform_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetModel", |lua, this, path: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.model_path = Some(path);
            frame.model_file_id = None;
        }
        Ok(())
    });
    methods.add_method("GetModel", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok::<String, _>(
            state
                .widgets
                .get(this.0)
                .and_then(|frame| frame.model_path.clone())
                .unwrap_or_default(),
        )
    });
    methods.add_method("SetModelScale", |lua, this, scale: f64| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_transform.scale = scale as f32;
        });
        Ok(())
    });
    methods.add_method("GetModelScale", |lua, this, ()| {
        Ok(
            read_model_frame(lua, this.0, |frame| frame.model_transform.scale as f64)
                .unwrap_or(1.0),
        )
    });
    methods.add_method("SetPosition", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_transform.position = parse_model_vec3(&args);
        });
        Ok(())
    });

    methods.add_method("GetPosition", |lua, this, ()| {
        let id = this.0;
        if let Some((func, frame_ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "GetPosition")
        {
            return func.call::<mlua::MultiValue>(frame_ud);
        }
        let position = read_model_frame(lua, id, |frame| frame.model_transform.position)
            .unwrap_or((0.0, 0.0, 0.0));
        (position.0 as f64, position.1 as f64, position.2 as f64).into_lua_multi(lua)
    });

    methods.add_method("SetFacing", |lua, this, radians: f64| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_transform.facing = radians as f32;
        });
        Ok(())
    });
    methods.add_method("GetFacing", |lua, this, ()| {
        Ok(
            read_model_frame(lua, this.0, |frame| frame.model_transform.facing as f64)
                .unwrap_or(0.0),
        )
    });
    add_model_set_rotation(methods);
}

fn add_model_set_rotation<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetRotation", |lua, this, radians: Value| {
        let id = this.0;
        if let Some((func, frame_ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "SetRotation")
        {
            return func.call::<()>((frame_ud, radians));
        }
        let rad_f64 = match radians {
            Value::Number(n) => n,
            Value::Integer(n) => n as f64,
            _ => 0.0,
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.rotation = rad_f64 as f32;
        }
        Ok(())
    });
}

fn add_model_appearance_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetUnit", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        // GameTooltip:SetUnit populates tooltip with unit info and returns bool
        {
            let state_rc = super::super::handle::get_sim_state(lua);
            let state = state_rc.borrow();
            if state.tooltips.contains_key(&id) {
                drop(state);
                return super::widget_tooltip::set_unit_for_tooltip(lua, id, args);
            }
        }
        if let Some((func, frame_ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "SetUnit")
        {
            let mut call_args = vec![frame_ud];
            call_args.extend(args);
            return func.call::<mlua::Value>(mlua::MultiValue::from_iter(call_args));
        }
        Ok(mlua::Value::Nil)
    });
    add_model_appearance_stubs(methods);
}

fn add_model_appearance_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAutoDress", |_, _this, _auto: bool| Ok(()));
    methods.add_method("SetDisplayInfo", |lua, this, id: i32| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_path = None;
            frame.model_file_id = None;
            frame.model_appearance.display_info = Some(id);
            frame.model_appearance.creature_id = None;
        });
        Ok(())
    });
    methods.add_method("SetCreature", |lua, this, id: i32| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_path = None;
            frame.model_file_id = None;
            frame.model_appearance.display_info = None;
            frame.model_appearance.creature_id = Some(id);
        });
        Ok(())
    });
    methods.add_method("SetAnimation", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_appearance.animation_id = parse_first_model_i32(&args);
        });
        Ok(())
    });
    methods.add_method("SetCamDistanceScale", |_, _this, _scale: f64| Ok(()));
    methods.add_method("GetCamDistanceScale", |_, _this, ()| Ok(1.0_f64));
    methods.add_method("SetCamera", |_, _this, _cam: i32| Ok(()));
    methods.add_method("SetPortraitZoom", |_, _this, _zoom: f64| Ok(()));
    methods.add_method("SetDesaturation", |_, _this, _desat: f64| Ok(()));
    methods.add_method("SetLight", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ResetLights", |_, _this, ()| Ok(()));
    methods.add_method("SetSequence", |lua, this, seq: i32| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_appearance.sequence_id = Some(seq);
            frame.model_appearance.sequence_time_ms = None;
        });
        Ok(())
    });
    methods.add_method("SetSequenceTime", |lua, this, (seq, time): (i32, i32)| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_appearance.sequence_id = Some(seq);
            frame.model_appearance.sequence_time_ms = Some(time);
        });
        Ok(())
    });
    methods.add_method("ClearModel", |lua, this, ()| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_path = None;
            frame.model_file_id = None;
            frame.model_appearance.display_info = None;
            frame.model_appearance.creature_id = None;
            frame.model_appearance.animation_id = None;
            frame.model_appearance.sequence_id = None;
            frame.model_appearance.sequence_time_ms = None;
        });
        Ok(())
    });
    methods.add_method("RefreshUnit", |lua, this, ()| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_appearance.refresh_unit_count += 1;
        });
        Ok(())
    });
    methods.add_method("RefreshCamera", |lua, this, ()| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_appearance.refresh_camera_count += 1;
        });
        Ok(())
    });
}

fn add_model_scene_id_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "TransitionToModelSceneID",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("SetFromModelSceneID", |_, _this, _id: i32| Ok(()));
    methods.add_method("GetModelSceneID", |_, _this, ()| Ok(0i32));
    methods.add_method("CycleVariation", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetUpperEmblemTexture", |_, _this, ()| {
        Ok::<Option<String>, _>(None)
    });
    methods.add_method("GetLowerEmblemTexture", |_, _this, ()| {
        Ok::<Option<String>, _>(None)
    });
}

fn add_model_camera_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCameraDistance", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_transform.camera.distance as f64
        })
        .unwrap_or(0.0))
    });
    methods.add_method("SetCameraDistance", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_transform.camera.distance = parse_first_model_number(&args) as f32;
        });
        Ok(())
    });
    methods.add_method("GetCameraFacing", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_transform.camera.facing as f64
        })
        .unwrap_or(0.0))
    });
    methods.add_method("SetCameraFacing", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_transform.camera.facing = parse_first_model_number(&args) as f32;
        });
        Ok(())
    });
    methods.add_method("GetCameraRoll", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_transform.camera.roll as f64
        })
        .unwrap_or(0.0))
    });
    methods.add_method("SetCameraRoll", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_transform.camera.roll = parse_first_model_number(&args) as f32;
        });
        Ok(())
    });
    methods.add_method("GetCameraTarget", |lua, this, ()| {
        let target = read_model_frame(lua, this.0, |frame| frame.model_transform.camera.target)
            .unwrap_or((0.0, 0.0, 0.0));
        Ok((target.0 as f64, target.1 as f64, target.2 as f64))
    });
    methods.add_method("SetCameraTarget", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_transform.camera.target = parse_model_vec3(&args);
        });
        Ok(())
    });
    methods.add_method(
        "SetCustomCamera",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("HasCustomCamera", |_, _this, ()| Ok(false));
    methods.add_method(
        "MakeCurrentCameraCustom",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
}

fn add_model_transform_extra_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AdvanceTime", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ClearTransform", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetTransform", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetPitch", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("SetPitch", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetRoll", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("SetRoll", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetWorldScale", |_, _this, ()| Ok(1.0_f64));
    methods.add_method(
        "TransformCameraSpaceToModelSpace",
        |_, _this, _args: mlua::MultiValue| Ok(mlua::Value::Nil),
    );
    methods.add_method(
        "UseModelCenterToTransform",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("IsUsingModelCenterToTransform", |_, _this, ()| Ok(false));
    methods.add_method("SetViewTranslation", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
}

fn add_model_rendering_extra_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    // GetDrawLayer/SetDrawLayer are implemented in methods_texture.rs — don't override with stubs.
    add_model_string_getter(methods, "GetModelDrawLayer", "ARTWORK");
    add_model_variadic_stub(
        methods,
        &[
            "SetModelDrawLayer",
            "ReplaceIconTexture",
            "SetGlow",
            "SetGradientMask",
        ],
    );
    methods.add_method("SetModelAlpha", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_rendering.alpha = parse_first_model_number(&args) as f32;
        });
        Ok(())
    });
    methods.add_method("GetModelAlpha", |lua, this, ()| {
        Ok(
            read_model_frame(lua, this.0, |frame| frame.model_rendering.alpha as f64)
                .unwrap_or(1.0),
        )
    });
    methods.add_method("GetModelFileID", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .and_then(|frame| frame.model_file_id)
            .unwrap_or(0))
    });
    methods.add_method("SetShadowEffect", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_rendering.shadow_effect = parse_first_model_number(&args) as f32;
        });
        Ok(())
    });
    methods.add_method("GetShadowEffect", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_rendering.shadow_effect as f64
        })
        .unwrap_or(0.0))
    });
    methods.add_method(
        "SetParticlesEnabled",
        |lua, this, args: mlua::MultiValue| {
            update_model_frame(lua, this.0, |frame| {
                frame.model_rendering.particles_enabled = parse_first_model_bool(&args);
            });
            Ok(())
        },
    );
    methods.add_method("SetUseGBuffer", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_rendering.use_gbuffer = parse_first_model_bool(&args);
        });
        Ok(())
    });
    add_model_bool_getter(methods, "GetPaused", false);
    add_model_bool_getter(methods, "HasAttachmentPoints", false);
    methods.add_method("GetLight", |_, _this, ()| Ok(mlua::Value::Nil));
    methods.add_method("GetFogColor", |_, _this, ()| {
        Ok((0.0_f64, 0.0_f64, 0.0_f64))
    });
    add_model_f64_getter(methods, "GetFogFar", 0.0);
    add_model_f64_getter(methods, "GetFogNear", 0.0);
}

fn add_model_variadic_stub<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    names: &[&'static str],
) {
    for name in names {
        methods.add_method(*name, |_, _this, _args: mlua::MultiValue| Ok(()));
    }
}

fn add_model_string_getter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    value: &'static str,
) {
    methods.add_method(name, move |_, _this, ()| Ok(value.to_string()));
}

fn add_model_bool_getter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    value: bool,
) {
    methods.add_method(name, move |_, _this, ()| Ok(value));
}

fn add_model_f64_getter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    value: f64,
) {
    methods.add_method(name, move |_, _this, ()| Ok(value));
}

fn update_model_frame(lua: &mlua::Lua, id: u64, update: impl FnOnce(&mut crate::widget::Frame)) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        update(frame);
    }
}

fn read_model_frame<T>(
    lua: &mlua::Lua,
    id: u64,
    read: impl FnOnce(&crate::widget::Frame) -> T,
) -> Option<T> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state.widgets.get(id).map(read)
}

fn parse_first_model_number(args: &mlua::MultiValue) -> f64 {
    args.front().map(lua_value_to_f64).unwrap_or(0.0)
}

fn parse_first_model_i32(args: &mlua::MultiValue) -> Option<i32> {
    args.front().map(lua_value_to_i32)
}

fn parse_first_model_bool(args: &mlua::MultiValue) -> bool {
    args.front().map(lua_value_to_bool).unwrap_or(false)
}

fn parse_model_vec3(args: &mlua::MultiValue) -> (f32, f32, f32) {
    (
        args.front().map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(1).map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(2).map(lua_value_to_f64).unwrap_or(0.0) as f32,
    )
}

fn lua_value_to_f64(value: &Value) -> f64 {
    match value {
        Value::Number(n) => *n,
        Value::Integer(n) => *n as f64,
        _ => 0.0,
    }
}

fn lua_value_to_i32(value: &Value) -> i32 {
    match value {
        Value::Number(n) => *n as i32,
        Value::Integer(n) => *n as i32,
        _ => 0,
    }
}

fn lua_value_to_bool(value: &Value) -> bool {
    match value {
        Value::Boolean(flag) => *flag,
        Value::Number(n) => *n != 0.0,
        Value::Integer(n) => *n != 0,
        _ => false,
    }
}

/// Native ModelScene methods (C++ side in WoW, stubs here).
pub fn add_model_scene_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_scene_rendering_stubs(methods);
    add_model_scene_camera_stubs(methods);
    add_model_scene_light_stubs(methods);
    add_model_scene_fog_stubs(methods);
    add_model_scene_actor_stubs(methods);
    add_model_scene_query_stubs(methods);
}

fn add_model_scene_rendering_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_scene_overlap_methods(methods);
    add_model_scene_pause_methods(methods);
    methods.add_method("Project3DPointTo2D", |_, _this, _args: mlua::MultiValue| {
        Ok::<(f64, f64, f64), _>((0.0, 0.0, 1.0))
    });
    add_model_scene_view_methods(methods);
}

fn add_model_scene_overlap_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAllowOverlappedModels", |lua, this, allow: bool| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.allow_overlapped_models = allow;
        });
        Ok(())
    });
    methods.add_method("IsAllowOverlappedModels", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.allow_overlapped_models
        })
        .unwrap_or(false))
    });
}

fn add_model_scene_pause_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPaused", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.paused = parse_first_model_bool(&args);
        });
        Ok(())
    });
}

fn add_model_scene_view_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetViewInsets", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.view_insets = parse_model_vec4(&args);
        });
        Ok(())
    });
    methods.add_method("GetViewInsets", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple4_to_f64(frame.model_scene_state.view_insets)
        })
        .unwrap_or((0.0, 0.0, 0.0, 0.0)))
    });
    methods.add_method("SetViewTranslation", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.view_translation = parse_model_vec2(&args);
        });
        Ok(())
    });
    methods.add_method("GetViewTranslation", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple2_to_f64(frame.model_scene_state.view_translation)
        })
        .unwrap_or((0.0, 0.0)))
    });
}

fn add_model_scene_camera_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_scene_camera_position_methods(methods);
    add_model_scene_camera_orientation_methods(methods);
    add_model_scene_camera_clip_methods(methods);
}

fn add_model_scene_camera_position_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCameraPosition", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.camera.position = parse_model_vec3(&args);
        });
        Ok(())
    });
    methods.add_method("GetCameraPosition", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple3_to_f64(frame.model_scene_state.camera.position)
        })
        .unwrap_or((0.0, 0.0, 0.0)))
    });
}

fn add_model_scene_camera_orientation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetCameraOrientationByYawPitchRoll",
        |lua, this, args: mlua::MultiValue| {
            let yaw = args.front().map(lua_value_to_f64).unwrap_or(0.0) as f32;
            let pitch = args.get(1).map(lua_value_to_f64).unwrap_or(0.0) as f32;
            let roll = args.get(2).map(lua_value_to_f64).unwrap_or(0.0) as f32;
            let (forward, right, up) = model_scene_axes_from_yaw_pitch_roll(yaw, pitch, roll);
            update_model_frame(lua, this.0, |frame| {
                frame.model_scene_state.camera.forward = forward;
                frame.model_scene_state.camera.right = right;
                frame.model_scene_state.camera.up = up;
            });
            Ok(())
        },
    );
    methods.add_method(
        "SetCameraOrientationByAxisVectors",
        |lua, this, args: mlua::MultiValue| {
            let forward = parse_model_vec3(&args);
            let right = (
                args.get(3).map(lua_value_to_f64).unwrap_or(1.0) as f32,
                args.get(4).map(lua_value_to_f64).unwrap_or(0.0) as f32,
                args.get(5).map(lua_value_to_f64).unwrap_or(0.0) as f32,
            );
            let up = (
                args.get(6).map(lua_value_to_f64).unwrap_or(0.0) as f32,
                args.get(7).map(lua_value_to_f64).unwrap_or(1.0) as f32,
                args.get(8).map(lua_value_to_f64).unwrap_or(0.0) as f32,
            );
            update_model_frame(lua, this.0, |frame| {
                frame.model_scene_state.camera.forward = forward;
                frame.model_scene_state.camera.right = right;
                frame.model_scene_state.camera.up = up;
            });
            Ok(())
        },
    );
    methods.add_method("GetCameraForward", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple3_to_f64(frame.model_scene_state.camera.forward)
        })
        .unwrap_or((0.0, 0.0, 1.0)))
    });
    methods.add_method("GetCameraRight", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple3_to_f64(frame.model_scene_state.camera.right)
        })
        .unwrap_or((1.0, 0.0, 0.0)))
    });
    methods.add_method("GetCameraUp", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple3_to_f64(frame.model_scene_state.camera.up)
        })
        .unwrap_or((0.0, 1.0, 0.0)))
    });
}

fn add_model_scene_camera_clip_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCameraFieldOfView", |lua, this, fov: f64| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.camera.field_of_view = fov as f32;
        });
        Ok(())
    });
    methods.add_method("GetCameraFieldOfView", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.camera.field_of_view as f64
        })
        .unwrap_or(0.785))
    });
    methods.add_method("SetCameraNearClip", |lua, this, clip: f64| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.camera.near_clip = clip as f32;
        });
        Ok(())
    });
    methods.add_method("SetCameraFarClip", |lua, this, clip: f64| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.camera.far_clip = clip as f32;
        });
        Ok(())
    });
    methods.add_method("GetCameraNearClip", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.camera.near_clip as f64
        })
        .unwrap_or(0.1))
    });
    methods.add_method("GetCameraFarClip", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.camera.far_clip as f64
        })
        .unwrap_or(100.0))
    });
}

fn add_model_scene_light_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_scene_light_geometry_methods(methods);
    add_model_scene_light_color_methods(methods);
}

fn add_model_scene_light_geometry_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetLightType", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.light.light_type = parse_first_model_i32(&args).unwrap_or(0);
        });
        Ok(())
    });
    methods.add_method("SetLightPosition", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.light.position = parse_model_vec3(&args);
        });
        Ok(())
    });
    methods.add_method("GetLightPosition", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple3_to_f64(frame.model_scene_state.light.position)
        })
        .unwrap_or((0.0, 0.0, 0.0)))
    });
    methods.add_method("SetLightDirection", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.light.direction = parse_model_vec3(&args);
        });
        Ok(())
    });
    methods.add_method("GetLightDirection", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            tuple3_to_f64(frame.model_scene_state.light.direction)
        })
        .unwrap_or((0.0, -1.0, 0.0)))
    });
}

fn add_model_scene_light_color_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetLightAmbientColor",
        |lua, this, args: mlua::MultiValue| {
            update_model_frame(lua, this.0, |frame| {
                frame.model_scene_state.light.ambient_color = parse_model_rgb(&args);
            });
            Ok(())
        },
    );
    methods.add_method(
        "SetLightDiffuseColor",
        |lua, this, args: mlua::MultiValue| {
            update_model_frame(lua, this.0, |frame| {
                frame.model_scene_state.light.diffuse_color = parse_model_rgb(&args);
            });
            Ok(())
        },
    );
    methods.add_method("SetLightVisible", |lua, this, vis: bool| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.light.visible = vis;
        });
        Ok(())
    });
    methods.add_method("IsLightVisible", |lua, this, ()| {
        Ok(
            read_model_frame(lua, this.0, |frame| frame.model_scene_state.light.visible)
                .unwrap_or(true),
        )
    });
}

fn add_model_scene_fog_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFogNear", |lua, this, near: f64| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.fog.near = near as f32;
        });
        Ok(())
    });
    methods.add_method("SetFogFar", |lua, this, far: f64| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.fog.far = far as f32;
        });
        Ok(())
    });
    methods.add_method("SetFogColor", |lua, this, args: mlua::MultiValue| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.fog.color = parse_model_rgb(&args);
        });
        Ok(())
    });
    methods.add_method("ClearFog", |lua, this, ()| {
        update_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.fog.near = 0.0;
            frame.model_scene_state.fog.far = 0.0;
            frame.model_scene_state.fog.color = crate::widget::Color::rgb(0.0, 0.0, 0.0);
        });
        Ok(())
    });
}

fn add_model_scene_actor_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CreateActor", |lua, this, _args: mlua::MultiValue| {
        use crate::lua_api::frame::handle::{frame_ref as mk_frame_ref, get_sim_state};
        use crate::widget::{Frame, WidgetType};
        let id = this.0;
        let mut child = Frame::new(WidgetType::Frame, None, Some(id));
        child.object_type_name = Some("ModelSceneActor".to_string());
        let child_id = child.id;
        {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            state.widgets.register(child);
            state.widgets.add_child(id, child_id);
        }
        mk_frame_ref(lua, child_id)
    });
    methods.add_method("GetActorAtIndex", |_, _this, _args: mlua::MultiValue| {
        Ok(mlua::Value::Nil)
    });
    methods.add_method("GetNumActors", |_, _this, ()| Ok(0i32));
    methods.add_method("TakeActor", |_, _this, _args: mlua::MultiValue| {
        Ok(mlua::Value::Nil)
    });
}

fn add_model_scene_query_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_scene_state_query_methods(methods);
    add_model_scene_light_query_methods(methods);
    add_model_scene_fog_query_methods(methods);
}

fn add_model_scene_state_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetAllowOverlappedModels", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.allow_overlapped_models
        })
        .unwrap_or(false))
    });
    methods.add_method("GetDesaturation", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("GetPaused", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| frame.model_scene_state.paused).unwrap_or(false))
    });
}

fn add_model_scene_light_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetLightAmbientColor", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            color_to_rgb_f64(frame.model_scene_state.light.ambient_color)
        })
        .unwrap_or((1.0, 1.0, 1.0)))
    });
    methods.add_method("GetLightDiffuseColor", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            color_to_rgb_f64(frame.model_scene_state.light.diffuse_color)
        })
        .unwrap_or((1.0, 1.0, 1.0)))
    });
    methods.add_method("GetLightType", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            frame.model_scene_state.light.light_type
        })
        .unwrap_or(0))
    });
}

fn add_model_scene_fog_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetFogColor", |lua, this, ()| {
        Ok(read_model_frame(lua, this.0, |frame| {
            color_to_rgb_f64(frame.model_scene_state.fog.color)
        })
        .unwrap_or((0.0, 0.0, 0.0)))
    });
    methods.add_method("GetFogNear", |lua, this, ()| {
        Ok(
            read_model_frame(lua, this.0, |frame| frame.model_scene_state.fog.near as f64)
                .unwrap_or(0.0),
        )
    });
    methods.add_method("GetFogFar", |lua, this, ()| {
        Ok(
            read_model_frame(lua, this.0, |frame| frame.model_scene_state.fog.far as f64)
                .unwrap_or(0.0),
        )
    });
}

fn parse_model_vec2(args: &mlua::MultiValue) -> (f32, f32) {
    (
        args.front().map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(1).map(lua_value_to_f64).unwrap_or(0.0) as f32,
    )
}

fn parse_model_vec4(args: &mlua::MultiValue) -> (f32, f32, f32, f32) {
    (
        args.front().map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(1).map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(2).map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(3).map(lua_value_to_f64).unwrap_or(0.0) as f32,
    )
}

fn parse_model_rgb(args: &mlua::MultiValue) -> crate::widget::Color {
    crate::widget::Color::rgb(
        args.front().map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(1).map(lua_value_to_f64).unwrap_or(0.0) as f32,
        args.get(2).map(lua_value_to_f64).unwrap_or(0.0) as f32,
    )
}

fn tuple2_to_f64(values: (f32, f32)) -> (f64, f64) {
    (values.0 as f64, values.1 as f64)
}

fn tuple3_to_f64(values: (f32, f32, f32)) -> (f64, f64, f64) {
    (values.0 as f64, values.1 as f64, values.2 as f64)
}

fn tuple4_to_f64(values: (f32, f32, f32, f32)) -> (f64, f64, f64, f64) {
    (
        values.0 as f64,
        values.1 as f64,
        values.2 as f64,
        values.3 as f64,
    )
}

fn color_to_rgb_f64(color: crate::widget::Color) -> (f64, f64, f64) {
    (color.r as f64, color.g as f64, color.b as f64)
}

fn model_scene_axes_from_yaw_pitch_roll(
    yaw: f32,
    pitch: f32,
    roll: f32,
) -> ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) {
    let cy = yaw.cos();
    let sy = yaw.sin();
    let cp = pitch.cos();
    let sp = pitch.sin();
    let cr = roll.cos();
    let sr = roll.sin();

    let right = (cy * cp, sy * cp, -sp);
    let up = (cy * sp * sr - sy * cr, sy * sp * sr + cy * cr, cp * sr);
    let forward = (cy * sp * cr + sy * sr, sy * sp * cr - cy * sr, cp * cr);
    (forward, right, up)
}
