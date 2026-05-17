use rilua::Val;
use rilua::vm::state::LuaState;

use crate::lua_api::methods::create_string;

pub(super) fn push_missing_addon_info(state: &mut LuaState, addon_name: &str) -> u32 {
    let name = create_string(state, addon_name);
    let reason = create_string(state, "MISSING");
    state.push(name);
    state.push(Val::Nil);
    state.push(Val::Nil);
    state.push(Val::Bool(false));
    state.push(reason);
    5
}
