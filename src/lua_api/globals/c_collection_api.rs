//! C_Collection namespaces for mounts, pets, toys, transmog, and heirlooms.
//!
//! Contains collection journal API functions for various game collectibles.

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Map Enum.TransmogCollectionType category ID → (name, is_weapon, can_enchant, can_main_hand, can_off_hand).
///
/// Armor categories map to specific equipment slots via armorCategoryID in TRANSMOG_SLOTS.
/// Weapon categories use can_main_hand / can_off_hand to determine slot.
fn transmog_category_info(category_id: i32) -> Option<(&'static str, bool, bool, bool, bool)> {
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
fn bool_or_int_to_i32(v: Value) -> i32 {
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
    register_transmog_collection(lua, Rc::clone(&state))?;
    register_transmog(lua, Rc::clone(&state))?;
    register_transmog_util(lua)?;
    register_heirloom(lua, state)?;
    register_transmog_sets(lua)?;
    Ok(())
}

/// C_PetJournal namespace - battle pet utilities.
fn register_pet_journal(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_pet_count_methods(lua, &t, Rc::clone(&state))?;
    register_pet_info_methods(lua, &t, state)?;
    register_pet_info_stubs(lua, &t)?;
    lua.globals().set("C_PetJournal", t)?;
    Ok(())
}

fn register_pet_count_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
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

fn register_pet_info_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetPetInfoByIndex", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, index: i32| {
            let st = s.borrow();
            let i = (index - 1) as usize;
            let Some(p) = st.world.pets.get(i) else {
                return Ok(mlua::MultiValue::new());
            };
            build_pet_info_multi(lua, p)
        }
    })?)?;
    t.set("GetPetInfoByPetID", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, pet_id: String| {
            let st = s.borrow();
            let Some(p) = st.world.pets.iter().find(|p| p.pet_id == pet_id) else {
                return Ok(mlua::MultiValue::new());
            };
            build_pet_info_multi(lua, p)
        }
    })?)?;
    t.set("GetPetInfoBySpeciesID", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, species_id: u32| {
            let st = s.borrow();
            let Some(p) = st.world.pets.iter().find(|p| p.species_id == species_id) else {
                return Ok(mlua::MultiValue::new());
            };
            build_pet_info_multi(lua, p)
        }
    })?)?;
    Ok(())
}

/// Build the multi-return for pet info queries.
/// Returns: speciesID, customName, level, xp, maxXp, displayID, isFavorite,
///          name, icon, petType, creatureID, sourceText, description,
///          isWild, canBattle, isTradeable, isUnique
fn build_pet_info_multi(
    lua: &Lua,
    p: &crate::lua_api::state_types::PetData,
) -> mlua::Result<mlua::MultiValue> {
    Ok(mlua::MultiValue::from_vec(vec![
        Value::Integer(p.species_id as i64),           // speciesID
        Value::Nil,                                    // customName
        Value::Integer(p.level as i64),                // level
        Value::Integer(0),                             // xp
        Value::Integer(0),                             // maxXp
        Value::Integer(0),                             // displayID
        Value::Boolean(false),                         // isFavorite
        Value::String(lua.create_string(&p.name)?),    // name
        Value::Integer(p.icon as i64),                 // icon
        Value::Integer(p.pet_type as i64),             // petType
        Value::Integer(0),                             // creatureID
        Value::String(lua.create_string("")?),         // sourceText
        Value::String(lua.create_string("")?),         // description
        Value::Boolean(false),                         // isWild
        Value::Boolean(true),                          // canBattle
        Value::Boolean(false),                         // isTradeable
        Value::Boolean(false),                         // isUnique
    ]))
}

/// C_MountJournal namespace - mount collection.
fn register_mount_journal(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_mount_info_methods(lua, &t, state)?;
    register_mount_filter_methods(lua, &t)?;
    lua.globals().set("C_MountJournal", t)?;
    Ok(())
}

