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
}
