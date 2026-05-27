//! Permanent `C_FogOfWar` lookup surface.
//!
//! The simulator does not model player-specific fog discovery state. This
//! namespace exposes stable DB2-derived fog visualization rows so world-map
//! fog pins can render deterministic assets without pretending the richer
//! discovery system is state-backed.

use super::c_map_api;
use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_fog_of_war_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_FogOfWar")?;
    table_set_rust_fn_static(state, ns, "GetFogOfWarForMap", get_fog_of_war_for_map)?;
    table_set_rust_fn_static(state, ns, "GetFogOfWarInfo", get_fog_of_war_info)?;
    Ok(())
}

fn get_fog_of_war_for_map(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = i32::from_stack(state, 1)?;
    match c_map_api::fog_of_war_id_for_map(map_id) {
        Some(fog_id) => state.push(Val::Num(fog_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_fog_of_war_info(state: &mut LuaState) -> LuaResult<u32> {
    let info = create_table(state);
    let fog_info = match stack_val(state, 1) {
        Val::Num(value) => c_map_api::fog_of_war_info_for_id(value as i32),
        _ => None,
    };
    match fog_info {
        Some(fog_info) => {
            match fog_info.background_atlas {
                Some(atlas) => {
                    let atlas = create_string(state, atlas);
                    table_set(state, info, "backgroundAtlas", atlas);
                }
                None => table_set(state, info, "backgroundAtlas", Val::Nil),
            }
            match fog_info.mask_atlas {
                Some(atlas) => {
                    let atlas = create_string(state, atlas);
                    table_set(state, info, "maskAtlas", atlas);
                }
                None => table_set(state, info, "maskAtlas", Val::Nil),
            }
            table_set(state, info, "maskScalar", Val::Num(fog_info.mask_scalar));
        }
        None => {
            table_set(state, info, "backgroundAtlas", Val::Nil);
            table_set(state, info, "maskAtlas", Val::Nil);
            table_set(state, info, "maskScalar", Val::Num(1.0));
        }
    }
    state.push(info);
    Ok(1)
}
