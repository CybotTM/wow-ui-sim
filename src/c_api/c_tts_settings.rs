//! Persisted C_TTSSettings preferences, never public preference CVars.
//! Contract/evidence and unresolved native policies: docs/specs/c-factory-cf2-tts.md.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::stack_val;
use crate::saved_variables::SavedVariablesManager;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn storage_manager(state: &LuaState) -> LuaResult<SavedVariablesManager> {
    let directory = borrow_state(state)?.cvars.model_storage_dir();
    Ok(SavedVariablesManager::with_storage_dir(directory))
}

pub(crate) fn register(lua: &mut rilua::Lua) -> crate::Result<()> {
    // All 18 functions are declared on the other six pinned profiles, not Wrath.
    if ACTIVE == ClientProfile::Wrath {
        return Ok(());
    }
    super::ensure_namespace(lua.state_mut(), "C_TTSSettings")?;
    let manager = storage_manager(lua.state_mut())?;
    let saved = manager.load_model(lua.state_mut(), "TTSSettings")?;
    super::set_global_val(lua.state_mut(), "__tts_saved_value", saved);
    LuaApiMut::register_function(lua, "__tts_persist", persist_preferences)?;
    lua.exec(include_str!("c_tts_settings.lua"))?;
    Ok(())
}

fn persist_preferences(state: &mut LuaState) -> LuaResult<u32> {
    let value = stack_val(state, 1);
    if !matches!(value, Val::Table(_)) {
        return Err(rilua::runtime_error(
            "TTS persistence requires a preference table",
        ));
    }
    storage_manager(state)?
        .save_model(state, "TTSSettings", value)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    Ok(0)
}