/// Mount info query methods: counts, GetMountInfoByID, GetMountIDs.
fn register_mount_info_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetNumMounts", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.mounts.len() as i32)
    })?)?;
    t.set("GetNumDisplayedMounts", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.mounts.len() as i32)
    })?)?;
    register_get_mount_info_by_id(lua, t, Rc::clone(&state))?;
    register_get_mount_info_extra_by_id(lua, t, Rc::clone(&state))?;
    register_get_displayed_mount_info(lua, t, Rc::clone(&state))?;
    add_empty_table_stub(lua, t, "GetMountIDs")?;
    add_i32_stub(lua, t, "GetNumMountsNeedingFanfare", 0)?;
    Ok(())
}

fn register_get_displayed_mount_info(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetDisplayedMountInfo", lua.create_function(move |lua, index: i32| {
        let st = state.borrow();
        let i = (index - 1) as usize;
        let Some(m) = st.world.mounts.get(i) else {
            return Ok(mlua::MultiValue::new());
        };
        // name, spellID, icon, isActive, isUsable, sourceType,
        // isFavorite, isFactionSpecific, faction, shouldHideOnChar, isCollected, mountID
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(&m.name)?),
            Value::Integer(m.spell_id as i64),
            Value::Integer(m.icon as i64),
            Value::Boolean(false),
            Value::Boolean(m.is_usable),
            Value::Integer(0),
            Value::Boolean(false),
            Value::Boolean(false),
            Value::Nil,
            Value::Boolean(false),
            Value::Boolean(m.is_collected),
            Value::Integer(m.mount_id as i64),
        ]))
    })?)
}

/// GetMountInfoByID returns the same 12 values as GetDisplayedMountInfo but keyed by mount_id.
fn register_get_mount_info_by_id(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetMountInfoByID", lua.create_function(move |lua, mount_id: u32| {
        let st = state.borrow();
        let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else {
            return Ok(mlua::MultiValue::new());
        };
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(&m.name)?),
            Value::Integer(m.spell_id as i64),
            Value::Integer(m.icon as i64),
            Value::Boolean(false),
            Value::Boolean(m.is_usable),
            Value::Integer(0),
            Value::Boolean(false),
            Value::Boolean(false),
            Value::Nil,
            Value::Boolean(false),
            Value::Boolean(m.is_collected),
            Value::Integer(m.mount_id as i64),
        ]))
    })?)
}

/// GetMountInfoExtraByID returns: creatureDisplayInfoID, description, source, isSelfMount,
/// mountTypeID, uiModelSceneID, animID, spellVisualKitID, disablePlayerMountPreview.
fn register_get_mount_info_extra_by_id(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetMountInfoExtraByID", lua.create_function(move |lua, mount_id: u32| {
        let st = state.borrow();
        let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else {
            return Ok(mlua::MultiValue::new());
        };
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Integer(0),                                 // creatureDisplayInfoID
            Value::String(lua.create_string("")?),             // description
            Value::String(lua.create_string("Drop")?),         // source
            Value::Boolean(false),                             // isSelfMount
            Value::Integer(m.mount_type as i64),               // mountTypeID
            Value::Integer(0),                                 // uiModelSceneID
            Value::Integer(0),                                 // animID
            Value::Integer(0),                                 // spellVisualKitID
            Value::Boolean(false),                             // disablePlayerMountPreview
        ]))
    })?)
}

fn add_i32_stub(lua: &Lua, t: &mlua::Table, name: &str, value: i32) -> Result<()> {
    t.set(name, lua.create_function(move |_, ()| Ok(value))?)
}

fn add_empty_table_stub(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(name, lua.create_function(|lua, ()| lua.create_table())?)
}

fn add_i32_stub_with_arg<A: mlua::FromLuaMulti>(
    lua: &Lua,
    t: &mlua::Table,
    name: &str,
    value: i32,
) -> Result<()> {
    t.set(name, lua.create_function(move |_, _: A| Ok(value))?)
}

fn add_bool_stub_with_arg<A: mlua::FromLuaMulti>(
    lua: &Lua,
    t: &mlua::Table,
    name: &str,
    value: bool,
) -> Result<()> {
    t.set(name, lua.create_function(move |_, _: A| Ok(value))?)
}

