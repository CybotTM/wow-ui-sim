//! Temporary formatting and utility globals for partial Blizzard loads.
//!
//! These helpers mirror small Blizzard utility surfaces that are expected by
//! startup Lua, but they are still shallow compatibility defaults rather than a
//! modeled formatting subsystem.

const FORMATTING_UTILITY_DEFAULTS_LUA: &str = r#"
if GetScreenDPIScale == nil then
  function GetScreenDPIScale()
    return 1
  end
end

if difftime == nil and os ~= nil and os.difftime ~= nil then
  difftime = os.difftime
end

if FindInTableIf == nil then
  function FindInTableIf(tbl, predicate)
    if type(tbl) ~= "table" or type(predicate) ~= "function" then
      return nil
    end
    for index, value in ipairs(tbl) do
      if predicate(value, index) then
        return index, value
      end
    end
    return nil
  end
end

local function __wow_install_string_color_helper(name, color)
  local method = "SetColor" .. name
  if string[method] == nil then
    string[method] = function(self)
      return color:WrapTextInColorCode(self)
    end
  end
end

__wow_install_string_color_helper("Orange", CreateColor(1.00, 0.50, 0.25))
__wow_install_string_color_helper("Yellow", CreateColor(1.00, 0.82, 0.00))
__wow_install_string_color_helper("AddonBlue", CreateColor(0.11, 0.57, 0.76))

if string.K_ReplaceVars == nil then
  string.K_ReplaceVars = function(self, vars)
    local text = tostring(self or "")
    if type(vars) ~= "table" then
      return text
    end
    return (text:gsub("({([^}]+)})", function(whole, key)
      local replacement = vars[key]
      if replacement == nil then
        return whole
      end
      return tostring(replacement)
    end))
  end
end

if string.K_AddDefaultValueText == nil then
  string.K_AddDefaultValueText = function(self)
    return tostring(self or "")
  end
end

if GetMoneyString == nil then
  local function __wow_separate_thousands(n)
    local digits = tostring(n)
    if #digits <= 3 then
      return digits
    end
    local out = digits:sub(-3)
    local i = #digits - 3
    while i > 0 do
      local chunk_start = math.max(1, i - 2)
      out = digits:sub(chunk_start, i) .. "," .. out
      i = chunk_start - 1
    end
    return out
  end

  function GetMoneyString(money, separateThousands)
    money = math.floor(tonumber(money) or 0)
    if money < 0 then money = 0 end
    local gold = math.floor(money / 10000)
    local silver = math.floor((money - gold * 10000) / 100)
    local copper = money % 100
    local gold_text = separateThousands and __wow_separate_thousands(gold) or tostring(gold)
    local parts = {}
    if gold > 0 then parts[#parts + 1] = gold_text .. "g" end
    if silver > 0 then parts[#parts + 1] = silver .. "s" end
    if copper > 0 or #parts == 0 then parts[#parts + 1] = copper .. "c" end
    return table.concat(parts, " ")
  end
end

if GetColorForCurrencyReward == nil then
  function GetColorForCurrencyReward(_currencyID, _rewardQuantity, defaultColor)
    if defaultColor ~= nil then
      return defaultColor
    end
    if HIGHLIGHT_FONT_COLOR ~= nil then
      return HIGHLIGHT_FONT_COLOR
    end
    return CreateColor(1, 1, 1, 1)
  end
end

local __wow_console_font_height = 14

if ConsoleGetColorFromType == nil then
  function ConsoleGetColorFromType(_colorType)
    return CreateColor(1, 1, 1)
  end
end

if ConsoleGetFontHeight == nil then
  function ConsoleGetFontHeight()
    return __wow_console_font_height
  end
end

if ConsoleSetFontHeight == nil then
  function ConsoleSetFontHeight(fontHeightInPixels)
    __wow_console_font_height = tonumber(fontHeightInPixels) or __wow_console_font_height
  end
end

if AbbreviateLargeNumbers == nil then
  function AbbreviateLargeNumbers(value)
    return tostring(math.floor(tonumber(value) or 0))
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(FORMATTING_UTILITY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_formatting_utility_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if GetScreenDPIScale() ~= 1 then return "dpi" end
                if difftime(10, 3) ~= 7 then return "difftime" end
                local index, value = FindInTableIf({ "a", "b", "c" }, function(v) return v == "b" end)
                if index ~= 2 or value ~= "b" then return "find" end
                if ("label"):SetColorOrange():sub(1, 2) ~= "|c" then return "orange" end
                if ("Hello {name}"):K_ReplaceVars({ name = "Sim" }) ~= "Hello Sim" then return "replace" end
                if ("Plain"):K_AddDefaultValueText() ~= "Plain" then return "default_text" end
                if GetMoneyString(1234567, true) ~= "123g 45s 67c" then return "money" end
                if type(GetColorForCurrencyReward(1, 1).GetRGB) ~= "function" then return "currency_color" end
                ConsoleSetFontHeight(18)
                if ConsoleGetFontHeight() ~= 18 then return "console_font" end
                if type(ConsoleGetColorFromType(0).GetRGB) ~= "function" then return "console_color" end
                if AbbreviateLargeNumbers(123.9) ~= "123" then return "abbrev" end
                return "ok"
                "#,
            )
            .expect("formatting utility defaults probe should run");

        assert_eq!(result, "ok");
    }
}
