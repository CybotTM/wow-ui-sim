//! Progression, currency, and miscellaneous C_* namespace stubs.

use mlua::{Lua, Result, Value};

pub(super) fn register_all(lua: &Lua) -> Result<()> {
    register_c_azerite_essence(lua)?;
    register_c_auction_house(lua)?;
    register_c_bank(lua)?;
    register_c_encounter_journal(lua)?;
    register_c_gm_ticket_info(lua)?;
    register_c_unit_auras(lua)?;
    register_c_currency_info(lua)?;
    register_c_neighborhood_initiative(lua)?;
    Ok(())
}

fn register_c_azerite_essence(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetEssences",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetMilestones",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetEssenceInfo",
        lua.create_function(|lua, _id: i32| {
            let info = lua.create_table()?;
            info.set("ID", 0)?;
            info.set("name", "Unknown Essence")?;
            info.set("icon", 0)?;
            info.set("valid", false)?;
            info.set("unlocked", false)?;
            info.set("rank", 0)?;
            Ok(Value::Table(info))
        })?,
    )?;
    t.set(
        "GetMilestoneEssence",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetNumUnlockedEssences",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetNumUnlockedSlots",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set("CanOpenUI", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_AzeriteEssence", t)?;
    Ok(())
}

fn register_c_auction_house(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetNumReplicateItems",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    lua.globals().set("C_AuctionHouse", t)?;
    Ok(())
}

fn register_c_bank(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "FetchDepositedMoney",
        lua.create_function(|_, _bt: i32| Ok(0i64))?,
    )?;
    lua.globals().set("C_Bank", t)?;
    Ok(())
}

