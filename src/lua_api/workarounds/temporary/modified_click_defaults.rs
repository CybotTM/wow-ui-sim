//! Temporary modified-click setting defaults.
//!
//! Real WoW stores these in client CVar/input settings. The simulator only
//! needs enough state for startup UI paths until input settings are modeled.

const MODIFIED_CLICK_DEFAULTS_LUA: &str = r#"
local __wow_modified_clicks = __wow_modified_clicks or {}
if GetModifiedClick == nil then
    function GetModifiedClick(action)
        return __wow_modified_clicks[action] or "NONE"
    end
end
if SetModifiedClick == nil then
    function SetModifiedClick(action, modifier)
        __wow_modified_clicks[action] = modifier or "NONE"
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(MODIFIED_CLICK_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_modified_click_state_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("GetModifiedClick = nil; SetModifiedClick = nil")
            .expect("fixture should clear modified-click globals");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("modified-click defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if GetModifiedClick("SELFCAST") ~= "NONE" then return "default" end
                SetModifiedClick("SELFCAST", "ALT")
                if GetModifiedClick("SELFCAST") ~= "ALT" then return "updated" end
                SetModifiedClick("SELFCAST", nil)
                if GetModifiedClick("SELFCAST") ~= "NONE" then return "nil_default" end
                return "ok"
                "#,
            )
            .expect("modified-click state probe should run");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_modified_click_globals() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            GetModifiedClick = function(action) return "custom:" .. action end
            SetModifiedClick = function(action, modifier)
                modifiedClickSet = action .. "=" .. tostring(modifier)
            end
            "#,
        )
        .expect("fixture should install existing modified-click globals");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("modified-click defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if GetModifiedClick("SELFCAST") ~= "custom:SELFCAST" then return "get" end
                SetModifiedClick("SELFCAST", "SHIFT")
                if modifiedClickSet ~= "SELFCAST=SHIFT" then return "set" end
                return "ok"
                "#,
            )
            .expect("modified-click preservation probe should run");

        assert_eq!(result, "ok");
    }
}
