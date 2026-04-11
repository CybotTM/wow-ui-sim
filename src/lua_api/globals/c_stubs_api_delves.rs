//! C_DelvesUI stub — Delves companion/tier/entrance data.

use mlua::{Lua, Result, Value};

pub(super) fn register_c_delves_ui(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    let delve_entrance_tiers = seeded_delve_entrance_tiers(lua)?;

    // Active tier seed data
    let active_delve_tier = lua.create_table()?;
    active_delve_tier.set("tier", 4)?;
    active_delve_tier.set("tierDescription", "Tier 4")?;
    active_delve_tier.set("unlocked", true)?;
    active_delve_tier.set("modifierUIWidgetSetID", 4404)?;
    active_delve_tier.set("suggestedILvl", 603)?;
    active_delve_tier.set("lockedReason", Value::Nil)?;
    t.set("__activeDelveTier", active_delve_tier)?;
    t.set("__delveEntranceBackgroundWidgetSetID", 5501)?;
    t.set("__delveEntranceDescriptionString", "The Fungal Folly winds deeper with every tier.")?;
    t.set("__delveEntranceHeaderString", "Fungal Folly")?;
    t.set("__delveEntranceMapID", 2339)?;
    t.set("__delveEntranceTiers", delve_entrance_tiers)?;
    t.set("__selectedDelveEntranceTier", 4)?;
    t.set("__tieredEntranceOptionalAffixTraitTreeID", 77001)?;

    register_companion_methods(lua, &t)?;
    register_seasonal_methods(lua, &t)?;
    register_entrance_methods(lua, &t)?;
    register_affix_methods(lua, &t)?;
    register_misc_methods(lua, &t)?;

    lua.globals().set("C_DelvesUI", t)?;
    Ok(())
}

/// Companion config methods (trait tree, role, curio, creature display).
fn register_companion_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetTraitTreeForCompanion", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetRoleNodeForCompanion", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("GetRoleSubtreeForCompanion", lua.create_function(|_, _role_type: Value| Ok(0i32))?)?;
    t.set("GetCreatureDisplayInfoForCompanion", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetCurioNodeForCompanion", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("GetFactionForCompanion", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("GetUnseenCuriosBySlotType", lua.create_function(|lua, _slot_type: Value| lua.create_table())?)?;
    t.set("SaveSeenCuriosBySlotType", lua.create_function(|_, (_slot_type, _table): (Value, Value)| Ok(()))?)?;
    Ok(())
}

/// Seasonal/progression methods.
fn register_seasonal_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetCurrentDelvesSeasonNumber", lua.create_function(|_, ()| Ok(1i32))?)?;
    t.set("GetDelvesMinRequiredLevel", lua.create_function(|_, ()| Ok(80i32))?)?;
    t.set("HasActiveDelve", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetDelvesFactionForSeason", lua.create_function(|_, _season: Value| Ok(Value::Nil))?)?;
    Ok(())
}

/// Entrance data methods (active tier, tiers list, map ID, header/description, background widget).
fn register_entrance_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let s = t.clone();
    t.set("GetActiveDelveTier", lua.create_function(move |_, ()| s.get::<mlua::Table>("__activeDelveTier"))?)?;

    let s = t.clone();
    t.set("GetDelveEntranceBackgroundWidgetSetID", lua.create_function(move |_, ()| s.get::<i32>("__delveEntranceBackgroundWidgetSetID"))?)?;

    let s = t.clone();
    t.set("GetDelveEntranceDescriptionString", lua.create_function(move |_, ()| s.get::<String>("__delveEntranceDescriptionString"))?)?;

    let s = t.clone();
    t.set("GetDelveEntranceHeaderString", lua.create_function(move |_, ()| s.get::<String>("__delveEntranceHeaderString"))?)?;

    let s = t.clone();
    t.set("GetDelveEntranceMapID", lua.create_function(move |_, ()| s.get::<i32>("__delveEntranceMapID"))?)?;

    let s = t.clone();
    t.set("GetDelveEntranceTiers", lua.create_function(move |_, ()| s.get::<mlua::Table>("__delveEntranceTiers"))?)?;

    let s = t.clone();
    t.set(
        "IsDelveEntranceTierEnabled",
        lua.create_function(move |lua, tier: i32| {
            let tiers = s.get::<mlua::Table>("__delveEntranceTiers")?;
            for pair in tiers.sequence_values::<mlua::Table>() {
                let info = pair?;
                if info.get::<i32>("tier")? == tier {
                    let unlocked = info.get::<bool>("unlocked")?;
                    let locked_reason = info.get::<Value>("lockedReason")?;
                    return Ok(mlua::MultiValue::from_vec(vec![
                        Value::Boolean(unlocked),
                        if unlocked { Value::Nil } else { locked_reason },
                    ]));
                }
            }
            Ok(mlua::MultiValue::from_vec(vec![
                Value::Boolean(false),
                Value::String(lua.create_string("Unknown tier")?),
            ]))
        })?,
    )?;

    let s = t.clone();
    t.set(
        "SelectDelveEntranceTier",
        lua.create_function(move |_, tier: i32| {
            s.set("__selectedDelveEntranceTier", tier)?;
            let tiers = s.get::<mlua::Table>("__delveEntranceTiers")?;
            for pair in tiers.sequence_values::<mlua::Table>() {
                let info = pair?;
                if info.get::<i32>("tier")? == tier {
                    s.set("__activeDelveTier", info)?;
                    break;
                }
            }
            Ok(())
        })?,
    )?;

    t.set("RequestPartyEligibilityForDelveTiers", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

/// Optional affix trait tree method.
fn register_affix_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let s = t.clone();
    t.set("GetTieredEntranceOptionalAffixTraitTreeID", lua.create_function(move |_, ()| s.get::<i32>("__tieredEntranceOptionalAffixTraitTreeID"))?)?;
    Ok(())
}

/// Misc delves stubs with no state dependency.
fn register_misc_methods(_lua: &Lua, _t: &mlua::Table) -> Result<()> {
    Ok(())
}

fn seeded_delve_entrance_tiers(lua: &Lua) -> Result<mlua::Table> {
    let tiers = lua.create_table()?;
    for (index, (tier, unlocked, widget_set_id, suggested_ilvl, locked_reason)) in [
        (1, true, 4401, 571, None),
        (2, true, 4402, 584, None),
        (3, true, 4403, 597, None),
        (4, true, 4404, 603, None),
        (5, false, 4405, 610, Some("Complete Tier 4 to unlock this delve tier.")),
    ]
    .into_iter()
    .enumerate()
    {
        let info = lua.create_table()?;
        info.set("tier", tier)?;
        info.set("tierDescription", format!("Tier {tier}"))?;
        info.set("unlocked", unlocked)?;
        info.set("modifierUIWidgetSetID", widget_set_id)?;
        info.set("suggestedILvl", suggested_ilvl)?;
        match locked_reason {
            Some(reason) => info.set("lockedReason", reason)?,
            None => info.set("lockedReason", Value::Nil)?,
        }
        tiers.set(index + 1, info)?;
    }
    Ok(tiers)
}
