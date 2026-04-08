use crate::lua_api::state::SimState;
use crate::lua_api::state_types::CharacterStats;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

fn read_player_stat(lua: &Lua, f: impl Fn(&CharacterStats) -> f64) -> f64 {
    lua.app_data_ref::<Rc<RefCell<SimState>>>()
        .map(|s| f(&s.borrow().player.stats))
        .unwrap_or(0.0)
}

pub(crate) fn lookup_combat_rating(lua: &Lua, id: i32) -> Result<i32> {
    Ok(lua
        .app_data_ref::<Rc<RefCell<SimState>>>()
        .map(|s| {
            let st = &s.borrow().player.stats;
            match id {
                9 => st.crit_rating,
                6 => st.haste_rating,
                26 => st.mastery_rating,
                14 => st.versatility_rating,
                15 => st.speed_rating,
                17 => st.leech_rating,
                18 => st.avoidance_rating,
                _ => 0,
            }
        })
        .unwrap_or(0))
}

pub(crate) fn lookup_combat_rating_bonus(lua: &Lua, id: i32) -> Result<f64> {
    Ok(lua
        .app_data_ref::<Rc<RefCell<SimState>>>()
        .map(|s| {
            let st = &s.borrow().player.stats;
            match id {
                9 => st.crit_pct(),
                6 => st.haste_pct(),
                26 => st.mastery_pct(),
                14 => st.versatility_pct(),
                _ => 0.0,
            }
        })
        .unwrap_or(0.0))
}

pub(crate) fn lookup_mastery_effect(lua: &Lua, _: ()) -> Result<(f64, f64)> {
    let mastery = read_player_stat(lua, |s| s.mastery_pct());
    Ok((mastery + 8.0, mastery))
}

pub(crate) fn lookup_haste(lua: &Lua, _: ()) -> Result<f64> {
    Ok(read_player_stat(lua, |s| s.haste_pct()))
}

pub(crate) fn lookup_versatility(lua: &Lua, _: Value) -> Result<f64> {
    Ok(read_player_stat(lua, |s| s.versatility_pct()))
}

pub(crate) fn lookup_lifesteal(lua: &Lua, _: ()) -> Result<f64> {
    Ok(read_player_stat(lua, |s| s.leech_rating as f64 / 100.0))
}

pub(crate) fn lookup_avoidance(lua: &Lua, _: ()) -> Result<f64> {
    Ok(read_player_stat(lua, |s| s.avoidance_rating as f64 / 72.0))
}

pub(crate) fn lookup_speed(lua: &Lua, _: ()) -> Result<f64> {
    Ok(read_player_stat(lua, |s| s.speed_rating as f64 / 50.0))
}

pub(crate) fn stub_difficulty_info(lua: &Lua, _id: Value) -> Result<mlua::MultiValue> {
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string("")?),
        Value::String(lua.create_string("")?),
        Value::Boolean(false),
        Value::Boolean(false),
        Value::Integer(0),
    ]))
}

pub(crate) fn format_large_number(_: &Lua, amount: Value) -> Result<String> {
    Ok(match amount {
        Value::Integer(n) => n.to_string(),
        Value::Number(n) => format!("{:.0}", n),
        _ => "0".to_string(),
    })
}

pub(crate) fn ambiguate_name(_: &Lua, (full_name, context): (String, String)) -> Result<String> {
    let Some((name, _realm)) = full_name.split_once('-') else {
        return Ok(full_name);
    };
    let shortened = match context.as_str() {
        "none" => full_name,
        "short" | "guild" | "all" => name.to_string(),
        _ => name.to_string(),
    };
    Ok(shortened)
}
