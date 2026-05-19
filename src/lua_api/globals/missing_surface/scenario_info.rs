//! `C_ScenarioInfo` probe surface backed by `SimState.scenario`.
//!
//! Migrates 4 entries off the namespace stub tables:
//!
//! - `GetScenarioInfo()` — returns a `ScenarioInformation` table or
//!   nothing when `in_scenario` is false.
//! - `GetScenarioStepInfo(stepID?)` — returns the current step table
//!   (uses `current_step` when stepID is nil) or nothing.
//! - `GetScenarioBonusStepRewardQuestID(stepID)` — returns the
//!   bonus-reward quest id for the given step, or nil.
//! - `IsTieredEntranceScenario()` — returns the `is_tiered_entrance`
//!   bool flag.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set_static};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_scenario_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ScenarioInfo")?;
    table_set_rust_fn_static(state, ns, "GetScenarioInfo", get_scenario_info)?;
    table_set_rust_fn_static(state, ns, "GetScenarioStepInfo", get_scenario_step_info)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetScenarioBonusStepRewardQuestID",
        get_scenario_bonus_step_reward_quest_id,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "IsTieredEntranceScenario",
        is_tiered_entrance_scenario,
    )?;
    Ok(())
}

fn get_scenario_info(state: &mut LuaState) -> LuaResult<u32> {
    let sim = borrow_state(state)?;
    if !sim.scenario.in_scenario {
        return Ok(0);
    }
    let s = sim.scenario.clone();
    drop(sim);
    let t = create_table(state);
    let name_val = create_string(state, &s.name);
    table_set_static(state, t, "name", name_val);
    table_set_static(state, t, "currentStage", Val::Num(s.current_step as f64));
    table_set_static(state, t, "numStages", Val::Num(s.num_steps as f64));
    table_set_static(state, t, "scenarioID", Val::Num(s.scenario_id as f64));
    table_set_static(state, t, "type", Val::Num(s.scenario_type as f64));
    let kit_val = create_string(state, &s.texture_kit);
    table_set_static(state, t, "uiTextureKit", kit_val);
    table_set_static(state, t, "isComplete", Val::Bool(false));
    table_set_static(state, t, "flags", Val::Num(0.0));
    table_set_static(state, t, "money", Val::Num(0.0));
    table_set_static(state, t, "xp", Val::Num(0.0));
    let area_val = create_string(state, "");
    table_set_static(state, t, "area", area_val);
    state.push(t);
    Ok(1)
}

fn get_scenario_step_info(state: &mut LuaState) -> LuaResult<u32> {
    let step_id_arg = i32::from_stack(state, 1).ok();
    let sim = borrow_state(state)?;
    if !sim.scenario.in_scenario {
        return Ok(0);
    }
    let step_id = step_id_arg.unwrap_or(sim.scenario.current_step);
    let step = sim
        .scenario
        .steps
        .iter()
        .find(|s| s.step_id == step_id)
        .cloned();
    drop(sim);
    let Some(step) = step else {
        return Ok(0);
    };
    let t = create_table(state);
    table_set_static(state, t, "stepID", Val::Num(step.step_id as f64));
    let title_val = create_string(state, &step.title);
    table_set_static(state, t, "title", title_val);
    let desc_val = create_string(state, &step.description);
    table_set_static(state, t, "description", desc_val);
    table_set_static(state, t, "numCriteria", Val::Num(step.num_criteria as f64));
    table_set_static(state, t, "isBonusStep", Val::Bool(step.is_bonus_step));
    table_set_static(state, t, "stepFailed", Val::Bool(false));
    table_set_static(state, t, "shouldShowBonusObjective", Val::Bool(false));
    table_set_static(state, t, "isForCurrentStepOnly", Val::Bool(false));
    if let Some(quest_id) = step.bonus_reward_quest_id {
        table_set_static(state, t, "rewardQuestID", Val::Num(quest_id as f64));
    }
    state.push(t);
    Ok(1)
}

fn get_scenario_bonus_step_reward_quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let step_id = i32::from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let quest_id = sim
        .scenario
        .steps
        .iter()
        .find(|s| s.step_id == step_id)
        .and_then(|s| s.bonus_reward_quest_id);
    drop(sim);
    match quest_id {
        Some(id) => {
            state.push(Val::Num(id as f64));
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

fn is_tiered_entrance_scenario(state: &mut LuaState) -> LuaResult<u32> {
    let flag = borrow_state(state)?.scenario.is_tiered_entrance;
    state.push(Val::Bool(flag));
    Ok(1)
}
