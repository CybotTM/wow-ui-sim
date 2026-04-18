//! Model and ModelScene widget methods (stubs + partial impl).

use super::shared::{opt_bool, val_to_f64};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, frame_id_from_stack};
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

pub(super) fn set_do_blend(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.player_model_state.do_blend = enabled;
    }
    Ok(0)
}

pub(super) fn get_do_blend(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.player_model_state.do_blend)
        .unwrap_or(false);
    state.push(Val::Bool(enabled));
    Ok(1)
}

pub(super) fn set_keep_model_on_hide(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let keep = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.player_model_state.keep_model_on_hide = keep;
    }
    Ok(0)
}

pub(super) fn get_keep_model_on_hide(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let keep = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.player_model_state.keep_model_on_hide)
        .unwrap_or(false);
    state.push(Val::Bool(keep));
    Ok(1)
}

pub(super) fn set_item(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let item = stringish_arg(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.player_model_state.last_item = item;
    }
    Ok(0)
}

pub(super) fn set_item_appearance(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let appearance = stringish_arg(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.player_model_state.last_item_appearance = appearance;
    }
    Ok(0)
}

pub(super) fn play_anim_kit(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let anim_kit = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.player_model_state.active_anim_kit = Some(anim_kit);
    }
    Ok(0)
}

pub(super) fn stop_anim_kit(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.player_model_state.active_anim_kit = None;
    }
    Ok(0)
}

pub(super) fn can_set_unit(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn stringish_arg(state: &LuaState, index: i32) -> Option<String> {
    match stack_val(state, index) {
        Val::Str(str_ref) => {
            let lua_str = state.gc.string_arena.get(str_ref)?;
            String::from_utf8(lua_str.data().to_vec()).ok()
        }
        Val::Num(n) if n.fract() == 0.0 => Some((n as i64).to_string()),
        Val::Num(n) => Some(n.to_string()),
        _ => None,
    }
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

const MODEL_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    // Model source + transform
    ("SetModel", set_model),
    ("GetModel", get_model),
    ("SetModelScale", set_model_scale),
    ("GetModelScale", get_model_scale),
    ("SetPosition", set_position),
    ("GetPosition", get_position),
    ("SetFacing", set_facing),
    ("GetFacing", get_facing),
    ("SetRotation", set_rotation),
    // Animation / display info
    ("SetAnimation", set_animation),
    ("SetDisplayInfo", set_display_info),
    ("SetCreature", set_creature),
    ("ClearModel", clear_model),
    ("GetModelFileID", get_model_file_id),
    ("SetModelAlpha", set_model_alpha),
    ("GetModelAlpha", get_model_alpha),
    ("SetDoBlend", set_do_blend),
    ("GetDoBlend", get_do_blend),
    ("SetKeepModelOnHide", set_keep_model_on_hide),
    ("GetKeepModelOnHide", get_keep_model_on_hide),
    ("SetItem", set_item),
    ("SetItemAppearance", set_item_appearance),
    ("PlayAnimKit", play_anim_kit),
    ("StopAnimKit", stop_anim_kit),
    ("CanSetUnit", can_set_unit),
    ("SetSequence", set_sequence),
    ("SetSequenceTime", set_sequence_time),
    // Camera
    ("GetCameraDistance", get_camera_distance),
    ("SetCameraDistance", set_camera_distance),
    ("GetCameraFacing", get_camera_facing),
    ("SetCameraFacing", set_camera_facing),
    ("GetCameraTarget", get_camera_target),
    ("SetCameraTarget", set_camera_target),
    // Variadic no-op stubs — 3D rendering is intentionally out of scope
    ("SetAutoDress", stub_variadic),
    ("SetCamDistanceScale", stub_variadic),
    ("SetCamera", stub_variadic),
    ("SetPortraitZoom", stub_variadic),
    ("SetDesaturation", stub_variadic),
    ("SetLight", stub_variadic),
    ("ResetLights", stub_variadic),
    ("RefreshUnit", stub_variadic),
    ("RefreshCamera", stub_variadic),
    ("TransitionToModelSceneID", stub_variadic),
    ("SetFromModelSceneID", stub_variadic),
    ("CycleVariation", stub_variadic),
    ("AdvanceTime", stub_variadic),
    ("ClearTransform", stub_variadic),
    ("SetTransform", stub_variadic),
    ("SetPitch", stub_variadic),
    ("SetRoll", stub_variadic),
    ("UseModelCenterToTransform", stub_variadic),
    ("SetViewTranslation", stub_variadic),
    ("SetModelDrawLayer", stub_variadic),
    ("ReplaceIconTexture", stub_variadic),
    ("SetGlow", stub_variadic),
    ("SetGradientMask", stub_variadic),
    ("SetShadowEffect", stub_variadic),
    ("SetParticlesEnabled", stub_variadic),
    ("SetUseGBuffer", stub_variadic),
    ("SetCustomCamera", stub_variadic),
    ("MakeCurrentCameraCustom", stub_variadic),
    // Typed return stubs
    ("GetModelSceneID", stub_zero),
    ("GetCamDistanceScale", stub_one),
    ("HasCustomCamera", stub_false),
    ("GetPaused", stub_false),
    ("HasAttachmentPoints", stub_false),
    ("GetLight", stub_nil),
    ("GetPitch", stub_zero),
    ("GetRoll", stub_zero),
    ("GetWorldScale", stub_one),
    ("TransformCameraSpaceToModelSpace", stub_nil),
    ("IsUsingModelCenterToTransform", stub_false),
    ("GetShadowEffect", stub_zero),
    ("GetUpperEmblemTexture", stub_nil),
    ("GetLowerEmblemTexture", stub_nil),
    // ModelScene-specific (round-tripped state)
    (
        "SetAllowOverlappedModels",
        scene_set_allow_overlapped_models,
    ),
    ("IsAllowOverlappedModels", scene_is_allow_overlapped_models),
    ("SetViewInsets", scene_set_view_insets),
    ("GetViewInsets", scene_get_view_insets),
];

pub(super) fn register_model(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in MODEL_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
