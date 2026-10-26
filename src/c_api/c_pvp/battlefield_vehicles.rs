//! Supplied battlefield map observations; no native vehicle catalog is seeded.

use crate::c_api::helpers::set_table_array;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set_static};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::{LuaResult, Val, vm::gc::arena::GcRef, vm::state::LuaState, vm::table::Table};

/// Fields declared by Mists PvpInfoDocumentation's BattlefieldVehicleInfo.
#[derive(Debug, Clone)]
pub struct BattlefieldVehicle {
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub is_occupied: bool,
    pub atlas: String,
    pub texture_width: f64,
    pub texture_height: f64,
    pub facing: f64,
    pub is_player: bool,
    pub is_alive: bool,
    pub should_draw_below_player_blips: bool,
}

pub(crate) fn register(state: &mut LuaState, namespace: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, namespace, "GetBattlefieldVehicles", query_vehicles)?;
    table_set_rust_fn_static(state, namespace, "GetBattlefieldVehicleInfo", query_vehicle)
}

fn read_map_vehicles(state: &LuaState, map_id: i32) -> LuaResult<Option<Vec<BattlefieldVehicle>>> {
    let sim = borrow_state(state)?;
    Ok(if sim.is_active_battlefield {
        sim.battlefield_vehicles.get(&map_id).cloned()
    } else {
        None
    })
}

fn query_vehicles(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = i32::from_stack(state, 1)?;
    let Some(vehicles) = read_map_vehicles(state, map_id)? else {
        return Ok(0);
    };
    let array = create_table(state);
    for (index, vehicle) in vehicles.iter().enumerate() {
        let record = create_vehicle_table(state, vehicle);
        set_table_array(state, array, (index + 1) as i64, record);
    }
    state.push(array);
    Ok(1)
}

fn query_vehicle(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let map_id = i32::from_stack(state, 2)?;
    let Some(vehicles) = read_map_vehicles(state, map_id)? else {
        return Ok(0);
    };
    let Some(vehicle) = index
        .checked_sub(1)
        .and_then(|index| vehicles.get(index as usize))
    else {
        return Ok(0);
    };
    let record = create_vehicle_table(state, vehicle);
    state.push(record);
    Ok(1)
}

fn create_vehicle_table(state: &mut LuaState, vehicle: &BattlefieldVehicle) -> Val {
    let table = create_table(state);
    let name = create_string(state, &vehicle.name);
    let atlas = create_string(state, &vehicle.atlas);
    for (key, value) in [
        ("x", Val::Num(vehicle.x)),
        ("y", Val::Num(vehicle.y)),
        ("name", name),
        ("isOccupied", Val::Bool(vehicle.is_occupied)),
        ("atlas", atlas),
        ("textureWidth", Val::Num(vehicle.texture_width)),
        ("textureHeight", Val::Num(vehicle.texture_height)),
        ("facing", Val::Num(vehicle.facing)),
        ("isPlayer", Val::Bool(vehicle.is_player)),
        ("isAlive", Val::Bool(vehicle.is_alive)),
        (
            "shouldDrawBelowPlayerBlips",
            Val::Bool(vehicle.should_draw_below_player_blips),
        ),
    ] {
        table_set_static(state, table, key, value);
    }
    table
}
