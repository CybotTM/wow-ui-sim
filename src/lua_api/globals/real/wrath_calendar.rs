//! Native 12340 Calendar event labels and guild invite defaults.
use crate::lua_api::methods::{borrow_state, table_get};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "CalendarEventGetTypes", event_types)?;
    LuaApiMut::register_function(lua, "CalendarEventGetRepeatOptions", repeat_options)?;
    LuaApiMut::register_function(lua, "CalendarDefaultGuildFilter", default_guild_filter)
}

fn event_types(state: &mut LuaState) -> LuaResult<u32> {
    push_localized_labels(
        state,
        &[
            "CALENDAR_TYPE_RAID",
            "CALENDAR_TYPE_DUNGEON",
            "CALENDAR_TYPE_PVP",
            "CALENDAR_TYPE_MEETING",
            "CALENDAR_TYPE_OTHER",
        ],
    )
}

fn repeat_options(state: &mut LuaState) -> LuaResult<u32> {
    push_localized_labels(
        state,
        &[
            "CALENDAR_REPEAT_NEVER",
            "CALENDAR_REPEAT_WEEKLY",
            "CALENDAR_REPEAT_BIWEEKLY",
            "CALENDAR_REPEAT_MONTHLY",
        ],
    )
}

fn push_localized_labels(state: &mut LuaState, keys: &[&str]) -> LuaResult<u32> {
    for key in keys {
        let value = table_get(state, Val::Table(state.global), key);
        if !matches!(value, Val::Str(_)) {
            return Err(runtime_error(format!(
                "missing localized Calendar text: {key}"
            )));
        }
        state.push(value);
    }
    Ok(keys.len() as u32)
}

fn default_guild_filter(state: &mut LuaState) -> LuaResult<u32> {
    let values = {
        let sim = borrow_state(state)?;
        let context = &sim.wrath_legacy.calendar;
        if context.player_present {
            let cap = [60.0, 70.0, 80.0]
                .get(context.account_expansion as usize)
                .copied()
                .ok_or_else(|| runtime_error("invalid Wrath account expansion"))?;
            [cap, cap, sim.world.guild_ranks.len() as f64]
        } else {
            [0.0; 3]
        }
    };
    for value in values {
        state.push(Val::Num(value));
    }
    Ok(3)
}
