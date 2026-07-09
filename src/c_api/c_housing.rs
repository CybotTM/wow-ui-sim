//! `C_Housing` owned-house probes backed by `SimState.housing`.
//!
//! The simulator does not model the full War Within housing service yet, but it
//! already keeps house-favor display state. These 12.1 probes expose a small,
//! deterministic local contract: tests or future service glue may mark the
//! player as being inside an owned house and/or plot, and `ResetHouse` clears
//! that local housing/favor state.

use crate::c_api::helpers::ensure_namespace;
#[cfg(feature = "retail-12-1-0")]
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
#[cfg(feature = "retail-12-1-0")]
use crate::lua_api::state::HousingState;
#[cfg(feature = "retail-12-1-0")]
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
#[cfg(feature = "retail-12-1-0")]
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_housing_surface(state: &mut LuaState) -> LuaResult<()> {
    let housing = ensure_namespace(state, "C_Housing")?;
    register_patch_12_1_c_housing_surface(state, housing)
}

#[cfg(feature = "retail-12-1-0")]
fn register_patch_12_1_c_housing_surface(
    state: &mut LuaState,
    housing: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        housing,
        "HouseFinderIgnoreNeighborhood",
        house_finder_ignore_neighborhood,
    )?;
    table_set_rust_fn_static(
        state,
        housing,
        "IsInsideOwnedHouseOrPlot",
        is_inside_owned_house_or_plot,
    )?;
    table_set_rust_fn_static(state, housing, "IsInsideOwnedHouse", is_inside_owned_house)?;
    table_set_rust_fn_static(state, housing, "IsInsideOwnedPlot", is_inside_owned_plot)?;
    table_set_rust_fn_static(state, housing, "ResetHouse", reset_house)
}

#[cfg(not(feature = "retail-12-1-0"))]
fn register_patch_12_1_c_housing_surface(
    _state: &mut LuaState,
    _housing: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    Ok(())
}

#[cfg(feature = "retail-12-1-0")]
fn house_finder_ignore_neighborhood(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn is_inside_owned_house_or_plot(state: &mut LuaState) -> LuaResult<u32> {
    let is_inside = {
        let sim = borrow_state(state)?;
        sim.housing.inside_owned_house || sim.housing.inside_owned_plot
    };
    state.push(Val::Bool(is_inside));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn is_inside_owned_house(state: &mut LuaState) -> LuaResult<u32> {
    let inside_owned_house = { borrow_state(state)?.housing.inside_owned_house };
    state.push(Val::Bool(inside_owned_house));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn is_inside_owned_plot(state: &mut LuaState) -> LuaResult<u32> {
    let inside_owned_plot = { borrow_state(state)?.housing.inside_owned_plot };
    state.push(Val::Bool(inside_owned_plot));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn reset_house(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.housing = HousingState::default();
    Ok(0)
}
