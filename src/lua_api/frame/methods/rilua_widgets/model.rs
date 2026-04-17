//! Model and ModelScene widget methods (stubs + partial impl).

use super::shared::{opt_bool, val_to_f64};
use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(super) fn set_model(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = super::shared::opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = path;
        f.model_file_id = None;
    }
    Ok(0)
}

pub(super) fn get_model(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|f| f.model_path.clone())
            .unwrap_or_default()
    };
    let val = create_string(state, &path);
    val.into_stack(state)
}

pub(super) fn set_model_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scale = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.scale = scale;
    }
    Ok(0)
}

pub(super) fn get_model_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.scale as f64)
        .unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = val_to_f64(stack_val(state, 2)) as f32;
    let y = val_to_f64(stack_val(state, 3)) as f32;
    let z = val_to_f64(stack_val(state, 4)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.position = (x, y, z);
    }
    Ok(0)
}

pub(super) fn get_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (x, y, z) = sim
        .widgets
        .get(id)
        .map(|f| {
            let p = f.model_transform.position;
            (p.0 as f64, p.1 as f64, p.2 as f64)
        })
        .unwrap_or((0.0, 0.0, 0.0));
    drop(sim);
    (x, y, z).into_stack(state)
}

pub(super) fn set_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rad = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.facing = rad;
    }
    Ok(0)
}

pub(super) fn get_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.facing as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_rotation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rad = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.rotation = rad;
    }
    Ok(0)
}

pub(super) fn set_animation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let anim_id = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.animation_id = Some(anim_id);
    }
    Ok(0)
}

pub(super) fn set_display_info(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let display_id = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = None;
        f.model_file_id = None;
        f.model_appearance.display_info = Some(display_id);
        f.model_appearance.creature_id = None;
    }
    Ok(0)
}

pub(super) fn set_creature(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let creature_id = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = None;
        f.model_file_id = None;
        f.model_appearance.display_info = None;
        f.model_appearance.creature_id = Some(creature_id);
    }
    Ok(0)
}

pub(super) fn clear_model(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = None;
        f.model_file_id = None;
        f.model_appearance.display_info = None;
        f.model_appearance.creature_id = None;
        f.model_appearance.animation_id = None;
        f.model_appearance.sequence_id = None;
        f.model_appearance.sequence_time_ms = None;
    }
    Ok(0)
}

pub(super) fn get_model_file_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.model_file_id)
        .unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

pub(super) fn set_model_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_rendering.alpha = alpha;
    }
    Ok(0)
}

pub(super) fn get_model_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_rendering.alpha as f64)
        .unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_sequence(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let seq = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.sequence_id = Some(seq);
        f.model_appearance.sequence_time_ms = None;
    }
    Ok(0)
}

pub(super) fn set_sequence_time(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let seq = val_to_f64(stack_val(state, 2)) as i32;
    let time = val_to_f64(stack_val(state, 3)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.sequence_id = Some(seq);
        f.model_appearance.sequence_time_ms = Some(time);
    }
    Ok(0)
}

pub(super) fn get_camera_distance(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.camera.distance as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_camera_distance(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let dist = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.camera.distance = dist;
    }
    Ok(0)
}

pub(super) fn get_camera_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.camera.facing as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_camera_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rad = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.camera.facing = rad;
    }
    Ok(0)
}

pub(super) fn get_camera_target(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let t = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.camera.target)
        .unwrap_or((0.0, 0.0, 0.0));
    drop(sim);
    (t.0 as f64, t.1 as f64, t.2 as f64).into_stack(state)
}

pub(super) fn set_camera_target(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = val_to_f64(stack_val(state, 2)) as f32;
    let y = val_to_f64(stack_val(state, 3)) as f32;
    let z = val_to_f64(stack_val(state, 4)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.camera.target = (x, y, z);
    }
    Ok(0)
}

// Stubs

pub(super) fn stub_variadic(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub(super) fn stub_nil(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

pub(super) fn stub_zero(state: &mut LuaState) -> LuaResult<u32> {
    0.0_f64.into_stack(state)
}

pub(super) fn stub_one(state: &mut LuaState) -> LuaResult<u32> {
    1.0_f64.into_stack(state)
}

pub(super) fn stub_false(state: &mut LuaState) -> LuaResult<u32> {
    false.into_stack(state)
}

// ModelScene

pub(super) fn scene_set_allow_overlapped_models(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let allow = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.allow_overlapped_models = allow;
    }
    Ok(0)
}

pub(super) fn scene_set_view_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let l = val_to_f64(stack_val(state, 2)) as f32;
    let r = val_to_f64(stack_val(state, 3)) as f32;
    let t = val_to_f64(stack_val(state, 4)) as f32;
    let b = val_to_f64(stack_val(state, 5)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.view_insets = (l, r, t, b);
    }
    Ok(0)
}

pub(super) fn scene_get_view_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (l, r, t, b) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.model_scene_state.view_insets)
            .unwrap_or((0.0, 0.0, 0.0, 0.0))
    };
    (l as f64, r as f64, t as f64, b as f64).into_stack(state)
}

pub(super) fn scene_is_allow_overlapped_models(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let allow = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.allow_overlapped_models)
        .unwrap_or(false);
    state.push(Val::Bool(allow));
    Ok(1)
}

