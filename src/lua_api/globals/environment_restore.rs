//! Post-cleanup global restoration hooks.

use crate::lua_api::SimState;
use std::cell::RefCell;
use std::rc::Rc;

pub fn restore_post_cleanup_globals(
    lua: &mut rilua::Lua,
    state: Rc<RefCell<SimState>>,
) -> crate::Result<()> {
    super::register::register_globals(lua, state)
}
