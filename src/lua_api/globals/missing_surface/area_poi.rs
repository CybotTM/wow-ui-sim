//! `C_AreaPoiInfo` probe surface backed by `SimState.area_pois`.
//!
//! Migrates 2 entries off `NAMESPACE_NIL_STUBS`:
//!
//! - `C_AreaPoiInfo.GetAreaPOIInfo(uiMapID, areaPoiID)` — returns the
//!   `AreaPOIInfo` structure for the seeded POI, or nothing (retail
//!   `mayreturnnothing`). If `uiMapID` is non-nil it must match the
//!   POI's `ui_map_id`.
//! - `C_AreaPoiInfo.GetAreaPOISecondsLeft(areaPoiID)` — returns the
//!   remaining seconds for a time-limited POI, or nothing for
//!   permanent POIs / unknown ids.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_api::state::AreaPoiInfo;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_area_poi_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AreaPoiInfo")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetAreaPOIInfo",
        c_area_poi_info_get_area_poi_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetAreaPOISecondsLeft",
        c_area_poi_info_get_area_poi_seconds_left,
    )?;
    Ok(())
}

fn c_area_poi_info_get_area_poi_info(state: &mut LuaState) -> LuaResult<u32> {
    let ui_map_id = match stack_val(state, 1) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    };
    let area_poi_id = i32::from_stack(state, 2)?;

    let poi = {
        let sim = borrow_state(state)?;
        let Some(poi) = sim.area_pois.get(&area_poi_id).cloned() else {
            return Ok(0);
        };
        if let (Some(requested), Some(owned)) = (ui_map_id, poi.ui_map_id)
            && requested != owned
        {
            return Ok(0);
        }
        poi
    };

    let table = push_area_poi_info_table(state, &poi);
    state.push(table);
    Ok(1)
}

fn c_area_poi_info_get_area_poi_seconds_left(state: &mut LuaState) -> LuaResult<u32> {
    let area_poi_id = i32::from_stack(state, 1)?;
    let seconds = borrow_state(state)?
        .area_pois
        .get(&area_poi_id)
        .and_then(|p| p.seconds_left);
    let Some(seconds) = seconds else {
        return Ok(0);
    };
    state.push(Val::Num(seconds as f64));
    Ok(1)
}

fn push_area_poi_info_table(state: &mut LuaState, poi: &AreaPoiInfo) -> Val {
    let t = create_table(state);
    let name = create_string(state, &poi.name);
    let position = create_table(state);
    table_set(state, position, "x", Val::Num(poi.position.0));
    table_set(state, position, "y", Val::Num(poi.position.1));

    table_set(state, t, "areaPoiID", Val::Num(poi.area_poi_id as f64));
    table_set(state, t, "name", name);
    table_set(state, t, "position", position);
    table_set(state, t, "isCurrentEvent", Val::Bool(poi.is_current_event));
    table_set(state, t, "shouldGlow", Val::Bool(poi.should_glow));
    table_set(state, t, "highlightVignettesOnHover", Val::Bool(false));
    table_set(state, t, "highlightWorldQuestsOnHover", Val::Bool(false));
    table_set(state, t, "isAlwaysOnFlightmap", Val::Bool(false));
    table_set(state, t, "isPrimaryMapForPOI", Val::Bool(true));

    push_optional_string(state, t, "atlasName", poi.atlas_name.as_deref());
    push_optional_string(state, t, "description", poi.description.as_deref());
    push_optional_string(state, t, "uiTextureKit", None);
    push_optional_number(state, t, "factionID", poi.faction_id);
    push_optional_number(state, t, "iconWidgetSet", poi.icon_widget_set);
    push_optional_number(state, t, "linkedUiMapID", poi.linked_ui_map_id);
    t
}

fn push_optional_string(state: &mut LuaState, t: Val, key: &str, value: Option<&str>) {
    match value {
        Some(s) => {
            let v = create_string(state, s);
            table_set(state, t, key, v);
        }
        None => table_set(state, t, key, Val::Nil),
    }
}

fn push_optional_number(state: &mut LuaState, t: Val, key: &str, value: Option<i32>) {
    match value {
        Some(n) => table_set(state, t, key, Val::Num(n as f64)),
        None => table_set(state, t, key, Val::Nil),
    }
}
