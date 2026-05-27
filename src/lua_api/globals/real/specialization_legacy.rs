//! Legacy specialization globals.
//!
//! These functions predate `C_SpecializationInfo` and are published as plain
//! globals by Blizzard compatibility code. Keep them out of `c_api` so the
//! C API boundary contains only `C_*` namespace surfaces.

use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::globals::real::specialization_helpers::push_specialization_identity;
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use crate::specializations;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

type RustLuaFn = rilua::vm::closure::RustFn;

const CLASS_FILES: &[&str] = &[
    "WARRIOR",
    "PALADIN",
    "HUNTER",
    "ROGUE",
    "PRIEST",
    "DEATHKNIGHT",
    "SHAMAN",
    "MAGE",
    "WARLOCK",
    "MONK",
    "DRUID",
    "DEMONHUNTER",
    "EVOKER",
];

const LEGACY_SPECIALIZATION_GLOBALS: &[(&str, RustLuaFn)] = &[
    ("GetNumSpecGroups", get_num_spec_groups),
    ("GetNumSpecializations", get_num_specializations),
    ("GetSpecializationInfoByID", get_specialization_info_by_id),
    (
        "GetSpecializationInfoForClassID",
        get_specialization_info_for_class_id,
    ),
    ("GetSpecializationRole", get_specialization_role),
    ("GetSpecializationRoleEnum", get_specialization_role_enum),
    (
        "GetSpecializationRoleEnumByID",
        get_specialization_role_enum_by_id,
    ),
    ("GetLFGStringFromEnum", get_lfg_string_from_enum),
];

pub(crate) fn register_legacy_specialization_globals(state: &mut LuaState) -> LuaResult<()> {
    for (name, rust_fn) in LEGACY_SPECIALIZATION_GLOBALS {
        table_set_rust_fn_static(state, state.global, name, *rust_fn)?;
    }
    Ok(())
}

fn get_num_spec_groups(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    Ok(1)
}

fn get_num_specializations(state: &mut LuaState) -> LuaResult<u32> {
    let class_id = borrow_state(state)?.player.class_index.max(1) as u32;
    let count = specializations::specs_for_class(class_id).count() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn get_specialization_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = match stack_val(state, 1) {
        Val::Num(n) => n as u32,
        _ => 0,
    };
    let Some(spec) = specializations::spec_by_id(spec_id) else {
        return Ok(0);
    };
    let class_index = spec.class_id.max(1) as usize - 1;
    let class_name = create_string(state, CLASS_LABELS.get(class_index).copied().unwrap_or(""));
    let class_file = create_string(state, CLASS_FILES.get(class_index).copied().unwrap_or(""));
    let spec_name = create_string(state, spec.name);
    let spec_description = create_string(state, spec.description);
    let spec_role = create_string(state, spec.role);
    state.push(Val::Num(spec.id as f64));
    state.push(spec_name);
    state.push(spec_description);
    state.push(Val::Num(spec.icon_file_data_id as f64));
    state.push(spec_role);
    state.push(class_file);
    state.push(class_name);
    Ok(7)
}

fn get_specialization_info_for_class_id(state: &mut LuaState) -> LuaResult<u32> {
    let class_id = match stack_val(state, 1) {
        Val::Num(n) => n as u32,
        _ => 0,
    };
    let spec_index = match stack_val(state, 2) {
        Val::Num(n) if n >= 1.0 => n as usize,
        _ => return Ok(0),
    };
    let Some(spec) = specializations::specs_for_class(class_id).nth(spec_index - 1) else {
        return Ok(0);
    };

    push_class_specialization_info(state, spec);
    Ok(9)
}

fn get_specialization_role(state: &mut LuaState) -> LuaResult<u32> {
    let Some(role) = requested_spec_role(state) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let role = create_string(state, role);
    state.push(role);
    Ok(1)
}

fn get_specialization_role_enum(state: &mut LuaState) -> LuaResult<u32> {
    let Some(role) = requested_spec_role(state) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Num(role_enum_value(role)));
    Ok(1)
}

fn get_specialization_role_enum_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = match stack_val(state, 1) {
        Val::Num(n) => n as u32,
        _ => 0,
    };
    let Some(spec) = specializations::spec_by_id(spec_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Num(role_enum_value(spec.role)));
    Ok(1)
}

fn get_lfg_string_from_enum(state: &mut LuaState) -> LuaResult<u32> {
    let role = match stack_val(state, 1) {
        Val::Num(0.0) => "TANK",
        Val::Num(1.0) => "HEALER",
        Val::Num(_) => "DAMAGER",
        _ => "",
    };
    let role = create_string(state, role);
    state.push(role);
    Ok(1)
}

fn requested_spec_role(state: &LuaState) -> Option<&'static str> {
    let requested_index = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 1,
    };
    requested_or_active_spec(state, requested_index).map(|spec| spec.role)
}

fn requested_or_active_spec(
    state: &LuaState,
    requested_index: i32,
) -> Option<&'static specializations::SpecInfo> {
    let (class_id, active_spec_index) = {
        let sim = borrow_state(state).ok()?;
        (
            sim.player.class_index.max(1) as u32,
            sim.player.active_spec_index,
        )
    };
    let requested_spec_index = requested_index.max(1);
    specializations::specs_for_class(class_id)
        .nth((requested_spec_index - 1) as usize)
        .or_else(|| {
            let active_spec_index = active_spec_index.max(1);
            specializations::specs_for_class(class_id).nth((active_spec_index - 1) as usize)
        })
}

fn push_class_specialization_info(state: &mut LuaState, spec: &specializations::SpecInfo) {
    push_specialization_identity(state, spec);
    state.push(Val::Bool(false));
    state.push(Val::Bool(true));
    state.push(Val::Nil);
    state.push(Val::Nil);
}

fn role_enum_value(role: &str) -> f64 {
    match role {
        "TANK" => 0.0,
        "HEALER" => 1.0,
        _ => 2.0,
    }
}
