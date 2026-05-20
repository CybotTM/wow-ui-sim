//! C_GossipInfo temporary shims — gossip POI lookup state is not modeled.
//!
//! The state-backed gossip surface handles options, quests, text, and dialog
//! transitions. POI data for a map remains unmodeled, so the lookup returns nil.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_gossip_info_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_GossipInfo")?;
    table_set_rust_fn_static(state, ns, "GetPoiForUiMapID", get_poi_for_ui_map_id)?;
    Ok(())
}

fn get_poi_for_ui_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let _ui_map_id = Option::<u32>::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}
