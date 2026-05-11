//! `C_SpecializationInfo` implementation.

use crate::lua_api::game_data::CastingState;
use crate::lua_api::globals::real::specialization_helpers::push_specialization_identity;
use crate::lua_api::globals::spellbook_data;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use crate::specializations;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

use super::helpers::set_global_val;

type LuaTableRef = GcRef<Table>;
type RustLuaFn = rilua::vm::closure::RustFn;

const C_SPECIALIZATION_INFO_METHODS: &[(&str, RustLuaFn)] = &[
    ("GetSpecialization", c_spec_get_specialization),
    ("GetSpecializationInfo", c_spec_get_specialization_info),
    ("GetClassIDFromSpecID", c_spec_get_class_id_from_spec_id),
    (
        "GetNumSpecializationsForClassID",
        c_spec_get_num_specializations_for_class_id,
    ),
    ("IsInitialized", c_spec_is_initialized),
    (
        "CanPlayerUseTalentSpecUI",
        c_spec_can_player_use_talent_spec_ui,
    ),
    ("CanPlayerUseTalentUI", c_spec_can_player_use_talent_ui),
    ("GetActiveSpecGroup", c_spec_get_active_spec_group),
    #[cfg(feature = "client-mists")]
    ("GetTalentInfo", c_spec_get_talent_info),
    ("GetSpellsDisplay", c_spec_get_spells_display),
    ("GetSpecIDs", c_spec_get_spec_ids),
    ("SetSpecialization", c_spec_set_specialization),
];

const SPEC_ACTIVATION_SPELL_ID: u32 = 200749;
const SPEC_ACTIVATION_CAST_SECONDS: f64 = 1.5;

pub fn register_c_specialization_info(state: &mut LuaState) -> LuaResult<()> {
    let t = create_table(state);
    let Val::Table(t_ref) = t else {
        unreachable!("create_table must return a table");
    };
    register_c_specialization_info_methods(state, t_ref)?;
    set_global_val(state, "C_SpecializationInfo", t);
    Ok(())
}

fn register_c_specialization_info_methods(
    state: &mut LuaState,
    table_ref: LuaTableRef,
) -> LuaResult<()> {
    for (name, rust_fn) in C_SPECIALIZATION_INFO_METHODS {
        table_set_rust_fn_static(state, table_ref, name, *rust_fn)?;
    }
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
    let spec = requested_or_active_spec(state, requested_index);
    let Some(spec) = spec else {
        return Ok(0);
    };
    push_specialization_info(state, spec);
    Ok(10)
}

fn c_spec_get_spec_ids(state: &mut LuaState) -> LuaResult<u32> {
    let class_id = match stack_val(state, 1) {
        Val::Num(n) if n > 0.0 => n as u32,
        _ => borrow_state(state)?.player.class_index as u32,
    };
    let spec_ids = create_table(state);
    let Val::Table(spec_ids_ref) = spec_ids else {
        unreachable!("create_table must return a table");
    };
    for (index, spec) in specializations::specs_for_class(class_id).enumerate() {
        if let Some(table) = state.gc.tables.get_mut(spec_ids_ref) {
            let _ = table.raw_set(
                Val::Num((index + 1) as f64),
                Val::Num(spec.id as f64),
                &state.gc.string_arena,
            );
        }
    }
    state.gc.barrier_back(spec_ids_ref);
    state.push(spec_ids);
    Ok(1)
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

fn c_spec_is_initialized(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_spec_can_player_use_talent_spec_ui(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Nil);
    Ok(2)
}

fn c_spec_can_player_use_talent_ui(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Nil);
    Ok(2)
}

fn c_spec_get_active_spec_group(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    Ok(1)
}

#[cfg(feature = "client-mists")]
fn c_spec_get_talent_info(state: &mut LuaState) -> LuaResult<u32> {
    super::mists_talents::get_talent_info(state)
}

fn c_spec_get_spells_display(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let Some(spell_ids) = spec_display_spell_ids(spec_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let spells = create_table(state);
    let Val::Table(spells_ref) = spells else {
        unreachable!("create_table must return a table");
    };
    for (index, spell_id) in spell_ids.iter().copied().enumerate() {
        if let Some(table) = state.gc.tables.get_mut(spells_ref) {
            let _ = table.raw_set(
                Val::Num((index + 1) as f64),
                Val::Num(spell_id as f64),
                &state.gc.string_arena,
            );
        }
    }
    state.gc.barrier_back(spells_ref);
    state.push(spells);
    Ok(1)
}

fn c_spec_set_specialization(state: &mut LuaState) -> LuaResult<u32> {
    let requested_index = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let can_set = requested_or_active_spec(state, requested_index).is_some();
    if can_set && start_specialization_change(state, requested_index.max(1))? {
        let player = create_string(state, "player");
        let spell_id_val = Val::Num(SPEC_ACTIVATION_SPELL_ID as f64);
        fire_named_event_state(state, "UNIT_SPELLCAST_START", &[player, spell_id_val]);
    }
    state.push(Val::Bool(can_set));
    Ok(1)
}

fn start_specialization_change(state: &mut LuaState, target_index: i32) -> LuaResult<bool> {
    let mut sim = borrow_state_mut(state)?;
    if sim.player.active_spec_index == target_index && sim.player.pending_spec_change.is_none() {
        return Ok(false);
    }

    sim.player.pending_spec_change = Some(target_index);
    let now = sim.start_time.elapsed().as_secs_f64();
    let cast_id = sim.next_cast_id;
    sim.next_cast_id = sim.next_cast_id.wrapping_add(1);
    sim.casting = Some(CastingState {
        spell_id: SPEC_ACTIVATION_SPELL_ID,
        spell_name: "Activate Specialization".to_string(),
        icon_path: String::new(),
        start_time: now,
        end_time: now + SPEC_ACTIVATION_CAST_SECONDS,
        cast_id,
    });
    Ok(true)
}

fn requested_or_active_spec(
    state: &LuaState,
    requested_index: i32,
) -> Option<&'static specializations::SpecInfo> {
    let (class_id, active_spec_index) = {
        let sim = borrow_state(state).ok()?;
        (sim.player.class_index as u32, sim.player.active_spec_index)
    };
    let requested_spec_index = requested_index.max(1);
    specializations::specs_for_class(class_id)
        .nth((requested_spec_index - 1) as usize)
        .or_else(|| {
            let active_spec_index = active_spec_index.max(1);
            specializations::specs_for_class(class_id).nth((active_spec_index - 1) as usize)
        })
}

fn push_specialization_info(state: &mut LuaState, spec: &specializations::SpecInfo) {
    push_specialization_identity(state, spec);
    state.push(Val::Num(spec.primary_stat as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Nil);
    state.push(Val::Num(0.0));
    state.push(Val::Bool(true));
}

fn spec_display_spell_ids(spec_id: i32) -> Option<Vec<u32>> {
    for skill_line_index in 1..=spellbook_data::num_skill_lines() {
        let skill_line = spellbook_data::get_skill_line(skill_line_index)?;
        if skill_line.spec_id == Some(spec_id) {
            return Some(
                skill_line
                    .spells
                    .iter()
                    .map(|entry| entry.spell_id)
                    .collect(),
            );
        }
    }
    None
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