fn add_bool_stub(lua: &Lua, t: &mlua::Table, name: &str, value: bool) -> Result<()> {
    t.set(name, lua.create_function(move |_, ()| Ok(value))?)
}

fn add_table_stub_with_arg<A: mlua::FromLuaMulti>(
    lua: &Lua,
    t: &mlua::Table,
    name: &str,
) -> Result<()> {
    t.set(name, lua.create_function(|lua, _: A| lua.create_table())?)
}

fn add_nil_stub_with_arg<A: mlua::FromLuaMulti>(
    lua: &Lua,
    t: &mlua::Table,
    name: &str,
) -> Result<()> {
    t.set(name, lua.create_function(|_, _: A| Ok(Value::Nil))?)
}

/// Mount filter and favorite methods: collected filter, favorites, summon/dismiss.
fn register_mount_filter_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetCollectedFilterSetting",
        lua.create_function(|_, _filter_index: i32| Ok(true))?,
    )?;
    t.set(
        "SetCollectedFilterSetting",
        lua.create_function(|_, (_filter_index, _is_checked): (i32, bool)| Ok(()))?,
    )?;
    t.set(
        "GetIsFavorite",
        lua.create_function(|_, _mount_index: i32| Ok((false, false)))?,
    )?;
    t.set(
        "SetIsFavorite",
        lua.create_function(|_, (_mount_index, _is_favorite): (i32, bool)| Ok(()))?,
    )?;
    t.set("Summon", lua.create_function(|_, _mount_id: i32| Ok(()))?)?;
    t.set("Dismiss", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

/// C_ToyBox namespace - toy collection.
fn register_toy_box(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_toy_count_methods(lua, &t, Rc::clone(&state))?;
    register_toy_info_methods(lua, &t, Rc::clone(&state))?;
    t.set("IsToyUsable", lua.create_function({
        let s = Rc::clone(&state);
        move |_, item_id: i32| {
            let st = s.borrow();
            Ok(st.world.toys.iter()
                .find(|t| t.item_id == item_id as u32)
                .map(|t| t.is_usable)
                .unwrap_or(false))
        }
    })?)?;
    t.set("GetIsFavorite", lua.create_function({
        let s = Rc::clone(&state);
        move |_, item_id: i32| Ok(s.borrow().world.favorite_toys.contains(&(item_id as u32)))
    })?)?;
    t.set("HasFavorites", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(!s.borrow().world.favorite_toys.is_empty())
    })?)?;
    t.set("SetIsFavorite", lua.create_function({
        let s = state;
        move |_, (item_id, is_fav): (i32, bool)| {
            let mut st = s.borrow_mut();
            if is_fav {
                st.world.favorite_toys.insert(item_id as u32);
            } else {
                st.world.favorite_toys.remove(&(item_id as u32));
            }
            Ok(())
        }
    })?)?;
    // Filter stubs: show all by default, setters are no-ops
    t.set("GetCollectedShown", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("GetUncollectedShown", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("GetUnusableShown", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("SetCollectedShown", lua.create_function(|_, _: bool| Ok(()))?)?;
    t.set("SetUncollectedShown", lua.create_function(|_, _: bool| Ok(()))?)?;
    t.set("SetUnusableShown", lua.create_function(|_, _: bool| Ok(()))?)?;
    lua.globals().set("C_ToyBox", t)?;
    Ok(())
}

fn register_toy_count_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetNumTotalDisplayedToys", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.toys.len() as i32)
    })?)?;
    t.set("GetNumLearnedDisplayedToys", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| {
            Ok(s.borrow().world.toys.iter().filter(|t| t.is_collected).count() as i32)
        }
    })?)?;
    t.set("GetNumToys", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.toys.len() as i32)
    })?)?;
    t.set("GetNumFilteredToys", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.toys.len() as i32)
    })?)
}

