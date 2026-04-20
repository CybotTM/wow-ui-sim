//! C_PetJournal namespace.

use crate::lua_api::methods::{borrow_state, create_string, val_to_string};
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder, stack_val};
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
    let _ = i32::from_stack(state, 1)?;
    let st = borrow_state(state)?;
    let total = st.world.pets.len() as i32;
    let collected = st.world.pets.iter().filter(|pet| pet.is_collected).count() as i32;
    drop(st);
    (collected, total).into_stack(state)
}

#[derive(Clone)]
struct PetInfoSnapshot {
    species_id: u32,
    name: String,
    icon: u32,
    pet_type: i32,
    level: i32,
    is_collected: bool,
}

impl PetInfoSnapshot {
    fn from_pet(pet: &crate::lua_api::state_types::PetData) -> Self {
        Self {
            species_id: pet.species_id,
            name: pet.name.clone(),
            icon: pet.icon,
            pet_type: pet.pet_type,
            level: pet.level,
            is_collected: pet.is_collected,
        }
    }
}

fn push_pet_info(state: &mut LuaState, pet: PetInfoSnapshot) -> u32 {
    let name = create_string(state, &pet.name);
    let empty = create_string(state, "");
    state.push(Val::Num(pet.species_id as f64));
    state.push(empty.clone());
    state.push(Val::Num(pet.level as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Bool(pet.is_collected));
    state.push(name);
    state.push(Val::Num(pet.icon as f64));
    state.push(Val::Num(pet.pet_type as f64));
    state.push(Val::Num(pet.species_id as f64));
    10
}

fn pet_get_info_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let pet = {
        let st = borrow_state(state)?;
        let pet_index = (index - 1) as usize;
        st.world.pets.get(pet_index).map(PetInfoSnapshot::from_pet)
    };
    let Some(pet) = pet else {
        return Ok(0);
    };
    Ok(push_pet_info(state, pet))
}

fn pet_get_info_by_pet_id(state: &mut LuaState) -> LuaResult<u32> {
    let value = stack_val(state, 1);
    let pet = match value {
        Val::Str(_) => {
            let Some(pet_id) = val_to_string(state, value) else {
                return Ok(0);
            };
            let pet = {
                let st = borrow_state(state)?;
                st.world
                    .pets
                    .iter()
                    .find(|pet| pet.pet_id == pet_id)
                    .map(PetInfoSnapshot::from_pet)
            };
            if pet.is_some() {
                pet
            } else {
                pet_id.parse::<u32>().ok().and_then(|species_id| {
                    let st = borrow_state(state).ok()?;
                    st.world
                        .pets
                        .iter()
                        .find(|pet| pet.species_id == species_id)
                        .map(PetInfoSnapshot::from_pet)
                })
            }
        }
        Val::Num(species_id) => {
            let st = borrow_state(state)?;
            st.world
                .pets
                .iter()
                .find(|pet| pet.species_id == species_id as u32)
                .map(PetInfoSnapshot::from_pet)
        }
        _ => return Ok(0),
    };
    let Some(pet) = pet else {
        return Ok(0);
    };
    Ok(push_pet_info(state, pet))
}

fn pet_get_info_by_species_id(state: &mut LuaState) -> LuaResult<u32> {
    let species_id = u32::from_stack(state, 1)?;
    let pet = {
        let st = borrow_state(state)?;
        st.world
            .pets
            .iter()
            .find(|pet| pet.species_id == species_id)
            .map(PetInfoSnapshot::from_pet)
    };
    let Some(pet) = pet else {
        return Ok(0);
    };
    Ok(push_pet_info(state, pet))
}