// ---------------------------------------------------------------------------
// register_model
// ---------------------------------------------------------------------------

pub(super) fn register_model(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, metatable, "SetModel", set_model)?;
    table_set_rust_fn(state, metatable, "GetModel", get_model)?;
    table_set_rust_fn(state, metatable, "SetModelScale", set_model_scale)?;
    table_set_rust_fn(state, metatable, "GetModelScale", get_model_scale)?;
    table_set_rust_fn(state, metatable, "SetPosition", set_position)?;
    table_set_rust_fn(state, metatable, "GetPosition", get_position)?;
    table_set_rust_fn(state, metatable, "SetFacing", set_facing)?;
    table_set_rust_fn(state, metatable, "GetFacing", get_facing)?;
    table_set_rust_fn(state, metatable, "SetRotation", set_rotation)?;
    table_set_rust_fn(state, metatable, "SetAnimation", set_animation)?;
    table_set_rust_fn(state, metatable, "SetDisplayInfo", set_display_info)?;
    table_set_rust_fn(state, metatable, "SetCreature", set_creature)?;
    table_set_rust_fn(state, metatable, "ClearModel", clear_model)?;
    table_set_rust_fn(state, metatable, "GetModelFileID", get_model_file_id)?;
    table_set_rust_fn(state, metatable, "SetModelAlpha", set_model_alpha)?;
    table_set_rust_fn(state, metatable, "GetModelAlpha", get_model_alpha)?;
    table_set_rust_fn(state, metatable, "SetSequence", set_sequence)?;
    table_set_rust_fn(state, metatable, "SetSequenceTime", set_sequence_time)?;
    table_set_rust_fn(state, metatable, "GetCameraDistance", get_camera_distance)?;
    table_set_rust_fn(state, metatable, "SetCameraDistance", set_camera_distance)?;
    table_set_rust_fn(state, metatable, "GetCameraFacing", get_camera_facing)?;
    table_set_rust_fn(state, metatable, "SetCameraFacing", set_camera_facing)?;
    table_set_rust_fn(state, metatable, "GetCameraTarget", get_camera_target)?;
    table_set_rust_fn(state, metatable, "SetCameraTarget", set_camera_target)?;
    // Stubs
    table_set_rust_fn(state, metatable, "SetAutoDress", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetCamDistanceScale", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetCamera", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetPortraitZoom", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetDesaturation", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetLight", stub_variadic)?;
    table_set_rust_fn(state, metatable, "ResetLights", stub_variadic)?;
    table_set_rust_fn(state, metatable, "RefreshUnit", stub_variadic)?;
    table_set_rust_fn(state, metatable, "RefreshCamera", stub_variadic)?;
    table_set_rust_fn(state, metatable, "TransitionToModelSceneID", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetFromModelSceneID", stub_variadic)?;
    table_set_rust_fn(state, metatable, "CycleVariation", stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetModelSceneID", stub_zero)?;
    table_set_rust_fn(state, metatable, "GetCamDistanceScale", stub_one)?;
    table_set_rust_fn(state, metatable, "HasCustomCamera", stub_false)?;
    table_set_rust_fn(state, metatable, "GetPaused", stub_false)?;
    table_set_rust_fn(state, metatable, "HasAttachmentPoints", stub_false)?;
    table_set_rust_fn(state, metatable, "GetLight", stub_nil)?;
    table_set_rust_fn(state, metatable, "AdvanceTime", stub_variadic)?;
    table_set_rust_fn(state, metatable, "ClearTransform", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetTransform", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetPitch", stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetPitch", stub_zero)?;
    table_set_rust_fn(state, metatable, "SetRoll", stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetRoll", stub_zero)?;
    table_set_rust_fn(state, metatable, "GetWorldScale", stub_one)?;
    table_set_rust_fn(
        state,
        metatable,
        "TransformCameraSpaceToModelSpace",
        stub_nil,
    )?;
    table_set_rust_fn(state, metatable, "UseModelCenterToTransform", stub_variadic)?;
    table_set_rust_fn(
        state,
        metatable,
        "IsUsingModelCenterToTransform",
        stub_false,
    )?;
    table_set_rust_fn(state, metatable, "SetViewTranslation", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetModelDrawLayer", stub_variadic)?;
    table_set_rust_fn(
        state,
        metatable,
        "SetAllowOverlappedModels",
        scene_set_allow_overlapped_models,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "IsAllowOverlappedModels",
        scene_is_allow_overlapped_models,
    )?;
    table_set_rust_fn(state, metatable, "SetViewInsets", scene_set_view_insets)?;
    table_set_rust_fn(state, metatable, "GetViewInsets", scene_get_view_insets)?;
    table_set_rust_fn(state, metatable, "ReplaceIconTexture", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetGlow", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetGradientMask", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetShadowEffect", stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetShadowEffect", stub_zero)?;
    table_set_rust_fn(state, metatable, "SetParticlesEnabled", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetUseGBuffer", stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetCustomCamera", stub_variadic)?;
    table_set_rust_fn(state, metatable, "MakeCurrentCameraCustom", stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetUpperEmblemTexture", stub_nil)?;
    table_set_rust_fn(state, metatable, "GetLowerEmblemTexture", stub_nil)?;
    Ok(())
}