fn register_toy_info_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetToyFromIndex", lua.create_function({
        let s = Rc::clone(&state);
        move |_, index: i32| {
            let st = s.borrow();
            let i = (index - 1) as usize;
            Ok(st.world.toys.get(i).map_or(0, |t| t.item_id as i32))
        }
    })?)?;
    t.set("GetToyInfo", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, item_id: u32| {
            let st = s.borrow();
            let Some(toy) = st.world.toys.iter().find(|t| t.item_id == item_id) else {
                return Ok(mlua::MultiValue::new());
            };
            // itemID, toyName, icon, isFavorite, hasFanfare, itemQuality
            Ok(mlua::MultiValue::from_vec(vec![
                Value::Integer(toy.item_id as i64),
                Value::String(lua.create_string(&toy.name)?),
                Value::Integer(toy.icon as i64),
                Value::Boolean(false),
                Value::Boolean(false),
                Value::Integer(1), // common quality
            ]))
        }
    })?)
}

/// C_TransmogCollection namespace - transmog/appearance collection.
fn register_transmog_collection(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_transmog_appearance_methods(lua, &t, &state)?;
    register_transmog_outfit_methods(lua, &t)?;
    register_transmog_source_methods(lua, &t, &state)?;
    lua.globals().set("C_TransmogCollection", t)?;
    Ok(())
}

/// Appearance query methods: sources, info, camera, categories.
fn register_transmog_appearance_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("GetAppearanceSources", lua.create_function({
        let s = Rc::clone(state);
        move |lua, visual_id: i32| {
            let st = s.borrow();
            let sources: Vec<_> = st.world.transmog_appearances.iter()
                .filter(|a| a.visual_id == visual_id)
                .collect();
            let result = lua.create_table()?;
            for (i, a) in sources.iter().enumerate() {
                result.set(i + 1, build_source_info(lua, a)?)?;
            }
            Ok(result)
        }
    })?)?;
    t.set(
        "GetSourceInfo",
        lua.create_function({
            let s = Rc::clone(state);
            move |lua, source_id: i32| {
                let st = s.borrow();
                if let Some(a) = st.world.transmog_appearances.iter().find(|a| a.source_id == source_id) {
                    build_source_info(lua, a)
                } else {
                    build_empty_source_info(lua)
                }
            }
        })?,
    )?;
    add_table_stub_with_arg::<i32>(lua, t, "GetAllAppearanceSources")?;
    add_i32_stub_with_arg::<i32>(lua, t, "GetAppearanceCameraID", 0)?;
    t.set("GetCategoryAppearances", lua.create_function({
        let s = Rc::clone(state);
        move |lua, (category_id, _location): (i32, Value)| {
            let st = s.borrow();
            // Group by visual_id, deduplicate (one entry per unique visual)
            let mut seen_visuals = std::collections::HashSet::new();
            let result = lua.create_table()?;
            let mut idx = 0;
            for a in &st.world.transmog_appearances {
                if a.category_id == category_id && seen_visuals.insert(a.visual_id) {
                    idx += 1;
                    let entry = lua.create_table()?;
                    entry.set("visualID", a.visual_id)?;
                    entry.set("isCollected", a.is_collected)?;
                    entry.set("isUsable", true)?;
                    entry.set("isFavorite", false)?;
                    entry.set("isHideVisual", false)?;
                    entry.set("uiOrder", idx)?;
                    entry.set("hasActiveRequiredHoliday", false)?;
                    entry.set("hasRequiredHoliday", false)?;
                    entry.set("canDisplayOnPlayer", true)?;
                    entry.set("exclusions", 0)?;
                    result.set(idx, entry)?;
                }
            }
            Ok(result)
        }
    })?)?;
    add_bool_stub_with_arg::<i32>(lua, t, "IsAppearanceHiddenVisual", false)?;
    t.set(
        "GetCategoryInfo",
        lua.create_function(|_, cat_id: i32| {
            if let Some((name, is_weapon, can_enchant, can_main, can_off)) =
                transmog_category_info(cat_id)
            {
                Ok((name.to_string(), is_weapon, can_enchant, can_main, can_off))
            } else {
                Ok((String::new(), false, false, false, false))
            }
        })?,
    )?;
    t.set("GetNumTransmogSources", lua.create_function({
        let s = Rc::clone(state);
        move |_, ()| Ok(s.borrow().world.transmog_appearances.len() as i32)
    })?)?;
    Ok(())
}

