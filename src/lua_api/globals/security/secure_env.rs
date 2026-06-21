//! Secure environment (secure_env.rs counterpart).
//!
//! `create_secure_environment` and `mark_secure` mirror the old mlua
//! secureenv path: create a shallow copy of `_G`, give it its own `Enum`,
//! expose it as `__secureenv`, and retarget secure addon chunks to that env.
//! Secureenv is deliberately separate from `_G`; later `_G` writes are not
//! visible unless a Blizzard bridge explicitly copies them into secureenv.

use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

use crate::lua_api::methods::{registry_get, registry_set};

const CREATE_SECURE_ENV_LUA: &str = r##"
    local genv = _G
    local secureenv = {}
    for k, v in pairs(genv) do
        if k ~= "SOUNDKIT" then
            secureenv[k] = v
        end
    end
    if genv.Enum then
        local se = {}
        for k, v in pairs(genv.Enum) do
            se[k] = v
        end
        secureenv.Enum = se
    end
    secureenv._G = secureenv
    return secureenv
	"##;

pub(crate) fn secure_env_table(
    state: &mut LuaState,
) -> LuaResult<rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>> {
    match registry_get(state, "__secureenv") {
        Val::Table(table_ref) => Ok(table_ref),
        _ => Err(runtime_error("secure environment not initialized")),
    }
}

pub(crate) fn set_secure_env_key_state(
    state: &mut LuaState,
    key: &str,
    value: Val,
) -> LuaResult<()> {
    let secureenv_ref = secure_env_table(state)?;
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(secureenv) = state.gc.tables.get_mut(secureenv_ref) {
        let _ = secureenv.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(secureenv_ref);
    Ok(())
}

/// Create the secure environment as a shallow copy of `_G`.
///
/// This preserves secure APIs when `Blizzard_EnvironmentCleanup` nils them
/// from `_G`. Later `_G` registrations do not fall through; secure-visible
/// exports must be written directly into secureenv.
pub fn create_secure_environment(lua: &mut rilua::Lua) -> LuaResult<()> {
    if matches!(LuaApiMut::get_global_val(lua, "__secureenv"), Val::Table(_)) {
        return Ok(());
    }

    let chunk = LuaApiMut::load(lua, CREATE_SECURE_ENV_LUA)?;
    let secureenv = lua
        .call_function(&chunk, &[])?
        .into_iter()
        .next()
        .ok_or_else(|| runtime_error("create_secure_environment: missing return value"))?;
    let Val::Table(_) = secureenv else {
        return Err(runtime_error(
            "create_secure_environment: expected secureenv table",
        ));
    };

    {
        let state = lua.state_mut();
        registry_set(state, "__secureenv", secureenv);
    }
    LuaApiMut::set_global_val(lua, "__secureenv", secureenv)?;
    Ok(())
}

/// Mark a compiled function as running under the secure environment.
///
/// Swaps the function's fenv to the registry-stored `__secureenv` table so
/// the closure and every inner closure it creates see `secureenv` as their
/// globals table. Matches Blizzard's `setfenv(chunk, secureenv)` step for
/// `[LoadIntoEnvironment secure]` files — caller never passes secureenv
/// explicitly; it's looked up via the registry.
pub fn mark_secure(lua: &mut rilua::Lua, func: &rilua::Function) -> LuaResult<()> {
    mark_secure_state(lua.state_mut(), func)
}

/// Raw-state variant of [`mark_secure`] for callers holding `&mut LuaState`
/// (e.g. inside a `with_state` closure or a `RustFn`).
pub fn mark_secure_state(state: &mut LuaState, func: &rilua::Function) -> LuaResult<()> {
    let secureenv_ref = secure_env_table(state)?;
    let secureenv = rilua::Table::from_gc_ref(secureenv_ref);
    rilua::api::state_set_fenv(state, func, &secureenv)
}

/// Set a key in both the global table and secureenv.
///
/// Used for names that must stay visible in secure code even after cleanup
/// strips them from `_G`.
pub fn set_in_both_envs_rilua(lua: &mut rilua::Lua, key: &str, val: Val) -> LuaResult<()> {
    LuaApiMut::set_global_val(lua, key, val)?;
    let state = lua.state_mut();
    if let Ok(secureenv_ref) = secure_env_table(state) {
        let secureenv = rilua::Table::from_gc_ref(secureenv_ref);
        let key_ref = state.gc.intern_string(key.as_bytes());
        secureenv.raw_set(state, Val::Str(key_ref), val)?;
    }
    Ok(())
}
