//! Zone-text probes + `C_PvP.GetZonePVPInfo`, all backed by `SimState::world`.
//!
//! Real WoW distinguishes four zone strings:
//!
//! - `GetZoneText()`          — the enclosing zone (e.g. `"Durotar"`).
//! - `GetSubZoneText()`       — the sub-zone (e.g. `"Razor Hill"`), empty when
//!                              none.
//! - `GetMinimapZoneText()`   — what the minimap header renders: sub-zone
//!                              when available, else the zone.
//! - `GetBindLocation()`      — the player's hearth/bind location.
//! - `GetRealZoneText()`      — the "real" zone label, which in instances is
//!                              the instance name (e.g. `"Deadmines"`) and
//!                              matches `GetZoneText()` otherwise.
//! - `C_PvP.GetZonePVPInfo()` — returns `(pvpType, isSubZonePvP, factionName)`.
//!                              pvpType is one of `"contested"` / `"sanctuary"`
//!                              / `"arena"` / `"friendly"` / `"hostile"` /
//!                              `"combat"`. factionName is `"Alliance"` /
//!                              `"Horde"` for faction-locked zones, else nil.
//!
//! The sim already models `world.zone_name`, `world.sub_zone_name`,
//! `world.instance_name`, and `world.in_instance`. PvP zone metadata sits on
//! `world.pvp_type`, `world.is_sub_zone_pvp`, and `world.pvp_faction_name`
//! (defaults `"contested"` / false / None). Admin API
//! `A_Admin.SetZone(name, id)` / `SetSubZone(name)` / `SetInstanceInfo(...)`
//! / `SetZonePVP(pvpType, isSubZonePvp, factionName)` drive the values.

use crate::lua_api::methods::{borrow_state, create_string, create_table};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

fn push_string(state: &mut LuaState, text: &str) -> LuaResult<u32> {
    let val = create_string(state, text);
    state.push(val);
    Ok(1)
}

pub fn get_zone_text(state: &mut LuaState) -> LuaResult<u32> {
    let text = borrow_state(state)?.world.zone_name.clone();
    push_string(state, &text)
}

pub fn get_sub_zone_text(state: &mut LuaState) -> LuaResult<u32> {
    let text = borrow_state(state)?.world.sub_zone_name.clone();
    push_string(state, &text)
}

/// Minimap header: sub-zone when set, otherwise the enclosing zone.
pub fn get_minimap_zone_text(state: &mut LuaState) -> LuaResult<u32> {
    let text = {
        let sim = borrow_state(state)?;
        if sim.world.sub_zone_name.is_empty() {
            sim.world.zone_name.clone()
        } else {
            sim.world.sub_zone_name.clone()
        }
    };
    push_string(state, &text)
}

pub fn get_bind_location(state: &mut LuaState) -> LuaResult<u32> {
    let text = borrow_state(state)?.bind_location.clone();
    push_string(state, &text)
}

/// "Real" zone label: instance name when in an instance, else the zone name.
pub fn get_real_zone_text(state: &mut LuaState) -> LuaResult<u32> {
    let text = {
        let sim = borrow_state(state)?;
        if sim.world.in_instance && !sim.world.instance_name.is_empty() {
            sim.world.instance_name.clone()
        } else {
            sim.world.zone_name.clone()
        }
    };
    push_string(state, &text)
}

pub fn get_zone_pvp_info(state: &mut LuaState) -> LuaResult<u32> {
    let (pvp_type, is_sub_zone, faction) = {
        let sim = borrow_state(state)?;
        (
            sim.world.pvp_type.clone(),
            sim.world.is_sub_zone_pvp,
            sim.world.pvp_faction_name.clone(),
        )
    };
    let pvp_type_val = create_string(state, &pvp_type);
    state.push(pvp_type_val);
    state.push(Val::Bool(is_sub_zone));
    match faction {
        Some(name) => {
            let faction_val = create_string(state, &name);
            state.push(faction_val);
        }
        None => state.push(Val::Nil),
    }
    Ok(3)
}

/// Ensure `C_PvP` global exists and return its table ref. Reuses an existing
/// table if one is already installed (the `stubs::namespace_stubs` pass can
/// create it), otherwise allocates and publishes a fresh one.
fn ensure_c_pvp_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_PvP");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(existing)) = existing {
        return existing;
    }
    let new_table_val = create_table(state);
    let Val::Table(new_table) = new_table_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_table_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_table
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let g = state.global;
    table_set_rust_fn_static(state, g, "GetZoneText", get_zone_text)?;
    table_set_rust_fn_static(state, g, "GetSubZoneText", get_sub_zone_text)?;
    table_set_rust_fn_static(state, g, "GetMinimapZoneText", get_minimap_zone_text)?;
    table_set_rust_fn_static(state, g, "GetBindLocation", get_bind_location)?;
    table_set_rust_fn_static(state, g, "GetRealZoneText", get_real_zone_text)?;
    let c_pvp = ensure_c_pvp_table(state);
    table_set_rust_fn_static(state, c_pvp, "GetZonePVPInfo", get_zone_pvp_info)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn default_bind_location_is_seeded() {
        let env = WowLuaEnv::new().unwrap();
        let bind_location: String = env.eval("return GetBindLocation()").unwrap();

        assert_eq!(bind_location, "Stormwind City");
    }

    #[test]
    fn admin_can_set_bind_location() {
        let env = WowLuaEnv::new().unwrap();
        env.exec(r#"A_Admin.SetBindLocation("Razor Hill")"#)
            .unwrap();
        let bind_location: String = env.eval("return GetBindLocation()").unwrap();

        assert_eq!(bind_location, "Razor Hill");
    }
}
