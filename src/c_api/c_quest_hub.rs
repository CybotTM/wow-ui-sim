//! `C_QuestHub` 12.0.7 probe surface.
//!
//! Dragonriding race discovery is service/content backed. Until that content
//! model exists, expose an empty deterministic race list for area POI probes.

use crate::c_api::helpers::ensure_namespace;
#[cfg(any(feature = "retail-12-0-7", feature = "retail-12-1-0"))]
use crate::lua_api::methods::create_table;
#[cfg(any(feature = "retail-12-0-7", feature = "retail-12-1-0"))]
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_quest_hub_surface(state: &mut LuaState) -> LuaResult<()> {
    let quest_hub = ensure_namespace(state, "C_QuestHub")?;
    register_patch_12_0_7_quest_hub_surface(state, quest_hub)
}

#[cfg(any(feature = "retail-12-0-7", feature = "retail-12-1-0"))]
fn register_patch_12_0_7_quest_hub_surface(
    state: &mut LuaState,
    quest_hub: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        quest_hub,
        "GetDragonridingRacesForAreaPOI",
        get_dragonriding_races_for_area_poi,
    )
}

#[cfg(not(any(feature = "retail-12-0-7", feature = "retail-12-1-0")))]
fn register_patch_12_0_7_quest_hub_surface(
    _state: &mut LuaState,
    _quest_hub: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    Ok(())
}

#[cfg(any(feature = "retail-12-0-7", feature = "retail-12-1-0"))]
fn get_dragonriding_races_for_area_poi(state: &mut LuaState) -> LuaResult<u32> {
    let races = create_table(state);
    state.push(races);
    Ok(1)
}