fn build_source_info(
    lua: &Lua,
    a: &crate::lua_api::state_types::TransmogAppearance,
) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("sourceID", a.source_id)?;
    info.set("visualID", a.visual_id)?;
    info.set("categoryID", a.category_id)?;
    info.set("itemID", a.item_id)?;
    info.set("isCollected", a.is_collected)?;
    info.set("sourceType", a.source_type)?;
    info.set("itemModID", a.item_mod_id)?;
    Ok(info)
}

fn build_empty_source_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("sourceID", 0)?;
    info.set("visualID", 0)?;
    info.set("categoryID", 0)?;
    info.set("itemID", 0)?;
    info.set("isCollected", false)?;
    info.set("sourceType", 0)?;
    info.set("itemModID", 0)?;
    Ok(info)
}

/// Outfit methods: illusions, outfits, outfit info.
fn register_transmog_outfit_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetIllusions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetOutfits",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set("GetNumMaxOutfits", lua.create_function(|_, ()| Ok(20i32))?)?;
    t.set(
        "GetOutfitInfo",
        lua.create_function(|_, _outfit_id: i32| {
            Ok((Value::Nil, Value::Nil)) // name, icon
        })?,
    )?;
    Ok(())
}

/// Source and player ownership methods: transmog checks, filters, item info.
fn register_transmog_source_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    add_player_has_transmog(lua, t, Rc::clone(state))?;
    add_player_has_transmog_by_item_info(lua, t, Rc::clone(state))?;
    add_player_has_transmog_item_modified_appearance(lua, t, Rc::clone(state))?;
    add_nil_stub_with_arg::<i32>(lua, t, "GetItemInfo")?;
    add_bool_stub_with_arg::<i32>(lua, t, "PlayerKnowsSource", false)?;
    add_bool_stub_with_arg::<i32>(lua, t, "IsSourceTypeFilterChecked", true)?;
    add_bool_stub(lua, t, "GetShowMissingSourceInItemTooltips", true)?;
    Ok(())
}

fn add_player_has_transmog(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set(
        "PlayerHasTransmog",
        lua.create_function(move |_, (item_id, _appearance_mod): (i32, Option<i32>)| {
            Ok(player_has_transmog(&state.borrow(), item_id))
        })?,
    )
}

fn add_player_has_transmog_by_item_info(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "PlayerHasTransmogByItemInfo",
        lua.create_function(move |_, item_info: String| {
            Ok(player_has_transmog(
                &state.borrow(),
                item_id_from_item_info(&item_info),
            ))
        })?,
    )
}

fn add_player_has_transmog_item_modified_appearance(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "PlayerHasTransmogItemModifiedAppearance",
        lua.create_function(move |_, id: i32| Ok(player_has_transmog(&state.borrow(), id)))?,
    )
}

fn player_has_transmog(state: &crate::lua_api::state::SimState, item_id: i32) -> bool {
    state.world.collected_transmogs.contains(&item_id)
}

fn item_id_from_item_info(item_info: &str) -> i32 {
    item_info
        .split(':')
        .nth(1)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0)
}

/// C_Transmog namespace - transmogrification API.
fn register_transmog(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAllSetAppearancesByID",
        lua.create_function(|lua, _set_id: i32| lua.create_table())?,
    )?;
    t.set("GetAppliedSourceID", lua.create_function({
        let s = Rc::clone(&state);
        move |_, slot: i32| {
            let st = s.borrow();
            match st.world.applied_transmog_slots.get(&slot) {
                Some(&source_id) => Ok(Value::Integer(source_id as i64)),
                None => Ok(Value::Nil),
            }
        }
    })?)?;
    // GetSlotInfo returns: isTransmogrified, hasPending, isPendingCollected,
    // canTransmogrify, cannotTransmogrifyReason, hasUndo
    // No transmog NPC open, so nothing is transmogrified or pending.
    t.set(
        "GetSlotInfo",
        lua.create_function(|_, _slot: i32| Ok((false, false, false, false, false, false)))?,
    )?;
    lua.globals().set("C_Transmog", t)?;
    Ok(())
}

