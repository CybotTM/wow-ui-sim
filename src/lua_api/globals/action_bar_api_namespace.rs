use super::action_bar_api::{action_texture_path, slot_from_value, spell_cooldown_times};
use super::lua_duration_object::LuaDurationObject;
use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

const NUM_ACTIONBAR_PAGES: i32 = 6;

/// C_ActionBar namespace — called from c_stubs_api after globals are set.
pub fn register_c_action_bar_namespace(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = lua.create_table()?;
    register_c_action_bar_stub_methods(lua, &t)?;
    register_c_action_bar_slot_stubs(lua, &t)?;
    register_c_action_bar_stateful(lua, &t, &state)?;
    lua.globals().set("C_ActionBar", t)?;
    Ok(())
}

fn register_c_action_bar_stub_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_c_action_bar_general_stubs(lua, t)?;
    register_c_action_bar_page_methods(lua, t)?;
    register_c_action_bar_state_queries(lua, t)?;
    Ok(())
}

fn register_c_action_bar_general_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetBonusBarIndexForSlot",
        lua.create_function(|_, _s: i32| Ok(0i32))?,
    )?;
    t.set(
        "IsOnBarOrSpecialBar",
        lua.create_function(|_, _s: i32| Ok(false))?,
    )?;
    t.set(
        "FindSpellActionButtons",
        lua.create_function(|lua, _: i32| lua.create_table())?,
    )?;
    t.set(
        "GetCurrentActionBarByClass",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    t.set(
        "HasFlyoutActionButtons",
        lua.create_function(|_, _: i32| Ok(false))?,
    )?;
    t.set(
        "EnableActionRangeCheck",
        lua.create_function(|_, (_, _): (Value, bool)| Ok(()))?,
    )?;
    t.set(
        "IsAssistedCombatAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "HasAssistedCombatActionButtons",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}

fn register_c_action_bar_page_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetActionBarPage", lua.create_function(|_, ()| Ok(1i32))?)?;
    t.set(
        "SetActionBarPage",
        lua.create_function(|_, _: Value| Ok(()))?,
    )?;
    t.set("GetExtraBarIndex", lua.create_function(|_, ()| Ok(13i32))?)?;
    t.set(
        "GetMultiCastBarIndex",
        lua.create_function(|_, ()| Ok(7i32))?,
    )?;
    t.set(
        "GetVehicleBarIndex",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetOverrideBarIndex",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetTempShapeshiftBarIndex",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("GetBonusBarIndex", lua.create_function(|_, ()| Ok(0i32))?)?;
    let table_ref = t.clone();
    t.set(
        "GetBonusBarOffset",
        lua.create_function(move |_, ()| current_bonus_bar_offset(&table_ref))?,
    )?;
    t.set(
        "GetOverrideBarSkin",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn current_bonus_bar_offset(action_bar_table: &mlua::Table) -> Result<i32> {
    let bonus_bar_index = match action_bar_table.get::<Value>("GetBonusBarIndex")? {
        Value::Function(get_bonus_bar_index) => get_bonus_bar_index.call::<i32>(())?,
        _ => 0,
    };
    Ok((bonus_bar_index - NUM_ACTIONBAR_PAGES).max(0))
}

fn register_c_action_bar_state_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "HasVehicleActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "HasOverrideActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("HasBonusActionBar", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "HasTempShapeshiftActionBar",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("HasExtraActionBar", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "IsPossessBarVisible",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}

fn register_c_action_bar_slot_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_c_action_bar_basic_slot_stubs(lua, t)?;
    register_c_action_bar_cooldown_slot_stubs(lua, t)?;
    register_c_action_bar_pet_slot_stubs(lua, t)?;
    Ok(())
}

fn register_c_action_bar_basic_slot_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetActionText",
        lua.create_function(|_, _: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetActionCount",
        lua.create_function(|_, _: Value| Ok(0i32))?,
    )?;
    t.set(
        "GetActionDisplayCount",
        lua.create_function(|_, (_, _): (Value, Value)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetActionUseCount",
        lua.create_function(|_, _: Value| Ok(0i32))?,
    )?;
    t.set(
        "IsConsumableAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsStackableAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsItemAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsAttackAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsAutoRepeatAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsEquippedAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsEquippedGearOutfitAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsHelpfulAction",
        lua.create_function(|_, (_, _): (Value, Value)| Ok(false))?,
    )?;
    t.set(
        "IsHarmfulAction",
        lua.create_function(|_, (_, _): (Value, Value)| Ok(false))?,
    )?;
    Ok(())
}

fn register_c_action_bar_cooldown_slot_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetActionLossOfControlCooldown",
        lua.create_function(|_, _: Value| Ok((0.0_f64, 0.0_f64)))?,
    )?;
    t.set(
        "GetActionLossOfControlCooldownInfo",
        lua.create_function(|lua, _: Value| {
            let info = lua.create_table()?;
            info.set("isActive", false)?;
            info.set("startTime", 0)?;
            info.set("duration", 0)?;
            info.set("modRate", 1.0_f64)?;
            info.set("shouldReplaceNormalCooldown", false)?;
            Ok(info)
        })?,
    )?;
    t.set(
        "UsesActionText",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "GetActionChargeDuration",
        lua.create_function(|_, _: Value| Ok(LuaDurationObject::new()))?,
    )?;
    t.set(
        "GetActionCooldownDuration",
        lua.create_function(|_, _: Value| Ok(LuaDurationObject::new()))?,
    )?;
    t.set(
        "GetActionLossOfControlCooldownDuration",
        lua.create_function(|_, _: Value| Ok(LuaDurationObject::new()))?,
    )?;
    t.set(
        "GetSpell",
        lua.create_function(|_, _: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetItemActionOnEquipSpellID",
        lua.create_function(|_, _: Value| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn register_c_action_bar_pet_slot_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "FindFlyoutActionButtons",
        lua.create_function(|lua, _: i32| lua.create_table())?,
    )?;
    t.set(
        "FindPetActionButtons",
        lua.create_function(|lua, _: Value| lua.create_table())?,
    )?;
    t.set(
        "GetPetActionPetBarIndices",
        lua.create_function(|lua, _: Value| lua.create_table())?,
    )?;
    t.set(
        "RegisterActionUIButton",
        lua.create_function(|_, _: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "IsAutoCastPetAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "IsEnabledAutoCastPetAction",
        lua.create_function(|_, _: Value| Ok(false))?,
    )?;
    t.set(
        "ToggleAutoCastPetAction",
        lua.create_function(|_, _: Value| Ok(()))?,
    )?;
    Ok(())
}

fn register_c_action_bar_stateful(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    register_c_action_bar_has_action(lua, t, state)?;
    register_c_action_bar_texture_query(lua, t, state)?;
    register_c_action_bar_usable_query(lua, t, state)?;
    register_c_action_bar_current_query(lua, t, state)?;
    register_c_action_bar_cooldowns(lua, t, state)?;
    Ok(())
}

fn register_c_action_bar_has_action(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let st = Rc::clone(state);
    t.set(
        "HasAction",
        lua.create_function(move |_, slot: Value| {
            let s = st.borrow();
            Ok(slot_from_value(&slot).is_some_and(|n| s.action_bars.contains_key(&n)))
        })?,
    )?;
    Ok(())
}

fn register_c_action_bar_texture_query(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let st = Rc::clone(state);
    t.set(
        "GetActionTexture",
        lua.create_function(move |lua, slot: Value| {
            let s = st.borrow();
            let Some(n) = slot_from_value(&slot) else {
                return Ok(Value::Nil);
            };
            match action_texture_path(&s, n) {
                Some(path) => Ok(Value::String(lua.create_string(&path)?)),
                None => Ok(Value::Nil),
            }
        })?,
    )?;
    Ok(())
}

fn register_c_action_bar_usable_query(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let st = Rc::clone(state);
    t.set(
        "IsUsableAction",
        lua.create_function(move |_, slot: Value| {
            let s = st.borrow();
            Ok((
                slot_from_value(&slot).is_some_and(|n| s.action_bars.contains_key(&n)),
                false,
            ))
        })?,
    )?;
    Ok(())
}

fn register_c_action_bar_current_query(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let st = Rc::clone(state);
    t.set(
        "IsCurrentAction",
        lua.create_function(move |_, slot: Value| {
            let n = slot_from_value(&slot).unwrap_or(0);
            let s = st.borrow();
            let Some(casting) = s.casting.as_ref().map(|c| c.spell_id) else {
                return Ok(false);
            };
            Ok(s.action_bars.get(&n).copied() == Some(casting))
        })?,
    )?;
    Ok(())
}

fn register_c_action_bar_cooldowns(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    register_c_action_bar_cooldown_query(lua, t, state)?;
    register_c_action_bar_charges_query(lua, t)?;
    Ok(())
}

fn register_c_action_bar_cooldown_query(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let st = Rc::clone(state);
    t.set(
        "GetActionCooldown",
        lua.create_function(move |lua, slot: Value| {
            let s = st.borrow();
            let (start, dur) = slot_from_value(&slot)
                .and_then(|n| {
                    s.action_bars.get(&n).map(|&id| {
                        spell_cooldown_times(&s, id, s.start_time.elapsed().as_secs_f64())
                    })
                })
                .unwrap_or((0.0, 0.0));
            action_cooldown_info(lua, start, dur)
        })?,
    )?;
    Ok(())
}

fn register_c_action_bar_charges_query(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetActionCharges",
        lua.create_function(|lua, _: Value| action_charges_info(lua))?,
    )?;
    Ok(())
}

fn action_cooldown_info(lua: &Lua, start: f64, dur: f64) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("startTime", start)?;
    info.set("duration", dur)?;
    info.set("isEnabled", true)?;
    info.set("modRate", 1.0_f64)?;
    Ok(info)
}

fn action_charges_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("currentCharges", 0)?;
    info.set("maxCharges", 0)?;
    info.set("cooldownStartTime", 0.0_f64)?;
    info.set("cooldownDuration", 0.0_f64)?;
    info.set("chargeModRate", 1.0_f64)?;
    Ok(info)
}
