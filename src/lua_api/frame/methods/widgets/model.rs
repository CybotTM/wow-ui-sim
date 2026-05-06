//! Model and ModelScene widget methods (stubs + partial impl).

use super::shared::{opt_bool, val_to_f64};
use crate::lua_api::frame::methods::methods_hierarchy::reparent_widget;
use crate::lua_api::globals::create_frame::helpers_shared::create_frame_instance;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, frame_ref,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use crate::widget::WidgetType;
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

/// Records `displayID` on the actor handle, mirroring
/// `Actor:SetModelByCreatureDisplayID` from
/// `vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/ModelSceneActorMixin.lua`.
/// The optional `useCachedModelIfAvailable` flag is stored verbatim — the
/// simulator's 3D path is intentionally stubbed, so the flag never drives
/// a real renderer; tests still see the value the addon passed.
pub(super) fn set_model_by_creature_display_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let display_id = val_to_f64(stack_val(state, 2)) as i32;
    let use_cached = opt_bool(state, 3).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = None;
        f.model_file_id = None;
        f.model_appearance.display_info = Some(display_id);
        f.model_appearance.creature_id = None;
        f.model_appearance.use_cached_model = use_cached;
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

pub(super) fn get_display_info(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.model_appearance.display_info)
        .unwrap_or(0);
    (value as f64).into_stack(state)
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

pub(super) fn apply_spell_visual_kit(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let anim_kit = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.player_model_state.active_anim_kit = Some(anim_kit);
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

pub(super) fn set_shadow_effect(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let effect = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_rendering.shadow_effect = effect;
    }
    Ok(0)
}

pub(super) fn get_shadow_effect(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_rendering.shadow_effect as f64)
        .unwrap_or(0.0);
    v.into_stack(state)
}

pub(super) fn set_particles_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_rendering.particles_enabled = enabled;
    }
    Ok(0)
}

pub(super) fn set_use_gbuffer(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_rendering.use_gbuffer = enabled;
    }
    Ok(0)
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

pub(super) fn has_animation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let has_animation = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.model_appearance.animation_id)
        .is_some();
    state.push(Val::Bool(has_animation));
    Ok(1)
}

pub(super) fn refresh_unit(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.refresh_unit_count += 1;
    }
    Ok(0)
}

pub(super) fn refresh_camera(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.refresh_camera_count += 1;
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

pub(super) fn get_camera_roll(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_transform.camera.roll as f64)
        .unwrap_or(0.0);
    v.into_stack(state)
}

pub(super) fn set_camera_roll(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let roll = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.camera.roll = roll;
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

pub(super) fn stub_true(state: &mut LuaState) -> LuaResult<u32> {
    true.into_stack(state)
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

pub(super) fn scene_set_view_translation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = val_to_f64(stack_val(state, 2)) as f32;
    let y = val_to_f64(stack_val(state, 3)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.view_translation = (x, y);
    }
    Ok(0)
}

pub(super) fn scene_set_camera_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = val_to_f64(stack_val(state, 2)) as f32;
    let y = val_to_f64(stack_val(state, 3)) as f32;
    let z = val_to_f64(stack_val(state, 4)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.camera.position = (x, y, z);
    }
    Ok(0)
}

pub(super) fn scene_get_camera_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let pos = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.camera.position)
        .unwrap_or((0.0, 0.0, 0.0));
    (pos.0 as f64, pos.1 as f64, pos.2 as f64).into_stack(state)
}

pub(super) fn scene_set_camera_orientation_by_axis_vectors(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let forward = (
        val_to_f64(stack_val(state, 2)) as f32,
        val_to_f64(stack_val(state, 3)) as f32,
        val_to_f64(stack_val(state, 4)) as f32,
    );
    let right = (
        val_to_f64(stack_val(state, 5)) as f32,
        val_to_f64(stack_val(state, 6)) as f32,
        val_to_f64(stack_val(state, 7)) as f32,
    );
    let up = (
        val_to_f64(stack_val(state, 8)) as f32,
        val_to_f64(stack_val(state, 9)) as f32,
        val_to_f64(stack_val(state, 10)) as f32,
    );
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.camera.forward = forward;
        frame.model_scene_state.camera.right = right;
        frame.model_scene_state.camera.up = up;
    }
    Ok(0)
}

