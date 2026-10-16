//! Wrath-only modeled legacy adapters; source-grounded positions, explicit inputs.
use super::wrath_legacy_state::Skill;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

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
fn set_map_zoom(state: &mut LuaState) -> LuaResult<u32> {
    let continent = i32::from_stack(state, 1)?;
    let zone = Option::<i32>::from_stack(state, 2)?.unwrap_or(0);
    borrow_state_mut(state)?.wrath_legacy.selected_map = (continent, zone);
    Ok(0)
}
fn get_current_map_continent(state: &mut LuaState) -> LuaResult<u32> {
    let continent = borrow_state(state)?.wrath_legacy.selected_map.0;
    state.push(Val::Num(continent as f64));
    Ok(1)
}
fn get_current_map_zone(state: &mut LuaState) -> LuaResult<u32> {
    let zone = borrow_state(state)?.wrath_legacy.selected_map.1;
    state.push(Val::Num(zone as f64));
    Ok(1)
}
fn get_num_map_landmarks(state: &mut LuaState) -> LuaResult<u32> {
    let count = {
        let sim = borrow_state(state)?;
        sim.wrath_legacy
            .landmarks
            .get(&sim.wrath_legacy.selected_map)
            .map_or(0, Vec::len)
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}
fn get_map_landmark_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let row = {
        let sim = borrow_state(state)?;
        sim.wrath_legacy
            .landmarks
            .get(&sim.wrath_legacy.selected_map)
            .and_then(|rows| usize::try_from(index - 1).ok().and_then(|i| rows.get(i)))
            .cloned()
    };
    let Some(row) = row else {
        return Ok(0);
    };
    let name = create_string(state, &row.name);
    let description = create_string(state, &row.description);
    state.push(name);
    state.push(description);
    for value in [
        row.texture_index as f64,
        row.x,
        row.y,
        row.map_link_id as f64,
    ] {
        state.push(Val::Num(value));
    }
    Ok(6)
}
fn request_battleground_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    sim.wrath_legacy.requested_battlefield = usize::try_from(index)
        .ok()
        .filter(|i| *i > 0 && *i <= sim.wrath_legacy.battlefields.len());
    Ok(0)
}
fn get_battlefield_info(state: &mut LuaState) -> LuaResult<u32> {
    let row = {
        let sim = borrow_state(state)?;
        sim.wrath_legacy
            .requested_battlefield
            .and_then(|i| sim.wrath_legacy.battlefields.get(i - 1))
            .cloned()
    };
    let Some(row) = row else {
        return Ok(0);
    };
    let name = create_string(state, &row.name);
    let description = create_string(state, &row.description);
    state.push(name);
    state.push(description);
    state.push(Val::Num(row.max_group as f64));
    Ok(3)
}
fn count_owned_critters(sim: &crate::lua_api::state::SimState) -> usize {
    let owned_species: std::collections::HashSet<i32> = sim
        .world
        .pets
        .iter()
        .filter(|pet| pet.is_collected)
        .map(|pet| pet.species_id as i32)
        .collect();
    sim.wrath_legacy
        .eligible_critters
        .intersection(&owned_species)
        .count()
}
fn count_owned_mounts(sim: &crate::lua_api::state::SimState) -> usize {
    sim.world
        .mounts
        .iter()
        .filter(|mount| mount.is_collected)
        .filter(|mount| {
            sim.wrath_legacy
                .eligible_mounts
                .contains(&(mount.mount_id as i32))
        })
        .count()
}
fn get_num_companions(state: &mut LuaState) -> LuaResult<u32> {
    let kind = String::from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        match kind.as_str() {
            "CRITTER" => count_owned_critters(&sim),
            "MOUNT" => count_owned_mounts(&sim),
            _ => return Err(runtime_error("GetNumCompanions expects CRITTER or MOUNT")),
        }
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}
fn set_who_to_ui(state: &mut LuaState) -> LuaResult<u32> {
    let flag = i32::from_stack(state, 1)?;
    let enabled = match flag {
        0 => false,
        1 => true,
        _ => return Err(runtime_error("SetWhoToUI expects 0 or 1")),
    };
    borrow_state_mut(state)?.who_routing.to_ui = enabled;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetNumSkillLines", get_num_skill_lines)?;
    LuaApiMut::register_function(lua, "GetSelectedSkill", get_selected_skill)?;
    LuaApiMut::register_function(lua, "SetSelectedSkill", set_selected_skill)?;
    LuaApiMut::register_function(lua, "GetSkillLineInfo", get_skill_line_info)?;
    LuaApiMut::register_function(lua, "SetMapZoom", set_map_zoom)?;
    LuaApiMut::register_function(lua, "GetCurrentMapContinent", get_current_map_continent)?;
    LuaApiMut::register_function(lua, "GetCurrentMapZone", get_current_map_zone)?;
    LuaApiMut::register_function(lua, "GetNumMapLandmarks", get_num_map_landmarks)?;
    LuaApiMut::register_function(lua, "GetMapLandmarkInfo", get_map_landmark_info)?;
    LuaApiMut::register_function(
        lua,
        "RequestBattlegroundInstanceInfo",
        request_battleground_info,
    )?;
    LuaApiMut::register_function(lua, "GetBattlefieldInfo", get_battlefield_info)?;
    LuaApiMut::register_function(lua, "GetNumCompanions", get_num_companions)?;
    LuaApiMut::register_function(lua, "SetWhoToUI", set_who_to_ui)?;
    LuaApiMut::register_function(lua, "SendWho", crate::c_api::who_routing::send_who)?;
    Ok(())
}