/// Build a TransmogLocation table with methods matching the C++ engine object.
///
/// Fields: slotName (optional), slotID (optional), transmogType, modification.
/// Methods derive from these fields (e.g. IsAppearance checks transmogType == 0).
fn build_transmog_location(lua: &Lua) -> Result<mlua::Table> {
    let methods: mlua::Table = lua
        .load(
            r#"
        local mt = {}
        mt.__index = {
            IsAppearance = function(self) return self.transmogType == 0 end,
            IsIllusion = function(self) return self.transmogType == 1 end,
            IsSecondary = function(self) return self.modification == 1 end,
            IsMainHand = function(self) return self.slotName == "MAINHANDSLOT" end,
            IsOffHand = function(self) return self.slotName == "SECONDARYHANDSLOT" end,
            IsEitherHand = function(self)
                return self.slotName == "MAINHANDSLOT" or self.slotName == "SECONDARYHANDSLOT"
            end,
            GetSlotName = function(self) return self.slotName end,
            GetSlotID = function(self) return self.slotID or 0 end,
            GetArmorCategoryID = function(self) return nil end,
            IsEqual = function(self, other)
                return other and self.slotName == other.slotName
                   and self.transmogType == other.transmogType
                   and self.modification == other.modification
            end,
        }
        return mt
    "#,
        )
        .eval()?;
    lua.set_named_registry_value("__transmog_location_mt", methods)?;
    Ok(lua.named_registry_value("__transmog_location_mt")?)
}

/// TransmogUtil - utility functions for transmog system.
fn register_transmog_util(lua: &Lua) -> Result<()> {
    build_transmog_location(lua)?;
    let t = lua.create_table()?;
    t.set(
        "GetTransmogLocation",
        lua.create_function(
            |lua, (slot, transmog_type, modification): (String, i32, Value)| {
                let location = lua.create_table()?;
                let mt: mlua::Table =
                    lua.named_registry_value("__transmog_location_mt")?;
                location.set_metatable(Some(mt));
                location.set("slotName", slot)?;
                location.set("transmogType", transmog_type)?;
                location.set("modification", bool_or_int_to_i32(modification))?;
                Ok(location)
            },
        )?,
    )?;
    t.set(
        "CreateTransmogLocation",
        lua.create_function(
            |lua, (slot_id, transmog_type, modification): (i32, i32, Value)| {
                let location = lua.create_table()?;
                let mt: mlua::Table =
                    lua.named_registry_value("__transmog_location_mt")?;
                location.set_metatable(Some(mt));
                location.set("slotID", slot_id)?;
                location.set("transmogType", transmog_type)?;
                location.set("modification", bool_or_int_to_i32(modification))?;
                Ok(location)
            },
        )?,
    )?;
    t.set(
        "GetBestItemModifiedAppearanceID",
        lua.create_function(|_, _item_loc: mlua::Value| Ok(Value::Nil))?,
    )?;
    lua.globals().set("TransmogUtil", t)?;
    Ok(())
}

