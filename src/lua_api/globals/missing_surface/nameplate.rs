//! `C_NamePlate` permissive stub surface.
//!
//! The simulator does not render 3D world nameplates. Retail's
//! `GetNamePlateForUnit(unit)` returns a Frame object rendered above
//! the unit's 3D model; here we return `nil` for all units, which is
//! the correct "no plate is currently shown" signal. Blizzard UI guards
//! like `if C_NamePlate.GetNamePlateForUnit(unit) then ... end` treat
//! nil as "no plate yet" and skip gracefully.
//!
//! `GetNamePlates()` returns an empty array — addon iteration loops
//! (`for _, plate in ipairs(C_NamePlate.GetNamePlates()) do`) will
//! exit immediately with zero iterations.
//!
//! Migrates 2 entries off the namespace stub tables:
//!
//! - `C_NamePlate.GetNamePlateForUnit(unitToken, [includeForbidden])` — nil
//! - `C_NamePlate.GetNamePlates([includeForbidden])` — empty table

use super::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_nameplate_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_NamePlate")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetNamePlateForUnit",
        c_nameplate_get_nameplate_for_unit,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetNamePlates",
        c_nameplate_get_nameplates,
    )?;
    Ok(())
}

fn c_nameplate_get_nameplate_for_unit(_state: &mut LuaState) -> LuaResult<u32> {
    // No 3D nameplates rendered — return nil (no plate for this unit).
    Ok(0)
}

fn c_nameplate_get_nameplates(state: &mut LuaState) -> LuaResult<u32> {
    // Return an empty array — no nameplates are currently rendered.
    let array = create_table(state);
    state.push(array);
    Ok(1)
}
