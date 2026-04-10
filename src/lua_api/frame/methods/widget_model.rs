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
    methods.add_method("SetDisplayInfo", |_, _this, _id: i32| Ok(()));
    methods.add_method("SetCreature", |_, _this, _id: i32| Ok(()));
    methods.add_method("SetAnimation", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetCamDistanceScale", |_, _this, _scale: f64| Ok(()));
    methods.add_method("GetCamDistanceScale", |_, _this, ()| Ok(1.0_f64));
    methods.add_method("SetCamera", |_, _this, _cam: i32| Ok(()));
    methods.add_method("SetPortraitZoom", |_, _this, _zoom: f64| Ok(()));
    methods.add_method("SetDesaturation", |_, _this, _desat: f64| Ok(()));
    methods.add_method("SetLight", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ResetLights", |_, _this, ()| Ok(()));
    methods.add_method("SetSequence", |_, _this, _seq: i32| Ok(()));
    methods.add_method("SetSequenceTime", |_, _this, (_seq, _time): (i32, i32)| {
        Ok(())
    });
    methods.add_method("ClearModel", |_, _this, ()| Ok(()));
    methods.add_method("RefreshUnit", |_, _this, ()| Ok(()));
    methods.add_method("RefreshCamera", |_, _this, ()| Ok(()));
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
            "SetModelAlpha",
            "SetShadowEffect",
            "ReplaceIconTexture",
            "SetGlow",
            "SetGradientMask",
            "SetParticlesEnabled",
            "SetUseGBuffer",
        ],
    );
    add_model_f64_getter(methods, "GetModelAlpha", 1.0);
    methods.add_method("GetModelFileID", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .and_then(|frame| frame.model_file_id)
            .unwrap_or(0))
    });
    add_model_f64_getter(methods, "GetShadowEffect", 0.0);
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
    methods.add_method("SetAllowOverlappedModels", |_, _this, _allow: bool| Ok(()));
    methods.add_method("IsAllowOverlappedModels", |_, _this, ()| Ok(false));
    methods.add_method("SetPaused", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("Project3DPointTo2D", |_, _this, _args: mlua::MultiValue| {
        Ok::<(f64, f64, f64), _>((0.0, 0.0, 1.0))
    });
    methods.add_method("SetViewInsets", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetViewInsets", |_, _this, ()| {
        Ok((0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64))
    });
    methods.add_method("GetViewTranslation", |_, _this, ()| Ok((0.0_f64, 0.0_f64)));
}

fn add_model_scene_camera_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCameraPosition", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
    methods.add_method("GetCameraPosition", |_, _this, ()| {
        Ok((0.0_f64, 0.0_f64, 0.0_f64))
    });
    methods.add_method(
        "SetCameraOrientationByYawPitchRoll",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method(
        "SetCameraOrientationByAxisVectors",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("GetCameraForward", |_, _this, ()| {
        Ok((0.0_f64, 0.0_f64, 1.0_f64))
    });
    methods.add_method("GetCameraRight", |_, _this, ()| {
        Ok((1.0_f64, 0.0_f64, 0.0_f64))
    });
    methods.add_method("GetCameraUp", |_, _this, ()| {
        Ok((0.0_f64, 1.0_f64, 0.0_f64))
    });
    methods.add_method("SetCameraFieldOfView", |_, _this, _fov: f64| Ok(()));
    methods.add_method("GetCameraFieldOfView", |_, _this, ()| Ok(0.785_f64));
    methods.add_method("SetCameraNearClip", |_, _this, _clip: f64| Ok(()));
    methods.add_method("SetCameraFarClip", |_, _this, _clip: f64| Ok(()));
    methods.add_method("GetCameraNearClip", |_, _this, ()| Ok(0.1_f64));
    methods.add_method("GetCameraFarClip", |_, _this, ()| Ok(100.0_f64));
}

fn add_model_scene_light_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetLightType", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetLightPosition", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
    methods.add_method("GetLightPosition", |_, _this, ()| {
        Ok((0.0_f64, 0.0_f64, 0.0_f64))
    });
    methods.add_method("SetLightDirection", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
    methods.add_method("GetLightDirection", |_, _this, ()| {
        Ok((0.0_f64, -1.0_f64, 0.0_f64))
    });
    methods.add_method(
        "SetLightAmbientColor",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method(
        "SetLightDiffuseColor",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("SetLightVisible", |_, _this, _vis: bool| Ok(()));
    methods.add_method("IsLightVisible", |_, _this, ()| Ok(true));
}

fn add_model_scene_fog_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFogNear", |_, _this, _near: f64| Ok(()));
    methods.add_method("SetFogFar", |_, _this, _far: f64| Ok(()));
    methods.add_method("SetFogColor", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ClearFog", |_, _this, ()| Ok(()));
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
    methods.add_method("GetAllowOverlappedModels", |_, _this, ()| Ok(false));
    methods.add_method("GetDesaturation", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("GetLightAmbientColor", |_, _this, ()| {
        Ok((1.0_f64, 1.0_f64, 1.0_f64))
    });
    methods.add_method("GetLightDiffuseColor", |_, _this, ()| {
        Ok((1.0_f64, 1.0_f64, 1.0_f64))
    });
    methods.add_method("GetLightType", |_, _this, ()| Ok(0i32));
}
