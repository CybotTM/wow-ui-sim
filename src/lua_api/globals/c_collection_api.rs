//! C_Collection namespaces for pets, mounts, and toys.
//!
//! Transmog/heirloom APIs are in c_collection_transmog.rs.

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Map Enum.TransmogCollectionType category ID → (name, is_weapon, can_enchant, can_main_hand, can_off_hand).
///
/// Armor categories map to specific equipment slots via armorCategoryID in TRANSMOG_SLOTS.
/// Weapon categories use can_main_hand / can_off_hand to determine slot.
pub(super) fn transmog_category_info(category_id: i32) -> Option<(&'static str, bool, bool, bool, bool)> {
    //                             name                weapon  enchant main   off
    match category_id {
        1  => Some(("Head",                false, false, false, false)),
        2  => Some(("Shoulder",            false, false, false, false)),
        3  => Some(("Back",                false, false, false, false)),
        4  => Some(("Chest",               false, false, false, false)),
        5  => Some(("Shirt",               false, false, false, false)),
        6  => Some(("Tabard",              false, false, false, false)),
        7  => Some(("Wrist",               false, false, false, false)),
        8  => Some(("Hands",               false, false, false, false)),
        9  => Some(("Waist",               false, false, false, false)),
        10 => Some(("Legs",                false, false, false, false)),
        11 => Some(("Feet",                false, false, false, false)),
        12 => Some(("Wand",                true,  true,  true,  true)),
        13 => Some(("One-Handed Axes",     true,  true,  true,  true)),
        14 => Some(("One-Handed Swords",   true,  true,  true,  true)),
        15 => Some(("One-Handed Maces",    true,  true,  true,  true)),
        16 => Some(("Daggers",             true,  true,  true,  true)),
        17 => Some(("Fist Weapons",        true,  true,  true,  true)),
        18 => Some(("Shields",             true,  false, false, true)),
        19 => Some(("Held In Off-hand",    true,  false, false, true)),
        20 => Some(("Two-Handed Axes",     true,  true,  true,  false)),
        21 => Some(("Two-Handed Swords",   true,  true,  true,  false)),
        22 => Some(("Two-Handed Maces",    true,  true,  true,  false)),
        23 => Some(("Staves",              true,  true,  true,  false)),
        24 => Some(("Polearms",            true,  true,  true,  false)),
        25 => Some(("Bows",                true,  true,  true,  false)),
        26 => Some(("Guns",                true,  true,  true,  false)),
        27 => Some(("Crossbows",           true,  true,  true,  false)),
        28 => Some(("Warglaives",          true,  true,  true,  true)),
        29 => Some(("Paired",              true,  true,  true,  false)),
        _  => None,
    }
}

/// Coerce a Lua value to i32: booleans map to 0/1, numbers pass through.
pub(super) fn bool_or_int_to_i32(v: Value) -> i32 {
    match v {
        Value::Boolean(b) => b as i32,
        Value::Integer(n) => n as i32,
        Value::Number(n) => n as i32,
        _ => 0,
    }
}

/// Register collection-related C_* namespaces.
pub fn register_c_collection_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_pet_journal(lua, Rc::clone(&state))?;
    register_mount_journal(lua, Rc::clone(&state))?;
    register_toy_box(lua, Rc::clone(&state))?;
    super::c_collection_transmog::register_transmog_and_heirloom_apis(lua, state)?;
    Ok(())
}

// ============================================================================
// Stub helpers (shared with c_collection_transmog)
// ============================================================================

pub(super) fn add_i32_stub(lua: &Lua, t: &mlua::Table, name: &str, value: i32) -> Result<()> {
    t.set(name, lua.create_function(move |_, ()| Ok(value))?)
}

pub(super) fn add_empty_table_stub(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(name, lua.create_function(|lua, ()| lua.create_table())?)
}

pub(super) fn add_i32_stub_with_arg<A: mlua::FromLuaMulti>(lua: &Lua, t: &mlua::Table, name: &str, value: i32) -> Result<()> {
    t.set(name, lua.create_function(move |_, _: A| Ok(value))?)
}

pub(super) fn add_bool_stub_with_arg<A: mlua::FromLuaMulti>(lua: &Lua, t: &mlua::Table, name: &str, value: bool) -> Result<()> {
    t.set(name, lua.create_function(move |_, _: A| Ok(value))?)
}

pub(super) fn add_bool_stub(lua: &Lua, t: &mlua::Table, name: &str, value: bool) -> Result<()> {
    t.set(name, lua.create_function(move |_, ()| Ok(value))?)
}

pub(super) fn add_table_stub_with_arg<A: mlua::FromLuaMulti>(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(name, lua.create_function(|lua, _: A| lua.create_table())?)
}

pub(super) fn add_nil_stub_with_arg<A: mlua::FromLuaMulti>(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(name, lua.create_function(|_, _: A| Ok(Value::Nil))?)
}

// ============================================================================
// C_PetJournal
// ============================================================================

