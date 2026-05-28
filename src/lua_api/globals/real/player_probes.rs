//! Player-state probe globals backed by `SimState`.
//!
//! Migrates 5 entries off `GLOBAL_FALSE_STUBS` onto real Rust impls:
//!
//! - `IsLoggedIn()`          -> `SimState.is_logged_in`
//! - `IsMenuOpen()`          -> `SimState.menu_open`
//! - `IsXPUserDisabled()`    -> `SimState.xp_disabled`
//! - `PlayerCanTeleport()`   -> `SimState.can_teleport`
//! - `PlayerHasHearthstone()` -> `SimState.has_hearthstone`
//!
//! The module name avoids clashing with existing `player_api.rs` so
//! registration order is straightforward.

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

macro_rules! define_bool_probe {
    ($fn_name:ident, $field:ident) => {
        fn $fn_name(state: &mut LuaState) -> LuaResult<u32> {
            let v = borrow_state(state)?.$field;
            state.push(Val::Bool(v));
            Ok(1)
        }
    };
}

define_bool_probe!(is_logged_in, is_logged_in);
define_bool_probe!(is_menu_open, menu_open);
define_bool_probe!(is_xp_user_disabled, xp_disabled);
define_bool_probe!(player_can_teleport, can_teleport);
define_bool_probe!(player_has_hearthstone, has_hearthstone);

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsLoggedIn", is_logged_in)?;
    LuaApiMut::register_function(lua, "IsMenuOpen", is_menu_open)?;
    LuaApiMut::register_function(lua, "IsXPUserDisabled", is_xp_user_disabled)?;
    LuaApiMut::register_function(lua, "PlayerCanTeleport", player_can_teleport)?;
    LuaApiMut::register_function(lua, "PlayerHasHearthstone", player_has_hearthstone)?;
    Ok(())
}
