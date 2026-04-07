//! Transmog, heirloom, and transmog set collection APIs.
//!
//! C_TransmogCollection, C_Transmog, TransmogUtil, C_Heirloom, C_TransmogSets.

use super::c_collection_api::{
    add_bool_stub, add_bool_stub_with_arg, add_empty_table_stub, add_i32_stub_with_arg,
    add_nil_stub_with_arg, add_table_stub_with_arg, bool_or_int_to_i32, transmog_category_info,
};
use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register transmog, heirloom, and transmog set namespaces.
pub(super) fn register_transmog_and_heirloom_apis(
    lua: &Lua,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    register_transmog_collection(lua, Rc::clone(&state))?;
    register_transmog(lua, Rc::clone(&state))?;
    register_transmog_util(lua)?;
    register_heirloom(lua, state)?;
    register_transmog_sets(lua)?;
    Ok(())
}

// ============================================================================
// C_TransmogCollection
// ============================================================================

fn register_transmog_collection(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_transmog_source_queries(lua, &t, &state)?;
    register_transmog_category_queries(lua, &t, &state)?;
    register_transmog_outfit_methods(lua, &t)?;
    register_transmog_ownership_methods(lua, &t, &state)?;
    lua.globals().set("C_TransmogCollection", t)?;
    Ok(())
}

fn register_transmog_source_queries(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetAppearanceSources",
        lua.create_function({
            let s = Rc::clone(state);
            move |lua, visual_id: i32| {
                let st = s.borrow();
                let sources: Vec<_> = st
                    .world
                    .transmog_appearances
                    .iter()
                    .filter(|a| a.visual_id == visual_id)
                    .collect();
                let result = lua.create_table()?;
                for (i, a) in sources.iter().enumerate() {
                    result.set(i + 1, build_source_info(lua, a)?)?;
                }
                Ok(result)
            }
        })?,
    )?;
    t.set(
        "GetSourceInfo",
        lua.create_function({
            let s = Rc::clone(state);
            move |lua, source_id: i32| {
                let st = s.borrow();
                if let Some(a) = st
                    .world
                    .transmog_appearances
                    .iter()
                    .find(|a| a.source_id == source_id)
                {
                    build_source_info(lua, a)
                } else {
                    build_empty_source_info(lua)
                }
            }
        })?,
    )?;
    add_table_stub_with_arg::<i32>(lua, t, "GetAllAppearanceSources")?;
    add_i32_stub_with_arg::<i32>(lua, t, "GetAppearanceCameraID", 0)?;
    t.set(
        "GetNumTransmogSources",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, ()| Ok(s.borrow().world.transmog_appearances.len() as i32)
        })?,
    )?;
    Ok(())
}

fn register_transmog_category_queries(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetCategoryAppearances",
        lua.create_function({
            let s = Rc::clone(state);
            move |lua, (category_id, _location): (i32, Value)| {
                let st = s.borrow();
                let mut seen_visuals = std::collections::HashSet::new();
                let result = lua.create_table()?;
                let mut idx = 0;
                for a in &st.world.transmog_appearances {
                    if a.category_id == category_id && seen_visuals.insert(a.visual_id) {
                        idx += 1;
                        result.set(idx, build_appearance_entry(lua, a, idx)?)?;
                    }
                }
                Ok(result)
            }
        })?,
    )?;
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
    Ok(())
}

fn build_appearance_entry(
    lua: &Lua,
    a: &crate::lua_api::state_types::TransmogAppearance,
    ui_order: i32,
) -> Result<mlua::Table> {
    let entry = lua.create_table()?;
    entry.set("visualID", a.visual_id)?;
    entry.set("isCollected", a.is_collected)?;
    entry.set("isUsable", true)?;
    entry.set("isFavorite", false)?;
    entry.set("isHideVisual", false)?;
    entry.set("uiOrder", ui_order)?;
    entry.set("hasActiveRequiredHoliday", false)?;
    entry.set("hasRequiredHoliday", false)?;
    entry.set("canDisplayOnPlayer", true)?;
    entry.set("exclusions", 0)?;
    Ok(entry)
}

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
        lua.create_function(|_, _id: i32| Ok((Value::Nil, Value::Nil)))?,
    )?;
    Ok(())
}