pub(super) fn scene_get_camera_forward(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.camera.forward)
        .unwrap_or((0.0, 0.0, 1.0));
    (value.0 as f64, value.1 as f64, value.2 as f64).into_stack(state)
}

pub(super) fn scene_get_camera_right(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.camera.right)
        .unwrap_or((1.0, 0.0, 0.0));
    (value.0 as f64, value.1 as f64, value.2 as f64).into_stack(state)
}

pub(super) fn scene_get_camera_up(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.camera.up)
        .unwrap_or((0.0, 1.0, 0.0));
    (value.0 as f64, value.1 as f64, value.2 as f64).into_stack(state)
}

pub(super) fn scene_set_camera_field_of_view(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.camera.field_of_view = value;
    }
    Ok(0)
}

pub(super) fn scene_get_camera_field_of_view(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.camera.field_of_view as f64)
        .unwrap_or(0.785);
    value.into_stack(state)
}

pub(super) fn scene_set_camera_near_clip(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.camera.near_clip = value;
    }
    Ok(0)
}

pub(super) fn scene_get_camera_near_clip(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.camera.near_clip as f64)
        .unwrap_or(1.0);
    value.into_stack(state)
}

pub(super) fn scene_set_camera_far_clip(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.camera.far_clip = value;
    }
    Ok(0)
}

pub(super) fn scene_get_camera_far_clip(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.camera.far_clip as f64)
        .unwrap_or(100.0);
    value.into_stack(state)
}

pub(super) fn scene_set_light_type(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.light.light_type = value;
    }
    Ok(0)
}

pub(super) fn scene_get_light_type(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.light.light_type as f64)
        .unwrap_or(0.0);
    value.into_stack(state)
}

pub(super) fn scene_set_light_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = (
        val_to_f64(stack_val(state, 2)) as f32,
        val_to_f64(stack_val(state, 3)) as f32,
        val_to_f64(stack_val(state, 4)) as f32,
    );
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.light.position = value;
    }
    Ok(0)
}

pub(super) fn scene_get_light_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.light.position)
        .unwrap_or((0.0, 0.0, 0.0));
    (value.0 as f64, value.1 as f64, value.2 as f64).into_stack(state)
}

pub(super) fn scene_set_light_direction(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = (
        val_to_f64(stack_val(state, 2)) as f32,
        val_to_f64(stack_val(state, 3)) as f32,
        val_to_f64(stack_val(state, 4)) as f32,
    );
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.light.direction = value;
    }
    Ok(0)
}

pub(super) fn scene_get_light_direction(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.light.direction)
        .unwrap_or((0.0, -1.0, 0.0));
    (value.0 as f64, value.1 as f64, value.2 as f64).into_stack(state)
}

fn color_arg(state: &LuaState, start: i32) -> crate::widget::Color {
    crate::widget::Color::rgb(
        val_to_f64(stack_val(state, start)) as f32,
        val_to_f64(stack_val(state, start + 1)) as f32,
        val_to_f64(stack_val(state, start + 2)) as f32,
    )
}

pub(super) fn scene_set_light_ambient_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = color_arg(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.light.ambient_color = value;
    }
    Ok(0)
}

pub(super) fn scene_get_light_ambient_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.light.ambient_color)
        .unwrap_or(crate::widget::Color::rgb(1.0, 1.0, 1.0));
    (value.r as f64, value.g as f64, value.b as f64).into_stack(state)
}

pub(super) fn scene_set_light_diffuse_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = color_arg(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.light.diffuse_color = value;
    }
    Ok(0)
}

pub(super) fn scene_get_light_diffuse_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.light.diffuse_color)
        .unwrap_or(crate::widget::Color::rgb(1.0, 1.0, 1.0));
    (value.r as f64, value.g as f64, value.b as f64).into_stack(state)
}

pub(super) fn scene_set_light_visible(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.light.visible = value;
    }
    Ok(0)
}

pub(super) fn scene_is_light_visible(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.light.visible)
        .unwrap_or(true);
    state.push(Val::Bool(value));
    Ok(1)
}

pub(super) fn scene_set_fog_near(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.fog.near = value;
    }
    Ok(0)
}

