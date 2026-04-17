//! C_PetJournal namespace.

use crate::lua_api::rilua_methods::{borrow_state, create_string};
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

use super::set_global_val;

fn push_pet_info(
    state: &mut LuaState,
    name: &str,
    species: f64,
    level: f64,
    icon: f64,
    pet_type: f64,
) -> u32 {
    state.push(Val::Num(species));
    state.push(Val::Nil);
    state.push(Val::Num(level));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    let name_val = create_string(state, name);
    let empty_1 = create_string(state, "");
    let empty_2 = create_string(state, "");
    state.push(Val::Bool(false));
    state.push(name_val);
    state.push(Val::Num(icon));
    state.push(Val::Num(pet_type));
    state.push(Val::Num(0.0));
    state.push(empty_1);
    state.push(empty_2);
    state.push(Val::Bool(false));
    state.push(Val::Bool(true));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    17
}

pub fn register_rilua_pet_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let t = TableBuilder::new(lua.state_mut())
        .set_function("GetNumPets", |state| {
            let st = borrow_state(state)?;
            let total = st.world.pets.len() as i32;
            let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            drop(st);
            (total, owned).into_stack(state)
        })?
        .set_function("GetNumCollectedInfo", |state| {
            let st = borrow_state(state)?;
            let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            let total = st.world.pets.len() as i32;
            drop(st);
            (owned, total).into_stack(state)
        })?
        .set_function("GetNumPetsNeedingFanfare", |state| (0i32).into_stack(state))?
        .set_function("GetPetInfoByIndex", |state| {
            let index = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let i = (index - 1) as usize;
            let Some(p) = st.world.pets.get(i) else {
                drop(st);
                return Ok(0);
            };
            let species = p.species_id as f64;
            let level = p.level as f64;
            let icon = p.icon as f64;
            let pet_type = p.pet_type as f64;
            let name_str = p.name.clone();
            drop(st);
            // speciesId, nil, level, 0, 0, 0, false, name, icon, petType, 0, "", "", false, true, false, false
            Ok(push_pet_info(
                state, &name_str, species, level, icon, pet_type,
            ))
        })?
        .set_function("GetPetInfoByPetID", |_state| {
            // TODO: lookup by pet_id string
            Ok(0)
        })?
        .set_function("GetPetInfoBySpeciesID", |_state| {
            // TODO: lookup by species_id
            Ok(0)
        })?
        .set_function("PetIsSummonable", |state| false.into_stack(state))?
        .build();

    set_global_val(lua.state_mut(), "C_PetJournal", t);
    Ok(())
}