fn register_transmog_ownership_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "PlayerHasTransmog",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, (item_id, _mod): (i32, Option<i32>)| {
                Ok(s.borrow().world.collected_transmogs.contains(&item_id))
            }
        })?,
    )?;
    t.set(
        "PlayerHasTransmogByItemInfo",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, item_info: String| {
                let item_id = item_id_from_item_info(&item_info);
                Ok(s.borrow().world.collected_transmogs.contains(&item_id))
            }
        })?,
    )?;
    t.set(
        "PlayerHasTransmogItemModifiedAppearance",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, id: i32| Ok(s.borrow().world.collected_transmogs.contains(&id))
        })?,
    )?;
    add_nil_stub_with_arg::<i32>(lua, t, "GetItemInfo")?;
    add_bool_stub_with_arg::<i32>(lua, t, "PlayerKnowsSource", false)?;
    add_bool_stub_with_arg::<i32>(lua, t, "IsSourceTypeFilterChecked", true)?;
    add_bool_stub(lua, t, "GetShowMissingSourceInItemTooltips", true)?;
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

fn item_id_from_item_info(item_info: &str) -> i32 {
    item_info
        .split(':')
        .nth(1)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0)
}

// ============================================================================
// C_Transmog
// ============================================================================

fn register_transmog(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAllSetAppearancesByID",
        lua.create_function(|lua, _: i32| lua.create_table())?,
    )?;
    t.set(
        "GetAppliedSourceID",
        lua.create_function({
            let s = Rc::clone(&state);
            move |_, slot: i32| {
                let st = s.borrow();
                match st.world.applied_transmog_slots.get(&slot) {
                    Some(&source_id) => Ok(Value::Integer(source_id as i64)),
                    None => Ok(Value::Nil),
                }
            }
        })?,
    )?;
    // isTransmogrified, hasPending, isPendingCollected, canTransmogrify, cannotTransmogrifyReason, hasUndo
    t.set(
        "GetSlotInfo",
        lua.create_function(|_, _: i32| Ok((false, false, false, false, false, false)))?,
    )?;
    lua.globals().set("C_Transmog", t)?;
    Ok(())
}

// ============================================================================
// TransmogUtil
// ============================================================================

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

fn new_transmog_location(
    lua: &Lua,
    transmog_type: i32,
    modification: Value,
) -> Result<mlua::Table> {
    let location = lua.create_table()?;
    let mt: mlua::Table = lua.named_registry_value("__transmog_location_mt")?;
    location.set_metatable(Some(mt));
    location.set("transmogType", transmog_type)?;
    location.set("modification", bool_or_int_to_i32(modification))?;
    Ok(location)
}

fn register_transmog_util(lua: &Lua) -> Result<()> {
    build_transmog_location(lua)?;
    let t = lua.create_table()?;
    t.set(
        "GetTransmogLocation",
        lua.create_function(
            |lua, (slot, transmog_type, modification): (String, i32, Value)| {
                let loc = new_transmog_location(lua, transmog_type, modification)?;
                loc.set("slotName", slot)?;
                Ok(loc)
            },
        )?,
    )?;
    t.set(
        "CreateTransmogLocation",
        lua.create_function(
            |lua, (slot_id, transmog_type, modification): (i32, i32, Value)| {
                let loc = new_transmog_location(lua, transmog_type, modification)?;
                loc.set("slotID", slot_id)?;
                Ok(loc)
            },
        )?,
    )?;
    t.set(
        "GetBestItemModifiedAppearanceID",
        lua.create_function(|_, _: mlua::Value| Ok(Value::Nil))?,
    )?;
    lua.globals().set("TransmogUtil", t)?;
    Ok(())
}

// ============================================================================
// C_Heirloom
// ============================================================================

fn register_heirloom(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_heirloom_info_methods(lua, &t, &state)?;
    register_heirloom_query_methods(lua, &t, &state)?;
    register_heirloom_filter_stubs(lua, &t)?;
    lua.globals().set("C_Heirloom", t)?;
    Ok(())
}

