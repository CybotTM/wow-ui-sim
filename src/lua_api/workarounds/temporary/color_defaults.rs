//! Temporary UI color defaults.
//!
//! These values keep Blizzard startup code and addon probes working until the
//! simulator has a real color registry surface.

const COLOR_DEFAULTS_LUA: &str = r#"
local function __wow_color_merge_namespace(existing, defaults)
  local namespace = type(existing) == "table" and existing or {}
  for key, value in pairs(defaults or {}) do
    if rawget(namespace, key) == nil then
      rawset(namespace, key, value)
    end
  end
  return namespace
end

C_UIColor = __wow_color_merge_namespace(C_UIColor, {
  GetColors = function()
    return {
      { baseTag = "HIGHLIGHT_FONT_COLOR", color = { r = 1, g = 1, b = 1, a = 1 } },
      { baseTag = "PLAYER_FACTION_COLOR_HORDE", color = { r = 1, g = 0.1, b = 0.1, a = 1 } },
      { baseTag = "PLAYER_FACTION_COLOR_ALLIANCE", color = { r = 0.2, g = 0.4, b = 1, a = 1 } },
      { baseTag = "NORMAL_FONT_COLOR", color = { r = 1, g = 0.82, b = 0, a = 1 } },
      -- Blizzard_Professions panels look up the tradeskill experience bar
      -- fill color by baseTag in the C_UIColor.GetColors() return value.
      { baseTag = "TRADESKILL_EXPERIENCE_COLOR", color = { r = 0.25, g = 0.25, b = 0.75, a = 1 } },
    }
  end,
})

QuestDifficultyColors = QuestDifficultyColors or {}
QuestDifficultyColors.trivial = QuestDifficultyColors.trivial or { r = 0.50, g = 0.50, b = 0.50 }
QuestDifficultyColors.standard = QuestDifficultyColors.standard or { r = 0.25, g = 0.75, b = 0.25 }
QuestDifficultyColors.difficult = QuestDifficultyColors.difficult or { r = 1.00, g = 1.00, b = 0.00 }
QuestDifficultyColors.verydifficult = QuestDifficultyColors.verydifficult or { r = 1.00, g = 0.50, b = 0.25 }
QuestDifficultyColors.impossible = QuestDifficultyColors.impossible or { r = 1.00, g = 0.10, b = 0.10 }

QuestDifficultyHighlightColors = QuestDifficultyHighlightColors or {}
QuestDifficultyHighlightColors.trivial = QuestDifficultyHighlightColors.trivial or { r = 0.70, g = 0.70, b = 0.70 }
QuestDifficultyHighlightColors.standard = QuestDifficultyHighlightColors.standard or { r = 0.50, g = 1.00, b = 0.50 }
QuestDifficultyHighlightColors.difficult = QuestDifficultyHighlightColors.difficult or { r = 1.00, g = 1.00, b = 0.50 }
QuestDifficultyHighlightColors.verydifficult = QuestDifficultyHighlightColors.verydifficult or { r = 1.00, g = 0.75, b = 0.50 }
QuestDifficultyHighlightColors.impossible = QuestDifficultyHighlightColors.impossible or { r = 1.00, g = 0.40, b = 0.40 }

C_ColorUtil = __wow_color_merge_namespace(C_ColorUtil, {
  ConvertRGBToHSV = function(r, g, b)
    return 0, 0, math.max(r or 0, g or 0, b or 0)
  end,
  ConvertHSVToHSL = function(h, s, v)
    return h or 0, s or 0, v or 0
  end,
  GenerateTextColorCode = function(color)
    local r = math.floor((color.r or 1) * 255)
    local g = math.floor((color.g or 1) * 255)
    local b = math.floor((color.b or 1) * 255)
    return string.format("ff%02x%02x%02x", r, g, b)
  end,
  WrapTextInColor = function(text, color)
    return "|c" .. C_ColorUtil.GenerateTextColorCode(color) .. tostring(text or "") .. "|r"
  end,
  WrapTextInColorCode = function(text, colorCode)
    local code = tostring(colorCode or "ffffffff"):gsub("^|c", "")
    return "|c" .. code .. tostring(text or "") .. "|r"
  end,
})
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(COLOR_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_color_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(C_UIColor.GetColors) ~= "function" then return "ui_colors" end
                local foundTradeskillColor = false
                for _, entry in ipairs(C_UIColor.GetColors()) do
                  if entry.baseTag == "TRADESKILL_EXPERIENCE_COLOR" then
                    foundTradeskillColor = entry.color.b == 0.75
                  end
                end
                if not foundTradeskillColor then return "tradeskill" end
                if C_ColorUtil.GenerateTextColorCode({ r = 1, g = 0.5, b = 0 }) ~= "ffff7f00" then return "text_code" end
                if C_ColorUtil.WrapTextInColorCode("Ready", "ff112233") ~= "|cff112233Ready|r" then return "wrap" end
                if QuestDifficultyColors.impossible.g ~= 0.10 then return "quest" end
                if QuestDifficultyHighlightColors.standard.g ~= 1.00 then return "highlight" end
                return "ok"
                "#,
            )
            .expect("color defaults probe should run");

        assert_eq!(result, "ok");
    }
}
