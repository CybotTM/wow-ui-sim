//! `C_ArdenwealdGardening` surface consumed by `Blizzard_ArdenwealdGardening`
//! and `Blizzard_GarrisonLandingPage`.
//!
//! State source:
//!
//! - `state.gardenweald.accessible` — `IsGardenAccessible()` returns
//!   this flag. `LandingPageMixin:UpdateArdenwealdGardeningSection`
//!   (`Blizzard_GarrisonLandingPage.lua:190`) early-returns when false
//!   and never instantiates the panel.
//! - `state.gardenweald.{active, ready, remaining_seconds}` —
//!   `GetGardenData()` returns the matching `ArdenwealdGardenData` table.
//!   `ArdenwealdGardeningButtonMixin:OnEnter` reads the three fields to
//!   pick between the active-count, ready-count, and dormant tooltip
//!   branches (`Blizzard_ArdenwealdGardening.lua:24-38`).
//!
//! Both functions return their data unconditionally; they do not nil
//! out when the namespace is "empty" (real WoW returns an all-zero
//! `ArdenwealdGardenData` table when the player has not planted
//! anything, which the addon's branches handle explicitly).

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_table_with_fields};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_ardenweald_gardening_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ArdenwealdGardening")?;
    table_set_rust_fn_static(state, ns, "GetGardenData", get_garden_data)?;
    table_set_rust_fn_static(state, ns, "IsGardenAccessible", is_garden_accessible)?;
    Ok(())
}

fn get_garden_data(state: &mut LuaState) -> LuaResult<u32> {
    let snapshot = borrow_state(state)?.gardenweald;
    let table = create_table_with_fields(
        state,
        &[
            ("active", Val::Num(snapshot.active as f64)),
            ("ready", Val::Num(snapshot.ready as f64)),
            (
                "remainingSeconds",
                Val::Num(snapshot.remaining_seconds as f64),
            ),
        ],
    );
    state.push(table);
    Ok(1)
}

fn is_garden_accessible(state: &mut LuaState) -> LuaResult<u32> {
    let accessible = borrow_state(state)?.gardenweald.accessible;
    state.push(Val::Bool(accessible));
    Ok(1)
}
