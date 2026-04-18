//! Rilua A_Admin handlers — addon registration scaffolding for tests.
//!
//! Production addon registration goes through the loader's
//! `load_single_addon` path. Tests that exercise the
//! `LoadAddOn`/`DisableAddOn` contract (without actually loading any
//! addon files from disk) need a way to push a fresh `AddonInfo` row
//! into `SimState.addons` so the enable/disable flag has something to
//! flip. `RegisterTestAddon(name)` is that hook — it pushes a stub
//! entry with `enabled = true`, `loaded = false`, no metadata; tests
//! can then call `C_AddOns.{Disable,Enable,Load}AddOn(name)` against it.

use crate::lua_api::AddonInfo;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::LuaResult;

pub(super) fn register_test_addon(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    if name.is_empty() {
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    if !sim.addons.iter().any(|a| a.folder_name == name) {
        sim.addons.push(AddonInfo {
            folder_name: name.clone(),
            title: name,
            enabled: true,
            loaded: false,
            ..Default::default()
        });
    }
    Ok(0)
}
