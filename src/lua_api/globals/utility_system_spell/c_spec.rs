//! C_SpecializationInfo and UIWidgetContainerMixin implementations.

use crate::lua_api::methods::{
    borrow_state, create_string, create_table, frame_id_from_stack,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn};
use crate::specializations;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::set_global_val;

pub fn register_c_specialization_info(state: &mut LuaState) -> LuaResult<()> {
    let t = create_table(state);
    let Val::Table(t_ref) = t else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(state, t_ref, "GetSpecialization", c_spec_get_specialization)?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetSpecializationInfo",
        c_spec_get_specialization_info,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetClassIDFromSpecID",
        c_spec_get_class_id_from_spec_id,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetNumSpecializationsForClassID",
        c_spec_get_num_specializations_for_class_id,
    )?;
    set_global_val(state, "C_SpecializationInfo", t);
    Ok(())
}

pub fn register_widget_container_mixin(state: &mut LuaState) -> LuaResult<()> {
    let mixin = create_table(state);
    let Val::Table(mixin_ref) = mixin else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        mixin_ref,
        "GetNumWidgetsShowing",
        ui_widget_container_get_num_widgets_showing,
    )?;
    let key_ref = state.gc.intern_string(b"UIWidgetContainerMixin");
    let global_ref = state.global;
    if let Some(global) = state.gc.tables.get_mut(global_ref) {
        let _ = global.raw_set(Val::Str(key_ref), mixin, &state.gc.string_arena);
    }
    state.gc.barrier_back(global_ref);
    Ok(())
}

fn c_spec_get_specialization(state: &mut LuaState) -> LuaResult<u32> {
    let active_spec_index = borrow_state(state)?.player.active_spec_index;
    state.push(Val::Num(active_spec_index as f64));
    Ok(1)
}

fn c_spec_get_specialization_info(state: &mut LuaState) -> LuaResult<u32> {
    let requested_index = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 1,
    };
    let (class_id, active_spec_index) = {
        let sim = borrow_state(state)?;
        (sim.player.class_index as u32, sim.player.active_spec_index)
    };
    let fallback = requested_index.max(1);
    let spec = specializations::specs_for_class(class_id)
        .nth((fallback - 1) as usize)
        .or_else(|| {
            let active = active_spec_index.max(1);
            specializations::specs_for_class(class_id).nth((active - 1) as usize)
        });
    let Some(spec) = spec else {
        return Ok(0);
    };
    let spec_name = create_string(state, spec.name);
    let spec_description = create_string(state, spec.description);
    let spec_role = create_string(state, spec.role);
    state.push(Val::Num(spec.id as f64));
    state.push(spec_name);
    state.push(spec_description);
    state.push(Val::Num(spec.icon_file_data_id as f64));
    state.push(spec_role);
    state.push(Val::Num(spec.primary_stat as f64));
    Ok(6)
}

fn c_spec_get_class_id_from_spec_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = match stack_val(state, 1) {
        Val::Num(n) => n as u32,
        _ => 0,
    };
    let class_id = specializations::spec_by_id(spec_id)
        .map(|spec| spec.class_id as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(class_id));
    Ok(1)
}

fn c_spec_get_num_specializations_for_class_id(state: &mut LuaState) -> LuaResult<u32> {
    let class_id = match stack_val(state, 1) {
        Val::Num(n) => n as u32,
        _ => 0,
    };
    let count = specializations::specs_for_class(class_id).count() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

pub fn player_get_timerunning_season_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = borrow_state(state)?.timerunning_season_id.unwrap_or(0);
    state.push(Val::Num(id as f64));
    Ok(1)
}

pub fn player_is_timerunning(state: &mut LuaState) -> LuaResult<u32> {
    let active = borrow_state(state)?.timerunning_season_id.is_some();
    state.push(Val::Bool(active));
    Ok(1)
}

fn ui_widget_container_get_num_widgets_showing(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(frame_id)
            .map(|frame| {
                frame
                    .children
                    .iter()
                    .filter(|&&child_id| {
                        sim.widgets
                            .get(child_id)
                            .map(|child| child.visible)
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0) as f64
    };
    state.push(Val::Num(count));
    Ok(1)
}
