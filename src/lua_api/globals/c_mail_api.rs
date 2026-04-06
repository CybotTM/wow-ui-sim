use crate::lua_api::state::SimState;
use mlua::{Lua, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_inbox_read_api(lua, state)?;
    Ok(())
}

fn register_inbox_read_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();

    g.set(
        "GetInboxNumItems",
        lua.create_function({
            let s = Rc::clone(&state);
            move |_, ()| {
                let st = s.borrow();
                let count = st.player.inbox.len() as i32;
                Ok((count, count))
            }
        })?,
    )?;

    Ok(())
}
