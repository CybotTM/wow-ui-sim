//! Action-bar getters that read `state.equipped_artifact` (and the
//! flat `state.artifact_point_costs` table). Consumed by
//! `Blizzard_ActionBar/Mainline/ArtifactBar.lua`.

use super::helpers::push_artifact_info_tuple;
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn get_equipped_artifact_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = borrow_state(state)?
        .equipped_artifact
        .as_ref()
        .map(|info| info.item_id);
    match item_id {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub(super) fn get_equipped_artifact_info(state: &mut LuaState) -> LuaResult<u32> {
    let info = borrow_state(state)?.equipped_artifact.clone();
    let Some(info) = info else {
        return Ok(0);
    };
    push_artifact_info_tuple(state, &info);
    Ok(13)
}

pub(super) fn is_equipped_artifact_maxed(state: &mut LuaState) -> LuaResult<u32> {
    let maxed = borrow_state(state)?
        .equipped_artifact
        .as_ref()
        .is_some_and(|info| info.maxed);
    state.push(Val::Bool(maxed));
    Ok(1)
}

pub(super) fn is_equipped_artifact_disabled(state: &mut LuaState) -> LuaResult<u32> {
    let disabled = borrow_state(state)?
        .equipped_artifact
        .as_ref()
        .is_some_and(|info| info.disabled);
    state.push(Val::Bool(disabled));
    Ok(1)
}

/// `GetArtifactXPRewardTargetInfo(artifactCategory) -> name, icon` —
/// returns the equipped artifact's display name and icon when its
/// `category` matches `artifactCategory`. Returns nothing (nil pair)
/// when no artifact is equipped or the category mismatches; matches
/// the `MayReturnNothing` shape in the docs.
pub(super) fn get_artifact_xp_reward_target_info(state: &mut LuaState) -> LuaResult<u32> {
    let requested_category = i32::from_stack(state, 1)?;
    let display = borrow_state(state)?
        .equipped_artifact
        .as_ref()
        .filter(|info| info.category == requested_category)
        .map(|info| (info.name.clone(), info.icon.clone()));
    let Some((name, icon)) = display else {
        return Ok(0);
    };
    let name_val = create_string(state, &name);
    let icon_val = create_string(state, &icon);
    state.push(name_val);
    state.push(icon_val);
    Ok(2)
}

pub(super) fn get_cost_for_point_at_rank(state: &mut LuaState) -> LuaResult<u32> {
    let points_spent = i32::from_stack(state, 1)?;
    let tier = i32::from_stack(state, 2)?;
    let cost = borrow_state(state)?
        .artifact_point_costs
        .get(&(points_spent, tier))
        .copied()
        .unwrap_or(0);
    state.push(Val::Num(cost as f64));
    Ok(1)
}
