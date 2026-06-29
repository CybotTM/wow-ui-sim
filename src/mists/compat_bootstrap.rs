//! Loader for the mists compatibility Lua bootstrap.
//!
//! Mists-specific stubs for ~46 globals that mists FrameXML/AddOns reference
//! but the simulator's retail-tuned `lua_api/globals/` doesn't register
//! (mostly pre-Cata leftovers MoP kept, plus a few mists-only helpers).

const MISTS_COMPAT_BOOTSTRAP_LUA: &str = include_str!("compat_bootstrap.lua");

pub fn init(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(MISTS_COMPAT_BOOTSTRAP_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn legacy_text_status_bar_globals_delegate_to_mixin_methods() {
        let env = WowLuaEnv::new().expect("env");

        let calls: String = env
            .eval(
                r#"
                local calls = {}
                local bar = {}

                function bar:InitializeTextStatusBar() table.insert(calls, "init") end
                function bar:SetBarText(text, left, right)
                  table.insert(calls, "text:" .. text .. ":" .. left .. ":" .. right)
                end
                function bar:SetBarTextPrefix(prefix) table.insert(calls, "prefix:" .. prefix) end
                function bar:TextStatusBarOnEvent(event, value) table.insert(calls, "event:" .. event .. ":" .. value) end
                function bar:UpdateTextString() table.insert(calls, "update") end
                function bar:OnStatusBarValueChanged() table.insert(calls, "value") end
                function bar:ShowStatusBarText() table.insert(calls, "show") end
                function bar:HideStatusBarText() table.insert(calls, "hide") end

                TextStatusBar_Initialize(bar)
                SetTextStatusBarText(bar, "XP", "L", "R")
                SetTextStatusBarTextPrefix(bar, "XP")
                TextStatusBar_OnEvent(bar, "CVAR_UPDATE", "status")
                TextStatusBar_UpdateTextString(bar)
                TextStatusBar_OnValueChanged(bar)
                ShowTextStatusBarText(bar)
                HideTextStatusBarText(bar)

                return table.concat(calls, ",")
                "#,
            )
            .expect("text status bar delegate probe should run");

        assert_eq!(
            calls,
            "init,text:XP:L:R,prefix:XP,event:CVAR_UPDATE:status,update,value,show,hide"
        );
    }

    #[test]
    fn mists_splash_frame_default_starts_closed_and_can_close() {
        let env = WowLuaEnv::new().expect("env");

        let result: String = env
            .eval(
                r#"
                if SplashFrame == nil then return "missing" end
                if SplashFrame:IsShown() then return "shown" end
                SplashFrame:Show()
                SplashFrame:Close()
                if SplashFrame:IsShown() then return "not_closed" end
                return "ok"
                "#,
            )
            .expect("splash frame probe should run");

        assert_eq!(result, "ok");
    }

    #[test]
    fn proving_grounds_world_state_handlers_are_startup_safe() {
        let env = WowLuaEnv::new().expect("env");

        let result: String = env
            .eval(
                r#"
                local frame = CreateFrame("Frame", "MistsProvingGroundsProbe", UIParent)
                frame:RegisterEvent("PROVING_GROUNDS_SCORE_UPDATE")
                frame.statusBar = CreateFrame("StatusBar", nil, frame)
                frame.statusBar.timeLeft = frame:CreateFontString(nil, "OVERLAY", "GameFontHighlight")
                frame.Wave = frame:CreateFontString(nil, "OVERLAY", "GameFontHighlight")
                frame.Score = frame:CreateFontString(nil, "OVERLAY", "GameFontHighlight")

                WorldStateProvingGrounds_OnLoad(frame)
                WorldStateProvingGrounds_OnEvent(frame, "PROVING_GROUNDS_SCORE_UPDATE", 42)
                WorldStateProvingGroundsTimer_OnUpdate(frame, 0.25)
                WorldStateProvingGroundsAnim_OnFinished({ GetParent = function() return frame end })

                if frame.Score:GetText() ~= "42" then return "score" end
                return "ok"
                "#,
            )
            .expect("proving grounds handlers should run");

        assert_eq!(result, "ok");
    }
}
