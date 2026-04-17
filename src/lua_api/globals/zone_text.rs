//! Zone-text probes backed by `SimState::world`.
//!
//! Real WoW distinguishes four zone strings:
//!
//! - `GetZoneText()`          — the enclosing zone (e.g. `"Durotar"`).
//! - `GetSubZoneText()`       — the sub-zone (e.g. `"Razor Hill"`), empty when
//!                              none.
//! - `GetMinimapZoneText()`   — what the minimap header renders: sub-zone
//!                              when available, else the zone.
//! - `GetRealZoneText()`      — the "real" zone label, which in instances is
//!                              the instance name (e.g. `"Deadmines"`) and
//!                              matches `GetZoneText()` otherwise.
//!
//! The sim already models `world.zone_name`, `world.sub_zone_name`,
//! `world.instance_name`, and `world.in_instance`, so the four getters can
//! fall out of the existing state without new fields. Admin API
//! `A_Admin.SetZone(name, id)` / `SetSubZone(name)` / `SetInstanceInfo(...)`
//! drive the values.

use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::LuaResult;

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

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let g = state.global;
    table_set_rust_fn(state, g, "GetZoneText", get_zone_text)?;
    table_set_rust_fn(state, g, "GetSubZoneText", get_sub_zone_text)?;
    table_set_rust_fn(state, g, "GetMinimapZoneText", get_minimap_zone_text)?;
    table_set_rust_fn(state, g, "GetRealZoneText", get_real_zone_text)?;
    Ok(())
}