/// C_Heirloom namespace - heirloom collection.
fn register_heirloom(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    // GetHeirloomInfo returns: name, equipLoc, isPvP, texture, upgradeLevel,
    // source, searchFiltered, effectiveLevel, minLevel, maxLevel
    t.set("GetHeirloomInfo", lua.create_function({
        let s = Rc::clone(&state);
        move |lua, item_id: i32| {
            let st = s.borrow();
            let Some(h) = st.world.heirlooms.iter().find(|h| h.item_id == item_id as u32) else {
                return empty_heirloom_info();
            };
            Ok((
                Value::String(lua.create_string(&h.name)?),
                Value::String(lua.create_string(&h.equip_loc)?),
                false,                       // isPvP
                h.icon as i32,               // texture (fileDataID)
                h.upgrade_level,             // upgradeLevel
                Value::String(lua.create_string(&h.source)?),
                false,                       // searchFiltered
                h.max_level,                 // effectiveLevel
                h.min_level,                 // minLevel
                h.max_level,                 // maxLevel
            ))
        }
    })?)?;
    add_i32_stub_with_arg::<i32>(lua, &t, "GetHeirloomMaxUpgradeLevel", 0)?;
    t.set("GetNumHeirlooms", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.heirlooms.len() as i32)
    })?)?;
    t.set("GetNumKnownHeirlooms", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.collected_heirlooms.len() as i32)
    })?)?;
    // No filtering — displayed = all heirlooms
    t.set("GetNumDisplayedHeirlooms", lua.create_function({
        let s = Rc::clone(&state);
        move |_, ()| Ok(s.borrow().world.heirlooms.len() as i32)
    })?)?;
    t.set("GetHeirloomItemIDFromDisplayedIndex", lua.create_function({
        let s = Rc::clone(&state);
        move |_, index: i32| {
            let st = s.borrow();
            let i = (index - 1) as usize; // 1-based → 0-based
            Ok(st.world.heirlooms.get(i).map(|h| h.item_id as i32).unwrap_or(0))
        }
    })?)?;
    t.set("PlayerHasHeirloom", lua.create_function({
        let s = Rc::clone(&state);
        move |_, item_id: i32| Ok(s.borrow().world.collected_heirlooms.contains(&(item_id as u32)))
    })?)?;
    t.set("GetHeirloomLink", lua.create_function({
        let s = Rc::clone(&state);
        move |_, item_id: i32| {
            let st = s.borrow();
            match st.world.heirlooms.iter().find(|h| h.item_id == item_id as u32) {
                Some(h) => Ok(Some(format!("|cff0070dd|Hitem:{}::::::::1:0|h[{}]|h|r", h.item_id, h.name))),
                None => Ok(None),
            }
        }
    })?)?;
    add_bool_stub_with_arg::<i32>(lua, &t, "CanHeirloomUpgradeFromPending", false)?;
    // Filter stubs: show all by default, setters are no-ops
    t.set("GetCollectedHeirloomFilter", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("GetUncollectedHeirloomFilter", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("SetCollectedHeirloomFilter", lua.create_function(|_, _: bool| Ok(()))?)?;
    t.set("SetUncollectedHeirloomFilter", lua.create_function(|_, _: bool| Ok(()))?)?;
    t.set(
        "GetClassAndSpecFilters",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    lua.globals().set("C_Heirloom", t)?;
    Ok(())
}

fn empty_heirloom_info() -> Result<(Value, Value, bool, i32, i32, Value, bool, i32, i32, i32)> {
    Ok((
        Value::Nil,   // name
        Value::Nil,   // equipLoc
        false,        // isPvP
        0i32,         // texture
        0i32,         // upgradeLevel
        Value::Nil,   // source
        false,        // searchFiltered
        0i32,         // effectiveLevel
        0i32,         // minLevel
        0i32,         // maxLevel
    ))
}

/// C_TransmogSets namespace - transmog set collection.
fn register_transmog_sets(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    add_i32_stub_with_arg::<i32>(lua, &t, "GetBaseSetID", 0)?;
    add_table_stub_with_arg::<i32>(lua, &t, "GetVariantSets")?;
    t.set(
        "GetSetInfo",
        lua.create_function(|lua, _set_id: i32| build_empty_set_info(lua))?,
    )?;
    add_table_stub_with_arg::<i32>(lua, &t, "GetSetPrimaryAppearances")?;
    add_empty_table_stub(lua, &t, "GetAllSets")?;
    add_empty_table_stub(lua, &t, "GetUsableSets")?;
    add_bool_stub_with_arg::<i32>(lua, &t, "IsBaseSetCollected", false)?;
    add_table_stub_with_arg::<(i32, i32)>(lua, &t, "GetSourcesForSlot")?;
    lua.globals().set("C_TransmogSets", t)?;
    Ok(())
}

fn build_empty_set_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("setID", 0)?;
    info.set("name", "")?;
    info.set("description", "")?;
    info.set("label", "")?;
    info.set("expansionID", 0)?;
    info.set("collected", false)?;
    Ok(info)
}
