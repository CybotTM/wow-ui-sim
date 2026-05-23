//! Temporary legacy container/global locale wrappers.
//!
//! `C_Container` is the state-backed API surface. These globals exist for older
//! Blizzard/addon callers that still use pre-namespace names.

const LEGACY_CONTAINER_GLOBALS_LUA: &str = r#"
if GetLocale == nil then
    function GetLocale()
        return "enUS"
    end
end
if GAME_LOCALE == nil then
    GAME_LOCALE = GetLocale()
end
if GetContainerItemInfo == nil and C_Container ~= nil then
    function GetContainerItemInfo(...)
        return C_Container.GetContainerItemInfo(...)
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(LEGACY_CONTAINER_GLOBALS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn keeps_game_locale_and_container_global_callable() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("legacy container globals should apply");
        }

        let result: String = env
            .eval(
                r#"
                if GetLocale() ~= "enUS" then return "get_locale" end
                if GAME_LOCALE ~= GetLocale() then return "locale" end
                if type(GetContainerItemInfo) ~= "function" then return "container_info" end
                local item_info = GetContainerItemInfo(0, 1)
                if type(item_info) ~= "table" then return "item_info" end
                return "ok"
                "#,
            )
            .expect("legacy container global probe should run");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_legacy_container_globals() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            GAME_LOCALE = "custom"
            GetLocale = function() return "frFR" end
            GetContainerItemInfo = function() return "existing" end
            "#,
        )
        .expect("fixture should install existing legacy container globals");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("legacy container globals should apply");
        }

        let (game_locale, locale, item_info): (String, String, String) = env
            .eval("return GAME_LOCALE, GetLocale(), GetContainerItemInfo(0, 1)")
            .expect("legacy container preservation probe should run");

        assert_eq!(game_locale, "custom");
        assert_eq!(locale, "frFR");
        assert_eq!(item_info, "existing");
    }
}
