//! Explicit RAF summon cooldown input, in seconds, shared by legacy and C_* APIs.
//! Zero timing and a disabled timer are simulator defaults, not native parity.

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Debug, Clone, Copy, Default)]
pub struct SummonFriendCooldown {
    pub start_time_seconds: f64,
    pub duration_seconds: f64,
    pub enable_cooldown_timer: bool,
}

pub(crate) fn get_summon_friend_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let cooldown = borrow_state(state)?.summon_friend_cooldown;
    state.push(Val::Num(cooldown.start_time_seconds));
    state.push(Val::Num(cooldown.duration_seconds));
    state.push(Val::Bool(cooldown.enable_cooldown_timer));
    Ok(3)
}
