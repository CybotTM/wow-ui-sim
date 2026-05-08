mod core;

use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(super) fn register_auction_house_surface(state: &mut LuaState) -> LuaResult<()> {
    core::register_auction_house_surface(state)
}
