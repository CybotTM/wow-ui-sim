//! Temporary top-level UI parent helpers.
//!
//! Blizzard UI code expects these globals during isolated addon loads. The
//! simulator only models the shallow parent-selection behavior here, so keep it
//! explicit in the temporary workaround layer.

const TOP_LEVEL_PARENT_DEFAULTS_LUA: &str = r#"
if type(GetAppropriateTopLevelParent) ~= "function" then
  __wow_root_ui_parent = rawget(_G, "UIParent")
  __wow_alternate_top_level_parent = nil

  function SetAlternateTopLevelParent(parent)
    __wow_alternate_top_level_parent = parent
    if type(EventRegistry) == "table" and type(EventRegistry.TriggerEvent) == "function" then
      EventRegistry:TriggerEvent("UI.AlternateTopLevelParentChanged", parent)
    end
  end

  function ClearAlternateTopLevelParent()
    __wow_alternate_top_level_parent = nil
    if type(EventRegistry) == "table" and type(EventRegistry.TriggerEvent) == "function" then
      EventRegistry:TriggerEvent("UI.AlternateTopLevelParentChanged")
    end
  end

  function GetAppropriateTopLevelParent(optionalExcludedParent)
    if __wow_alternate_top_level_parent
      and type(__wow_alternate_top_level_parent.IsShown) == "function"
      and __wow_alternate_top_level_parent:IsShown()
      and (not optionalExcludedParent or __wow_alternate_top_level_parent ~= optionalExcludedParent)
    then
      return __wow_alternate_top_level_parent
    end

    if __wow_root_ui_parent ~= nil and __wow_root_ui_parent ~= optionalExcludedParent then
      return __wow_root_ui_parent
    end

    return UIParent or GlueParent
  end

  function SetAppropriateTopLevelParent(frame)
    local parent = GetAppropriateTopLevelParent()
    if frame and parent and type(frame.SetParent) == "function" then
      frame:SetParent(parent)
    end
  end
end

if type(GetAppropriateTooltip) ~= "function" then
  function GetAppropriateTooltip()
    return UIParent and GameTooltip or GlueTooltip
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(TOP_LEVEL_PARENT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_top_level_parent_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("top-level parent defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                local seenEvent = false
                local originalTriggerEvent = EventRegistry.TriggerEvent
                EventRegistry.TriggerEvent = function(_self, eventName)
                  if eventName == "UI.AlternateTopLevelParentChanged" then
                    seenEvent = true
                  end
                end
                local alternate = CreateFrame("Frame", "TopLevelParentFallbackProbe", UIParent)
                alternate:Show()
                SetAlternateTopLevelParent(alternate)
                EventRegistry.TriggerEvent = originalTriggerEvent
                if not seenEvent then return "event" end
                if GetAppropriateTopLevelParent() ~= alternate then return "alternate" end
                if GetAppropriateTopLevelParent(alternate) ~= UIParent then return "excluded" end
                local child = CreateFrame("Frame", nil, UIParent)
                SetAppropriateTopLevelParent(child)
                if child:GetParent() ~= alternate then return "set_parent" end
                ClearAlternateTopLevelParent()
                if GetAppropriateTopLevelParent() ~= UIParent then return "cleared" end
                if GetAppropriateTooltip() ~= GameTooltip then return "tooltip" end
                return "ok"
                "#,
            )
            .expect("top-level parent defaults should be callable");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_top_level_parent_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function GetAppropriateTopLevelParent() return "existing_parent" end
            function GetAppropriateTooltip() return "existing_tooltip" end
            "#,
        )
        .expect("fixture should install existing parent helpers");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("top-level parent defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if GetAppropriateTopLevelParent() ~= "existing_parent" then return "overwrote_parent" end
                if GetAppropriateTooltip() ~= "existing_tooltip" then return "overwrote_tooltip" end
                return "ok"
                "#,
            )
            .expect("top-level parent preservation probe should run");

        assert_eq!(result, "ok");
    }
}
