//! Empty legacy timed-quest domain until a timed-quest model exists.
//! QuestLogState and quest_surface catalog rows have no countdown/expiry inputs;
//! GetQuestLogTimeLeft's seeded world-quest constant is not a legacy timer model.
//! Retire when quest-log state can supply active legacy quest countdowns.
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetQuestTimers", empty_quest_timers)?;
    Ok(())
}

// Empty vararg, not one nil or the number zero: no timer rows may be rendered.
fn empty_quest_timers(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
