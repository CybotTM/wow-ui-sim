//! Wrath 12340 legacy getters over host-supplied selected-rank bank inputs.

use crate::lua_api::methods::borrow_state;
use crate::lua_api::state::GuildBankRankPermissions;
use crate::lua_api::wrath_arguments::convert_number;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

const BANK_TAB_COUNT: usize = 6;
const BANK_TAB_VIEW: i32 = 0x1;
const BANK_TAB_DEPOSIT: i32 = 0x2;
const BANK_TAB_EDIT_INFO: i32 = 0x4;
const MONEY_UI_DIVISOR: i64 = 10_000;

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetGuildBankTabPermissions", tab_permissions)?;
    LuaApiMut::register_function(lua, "GetGuildBankWithdrawLimit", withdraw_limit)?;
    Ok(())
}

fn selected_rank_permissions(state: &LuaState) -> LuaResult<GuildBankRankPermissions> {
    let sim = borrow_state(state)?;
    let world = &sim.world;
    let selected = world.guild_selected_rank.checked_sub(1);
    let index = selected.and_then(|rank| usize::try_from(rank).ok());
    index
        .and_then(|index| world.guild_bank_rank_permissions.get(&index))
        .cloned()
        // Non-native gap policy: native rank -1 is source-undefined. Do not
        // fabricate zero/unlimited rights for an unselected or unsupplied row.
        .ok_or_else(|| runtime_error("Guild bank selected-rank input unavailable"))
}

fn tab_permissions(state: &mut LuaState) -> LuaResult<u32> {
    let argument = stack_val(state, 1);
    let tab = convert_number(state, argument)?
        .ok_or_else(|| runtime_error("Usage: GetGuildBankTabPermissions(tab)"))?;
    // Native integer conversion precedes subtraction and uint8 narrowing.
    let index = (tab as i64).wrapping_sub(1) as u8 as usize;
    let (rights, stacks) = if index < BANK_TAB_COUNT {
        let rank = selected_rank_permissions(state)?;
        (rank.tab_rights[index], rank.tab_stacks_per_day[index])
    } else {
        (0, 0)
    };
    for mask in [BANK_TAB_VIEW, BANK_TAB_DEPOSIT, BANK_TAB_EDIT_INFO] {
        let enabled = rights & mask != 0;
        state.push(if enabled { Val::Num(1.0) } else { Val::Nil });
    }
    state.push(Val::Num(stacks as f64));
    Ok(4)
}

fn withdraw_limit(state: &mut LuaState) -> LuaResult<u32> {
    let stored = selected_rank_permissions(state)?.money_per_day;
    let limit = if stored >= 0 {
        stored / MONEY_UI_DIVISOR
    } else {
        stored
    };
    state.push(Val::Num(limit as f64));
    Ok(1)
}
