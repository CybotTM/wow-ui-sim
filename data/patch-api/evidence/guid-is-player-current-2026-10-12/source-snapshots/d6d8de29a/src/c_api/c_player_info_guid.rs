//! Player GUID classification projected from currently represented unit identities.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::globals::unit_misc::{existing_guid_for_unit, party_guid_for_index};
use crate::lua_api::globals::unit_probes::resolve_unit_is_player;
use crate::lua_api::methods::{borrow_state, val_to_string};
use crate::lua_api::state::SimState;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_PlayerInfo")?;
    table_set_rust_fn_static(state, namespace, "GUIDIsPlayer", guid_is_player)
}

fn guid_is_player(state: &mut LuaState) -> LuaResult<u32> {
    let argument = rilua::table_security::unwrap_secret(state, stack_val(state, 1))?;
    let guid = val_to_string(state, argument).ok_or_else(|| {
        rilua::runtime_error("C_PlayerInfo.GUIDIsPlayer: argument 1 must be a UTF-8 string")
    })?;
    let is_player = {
        let sim = borrow_state(state)?;
        represented_guid_is_player(&sim, &guid)
    };
    state.push(Val::Bool(is_player));
    Ok(1)
}

fn represented_guid_is_player(sim: &SimState, guid: &str) -> bool {
    ["player", "target", "focus"].into_iter().any(|unit| {
        resolve_unit_is_player(sim, unit)
            && existing_guid_for_unit(sim, unit).as_deref() == Some(guid)
    }) || (sim.party_group_active
        && sim
            .party_members
            .iter()
            .enumerate()
            .any(|(index, _)| party_guid_for_index(index) == guid))
}
