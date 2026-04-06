//! C_Collection namespaces for mounts, pets, toys, transmog, and heirlooms.
//!
//! Contains collection journal API functions for various game collectibles.

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register collection-related C_* namespaces.
pub fn register_c_collection_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_pet_journal(lua, Rc::clone(&state))?;
    register_mount_journal(lua, Rc::clone(&state))?;
    register_toy_box(lua)?;
    register_transmog_collection(lua, state)?;
    register_transmog(lua)?;
    register_transmog_util(lua)?;
    register_heirloom(lua)?;
    register_transmog_sets(lua)?;
    Ok(())
}

/// C_PetJournal namespace - battle pet utilities.
fn register_pet_journal(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_pet_count_methods(lua, &t, state)?;
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
        move |_, ()| Ok(s.borrow().world.pets.len() as i32)
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
    t.set("GetPetInfoByIndex", lua.create_function(|_, _: i32| Ok(Value::Nil))?)?;
    t.set("GetPetInfoByPetID", lua.create_function(|_, _: String| Ok(Value::Nil))?)?;
    t.set("GetPetInfoBySpeciesID", lua.create_function(|_, _: i32| Ok(Value::Nil))?)?;
    t.set("PetIsSummonable", lua.create_function(|_, _: String| Ok(false))?)
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
fn register_toy_box(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetToyInfo",
        lua.create_function(|_, _item_id: i32| {
            // Returns: itemID, toyName, icon, isFavorite, hasFanfare, itemQuality
            Ok((0i32, "", 0i32, false, false, 0i32))
        })?,
    )?;
    t.set(
        "IsToyUsable",
        lua.create_function(|_, _item_id: i32| Ok(false))?,
    )?;
    t.set("GetNumToys", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "GetToyFromIndex",
        lua.create_function(|_, _index: i32| Ok(0i32))?,
    )?;
    t.set("GetNumFilteredToys", lua.create_function(|_, ()| Ok(0i32))?)?;
    lua.globals().set("C_ToyBox", t)?;
    Ok(())
}

/// C_TransmogCollection namespace - transmog/appearance collection.
fn register_transmog_collection(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_transmog_appearance_methods(lua, &t)?;
    register_transmog_outfit_methods(lua, &t)?;
    register_transmog_source_methods(lua, &t, &state)?;
    lua.globals().set("C_TransmogCollection", t)?;
    Ok(())
}

/// Appearance query methods: sources, info, camera, categories.
fn register_transmog_appearance_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    add_table_stub_with_arg::<i32>(lua, t, "GetAppearanceSources")?;
    t.set(
        "GetSourceInfo",
        lua.create_function(|lua, _source_id: i32| build_empty_source_info(lua))?,
    )?;
    add_table_stub_with_arg::<i32>(lua, t, "GetAllAppearanceSources")?;
    add_i32_stub_with_arg::<i32>(lua, t, "GetAppearanceCameraID", 0)?;
    add_table_stub_with_arg::<(i32, Value)>(lua, t, "GetCategoryAppearances")?;
    add_bool_stub_with_arg::<i32>(lua, t, "IsAppearanceHiddenVisual", false)?;
    Ok(())
}

fn build_empty_source_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("sourceID", 0)?;
    info.set("visualID", 0)?;
    info.set("categoryID", 0)?;
    info.set("itemID", 0)?;
    info.set("isCollected", false)?;
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
fn register_transmog(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAllSetAppearancesByID",
        lua.create_function(|lua, _set_id: i32| lua.create_table())?,
    )?;
    t.set(
        "GetAppliedSourceID",
        lua.create_function(|_, _slot: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetSlotInfo",
        lua.create_function(|_, _slot: i32| Ok((false, false, false, false, false, Value::Nil)))?,
    )?;
    lua.globals().set("C_Transmog", t)?;
    Ok(())
}

/// TransmogUtil - utility functions for transmog system.
fn register_transmog_util(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTransmogLocation",
        lua.create_function(
            |lua, (slot, transmog_type, modification): (String, i32, i32)| {
                let location = lua.create_table()?;
                location.set("slotName", slot)?;
                location.set("transmogType", transmog_type)?;
                location.set("modification", modification)?;
                Ok(location)
            },
        )?,
    )?;
    t.set(
        "CreateTransmogLocation",
        lua.create_function(
            |lua, (slot_id, transmog_type, modification): (i32, i32, i32)| {
                let location = lua.create_table()?;
                location.set("slotID", slot_id)?;
                location.set("transmogType", transmog_type)?;
                location.set("modification", modification)?;
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
fn register_heirloom(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetHeirloomInfo",
        lua.create_function(|_, _item_id: i32| empty_heirloom_info())?,
    )?;
    add_i32_stub_with_arg::<i32>(lua, &t, "GetHeirloomMaxUpgradeLevel", 0)?;
    add_i32_stub(lua, &t, "GetNumHeirlooms", 0)?;
    add_i32_stub(lua, &t, "GetNumKnownHeirlooms", 0)?;
    add_bool_stub_with_arg::<i32>(lua, &t, "PlayerHasHeirloom", false)?;
    add_nil_stub_with_arg::<i32>(lua, &t, "GetHeirloomLink")?;
    add_bool_stub_with_arg::<i32>(lua, &t, "CanHeirloomUpgradeFromPending", false)?;
    t.set(
        "GetClassAndSpecFilters",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    lua.globals().set("C_Heirloom", t)?;
    Ok(())
}

fn empty_heirloom_info() -> Result<(Value, Value, bool, i32, i32, i32, bool, i32, i32, i32)> {
    Ok((
        Value::Nil,
        Value::Nil,
        false,
        0i32,
        0i32,
        0i32,
        false,
        0i32,
        0i32,
        0i32,
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
