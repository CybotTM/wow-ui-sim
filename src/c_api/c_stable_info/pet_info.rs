//! Explicit modern stable-slot observations, independent of active-list membership.
//! Retail StableInfoDocumentation.lua:66–79,193–214; PTR:88–101,215–241.

use super::active_list::{decode_record, project_record};
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, table_get};
use crate::lua_bridge::{TableBuilder, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    if matches!(ACTIVE, ClientProfile::Retail | ClientProfile::Ptr) {
        let namespace = super::ensure_namespace(state, "C_StableInfo")?;
        table_set_rust_fn_static(state, namespace, "GetStablePetInfo", query_slot)?;
    }
    Ok(())
}

pub(crate) fn register_admin(builder: TableBuilder) -> LuaResult<TableBuilder> {
    if !matches!(ACTIVE, ClientProfile::Retail | ClientProfile::Ptr) {
        return Ok(builder);
    }
    builder
        .set_function("SetStablePetInfo", supply_slot)?
        .set_function("ClearStablePetInfo", clear_slot)
}

fn query_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot = read_slot(state)?;
    let observation = borrow_state(state)?.stable_pet_slots.get(&slot).cloned();
    let observation = observation.ok_or_else(|| {
        runtime_error(format!(
            "native-unknown: C_StableInfo.GetStablePetInfo has no supplied observation for slot {slot} or documented default"
        ))
    })?;
    let result = match observation {
        Some(pet) => project_record(state, &pet),
        None => Val::Nil,
    };
    state.push(result);
    Ok(1)
}

fn supply_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot = read_slot(state)?;
    let row = stack_val(state, 2);
    let observation = if matches!(row, Val::Nil) {
        None
    } else {
        let pet = decode_record(state, row)?;
        if table_get(state, row, "slotID") != Val::Num(f64::from(slot)) {
            return Err(runtime_error("PetInfo.slotID must match the supplied slot"));
        }
        Some(pet)
    };
    borrow_state_mut(state)?
        .stable_pet_slots
        .insert(slot, observation);
    Ok(0)
}

fn clear_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot = read_slot(state)?;
    borrow_state_mut(state)?.stable_pet_slots.remove(&slot);
    Ok(0)
}

fn read_slot(state: &LuaState) -> LuaResult<u32> {
    match stack_val(state, 1) {
        Val::Num(slot) if slot >= 1.0 && slot <= f64::from(u32::MAX) && slot.fract() == 0.0 => {
            Ok(slot as u32)
        }
        _ => Err(runtime_error(
            "stable slot must be a positive integral luaIndex",
        )),
    }
}
