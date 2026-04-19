//! C_PetJournal namespace.

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

use super::set_global_val;

fn pet_get_num_pets(state: &mut LuaState) -> LuaResult<u32> {
    let st = borrow_state(state)?;
    let total = st.world.pets.len() as i32;
    let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
    drop(st);
    (total, owned).into_stack(state)
}

fn pet_get_num_pet_types(state: &mut LuaState) -> LuaResult<u32> {
    (10i32).into_stack(state)
}

fn pet_get_num_pet_sources(state: &mut LuaState) -> LuaResult<u32> {
    (10i32).into_stack(state)
}

fn pet_get_num_collected_info(state: &mut LuaState) -> LuaResult<u32> {
    (0i32, 0i32).into_stack(state)
}

fn pet_get_info_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let _ = i32::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}

pub fn register_rilua_pet_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let t = TableBuilder::new(lua.state_mut())
        .set_function("ClearRecentFanfares", |_state| Ok(0))?
        .set_function("GetNumPets", pet_get_num_pets)?
        .set_function("GetNumPetTypes", pet_get_num_pet_types)?
        .set_function("GetNumPetSources", pet_get_num_pet_sources)?
        .set_function("GetNumCollectedInfo", pet_get_num_collected_info)?
        .set_function("GetNumPetsNeedingFanfare", |state| (0i32).into_stack(state))?
        .set_function("GetPetInfoByIndex", pet_get_info_by_index)?
        .set_function("GetPetInfoByPetID", |_state| {
            // TODO: lookup by pet_id string
            Ok(0)
        })?
        .set_function("GetPetInfoBySpeciesID", |_state| {
            // TODO: lookup by species_id
            Ok(0)
        })?
        .set_function("GetSummonedPetGUID", |_state| Ok(0))?
        .set_function("GetSummonBattlePetCooldown", |state| {
            (0.0f64, 0.0f64, false).into_stack(state)
        })?
        .set_function("PetNeedsFanfare", |state| false.into_stack(state))?
        .set_function("PetIsSummonable", |state| false.into_stack(state))?
        .build();

    set_global_val(lua.state_mut(), "C_PetJournal", t);
    Ok(())
}