fn register_pet_journal(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_pet_count_methods(lua, &t, Rc::clone(&state))?;
    register_pet_info_methods(lua, &t, state)?;
    register_pet_info_stubs(lua, &t)?;
    lua.globals().set("C_PetJournal", t)?;
    Ok(())
}

fn register_pet_count_methods(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetNumPets", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| {
            let st = s.borrow();
            let total = st.world.pets.len() as i32;
            let owned = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            Ok((total, owned))
        }
    })?)?;
    t.set("GetNumCollectedInfo", lua.create_function({
        let s = Rc::clone(&state);
        move |_, _species_id: i32| {
            let st = s.borrow();
            let collected = st.world.pets.iter().filter(|p| p.is_collected).count() as i32;
            let total = st.world.pets.len() as i32;
            Ok((collected, total))
        }
    })?)?;
    add_i32_stub(lua, t, "GetNumPetsNeedingFanfare", 0)
}

fn register_pet_info_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("PetIsSummonable", lua.create_function(|_, _: String| Ok(false))?)
}

fn register_pet_info_methods(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetPetInfoByIndex", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, index: i32| {
            let st = s.borrow();
            let i = (index - 1) as usize;
            let Some(p) = st.world.pets.get(i) else { return Ok(mlua::MultiValue::new()); };
            build_pet_info_multi(lua, p)
        }
    })?)?;
    t.set("GetPetInfoByPetID", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, pet_id: String| {
            let st = s.borrow();
            let Some(p) = st.world.pets.iter().find(|p| p.pet_id == pet_id) else { return Ok(mlua::MultiValue::new()); };
            build_pet_info_multi(lua, p)
        }
    })?)?;
    t.set("GetPetInfoBySpeciesID", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, species_id: u32| {
            let st = s.borrow();
            let Some(p) = st.world.pets.iter().find(|p| p.species_id == species_id) else { return Ok(mlua::MultiValue::new()); };
            build_pet_info_multi(lua, p)
        }
    })?)?;
    Ok(())
}

fn build_pet_info_multi(lua: &Lua, p: &crate::lua_api::state_types::PetData) -> mlua::Result<mlua::MultiValue> {
    Ok(mlua::MultiValue::from_vec(vec![
        Value::Integer(p.species_id as i64),
        Value::Nil,
        Value::Integer(p.level as i64),
        Value::Integer(0), Value::Integer(0), Value::Integer(0),
        Value::Boolean(false),
        Value::String(lua.create_string(&p.name)?),
        Value::Integer(p.icon as i64),
        Value::Integer(p.pet_type as i64),
        Value::Integer(0),
        Value::String(lua.create_string("")?),
        Value::String(lua.create_string("")?),
        Value::Boolean(false), Value::Boolean(true), Value::Boolean(false), Value::Boolean(false),
    ]))
}

// ============================================================================
// C_MountJournal
// ============================================================================

fn register_mount_journal(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_mount_info_methods(lua, &t, state)?;
    register_mount_filter_methods(lua, &t)?;
    lua.globals().set("C_MountJournal", t)?;
    Ok(())
}

fn register_mount_info_methods(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetNumMounts", lua.create_function({ let s = Rc::clone(&state); move |_, ()| Ok(s.borrow().world.mounts.len() as i32) })?)?;
    t.set("GetNumDisplayedMounts", lua.create_function({ let s = Rc::clone(&state); move |_, ()| Ok(s.borrow().world.mounts.len() as i32) })?)?;
    register_get_mount_info_by_id(lua, t, Rc::clone(&state))?;
    register_get_mount_info_extra_by_id(lua, t, Rc::clone(&state))?;
    register_get_displayed_mount_info(lua, t, Rc::clone(&state))?;
    add_empty_table_stub(lua, t, "GetMountIDs")?;
    add_i32_stub(lua, t, "GetNumMountsNeedingFanfare", 0)?;
    Ok(())
}

fn register_get_displayed_mount_info(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetDisplayedMountInfo", lua.create_function(move |lua, index: i32| {
        let st = state.borrow();
        let i = (index - 1) as usize;
        let Some(m) = st.world.mounts.get(i) else { return Ok(mlua::MultiValue::new()); };
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(&m.name)?), Value::Integer(m.spell_id as i64),
            Value::Integer(m.icon as i64), Value::Boolean(false), Value::Boolean(m.is_usable),
            Value::Integer(0), Value::Boolean(false), Value::Boolean(false), Value::Nil,
            Value::Boolean(false), Value::Boolean(m.is_collected), Value::Integer(m.mount_id as i64),
        ]))
    })?)
}

fn register_get_mount_info_by_id(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetMountInfoByID", lua.create_function(move |lua, mount_id: u32| {
        let st = state.borrow();
        let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else { return Ok(mlua::MultiValue::new()); };
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(&m.name)?), Value::Integer(m.spell_id as i64),
            Value::Integer(m.icon as i64), Value::Boolean(false), Value::Boolean(m.is_usable),
            Value::Integer(0), Value::Boolean(false), Value::Boolean(false), Value::Nil,
            Value::Boolean(false), Value::Boolean(m.is_collected), Value::Integer(m.mount_id as i64),
        ]))
    })?)
}

