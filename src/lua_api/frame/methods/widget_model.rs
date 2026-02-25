//! Model and ModelScene widget method stubs.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::{IntoLuaMulti, Value};

pub fn add_model_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_transform_methods(methods);
    add_model_appearance_methods(methods);
    add_model_scene_id_methods(methods);
}

fn add_model_transform_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetModel", |_, _this, _path: String| Ok(()));
    methods.add_method("GetModel", |_, _this, ()| Ok::<Option<String>, _>(None));
    methods.add_method("SetModelScale", |_, _this, _scale: f64| Ok(()));
    methods.add_method("GetModelScale", |_, _this, ()| Ok(1.0_f64));
    methods.add_method("SetPosition", |_, _this, _args: mlua::MultiValue| Ok(()));

    methods.add_method("GetPosition", |lua, this, ()| {
        let id = this.0;
        if let Some((func, frame_ud)) = super::methods_helpers::get_mixin_override(lua, id, "GetPosition") {
            return func.call::<mlua::MultiValue>(frame_ud);
        }
        (0.0_f64, 0.0_f64, 0.0_f64).into_lua_multi(lua)
    });

    methods.add_method("SetFacing", |_, _this, _radians: f64| Ok(()));
    methods.add_method("GetFacing", |_, _this, ()| Ok(0.0_f64));
    add_model_set_rotation(methods);
}

fn add_model_set_rotation<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetRotation", |lua, this, radians: Value| {
        let id = this.0;
        if let Some((func, frame_ud)) = super::methods_helpers::get_mixin_override(lua, id, "SetRotation") {
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
        if let Some((func, frame_ud)) = super::methods_helpers::get_mixin_override(lua, id, "SetUnit") {
            let mut call_args = vec![frame_ud];
            call_args.extend(args);
            return func.call::<()>(mlua::MultiValue::from_iter(call_args));
        }
        Ok(())
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
    methods.add_method("SetSequence", |_, _this, _seq: i32| Ok(()));
    methods.add_method("SetSequenceTime", |_, _this, (_seq, _time): (i32, i32)| Ok(()));
    methods.add_method("ClearModel", |_, _this, ()| Ok(()));
    methods.add_method("RefreshUnit", |_, _this, ()| Ok(()));
    methods.add_method("RefreshCamera", |_, _this, ()| Ok(()));
}

fn add_model_scene_id_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("TransitionToModelSceneID", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetFromModelSceneID", |_, _this, _id: i32| Ok(()));
    methods.add_method("GetModelSceneID", |_, _this, ()| Ok(0i32));
    methods.add_method("CycleVariation", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetUpperEmblemTexture", |_, _this, ()| Ok::<Option<String>, _>(None));
    methods.add_method("GetLowerEmblemTexture", |_, _this, ()| Ok::<Option<String>, _>(None));
}

/// Native ModelScene methods (C++ side in WoW, stubs here).
pub fn add_model_scene_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_model_scene_rendering_stubs(methods);
    add_model_scene_camera_stubs(methods);
    add_model_scene_light_stubs(methods);
    add_model_scene_fog_stubs(methods);
}

fn add_model_scene_rendering_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAllowOverlappedModels", |_, _this, _allow: bool| Ok(()));
    methods.add_method("IsAllowOverlappedModels", |_, _this, ()| Ok(false));
    methods.add_method("SetPaused", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("Project3DPointTo2D", |_, _this, _args: mlua::MultiValue| {
        Ok::<(f64, f64, f64), _>((0.0, 0.0, 1.0))
    });
    methods.add_method("SetViewInsets", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetViewInsets", |_, _this, ()| Ok((0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64)));
    methods.add_method("GetViewTranslation", |_, _this, ()| Ok((0.0_f64, 0.0_f64)));
}

fn add_model_scene_camera_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCameraPosition", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetCameraPosition", |_, _this, ()| Ok((0.0_f64, 0.0_f64, 0.0_f64)));
    methods.add_method("SetCameraOrientationByYawPitchRoll", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetCameraOrientationByAxisVectors", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetCameraForward", |_, _this, ()| Ok((0.0_f64, 0.0_f64, 1.0_f64)));
    methods.add_method("GetCameraRight", |_, _this, ()| Ok((1.0_f64, 0.0_f64, 0.0_f64)));
    methods.add_method("GetCameraUp", |_, _this, ()| Ok((0.0_f64, 1.0_f64, 0.0_f64)));
    methods.add_method("SetCameraFieldOfView", |_, _this, _fov: f64| Ok(()));
    methods.add_method("GetCameraFieldOfView", |_, _this, ()| Ok(0.785_f64));
    methods.add_method("SetCameraNearClip", |_, _this, _clip: f64| Ok(()));
    methods.add_method("SetCameraFarClip", |_, _this, _clip: f64| Ok(()));
    methods.add_method("GetCameraNearClip", |_, _this, ()| Ok(0.1_f64));
    methods.add_method("GetCameraFarClip", |_, _this, ()| Ok(100.0_f64));
}

fn add_model_scene_light_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetLightType", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetLightPosition", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetLightPosition", |_, _this, ()| Ok((0.0_f64, 0.0_f64, 0.0_f64)));
    methods.add_method("SetLightDirection", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetLightDirection", |_, _this, ()| Ok((0.0_f64, -1.0_f64, 0.0_f64)));
    methods.add_method("SetLightAmbientColor", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetLightDiffuseColor", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetLightVisible", |_, _this, _vis: bool| Ok(()));
    methods.add_method("IsLightVisible", |_, _this, ()| Ok(true));
}

fn add_model_scene_fog_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFogNear", |_, _this, _near: f64| Ok(()));
    methods.add_method("SetFogFar", |_, _this, _far: f64| Ok(()));
    methods.add_method("SetFogColor", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ClearFog", |_, _this, ()| Ok(()));
}