pub fn register_rilua_pet_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let t = TableBuilder::new(lua.state_mut())
        .set_function("ClearRecentFanfares", |_state| Ok(0))?
        .set_function("ClearFanfare", |_state| Ok(0))?
        .set_function("ClearHoveredBattlePet", |_state| Ok(0))?
        .set_function("IsUsingDefaultFilters", |state| true.into_stack(state))?
        .set_function("SetDefaultFilters", |_state| Ok(0))?
        .set_function("GetNumPets", pet_get_num_pets)?
        .set_function("GetNumPetTypes", pet_get_num_pet_types)?
        .set_function("GetNumPetSources", pet_get_num_pet_sources)?
        .set_function("GetNumCollectedInfo", pet_get_num_collected_info)?
        .set_function("GetNumPetsNeedingFanfare", |state| (0i32).into_stack(state))?
        .set_function("GetBattlePetLink", |state| {
            state.push(Val::Nil);
            Ok(1)
        })?
        .set_function("GetPetInfoByIndex", pet_get_info_by_index)?
        .set_function("GetPetInfoByPetID", pet_get_info_by_pet_id)?
        .set_function("GetPetInfoBySpeciesID", pet_get_info_by_species_id)?
        .set_function("GetPetAbilityInfo", |state| {
            state.push(Val::Nil);
            Ok(1)
        })?
        .set_function("GetPetAbilityList", |state| {
            use crate::lua_api::methods::create_table;
            create_table(state).into_stack(state)
        })?
        .set_function("GetPetCooldownByGUID", |state| {
            (0.0f64, 0.0f64, false).into_stack(state)
        })?
        .set_function("GetPetLoadOutInfo", |state| {
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            Ok(6)
        })?
        .set_function("GetPetModelSceneInfoBySpeciesID", |state| {
            state.push(Val::Nil);
            Ok(1)
        })?
        .set_function("GetPetSortParameter", |state| {
            state.push(Val::Num(0.0));
            Ok(1)
        })?
        .set_function("GetPetStats", |state| {
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            Ok(8)
        })?
        .set_function("GetPetSummonInfo", |state| {
            state.push(Val::Nil);
            Ok(1)
        })?
        .set_function("GetSummonedPetGUID", |_state| Ok(0))?
        .set_function("GetSummonBattlePetCooldown", |state| {
            (0.0f64, 0.0f64, false).into_stack(state)
        })?
        .set_function("PetNeedsFanfare", |state| false.into_stack(state))?
        .set_function("HasFavoritePets", |state| false.into_stack(state))?
        .set_function("IsFindBattleEnabled", |state| false.into_stack(state))?
        .set_function("IsJournalUnlocked", |state| true.into_stack(state))?
        .set_function("IsFilterChecked", |state| false.into_stack(state))?
        .set_function("IsPetTypeChecked", |state| false.into_stack(state))?
        .set_function("IsPetSourceChecked", |state| false.into_stack(state))?
        .set_function("SetPetTypeFilter", |_state| Ok(0))?
        .set_function("SetPetSourceChecked", |_state| Ok(0))?
        .set_function("SetAllPetTypesChecked", |_state| Ok(0))?
        .set_function("SetAllPetSourcesChecked", |_state| Ok(0))?
        .set_function("SetFilterChecked", |_state| Ok(0))?
        .set_function("SetAbility", |_state| Ok(0))?
        .set_function("SetCustomName", |_state| Ok(0))?
        .set_function("SetFavorite", |_state| Ok(0))?
        .set_function("SetHoveredBattlePet", |_state| Ok(0))?
        .set_function("SetPetSortParameter", |_state| Ok(0))?
        .set_function("SetSearchFilter", |_state| Ok(0))?
        .set_function("SpellTargetBattlePet", |_state| Ok(0))?
        .set_function("SummonPetByGUID", |_state| Ok(0))?
        .set_function("SummonRandomPet", |_state| Ok(0))?
        .set_function("PickupPet", |_state| Ok(0))?
        .set_function("PickupSummonRandomPet", |_state| Ok(0))?
        .set_function("ReleasePetByID", |_state| Ok(0))?
        .set_function("CagePetByID", |_state| Ok(0))?
        .set_function("PetIsFavorite", |state| false.into_stack(state))?
        .set_function("PetIsHurt", |state| false.into_stack(state))?
        .set_function("PetIsLockedForConvert", |state| false.into_stack(state))?
        .set_function("PetIsRevoked", |state| false.into_stack(state))?
        .set_function("PetIsSlotted", |state| false.into_stack(state))?
        .set_function("PetIsSummonable", |state| false.into_stack(state))?
        .set_function("PetIsTradable", |state| false.into_stack(state))?
        .build();

    set_global_val(lua.state_mut(), "C_PetJournal", t);
    Ok(())
}