fn register_c_encounter_journal(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetEncounterInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetSectionInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetLootInfoByIndex",
        lua.create_function(|_, _i: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetInstanceInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_EncounterJournal", t)?;
    Ok(())
}

fn register_c_gm_ticket_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("HasGMTicket", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_GMTicketInfo", t)?;
    Ok(())
}

fn register_c_unit_auras(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAuraDataByIndex",
        lua.create_function(|_, (_u, _i, _f): (String, i32, Option<String>)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAuraDataByAuraInstanceID",
        lua.create_function(|_, (_u, _id): (String, i32)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAuraDataBySlot",
        lua.create_function(|_, (_u, _s): (String, i32)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetBuffDataByIndex",
        lua.create_function(|_, (_u, _i, _f): (String, i32, Option<String>)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetDebuffDataByIndex",
        lua.create_function(|_, (_u, _i, _f): (String, i32, Option<String>)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetPlayerAuraBySpellID",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetCooldownAuraBySpellID",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "IsAuraFilteredOutByInstanceID",
        lua.create_function(|_, (_u, _id, _f): (String, i32, String)| Ok(false))?,
    )?;
    t.set(
        "WantsAlteredForm",
        lua.create_function(|_, _u: String| Ok(false))?,
    )?;
    t.set(
        "AddPrivateAuraAnchor",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(0i32))?,
    )?;
    t.set(
        "RemovePrivateAuraAnchor",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    t.set(
        "AddPrivateAuraAppliedSound",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "RemovePrivateAuraAppliedSound",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "SetPrivateWarningTextAnchor",
        lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "AuraIsBigDefensive",
        lua.create_function(|_, _spell_id: i32| Ok(false))?,
    )?;
    lua.globals().set("C_UnitAuras", t)?;
    Ok(())
}

fn register_c_currency_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_currency_query_methods(lua, &t)?;
    register_currency_list_methods(lua, &t)?;
    register_currency_id_methods(lua, &t)?;
    lua.globals().set("C_CurrencyInfo", t)?;
    Ok(())
}

fn register_currency_query_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetCurrencyInfo", lua.create_function(currency_info_by_id)?)?;
    t.set(
        "GetBasicCurrencyInfo",
        lua.create_function(basic_currency_info)?,
    )?;
    t.set(
        "GetCurrencyInfoFromLink",
        lua.create_function(|_, _l: String| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetCoinTextureString",
        lua.create_function(|lua, amount: i64| {
            let result = format_coin_texture_string(amount);
            Ok(mlua::Value::String(lua.create_string(&result)?))
        })?,
    )?;
    Ok(())
}

fn format_coin_texture_string(amount: i64) -> String {
    let gold = amount / 10000;
    let silver = (amount % 10000) / 100;
    let copper = amount % 100;
    let mut parts = Vec::new();
    if gold > 0 {
        parts.push(format!(
            "{}|TInterface\\MoneyFrame\\UI-GoldIcon:0:0:2:0|t",
            gold
        ));
    }
    if silver > 0 {
        parts.push(format!(
            "{}|TInterface\\MoneyFrame\\UI-SilverIcon:0:0:2:0|t",
            silver
        ));
    }
    if copper > 0 || parts.is_empty() {
        parts.push(format!(
            "{}|TInterface\\MoneyFrame\\UI-CopperIcon:0:0:2:0|t",
            copper
        ));
    }
    parts.join(" ")
}

fn register_currency_list_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    use super::currency_data;
    t.set(
        "GetCurrencyListSize",
        lua.create_function(|_, ()| Ok(currency_data::currency_list_size()))?,
    )?;
    t.set(
        "GetCurrencyListInfo",
        lua.create_function(currency_list_info)?,
    )?;
    t.set(
        "GetBackpackCurrencyInfo",
        lua.create_function(backpack_currency_info)?,
    )?;
    t.set(
        "ExpandCurrencyList",
        lua.create_function(|_, (_i, _e): (i32, bool)| Ok(()))?,
    )?;
    t.set("GetCurrencyFilter", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "SetCurrencyFilter",
        lua.create_function(|_, _f: i32| Ok(()))?,
    )?;
    t.set(
        "SetCurrencyBackpack",
        lua.create_function(|_, (_i, _w): (i32, bool)| Ok(()))?,
    )?;
    t.set(
        "SetCurrencyUnused",
        lua.create_function(|_, (_i, _u): (i32, bool)| Ok(()))?,
    )?;
    Ok(())
}

fn register_currency_id_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "DoesCurrentFilterRequireAccountCurrencyData",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "IsAccountCharacterCurrencyDataReady",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set(
        "GetWarResourcesCurrencyID",
        lua.create_function(|_, ()| Ok(1560))?,
    )?;
    t.set(
        "GetAzeriteCurrencyID",
        lua.create_function(|_, ()| Ok(1553))?,
    )?;
    Ok(())
}

fn basic_currency_info(lua: &Lua, (cid, _qty): (i32, Option<i32>)) -> Result<Value> {
    use super::currency_data;
    let info = lua.create_table()?;
    if let Some(c) = currency_data::get_currency_by_id(cid) {
        info.set("name", c.name)?;
        info.set("currencyID", c.currency_id)?;
        info.set("quantity", c.quantity)?;
        info.set("iconFileID", c.icon_file_id as i64)?;
        info.set("displayAmount", c.quantity)?;
    } else {
        info.set("name", format!("Currency {}", cid))?;
        info.set("currencyID", cid)?;
        info.set("quantity", 0)?;
        info.set("iconFileID", 0)?;
        info.set("displayAmount", 0)?;
    }
    Ok(Value::Table(info))
}

fn currency_info_by_id(lua: &Lua, cid: i32) -> Result<Value> {
    use super::currency_data;
    let info = lua.create_table()?;
    if let Some(c) = currency_data::get_currency_by_id(cid) {
        info.set("name", c.name)?;
        info.set("currencyID", c.currency_id)?;
        info.set("quantity", c.quantity)?;
        info.set("maxQuantity", c.max_quantity)?;
        info.set("quality", c.quality)?;
        info.set("iconFileID", c.icon_file_id as i64)?;
        info.set("discovered", c.is_discovered)?;
    } else {
        info.set("name", format!("Currency {}", cid))?;
        info.set("currencyID", cid)?;
        info.set("quantity", 0)?;
        info.set("maxQuantity", 0)?;
        info.set("quality", 1)?;
        info.set("iconFileID", 0)?;
        info.set("discovered", false)?;
    }
    info.set("isAccountWide", false)?;
    info.set("isAccountTransferable", false)?;
    info.set("transferPercentage", 0)?;
    Ok(Value::Table(info))
}

fn currency_list_info(lua: &Lua, index: i32) -> Result<Value> {
    use super::currency_data;
    let Some(c) = currency_data::get_currency_list_entry(index) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    info.set("name", c.name)?;
    info.set("currencyID", c.currency_id)?;
    info.set("quantity", c.quantity)?;
    info.set("maxQuantity", c.max_quantity)?;
    info.set("quality", c.quality)?;
    info.set("iconFileID", c.icon_file_id as i64)?;
    info.set("discovered", c.is_discovered)?;
    info.set("isHeader", c.is_header)?;
    info.set("isHeaderExpanded", c.is_header_expanded)?;
    info.set("currencyListDepth", c.depth)?;
    info.set("isTypeUnused", false)?;
    info.set("isShowInBackpack", c.is_show_in_backpack)?;
    info.set("isAccountWide", false)?;
    info.set("isAccountTransferable", false)?;
    info.set("transferPercentage", 0)?;
    Ok(Value::Table(info))
}

fn register_c_neighborhood_initiative(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTrackedInitiativeTasks",
        lua.create_function(|lua, ()| {
            let result = lua.create_table()?;
            result.set("trackedIDs", lua.create_table()?)?;
            Ok(result)
        })?,
    )?;
    t.set(
        "GetInitiativeTaskInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "RemoveTrackedInitiativeTask",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    t.set(
        "IsInitiativeEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_NeighborhoodInitiative", t)?;
    Ok(())
}

fn backpack_currency_info(lua: &Lua, index: i32) -> Result<Value> {
    use super::currency_data;
    let watched: Vec<_> = currency_data::backpack_currencies().collect();
    let Some(c) = watched.get((index - 1) as usize) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    info.set("name", c.name)?;
    info.set("quantity", c.quantity)?;
    info.set("iconFileID", c.icon_file_id as i64)?;
    info.set("currencyTypesID", c.currency_id)?;
    Ok(Value::Table(info))
}
