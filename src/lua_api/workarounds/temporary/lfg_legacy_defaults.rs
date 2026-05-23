//! Temporary legacy LFG global defaults.
//!
//! Modern LFG list behavior is state-backed elsewhere. This module keeps small
//! legacy global probes explicit until those callers are modeled properly.

const LFG_LEGACY_DEFAULTS_LUA: &str = r#"
if GetLFGCategoryForID == nil then
    function GetLFGCategoryForID() return 0 end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(LFG_LEGACY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_lfg_category_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let category_id: i32 = env
            .eval("return GetLFGCategoryForID(123)")
            .expect("legacy LFG category probe should run");

        assert_eq!(category_id, 0);
    }

    #[test]
    fn preserves_existing_lfg_category_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("function GetLFGCategoryForID() return 7 end")
            .expect("fixture should install existing LFG category function");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("legacy LFG defaults should apply");
        }

        let category_id: i32 = env
            .eval("return GetLFGCategoryForID(123)")
            .expect("legacy LFG category preservation probe should run");

        assert_eq!(category_id, 7);
    }
}
