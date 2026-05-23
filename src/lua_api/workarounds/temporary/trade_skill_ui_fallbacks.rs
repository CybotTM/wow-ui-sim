//! Temporary `C_TradeSkillUI` fallback helpers.
//!
//! Recipe tracking is Rust-backed by the profession surface. These remaining
//! helpers only keep profession UI startup paths tolerant while that broader
//! trade-skill model is incomplete.

const TRADE_SKILL_UI_FALLBACKS_LUA: &str = r#"
if type(C_TradeSkillUI) ~= "table" then
    C_TradeSkillUI = {}
end

if rawget(C_TradeSkillUI, "GetProfessionSkillLineID") == nil then
    function C_TradeSkillUI.GetProfessionSkillLineID(professionID)
        return tonumber(professionID) or 0
    end
end

if rawget(C_TradeSkillUI, "IsGuildTradeSkillsEnabled") == nil then
    function C_TradeSkillUI.IsGuildTradeSkillsEnabled()
        return false
    end
end

if rawget(C_TradeSkillUI, "GetTradeSkillTexture") == nil then
    function C_TradeSkillUI.GetTradeSkillTexture()
        return nil
    end
end

if rawget(C_TradeSkillUI, "GetTradeSkillDisplayName") == nil then
    function C_TradeSkillUI.GetTradeSkillDisplayName()
        return ""
    end
end

if rawget(C_TradeSkillUI, "OpenTradeSkill") == nil then
    function C_TradeSkillUI.OpenTradeSkill()
        local frame = rawget(_G, "ProfessionsFrame")
        if frame ~= nil and type(frame.Show) == "function" then
            frame:Show()
        end
        return frame ~= nil
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(TRADE_SKILL_UI_FALLBACKS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_ui_fallbacks_without_replacing_recipe_tracking() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(C_TradeSkillUI.SetRecipeTracked) ~= "function" then
                    return "missing_rust_recipe_tracking"
                end
                if C_TradeSkillUI.GetProfessionSkillLineID(164) ~= 164 then
                    return "bad_skill_line"
                end
                if pcall(C_TradeSkillUI.GetProfessionSkillLineID, "bad") then
                    return "lost_rust_type_check"
                end
                if C_TradeSkillUI.IsGuildTradeSkillsEnabled() then
                    return "bad_guild_flag"
                end
                if type(C_TradeSkillUI.GetTradeSkillTexture) ~= "function" then
                    return "missing_rust_texture"
                end
                if C_TradeSkillUI.GetTradeSkillDisplayName() ~= "" then
                    return "bad_display_name"
                end
                if type(C_TradeSkillUI.OpenTradeSkill) ~= "function" then
                    return "missing_rust_open"
                end
                return "ok"
                "#,
            )
            .expect("trade skill fallback probe should run");

        assert_eq!(result, "ok");
    }
}
