//! Temporary `C_SharedCharacterServices` fallback surface.
//!
//! Shared upgrade-distribution state is not modeled yet. Return an empty list
//! so character-service UI code sees no available shared upgrades.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_shared_character_services_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_SharedCharacterServices")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetUpgradeDistributions",
        get_upgrade_distributions,
    )
}

fn get_upgrade_distributions(state: &mut LuaState) -> LuaResult<u32> {
    let distributions = create_table(state);
    state.push(distributions);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn upgrade_distributions_default_to_empty_list() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let count: i32 = env
            .eval("local distributions = C_SharedCharacterServices.GetUpgradeDistributions(); return #distributions")
            .expect("upgrade distributions should be queryable");

        assert_eq!(count, 0);
    }
}
