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
    add_player_model_stubs(methods);
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

fn add_model_camera_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCameraDistance", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("SetCameraDistance", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetCameraFacing", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("SetCameraFacing", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetCameraRoll", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("SetCameraRoll", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetCameraTarget", |_, _this, ()| Ok((0.0_f64, 0.0_f64, 0.0_f64)));
    methods.add_method("SetCameraTarget", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetCustomCamera", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("HasCustomCamera", |_, _this, ()| Ok(false));
    methods.add_method("MakeCurrentCameraCustom", |_, _this, _args: mlua::MultiValue| Ok(()));
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
    methods.add_method("TransformCameraSpaceToModelSpace", |_, _this, _args: mlua::MultiValue| Ok(mlua::Value::Nil));
    methods.add_method("UseModelCenterToTransform", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("IsUsingModelCenterToTransform", |_, _this, ()| Ok(false));
    methods.add_method("SetViewTranslation", |_, _this, _args: mlua::MultiValue| Ok(()));
}

fn add_model_rendering_extra_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    // GetDrawLayer/SetDrawLayer are implemented in methods_texture.rs — don't override with stubs.
    methods.add_method("GetModelDrawLayer", |_, _this, ()| Ok("ARTWORK".to_string()));
    methods.add_method("SetModelDrawLayer", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetModelAlpha", |_, _this, ()| Ok(1.0_f64));
    methods.add_method("SetModelAlpha", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetModelFileID", |_, _this, ()| Ok(0i64));
    methods.add_method("GetShadowEffect", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("SetShadowEffect", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetPaused", |_, _this, ()| Ok(false));
    methods.add_method("HasAttachmentPoints", |_, _this, ()| Ok(false));
    methods.add_method("GetLight", |_, _this, ()| Ok(mlua::Value::Nil));
    methods.add_method("GetFogColor", |_, _this, ()| Ok((0.0_f64, 0.0_f64, 0.0_f64)));
    methods.add_method("GetFogFar", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("GetFogNear", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("ReplaceIconTexture", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetGlow", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetGradientMask", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetParticlesEnabled", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetUseGBuffer", |_, _this, _args: mlua::MultiValue| Ok(()));
}

fn add_player_model_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ApplySpellVisualKit", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("CanSetUnit", |_, _this, ()| Ok(false));
    methods.add_method("FreezeAnimation", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("GetDisplayInfo", |_, _this, ()| Ok(0i64));
    methods.add_method("GetDoBlend", |_, _this, ()| Ok(false));
    methods.add_method("GetKeepModelOnHide", |_, _this, ()| Ok(false));
    methods.add_method("HasAnimation", |_, _this, ()| Ok(false));
    methods.add_method("PlayAnimKit", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetBarberShopAlternateForm", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetDoBlend", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetItem", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetItemAppearance", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetKeepModelOnHide", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("StopAnimKit", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("ZeroCachedCenterXY", |_, _this, _args: mlua::MultiValue| Ok(()));
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
    methods.add_method("GetActorAtIndex", |_, _this, _args: mlua::MultiValue| Ok(mlua::Value::Nil));
    methods.add_method("GetNumActors", |_, _this, ()| Ok(0i32));
    methods.add_method("TakeActor", |_, _this, _args: mlua::MultiValue| Ok(mlua::Value::Nil));
}

fn add_model_scene_query_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetAllowOverlappedModels", |_, _this, ()| Ok(false));
    methods.add_method("GetDesaturation", |_, _this, ()| Ok(0.0_f64));
    methods.add_method("GetLightAmbientColor", |_, _this, ()| Ok((1.0_f64, 1.0_f64, 1.0_f64)));
    methods.add_method("GetLightDiffuseColor", |_, _this, ()| Ok((1.0_f64, 1.0_f64, 1.0_f64)));
    methods.add_method("GetLightType", |_, _this, ()| Ok(0i32));
}
