//! `C_ArtifactUI` artifact-bar surface consumed by
//! `Blizzard_ActionBar/Mainline/ArtifactBar.lua`.
//!
//! State sources:
//!
//! - `state.equipped_artifact: Option<ArtifactInfo>` — currently wielded
//!   artifact. `None` keeps `GetEquippedArtifactItemID` returning nil so
//!   `ArtifactBarMixin:Update` short-circuits.
//! - `state.artifact_point_costs: HashMap<(points_spent, tier), xp_cost>` —
//!   feeds `GetCostForPointAtRank`. A missing entry returns 0, which
//!   `ArtifactBarGetNumArtifactTraitsPurchasableFromXP` treats as "no further
//!   point purchasable".
//!
//! The `IsEquippedArtifactDisabled` and `IsEquippedArtifactMaxed` flags ride
//! on the same `ArtifactInfo`. With no artifact equipped both are reported as
//! false (matching the live client's nil-default behavior).

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_artifact_ui_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ArtifactUI")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetEquippedArtifactItemID",
        get_equipped_artifact_item_id,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetEquippedArtifactInfo",
        get_equipped_artifact_info,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "IsEquippedArtifactMaxed",
        is_equipped_artifact_maxed,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "IsEquippedArtifactDisabled",
        is_equipped_artifact_disabled,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetCostForPointAtRank",
        get_cost_for_point_at_rank,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetArtifactXPRewardTargetInfo",
        get_artifact_xp_reward_target_info,
    )?;
    Ok(())
}

fn get_equipped_artifact_item_id(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_equipped_artifact_info(state: &mut LuaState) -> LuaResult<u32> {
    let info = borrow_state(state)?.equipped_artifact.clone();
    let Some(info) = info else {
        return Ok(0);
    };
    let name_val = create_string(state, &info.name);
    let icon_val = create_string(state, &info.icon);
    state.push(Val::Num(info.item_id as f64));
    state.push(Val::Num(info.alt_item_id as f64));
    state.push(name_val);
    state.push(icon_val);
    state.push(Val::Num(info.total_xp as f64));
    state.push(Val::Num(info.points_spent as f64));
    state.push(Val::Num(info.quality as f64));
    state.push(Val::Num(info.artifact_appearance_id as f64));
    state.push(Val::Num(info.appearance_mod_id as f64));
    state.push(Val::Num(info.item_appearance_id as f64));
    state.push(Val::Num(info.alt_item_appearance_id as f64));
    state.push(Val::Bool(info.alt_on_top));
    state.push(Val::Num(info.tier as f64));
    Ok(13)
}

fn is_equipped_artifact_maxed(state: &mut LuaState) -> LuaResult<u32> {
    let maxed = borrow_state(state)?
        .equipped_artifact
        .as_ref()
        .is_some_and(|info| info.maxed);
    state.push(Val::Bool(maxed));
    Ok(1)
}

fn is_equipped_artifact_disabled(state: &mut LuaState) -> LuaResult<u32> {
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
fn get_artifact_xp_reward_target_info(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_cost_for_point_at_rank(state: &mut LuaState) -> LuaResult<u32> {
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
