//! Temporary `C_GameRules` namespace fallback.
//!
//! Game rule queries are Rust-backed by `SimState::game_rules`. A few
//! unmodeled compatibility probes still expect missing `C_GameRules` members
//! to resolve to no-op functions during startup.

const GAME_RULES_NAMESPACE_FALLBACK_LUA: &str = r#"
if type(C_GameRules) ~= "table" then
    C_GameRules = {}
end

GameRulesUtil = GameRulesUtil or {}
if rawget(GameRulesUtil, "ShouldShowPlayerCastBar") == nil then
    function GameRulesUtil.ShouldShowPlayerCastBar()
        return true
    end
end

local function GameRulesNamespaceFallback(t, key)
    if type(__wow_record_nil_symbol_access) == "function" then
        __wow_record_nil_symbol_access("C_GameRules", key, nil, nil)
    end
    local fn = function()
        return nil
    end
    rawset(t, key, fn)
    return fn
end

local mt = getmetatable(C_GameRules)
if mt == nil then
    setmetatable(C_GameRules, { __index = GameRulesNamespaceFallback })
elseif mt.__index == nil then
    mt.__index = GameRulesNamespaceFallback
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(GAME_RULES_NAMESPACE_FALLBACK_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn preserves_rust_backed_methods_and_adds_unmodeled_member_fallback() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("A_Admin.SetGameRule('Hardcore', true)")
            .expect("admin game rule should set");

        let result: String = env
            .eval(
                r#"
                if not C_GameRules.IsGameRuleActive("Hardcore") then
                    return "lost_rust_method"
                end
                if type(C_GameRules.IsHardcoreActive) ~= "function" then
                    return "missing_fallback"
                end
                if C_GameRules.IsHardcoreActive() ~= nil then
                    return "fallback_returned_value"
                end
                if type(GameRulesUtil) ~= "table" then
                    return "missing_util"
                end
                if GameRulesUtil.ShouldShowPlayerCastBar() ~= true then
                    return "bad_cast_bar_rule"
                end
                return "ok"
                "#,
            )
            .expect("game rules fallback probe should run");

        assert_eq!(result, "ok");
    }
}
