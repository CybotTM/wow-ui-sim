//! Permanent-shim namespace wrapper for lua_api globals.
//!
//! Real shim implementations live in `super::stubs`; this module makes the
//! stable shim bucket explicit in the directory tree.

use rilua::vm::state::LuaState;

pub fn register_all(state: &mut LuaState) {
    super::stubs::register_all(state);
}
