//! Shared legacy skill records and projection, extracted from the Wrath model.
//! Empty/no-selection inputs are simulator policy, not native invalid-index parity.
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

#[derive(Clone, Debug, Default)]
pub struct Skill {
    pub name: String,
    pub is_header: bool,
    pub is_expanded: bool,
    pub rank: i32,
    pub temporary_points: i32,
    pub modifier: i32,
    pub max_rank: i32,
    pub abandonable: bool,
    pub step_cost: Option<i32>,
    pub rank_cost: Option<i32>,
    pub min_level: i32,
    pub cost_type: i32,
    pub description: String,
}

fn get_num_skill_lines(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.wrath_legacy.skills.len();
    state.push(Val::Num(count as f64));
    Ok(1)
}
fn get_selected_skill(state: &mut LuaState) -> LuaResult<u32> {
    let selected = borrow_state(state)?.wrath_legacy.selected_skill;
    state.push(Val::Num(selected as f64));
    Ok(1)
}
fn set_selected_skill(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    let selection = usize::try_from(index)
        .ok()
        .filter(|i| *i <= sim.wrath_legacy.skills.len());
    sim.wrath_legacy.selected_skill = selection.unwrap_or(0);
    Ok(0)
}
fn get_skill_line_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let skill = {
        let sim = borrow_state(state)?;
        usize::try_from(index - 1)
            .ok()
            .and_then(|i| sim.wrath_legacy.skills.get(i))
            .cloned()
    };
    // Consumer arithmetic precedes absent-name check. Remaining no-selection
    // slots are simulator policy, not a measured native invalid-index tuple.
    push_skill(state, &skill.unwrap_or_default());
    Ok(13)
}
fn push_skill(state: &mut LuaState, skill: &Skill) {
    let name = if skill.name.is_empty() {
        Val::Nil
    } else {
        create_string(state, &skill.name)
    };
    let description = create_string(state, &skill.description);
    state.push(name);
    state.push(Val::Bool(skill.is_header));
    state.push(Val::Bool(skill.is_expanded));
    for value in [
        skill.rank,
        skill.temporary_points,
        skill.modifier,
        skill.max_rank,
    ] {
        state.push(Val::Num(value as f64));
    }
    state.push(Val::Bool(skill.abandonable));
    state.push(skill.step_cost.map_or(Val::Nil, |n| Val::Num(n as f64)));
    state.push(skill.rank_cost.map_or(Val::Nil, |n| Val::Num(n as f64)));
    state.push(Val::Num(skill.min_level as f64));
    state.push(Val::Num(skill.cost_type as f64));
    state.push(description);
}

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetNumSkillLines", get_num_skill_lines)?;
    LuaApiMut::register_function(lua, "GetSelectedSkill", get_selected_skill)?;
    LuaApiMut::register_function(lua, "SetSelectedSkill", set_selected_skill)?;
    LuaApiMut::register_function(lua, "GetSkillLineInfo", get_skill_line_info)?;
    Ok(())
}
