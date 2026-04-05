//! Achievement, tracking, and SimulatePing stubs split from c_stubs_api_extra.rs.

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Achievement category API stubs needed by Blizzard_AchievementUI at parse time.
pub fn register_achievement_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_achievement_empty_table_stubs(lua, &g)?;
    g.set(
        "GetCategoryInfo",
        lua.create_function(|_, _: Value| Ok((Value::Nil, -1i32, -1i32)))?,
    )?;
    g.set(
        "GetCategoryNumAchievements",
        lua.create_function(|_, _: Value| Ok((0i32, 0i32, 0i32)))?,
    )?;
    g.set(
        "GetTotalAchievementPoints",
        lua.create_function(|_, _: mlua::MultiValue| Ok(0i32))?,
    )?;
    g.set(
        "GetAchievementInfo",
        lua.create_function(stub_get_achievement_info)?,
    )?;
    g.set(
        "GetNumCompletedAchievements",
        lua.create_function(|_, _: Option<bool>| Ok((0i32, 0i32)))?,
    )?;
    Ok(())
}

fn register_achievement_empty_table_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let empty_table = lua.create_function(|lua, ()| lua.create_table())?;
    for name in [
        "GetCategoryList",
        "GetGuildCategoryList",
        "GetStatisticsCategoryList",
    ] {
        g.set(name, empty_table.clone())?;
    }
    let empty_multi = lua.create_function(|_, _: mlua::MultiValue| Ok(mlua::MultiValue::new()))?;
    for name in ["GetLatestCompletedAchievements", "GetTrackedAchievements"] {
        g.set(name, empty_multi.clone())?;
    }
    Ok(())
}

fn is_achievement_earned(lua: &Lua, aid: i32) -> bool {
    lua.app_data_ref::<Rc<RefCell<SimState>>>()
        .map(|s| s.borrow().world.earned_achievements.contains(&aid))
        .unwrap_or(false)
}

/// GetAchievementInfo — returns 14 values matching WoW's signature.
/// Checks earned_achievements HashSet for the completed flag.
fn stub_get_achievement_info(lua: &Lua, id: Value) -> Result<mlua::MultiValue> {
    let aid = match &id {
        Value::Integer(n) => *n as i32,
        Value::Number(n) => *n as i32,
        _ => return Ok(mlua::MultiValue::from_vec(vec![Value::Nil])),
    };
    let completed = is_achievement_earned(lua, aid);
    Ok(mlua::MultiValue::from_vec(vec![
        Value::Integer(aid as i64),
        Value::String(lua.create_string("Achievement")?),
        Value::Integer(10),
        Value::Boolean(completed),
        Value::Integer(if completed { 1 } else { 0 }),
        Value::Integer(if completed { 1 } else { 0 }),
        Value::Integer(2025),
        Value::String(lua.create_string("Achievement description")?),
        Value::Integer(0),
        Value::Integer(136243),
        Value::String(lua.create_string("")?),
        Value::Boolean(false),
        Value::Boolean(false),
        Value::Nil,
    ]))
}

/// SimulatePing(textureKit) - fires stored PingManager callbacks to render a pin.
pub fn register_simulate_ping(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        function SimulatePing(textureKit)
            textureKit = textureKit or "Attack"
            local cbs = _G.__PingSecureCallbacks
            if not cbs or not cbs.PingPinFrameAdded then
                print("SimulatePing: PingManager not initialized (no PingPinFrameAdded callback)")
                return
            end
            local anchor = CreateFrame("Frame", nil, UIParent)
            anchor:SetSize(1, 1)
            anchor:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
            anchor:Show()
            cbs.PingPinFrameAdded(anchor, textureKit, true)
            C_Timer.After(5, function()
                if cbs.PingPinFrameRemoved then cbs.PingPinFrameRemoved(anchor) end
            end)
        end
    "#,
    )
    .exec()
}

/// Loot, content-tracking, and achievement telemetry namespace stubs.
pub fn register_tracking_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();

    let cl = lua.create_table()?;
    cl.set(
        "GetLootRollDuration",
        lua.create_function(|_, _id: Value| Ok(0i32))?,
    )?;
    g.set("C_Loot", cl)?;

    let ct = lua.create_table()?;
    ct.set(
        "GetTrackedIDs",
        lua.create_function(|lua, _type: Value| lua.create_table())?,
    )?;
    ct.set(
        "IsTracking",
        lua.create_function(|_, (_type, _id): (Value, Value)| Ok(false))?,
    )?;
    ct.set(
        "GetCollectableSourceTrackingEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set("C_ContentTracking", ct)?;

    let at = lua.create_table()?;
    at.set("ShowAchievements", lua.create_function(|_, ()| Ok(()))?)?;
    at.set(
        "LinkAchievementInWhisper",
        lua.create_function(|_, _id: Value| Ok(()))?,
    )?;
    at.set(
        "LinkAchievementInClub",
        lua.create_function(|_, _id: Value| Ok(()))?,
    )?;
    g.set("C_AchievementTelemetry", at)?;

    Ok(())
}