pub(super) fn scene_get_fog_near(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.fog.near as f64)
        .unwrap_or(0.0);
    value.into_stack(state)
}

pub(super) fn scene_set_fog_far(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.fog.far = value;
    }
    Ok(0)
}

pub(super) fn scene_get_fog_far(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.fog.far as f64)
        .unwrap_or(0.0);
    value.into_stack(state)
}

pub(super) fn scene_set_fog_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = color_arg(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.fog.color = value;
    }
    Ok(0)
}

pub(super) fn scene_get_fog_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.fog.color)
        .unwrap_or(crate::widget::Color::rgb(0.0, 0.0, 0.0));
    (value.r as f64, value.g as f64, value.b as f64).into_stack(state)
}

pub(super) fn scene_set_paused(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let paused = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.model_scene_state.paused = paused;
    }
    Ok(0)
}

pub(super) fn scene_get_paused(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let paused = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.model_scene_state.paused)
        .unwrap_or(false);
    state.push(Val::Bool(paused));
    Ok(1)
}

pub(super) fn scene_project_3d_point_to_2d(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let point = point3_from_stack(state);
    let Some(scene) = read_scene_projection_state(state, id)? else {
        state.push(Val::Nil);
        return Ok(1);
    };

    match project_scene_point(point, scene) {
        Some(projected) => projected.into_stack(state),
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

#[derive(Clone, Copy)]
struct SceneProjectionState {
    width: f32,
    height: f32,
    view_insets: (f32, f32, f32, f32),
    view_translation: (f32, f32),
    camera_position: (f32, f32, f32),
    camera_field_of_view: f32,
    camera_near_clip: f32,
}

fn point3_from_stack(state: &LuaState) -> (f32, f32, f32) {
    (
        val_to_f64(stack_val(state, 2)) as f32,
        val_to_f64(stack_val(state, 3)) as f32,
        val_to_f64(stack_val(state, 4)) as f32,
    )
}

fn read_scene_projection_state(
    state: &mut LuaState,
    id: u64,
) -> LuaResult<Option<SceneProjectionState>> {
    let sim = borrow_state(state)?;
    Ok(sim.widgets.get(id).map(|frame| {
        let camera = frame.model_scene_state.camera;
        SceneProjectionState {
            width: frame.width,
            height: frame.height,
            view_insets: frame.model_scene_state.view_insets,
            view_translation: frame.model_scene_state.view_translation,
            camera_position: camera.position,
            camera_field_of_view: camera.field_of_view,
            camera_near_clip: camera.near_clip,
        }
    }))
}

fn project_scene_point(
    point: (f32, f32, f32),
    scene: SceneProjectionState,
) -> Option<(f64, f64, f64)> {
    let depth = point.2 - scene.camera_position.2;
    if depth <= 0.0 {
        return None;
    }

    let (viewport_width, viewport_height) = projection_viewport_size(scene);
    let focal = (viewport_height * 0.5) / (scene.camera_field_of_view * 0.5).tan();
    let x = projected_x(point, scene, viewport_width, focal, depth);
    let y = projected_y(point, scene, viewport_height, focal, depth);
    let depth_value = 1.0 - (scene.camera_near_clip / depth.max(scene.camera_near_clip));

    Some((x as f64, y as f64, depth_value as f64))
}

fn projection_viewport_size(scene: SceneProjectionState) -> (f32, f32) {
    let viewport_width = (scene.width - scene.view_insets.0 - scene.view_insets.1).max(0.0);
    let viewport_height = (scene.height - scene.view_insets.2 - scene.view_insets.3).max(0.0);
    (viewport_width, viewport_height)
}

fn projected_x(
    point: (f32, f32, f32),
    scene: SceneProjectionState,
    viewport_width: f32,
    focal: f32,
    depth: f32,
) -> f32 {
    scene.view_insets.0
        + viewport_width * 0.5
        + scene.view_translation.0 / 6.0
        + (point.0 - scene.camera_position.0) * focal / depth
}

fn projected_y(
    point: (f32, f32, f32),
    scene: SceneProjectionState,
    viewport_height: f32,
    focal: f32,
    depth: f32,
) -> f32 {
    scene.view_insets.2
        + viewport_height * 0.5
        + scene.view_translation.1 * 6.0
        + (point.1 - scene.camera_position.1) * focal / depth
}

pub(super) fn scene_create_actor(state: &mut LuaState) -> LuaResult<u32> {
    let scene_id = frame_id_from_stack(state, 1)?;
    let name = super::shared::opt_string(state, 2);
    let tag = name.clone();
    let actor_id = create_frame_instance(
        state,
        WidgetType::Frame,
        "ModelSceneActor",
        name,
        Some(scene_id),
        true,
        None,
    )?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(scene) = sim.widgets.get_mut_visual(scene_id) {
        scene.model_scene_actor_ids.push(actor_id);
        if let Some(tag) = tag.filter(|t| !t.is_empty()) {
            scene
                .model_scene_actor_tags
                .retain(|(existing, _)| existing != &tag);
            scene.model_scene_actor_tags.push((tag, actor_id));
        }
    }
    drop(sim);
    let actor = frame_ref(state, actor_id)?;
    state.push(actor);
    Ok(1)
}

/// Returns the actor whose script tag matches `tag`, mirroring
/// `ModelSceneMixin:GetActorByTag`
/// (`vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/ModelSceneMixin.lua:136`).
/// AlliedRacesFrameMixin:UpdateModel relies on this lookup to attach
/// the per-race actor before re-applying the creature display ID.
pub(super) fn scene_get_actor_by_tag(state: &mut LuaState) -> LuaResult<u32> {
    let scene_id = frame_id_from_stack(state, 1)?;
    let Some(tag) = super::shared::opt_string(state, 2) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let actor_id = borrow_state(state)?
        .widgets
        .get(scene_id)
        .and_then(|scene| {
            scene
                .model_scene_actor_tags
                .iter()
                .find(|(existing, _)| existing == &tag)
                .map(|(_, id)| *id)
        });
    if let Some(actor_id) = actor_id {
        let actor = frame_ref(state, actor_id)?;
        state.push(actor);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

pub(super) fn scene_get_num_actors(state: &mut LuaState) -> LuaResult<u32> {
    let scene_id = frame_id_from_stack(state, 1)?;
    let count = borrow_state(state)?
        .widgets
        .get(scene_id)
        .map(|scene| scene.model_scene_actor_ids.len() as f64)
        .unwrap_or(0.0);
    count.into_stack(state)
}

pub(super) fn scene_get_actor_at_index(state: &mut LuaState) -> LuaResult<u32> {
    let scene_id = frame_id_from_stack(state, 1)?;
    let index = val_to_f64(stack_val(state, 2)) as usize;
    let actor_id = borrow_state(state)?
        .widgets
        .get(scene_id)
        .and_then(|scene| {
            scene
                .model_scene_actor_ids
                .get(index.saturating_sub(1))
                .copied()
        });
    if let Some(actor_id) = actor_id {
        let actor = frame_ref(state, actor_id)?;
        state.push(actor);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

pub(super) fn scene_take_actor(state: &mut LuaState) -> LuaResult<u32> {
    let scene_id = frame_id_from_stack(state, 1)?;
    let actor_id = {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.get_mut_visual(scene_id).and_then(|scene| {
            let popped = scene.model_scene_actor_ids.pop()?;
            scene.model_scene_actor_tags.retain(|(_, id)| *id != popped);
            Some(popped)
        })
    };
    if let Some(actor_id) = actor_id {
        let mut sim = borrow_state_mut(state)?;
        reparent_widget(&mut sim.widgets, actor_id, None);
        drop(sim);
        let actor = frame_ref(state, actor_id)?;
        state.push(actor);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

/// Drops every actor from the scene's actor pool, mirroring
/// `ModelSceneMixin:ClearScene` /
/// `ModelSceneMixin:ReleaseAllActors`
/// (`vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/ModelSceneMixin.lua:16,217`).
/// Drained actors are reparented away from the scene so the scene's
/// `children` list and `model_scene_actor_ids` agree, matching what
/// `scene_take_actor` does for a single actor.
pub(super) fn scene_clear_scene(state: &mut LuaState) -> LuaResult<u32> {
    let scene_id = frame_id_from_stack(state, 1)?;
    let actor_ids: Vec<u64> = {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets
            .get_mut_visual(scene_id)
            .map(|scene| {
                scene.model_scene_actor_tags.clear();
                std::mem::take(&mut scene.model_scene_actor_ids)
            })
            .unwrap_or_default()
    };
    let mut sim = borrow_state_mut(state)?;
    for actor_id in actor_ids {
        reparent_widget(&mut sim.widgets, actor_id, None);
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

/// Rebuilds the scene's actor pool from the static `modelSceneID` manifest
/// (`SimState::model_scenes`), mirroring
/// `ModelSceneMixin:TransitionToModelSceneID`
/// (`vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/ModelSceneMixin.lua:71`).
///
/// Real WoW reads `C_ModelInfo.GetModelSceneInfoByID(modelSceneID)` and
/// re-creates one actor per declared row. We carry the same per-scene tag
/// list keyed on `modelSceneID`, so `ClearScene + Transition + GetActorByTag`
/// rounds-trips for `Blizzard_AlliedRacesFrameUI:UpdateModel`. Visual
/// transitions (camera pan, fade) are out of scope — actor pool + the
/// `self.modelSceneID` book-keeping is enough for tag lookup, the only
/// observable contract addons rely on.
///
/// We deliberately do not invoke `self.resetCallback` here. The Blizzard
/// mixin only fires the callback from `:Reset()`, and AlliedRaces' callback
/// (`AlliedRacesFrameMixin.OnModelSceneReset`) calls back into
/// `UpdateModel → TransitionToModelSceneID`, which would loop forever.
pub(super) fn scene_transition_to_model_scene_id(state: &mut LuaState) -> LuaResult<u32> {
    let scene_id = frame_id_from_stack(state, 1)?;
    let Val::Table(scene_ref) = stack_val(state, 1) else {
        return Ok(0);
    };
    let target_scene_id = val_to_f64(stack_val(state, 2)) as i64;
    let force = opt_bool(state, 5).unwrap_or(false);

    let tags = lookup_model_scene_actor_tags(state, target_scene_id)?;
    let Some(tags) = tags else { return Ok(0) };

    if !force && current_model_scene_id_matches(state, scene_ref, target_scene_id) {
        return Ok(0);
    }

    drain_existing_actors(state, scene_id)?;
    rebuild_actor_pool(state, scene_id, &tags)?;
    write_model_scene_id(state, scene_ref, target_scene_id);
    Ok(0)
}

fn lookup_model_scene_actor_tags(
    state: &mut LuaState,
    scene_id: i64,
) -> LuaResult<Option<Vec<String>>> {
    let sim = borrow_state(state)?;
    let tags = sim
        .model_scenes
        .get(&scene_id)
        .filter(|t| !t.is_empty())
        .cloned();
    Ok(tags)
}

fn current_model_scene_id_matches(
    state: &mut LuaState,
    scene_ref: GcRef<Table>,
    target_scene_id: i64,
) -> bool {
    let key = state.gc.intern_string(b"modelSceneID");
    let Some(scene_table) = state.gc.tables.get(scene_ref) else {
        return false;
    };
    matches!(
        scene_table.get_str(key, &state.gc.string_arena),
        Val::Num(n) if (n as i64) == target_scene_id
    )
}

fn drain_existing_actors(state: &mut LuaState, scene_id: u64) -> LuaResult<()> {
    let actor_ids: Vec<u64> = {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets
            .get_mut_visual(scene_id)
            .map(|scene| {
                scene.model_scene_actor_tags.clear();
                std::mem::take(&mut scene.model_scene_actor_ids)
            })
            .unwrap_or_default()
    };
    let mut sim = borrow_state_mut(state)?;
    for actor_id in actor_ids {
        reparent_widget(&mut sim.widgets, actor_id, None);
    }
    Ok(())
}

fn rebuild_actor_pool(state: &mut LuaState, scene_id: u64, tags: &[String]) -> LuaResult<()> {
    for tag in tags {
        let actor_id = create_frame_instance(
            state,
            WidgetType::Frame,
            "ModelSceneActor",
            Some(tag.clone()),
            Some(scene_id),
            true,
            None,
        )?;
        let mut sim = borrow_state_mut(state)?;
        if let Some(scene) = sim.widgets.get_mut_visual(scene_id) {
            scene.model_scene_actor_ids.push(actor_id);
            scene.model_scene_actor_tags.push((tag.clone(), actor_id));
        }
    }
    Ok(())
}

fn write_model_scene_id(state: &mut LuaState, scene_ref: GcRef<Table>, scene_id: i64) {
    let key = state.gc.intern_string(b"modelSceneID");
    if let Some(t) = state.gc.tables.get_mut(scene_ref) {
        let _ = t.raw_set(
            Val::Str(key),
            Val::Num(scene_id as f64),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(scene_ref);
}

/// Stores a Lua callback at `self.resetCallback`, mirroring
/// `ModelSceneMixin:SetResetCallback`
/// (`vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/ModelSceneMixin.lua:56`).
/// `ModelSceneMixin:Reset()` reads `self.resetCallback` and invokes it
/// with the scene as the only argument, so writing through the frame
/// metatable (same path `self.resetCallback = cb` would take) keeps both
/// the widget method and the inherited Blizzard `Reset()` consistent.
/// Non-function / non-nil arguments are ignored to match the mixin
/// contract (any value other than a callable would crash `Reset()`).
pub(super) fn scene_set_reset_callback(state: &mut LuaState) -> LuaResult<u32> {
    let frame_val = stack_val(state, 1);
    let Val::Table(frame_ref) = frame_val else {
        return Ok(0);
    };
    let callback = stack_val(state, 2);
    if !matches!(callback, Val::Function(_) | Val::Nil) {
        return Ok(0);
    }
    let key = state.gc.intern_string(b"resetCallback");
    if let Some(t) = state.gc.tables.get_mut(frame_ref) {
        let _ = t.raw_set(Val::Str(key), callback, &state.gc.string_arena);
    }
    state.gc.barrier_back(frame_ref);
    Ok(0)
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
    ("ApplySpellVisualKit", apply_spell_visual_kit),
    ("SetDisplayInfo", set_display_info),
    ("GetDisplayInfo", get_display_info),
    (
        "SetModelByCreatureDisplayID",
        set_model_by_creature_display_id,
    ),
    ("SetCreature", set_creature),
    ("ClearModel", clear_model),
    ("GetModelFileID", get_model_file_id),
    ("SetModelAlpha", set_model_alpha),
    ("GetModelAlpha", get_model_alpha),
    ("SetShadowEffect", set_shadow_effect),
    ("GetShadowEffect", get_shadow_effect),
    ("SetParticlesEnabled", set_particles_enabled),
    ("SetUseGBuffer", set_use_gbuffer),
    ("SetDoBlend", set_do_blend),
    ("GetDoBlend", get_do_blend),
    ("SetKeepModelOnHide", set_keep_model_on_hide),
    ("GetKeepModelOnHide", get_keep_model_on_hide),
    ("SetItem", set_item),
    ("SetItemAppearance", set_item_appearance),
    ("PlayAnimKit", play_anim_kit),
    ("StopAnimKit", stop_anim_kit),
    ("CanSetUnit", can_set_unit),
    ("HasAnimation", has_animation),
    ("SetSequence", set_sequence),
    ("SetSequenceTime", set_sequence_time),
    // Camera
    ("GetCameraDistance", get_camera_distance),
    ("SetCameraDistance", set_camera_distance),
    ("GetCameraFacing", get_camera_facing),
    ("SetCameraFacing", set_camera_facing),
    ("GetCameraTarget", get_camera_target),
    ("SetCameraTarget", set_camera_target),
    ("GetCameraRoll", get_camera_roll),
    ("SetCameraRoll", set_camera_roll),
    // Variadic no-op stubs — 3D rendering is intentionally out of scope
    ("SetAutoDress", stub_variadic),
    ("SetCamDistanceScale", stub_variadic),
    ("SetCamera", stub_variadic),
    ("SetPortraitZoom", stub_variadic),
    ("SetLight", stub_variadic),
    ("ResetLights", stub_variadic),
    ("RefreshUnit", refresh_unit),
    ("RefreshCamera", refresh_camera),
    (
        "TransitionToModelSceneID",
        scene_transition_to_model_scene_id,
    ),
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
    ("SetCustomCamera", stub_variadic),
    ("MakeCurrentCameraCustom", stub_variadic),
    // DressUpModel / transmog wardrobe — no real 3D, just absorb the calls
    ("SetUseTransmogSkin", stub_variadic),
    ("SetUseTransmogChoices", stub_variadic),
    ("SetObeyHideInTransmogFlag", stub_variadic),
    ("TryOn", stub_variadic),
    ("UndressSlot", stub_variadic),
    ("Undress", stub_variadic),
    ("SetUnit", stub_variadic),
    ("UpdateCamera", stub_variadic),
    ("FreezeAnimation", stub_variadic),
    // Typed return stubs
    ("GetModelSceneID", stub_zero),
    ("GetCamDistanceScale", stub_one),
    ("HasCustomCamera", stub_false),
    ("HasAttachmentPoints", stub_false),
    ("GetLight", stub_nil),
    ("GetPitch", stub_zero),
    ("GetRoll", stub_zero),
    ("GetWorldScale", stub_one),
    ("TransformCameraSpaceToModelSpace", stub_nil),
    ("IsUsingModelCenterToTransform", stub_false),
    ("GetUpperEmblemTexture", stub_nil),
    ("GetLowerEmblemTexture", stub_nil),
    // Wardrobe gates appearance enumeration on these — must be true so
    // the items list populates and the geometry-ready code path runs.
    ("IsSlotAllowed", stub_true),
    ("IsGeoReady", stub_true),
    ("HasTrackableSource", stub_false),
    // ModelScene-specific (round-tripped state)
    ("SetCameraPosition", scene_set_camera_position),
    ("GetCameraPosition", scene_get_camera_position),
    (
        "SetCameraOrientationByAxisVectors",
        scene_set_camera_orientation_by_axis_vectors,
    ),
    ("GetCameraForward", scene_get_camera_forward),
    ("GetCameraRight", scene_get_camera_right),
    ("GetCameraUp", scene_get_camera_up),
    ("SetCameraFieldOfView", scene_set_camera_field_of_view),
    ("GetCameraFieldOfView", scene_get_camera_field_of_view),
    ("SetCameraNearClip", scene_set_camera_near_clip),
    ("GetCameraNearClip", scene_get_camera_near_clip),
    ("SetCameraFarClip", scene_set_camera_far_clip),
    ("GetCameraFarClip", scene_get_camera_far_clip),
    ("SetLightType", scene_set_light_type),
    ("GetLightType", scene_get_light_type),
    ("SetLightPosition", scene_set_light_position),
    ("GetLightPosition", scene_get_light_position),
    ("SetLightDirection", scene_set_light_direction),
    ("GetLightDirection", scene_get_light_direction),
    ("SetLightAmbientColor", scene_set_light_ambient_color),
    ("GetLightAmbientColor", scene_get_light_ambient_color),
    ("SetLightDiffuseColor", scene_set_light_diffuse_color),
    ("GetLightDiffuseColor", scene_get_light_diffuse_color),
    ("SetLightVisible", scene_set_light_visible),
    ("IsLightVisible", scene_is_light_visible),
    ("SetFogNear", scene_set_fog_near),
    ("GetFogNear", scene_get_fog_near),
    ("SetFogFar", scene_set_fog_far),
    ("GetFogFar", scene_get_fog_far),
    ("SetFogColor", scene_set_fog_color),
    ("GetFogColor", scene_get_fog_color),
    ("SetPaused", scene_set_paused),
    ("GetPaused", scene_get_paused),
    (
        "SetAllowOverlappedModels",
        scene_set_allow_overlapped_models,
    ),
    ("IsAllowOverlappedModels", scene_is_allow_overlapped_models),
    ("SetViewInsets", scene_set_view_insets),
    ("GetViewInsets", scene_get_view_insets),
    ("SetViewTranslation", scene_set_view_translation),
    ("Project3DPointTo2D", scene_project_3d_point_to_2d),
    ("CreateActor", scene_create_actor),
    ("GetNumActors", scene_get_num_actors),
    ("GetActorAtIndex", scene_get_actor_at_index),
    ("GetActorByTag", scene_get_actor_by_tag),
    ("TakeActor", scene_take_actor),
    ("SetResetCallback", scene_set_reset_callback),
    ("ClearScene", scene_clear_scene),
];

pub(super) fn register_model(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in MODEL_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
