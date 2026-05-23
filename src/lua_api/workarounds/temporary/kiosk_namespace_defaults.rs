//! Temporary `Kiosk` namespace defaults.
//!
//! Kiosk mode is not modeled. These defaults keep startup probes inert while
//! making the unsupported surface explicit in the temporary workaround layer.

const KIOSK_NAMESPACE_DEFAULTS_LUA: &str = r#"
if type(Kiosk) ~= "table" then
    Kiosk = {}
end

if Kiosk.IsEnabled == nil then
    function Kiosk.IsEnabled()
        return false
    end
end

if Kiosk.IsCompetitiveModeEnabled == nil then
    function Kiosk.IsCompetitiveModeEnabled()
        return false
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(KIOSK_NAMESPACE_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_inert_kiosk_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(Kiosk) ~= "table" then
                    return "missing_namespace"
                end
                if Kiosk.IsEnabled() ~= false then
                    return "enabled"
                end
                if Kiosk.IsCompetitiveModeEnabled() ~= false then
                    return "competitive"
                end
                return "ok"
                "#,
            )
            .expect("kiosk defaults should be callable");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_kiosk_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            Kiosk = {
                IsEnabled = function() return true end,
                ExistingMember = 42,
            }
            "#,
        )
        .expect("fixture should install existing Kiosk table");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("kiosk defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if Kiosk.IsEnabled() ~= true then
                    return "overwrote_existing"
                end
                if Kiosk.ExistingMember ~= 42 then
                    return "lost_member"
                end
                if Kiosk.IsCompetitiveModeEnabled() ~= false then
                    return "missing_default"
                end
                return "ok"
                "#,
            )
            .expect("kiosk preservation probe should run");

        assert_eq!(result, "ok");
    }
}
