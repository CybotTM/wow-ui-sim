//! Wrath 12340 guild-info readers over existing world cache and rank rights.
//! Rights position 13 is native mask 0x10000 (see wrath-guild-rank-flags spec).
use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const EDIT_GUILD_INFO_RIGHT_INDEX: usize = 12;

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(
        lua,
        "GetGuildInfoText",
        super::super::guild_info::get_info_text,
    )?;
    LuaApiMut::register_function(lua, "CanEditGuildInfo", can_edit_guild_info)?;
    Ok(())
}

fn can_edit_guild_info(state: &mut LuaState) -> LuaResult<u32> {
    let allowed = {
        let sim = borrow_state(state)?;
        let world = &sim.world;
        let rank = world.guild_ranks.get(world.guild_player_rank);
        let has_right =
            rank.is_some_and(|rank| rank.flags.get(EDIT_GUILD_INFO_RIGHT_INDEX) == Some(&true));
        sim.player.present && world.guild_id != 0 && has_right
    };
    state.push(if allowed { Val::Num(1.0) } else { Val::Nil });
    Ok(1)
}
