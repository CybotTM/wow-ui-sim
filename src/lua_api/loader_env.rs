//! Lightweight loader environment for addon loading.

use super::env::WowLuaEnv;
use super::globals::rilua_security::apply_secure_env_rilua;
use super::rilua_methods::create_string;
use crate::Result;
use crate::lua_api::rilua_methods::create_table;
use crate::lua_bridge::table_set_rust_fn;
use rilua::LuaApiMut;
use rilua::Val;
use rilua::vm::state::LuaState;
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
use std::rc::Rc;

use super::state::SimState;

pub struct LoaderEnv<'a> {
    lua: Rc<std::cell::RefCell<rilua::Lua>>,
    state: Rc<std::cell::RefCell<SimState>>,
    _marker: PhantomData<&'a WowLuaEnv>,
}

impl<'a> LoaderEnv<'a> {
    pub fn new(env: &'a WowLuaEnv) -> Self {
        Self {
            lua: Rc::clone(&env.lua),
            state: Rc::clone(&env.state),
            _marker: PhantomData,
        }
    }

    pub fn from_parts(
        lua: Rc<std::cell::RefCell<rilua::Lua>>,
        state: Rc<std::cell::RefCell<SimState>>,
    ) -> LoaderEnv<'static> {
        LoaderEnv {
            lua,
            state,
            _marker: PhantomData,
        }
    }

    fn loading_addon_uses_secure_env(&self) -> bool {
        let state = self.state.borrow();
        state
            .loading_addon_index
            .and_then(|idx| state.addons.get(idx as usize))
            .map(|addon| addon.use_secure_env)
            .unwrap_or(false)
    }

    pub fn exec(&self, code: &str) -> Result<()> {
        let mut lua = self.lua.borrow_mut();
        let func = crate::loader::chunk_cache::load_chunk(&mut lua, code, "loader-exec")
            .map_err(|e| crate::Error::Other(e.to_string()))?;
        if self.loading_addon_uses_secure_env() {
            apply_secure_env_rilua(&mut lua, &func)?;
        }
        lua.call_function(&func, &[])?;
        Ok(())
    }

    pub fn exec_with_varargs(
        &self,
        code: &str,
        name: &str,
        addon_name: &str,
        addon_table: Val,
    ) -> Result<()> {
        let mut lua = self.lua.borrow_mut();
        let func = rilua::LuaApiMut::load_bytes(&mut *lua, code.as_bytes(), name)?;
        let addon_name = create_string(lua.state_mut(), addon_name);
        lua.call_function(&func, &[addon_name, addon_table])?;
        Ok(())
    }

    pub fn fire_event_with_args(&self, event: &str, args: &[Val]) -> Result<()> {
        let listeners = {
            let mut lua = self.lua.borrow_mut();
            crate::lua_api::rilua_script_helpers::get_event_listeners(lua.state_mut(), event)
        };
        for widget_id in listeners {
            let mut lua = self.lua.borrow_mut();
            let handler = {
                let state = lua.state_mut();
                crate::lua_api::rilua_script_helpers::get_script(state, widget_id, "OnEvent")
            };
            let Some(handler) = handler else {
                continue;
            };
            let frame = {
                let state = lua.state_mut();
                crate::lua_api::rilua_methods::frame_ref(state, widget_id)?
            };
            let event_name = {
                let state = lua.state_mut();
                crate::lua_api::rilua_methods::create_string(state, event)
            };
            let mut call_args = Vec::with_capacity(args.len() + 2);
            call_args.push(frame);
            call_args.push(event_name);
            call_args.extend_from_slice(args);
            let _ = crate::lua_api::rilua_methods::call_function(&mut lua, handler, &call_args);
        }
        Ok(())
    }

    pub fn create_addon_table(&self) -> Result<Val> {
        let mut lua = self.lua.borrow_mut();
        create_addon_table(&mut lua)
    }

    pub fn lua(&self) -> &Rc<std::cell::RefCell<rilua::Lua>> {
        &self.lua
    }

    pub fn rilua(&self) -> Ref<'_, rilua::Lua> {
        self.lua.borrow()
    }

    pub fn rilua_mut(&self) -> RefMut<'_, rilua::Lua> {
        self.lua.borrow_mut()
    }

    pub fn state(&self) -> &Rc<std::cell::RefCell<SimState>> {
        &self.state
    }
}

pub(crate) fn create_addon_table(lua: &mut rilua::Lua) -> Result<Val> {
    let state = lua.state_mut();
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(state, table_ref, "unpack", addon_table_unpack)?;
    Ok(table)
}

fn addon_table_unpack(state: &mut LuaState) -> rilua::LuaResult<u32> {
    let table = state.stack_get(state.base);
    let values = addon_table_values(state, table);
    for value in values {
        state.push(value);
    }
    Ok(4)
}

fn addon_table_values(state: &LuaState, table: Val) -> [Val; 4] {
    let Val::Table(table_ref) = table else {
        return [Val::Nil, Val::Nil, Val::Nil, Val::Nil];
    };
    let Some(table) = state.gc.tables.get(table_ref) else {
        return [Val::Nil, Val::Nil, Val::Nil, Val::Nil];
    };
    let values = table.array_slice();
    [
        values.first().copied().unwrap_or(Val::Nil),
        values.get(1).copied().unwrap_or(Val::Nil),
        values.get(2).copied().unwrap_or(Val::Nil),
        values.get(3).copied().unwrap_or(Val::Nil),
    ]
}