fn register_heirloom_info_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetHeirloomInfo",
        lua.create_function({
            let s = Rc::clone(state);
            move |lua, item_id: i32| {
                let st = s.borrow();
                let Some(h) = st
                    .world
                    .heirlooms
                    .iter()
                    .find(|h| h.item_id == item_id as u32)
                else {
                    return empty_heirloom_info();
                };
                Ok((
                    Value::String(lua.create_string(&h.name)?),
                    Value::String(lua.create_string(&h.equip_loc)?),
                    false,
                    h.icon as i32,
                    h.upgrade_level,
                    Value::String(lua.create_string(&h.source)?),
                    false,
                    h.max_level,
                    h.min_level,
                    h.max_level,
                ))
            }
        })?,
    )?;
    add_i32_stub_with_arg::<i32>(lua, t, "GetHeirloomMaxUpgradeLevel", 0)?;
    t.set(
        "GetHeirloomLink",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, item_id: i32| {
                let st = s.borrow();
                match st
                    .world
                    .heirlooms
                    .iter()
                    .find(|h| h.item_id == item_id as u32)
                {
                    Some(h) => Ok(Some(format!(
                        "|cff0070dd|Hitem:{}::::::::1:0|h[{}]|h|r",
                        h.item_id, h.name
                    ))),
                    None => Ok(None),
                }
            }
        })?,
    )?;
    Ok(())
}

fn register_heirloom_query_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetNumHeirlooms",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, ()| Ok(s.borrow().world.heirlooms.len() as i32)
        })?,
    )?;
    t.set(
        "GetNumKnownHeirlooms",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, ()| Ok(s.borrow().world.collected_heirlooms.len() as i32)
        })?,
    )?;
    t.set(
        "GetNumDisplayedHeirlooms",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, ()| Ok(s.borrow().world.heirlooms.len() as i32)
        })?,
    )?;
    t.set(
        "GetHeirloomItemIDFromDisplayedIndex",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, index: i32| {
                let i = (index - 1) as usize;
                Ok(s.borrow()
                    .world
                    .heirlooms
                    .get(i)
                    .map(|h| h.item_id as i32)
                    .unwrap_or(0))
            }
        })?,
    )?;
    t.set(
        "PlayerHasHeirloom",
        lua.create_function({
            let s = Rc::clone(state);
            move |_, item_id: i32| {
                Ok(s.borrow()
                    .world
                    .collected_heirlooms
                    .contains(&(item_id as u32)))
            }
        })?,
    )?;
    Ok(())
}

fn register_heirloom_filter_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    add_bool_stub_with_arg::<i32>(lua, t, "CanHeirloomUpgradeFromPending", false)?;
    t.set(
        "GetCollectedHeirloomFilter",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set(
        "GetUncollectedHeirloomFilter",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set(
        "SetCollectedHeirloomFilter",
        lua.create_function(|_, _: bool| Ok(()))?,
    )?;
    t.set(
        "SetUncollectedHeirloomFilter",
        lua.create_function(|_, _: bool| Ok(()))?,
    )?;
    t.set(
        "GetClassAndSpecFilters",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    Ok(())
}

fn empty_heirloom_info() -> Result<(Value, Value, bool, i32, i32, Value, bool, i32, i32, i32)> {
    Ok((
        Value::Nil,
        Value::Nil,
        false,
        0,
        0,
        Value::Nil,
        false,
        0,
        0,
        0,
    ))
}

// ============================================================================
// C_TransmogSets
// ============================================================================

fn register_transmog_sets(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    add_i32_stub_with_arg::<i32>(lua, &t, "GetBaseSetID", 0)?;
    add_table_stub_with_arg::<i32>(lua, &t, "GetVariantSets")?;
    t.set(
        "GetSetInfo",
        lua.create_function(|lua, _: i32| {
            let info = lua.create_table()?;
            info.set("setID", 0)?;
            info.set("name", "")?;
            info.set("description", "")?;
            info.set("label", "")?;
            info.set("expansionID", 0)?;
            info.set("collected", false)?;
            Ok(info)
        })?,
    )?;
    add_table_stub_with_arg::<i32>(lua, &t, "GetSetPrimaryAppearances")?;
    add_empty_table_stub(lua, &t, "GetAllSets")?;
    add_empty_table_stub(lua, &t, "GetUsableSets")?;
    add_bool_stub_with_arg::<i32>(lua, &t, "IsBaseSetCollected", false)?;
    add_table_stub_with_arg::<(i32, i32)>(lua, &t, "GetSourcesForSlot")?;
    lua.globals().set("C_TransmogSets", t)?;
    Ok(())
}
