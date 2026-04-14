//! Lightweight loader environment for addon loading.
//!
//! Borrows the Lua instance instead of owning it, allowing both startup loading
//! (via WowLuaEnv) and runtime on-demand loading (from Lua callbacks).

use super::env_init::{addon_unpack_function, addon_unpack_key};
use super::state::SimState;
use crate::Result;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

/// Lightweight loader environment that borrows the Lua instance.
pub struct LoaderEnv<'a> {
    pub(crate) lua: &'a Lua,
    pub(crate) state: Rc<RefCell<SimState>>,
}

impl<'a> LoaderEnv<'a> {
    fn loading_addon_uses_secure_env(&self) -> bool {
        let state = self.state.borrow();
        state
            .loading_addon_index
            .and_then(|idx| state.addons.get(idx as usize))
            .map(|addon| addon.use_secure_env)
            .unwrap_or(false)
    }

    /// Create from a Lua reference and shared state (for runtime loading).
    pub fn new(lua: &'a Lua, state: Rc<RefCell<SimState>>) -> Self {
        Self { lua, state }
    }

    /// Execute Lua code.
    pub fn exec(&self, code: &str) -> Result<()> {
        let func = crate::loader::chunk_cache::load_chunk(self.lua, code, "loader-exec")?;
        if self.loading_addon_uses_secure_env() {
            super::secure_env::apply_secure_env(self.lua, &func)?;
        }
        func.call::<()>(())?;
        Ok(())
    }

    /// Execute Lua code with varargs (addon loading pattern).
    pub fn exec_with_varargs(
        &self,
        code: &str,
        name: &str,
        addon_name: &str,
        addon_table: mlua::Table,
    ) -> Result<()> {
        let chunk = self.lua.load(code).set_name(name);
        let func: mlua::Function = chunk.into_function()?;
        func.call::<()>((addon_name, addon_table))?;
        Ok(())
    }

    /// Create a new empty table for addon private storage.
    pub fn create_addon_table(&self) -> Result<mlua::Table> {
        let table = self.lua.create_table()?;
        let unpack_fn = addon_unpack_function(self.lua)?;
        let unpack_key = addon_unpack_key(self.lua)?;
        table.raw_set(unpack_key, unpack_fn)?;
        Ok(table)
    }

    /// Get access to the Lua state.
    pub fn lua(&self) -> &Lua {
        self.lua
    }

    /// Get access to the simulator state.
    pub fn state(&self) -> &Rc<RefCell<SimState>> {
        &self.state
    }

    /// Fire an event with arguments to all registered frames.
    pub fn fire_event_with_args(&self, event: &str, args: &[mlua::Value]) -> Result<()> {
        use super::script_helpers::{
            call_error_handler, dispatch_frame_unit_event_callbacks, get_frame_ref, get_script,
        };
        use std::time::Instant;

        let listeners = {
            let state = self.state.borrow();
            state.widgets.get_event_listeners(event)
        };

        for widget_id in listeners {
            if let Some(frame) = get_frame_ref(self.lua, widget_id) {
                let addon_idx = self
                    .state
                    .borrow()
                    .widgets
                    .get(widget_id)
                    .and_then(|f| f.owner_addon);
                if let Some(handler) = get_script(self.lua, widget_id, "OnEvent") {
                    let mut call_args = vec![
                        frame.clone(),
                        mlua::Value::String(self.lua.create_string(event)?),
                    ];
                    call_args.extend(args.iter().cloned());
                    let start = Instant::now();
                    if let Err(e) = handler.call::<()>(mlua::MultiValue::from_vec(call_args)) {
                        call_error_handler(self.lua, &e.to_string());
                    }
                    if let Some(idx) = addon_idx {
                        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                        let mut state = self.state.borrow_mut();
                        if let Some(addon) = state.addons.get_mut(idx as usize) {
                            addon.runtime.current_frame_ms += elapsed_ms;
                        }
                    }
                }
                dispatch_frame_unit_event_callbacks(self.lua, widget_id, frame, args, event)?;
            }
        }

        Ok(())
    }
}