fn register_get_mount_info_extra_by_id(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetMountInfoExtraByID", lua.create_function(move |lua, mount_id: u32| {
        let st = state.borrow();
        let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else { return Ok(mlua::MultiValue::new()); };
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Integer(0), Value::String(lua.create_string("")?),
            Value::String(lua.create_string("Drop")?), Value::Boolean(false),
            Value::Integer(m.mount_type as i64), Value::Integer(0), Value::Integer(0),
            Value::Integer(0), Value::Boolean(false),
        ]))
    })?)
}

fn register_mount_filter_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetCollectedFilterSetting", lua.create_function(|_, _: i32| Ok(true))?)?;
    t.set("SetCollectedFilterSetting", lua.create_function(|_, (_, _): (i32, bool)| Ok(()))?)?;
    t.set("GetIsFavorite", lua.create_function(|_, _: i32| Ok((false, false)))?)?;
    t.set("SetIsFavorite", lua.create_function(|_, (_, _): (i32, bool)| Ok(()))?)?;
    t.set("Summon", lua.create_function(|_, _: i32| Ok(()))?)?;
    t.set("Dismiss", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

// ============================================================================
// C_ToyBox
// ============================================================================

fn register_toy_box(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_toy_count_methods(lua, &t, Rc::clone(&state))?;
    register_toy_info_methods(lua, &t, Rc::clone(&state))?;
    t.set("IsToyUsable", lua.create_function({ let s = Rc::clone(&state); move |_, item_id: i32| {
        Ok(s.borrow().world.toys.iter().find(|t| t.item_id == item_id as u32).map(|t| t.is_usable).unwrap_or(false))
    }})?)?;
    t.set("GetIsFavorite", lua.create_function({ let s = Rc::clone(&state); move |_, item_id: i32| Ok(s.borrow().world.favorite_toys.contains(&(item_id as u32))) })?)?;
    t.set("HasFavorites", lua.create_function({ let s = Rc::clone(&state); move |_, ()| Ok(!s.borrow().world.favorite_toys.is_empty()) })?)?;
    t.set("GetToyLink", lua.create_function({ let s = Rc::clone(&state); move |_, item_id: i32| {
        let st = s.borrow();
        match st.world.toys.iter().find(|t| t.item_id == item_id as u32) {
            Some(toy) => Ok(Some(format!("|cff0070dd|Hitem:{}::::::::1:0|h[{}]|h|r", toy.item_id, toy.name))),
            None => Ok(None),
        }
    }})?)?;
    t.set("SetIsFavorite", lua.create_function({ let s = state; move |_, (item_id, is_fav): (i32, bool)| {
        let mut st = s.borrow_mut();
        if is_fav { st.world.favorite_toys.insert(item_id as u32); } else { st.world.favorite_toys.remove(&(item_id as u32)); }
        Ok(())
    }})?)?;
    register_toy_filter_stubs(lua, &t)?;
    lua.globals().set("C_ToyBox", t)?;
    Ok(())
}

fn register_toy_filter_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetCollectedShown", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("GetUncollectedShown", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("GetUnusableShown", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("SetCollectedShown", lua.create_function(|_, _: bool| Ok(()))?)?;
    t.set("SetUncollectedShown", lua.create_function(|_, _: bool| Ok(()))?)?;
    t.set("SetUnusableShown", lua.create_function(|_, _: bool| Ok(()))?)?;
    t.set("ForceToyRefilter", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

fn register_toy_count_methods(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetNumTotalDisplayedToys", lua.create_function({ let s = Rc::clone(&state); move |_, ()| Ok(s.borrow().world.toys.len() as i32) })?)?;
    t.set("GetNumLearnedDisplayedToys", lua.create_function({ let s = Rc::clone(&state); move |_, ()| Ok(s.borrow().world.toys.iter().filter(|t| t.is_collected).count() as i32) })?)?;
    t.set("GetNumToys", lua.create_function({ let s = Rc::clone(&state); move |_, ()| Ok(s.borrow().world.toys.len() as i32) })?)?;
    t.set("GetNumFilteredToys", lua.create_function({ let s = Rc::clone(&state); move |_, ()| Ok(s.borrow().world.toys.len() as i32) })?)
}

fn register_toy_info_methods(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set("GetToyFromIndex", lua.create_function({ let s = Rc::clone(&state); move |_, index: i32| {
        let st = s.borrow(); let i = (index - 1) as usize;
        Ok(st.world.toys.get(i).map_or(0, |t| t.item_id as i32))
    }})?)?;
    t.set("GetToyInfo", lua.create_function({ let s = Rc::clone(&state); move |lua, item_id: u32| {
        let st = s.borrow();
        let Some(toy) = st.world.toys.iter().find(|t| t.item_id == item_id) else { return Ok(mlua::MultiValue::new()); };
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Integer(toy.item_id as i64), Value::String(lua.create_string(&toy.name)?),
            Value::Integer(toy.icon as i64), Value::Boolean(false), Value::Boolean(false), Value::Integer(1),
        ]))
    }})?)
}
