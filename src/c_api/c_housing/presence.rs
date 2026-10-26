//! Spatial housing queries share the supplied area, never ownership flags.

use super::HousingArea;
use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let housing = ensure_namespace(state, "C_Housing")?;
    table_set_rust_fn_static(state, housing, "IsInsideHouse", is_inside_house)?;
    table_set_rust_fn_static(
        state,
        housing,
        "IsInsideHouseOrPlot",
        is_inside_house_or_plot,
    )
}

fn is_inside_house(state: &mut LuaState) -> LuaResult<u32> {
    let inside =
        borrow_state(state)?.housing.current_housing_area == Some(HousingArea::HouseInterior);
    state.push(Val::Bool(inside));
    Ok(1)
}

fn is_inside_house_or_plot(state: &mut LuaState) -> LuaResult<u32> {
    // The supplied area has exactly two variants: interior or plot exterior.
    let inside = borrow_state(state)?.housing.current_housing_area.is_some();
    state.push(Val::Bool(inside));
    Ok(1)
}
