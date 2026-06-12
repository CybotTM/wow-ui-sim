local addonName = ...

local function valueSummary(value)
  local luaType = type(value)
  local scalarValue
  if luaType == "nil" or luaType == "boolean" or luaType == "number" or luaType == "string" then
    scalarValue = value
  end

  return {
    luaType = luaType,
    isNil = value == nil,
    value = scalarValue,
    tostring = tostring(value),
  }
end

local function packCall(fn)
  local results = { pcall(fn) }
  local ok = table.remove(results, 1)
  local packed = {}
  for index, value in ipairs(results) do
    packed[index] = valueSummary(value)
  end
  return {
    ok = ok,
    results = packed,
    error = ok and nil or tostring(results[1]),
  }
end

local function captureTexture(label, tex)
  local ok, value = pcall(function()
    return tex:GetTexture()
  end)
  return {
    label = label,
    getTextureOk = ok,
    getTexture = valueSummary(value),
  }
end

local function runProbe()
  local frame = CreateFrame("Frame")
  local tex = frame:CreateTexture()
  local cases = {}

  cases.default = captureTexture("default", tex)

  cases.setPath = packCall(function()
    return tex:SetTexture("Interface\\Buttons\\UI-Panel-Button-Up")
  end)
  cases.afterSetPath = captureTexture("afterSetPath", tex)

  cases.setNil = packCall(function()
    return tex:SetTexture(nil)
  end)
  cases.afterSetNil = captureTexture("afterSetNil", tex)

  cases.setNoArgs = packCall(function()
    return tex:SetTexture()
  end)
  cases.afterSetNoArgs = captureTexture("afterSetNoArgs", tex)

  TextureSetTextureProbeDB = {
    addonName = addonName,
    build = { GetBuildInfo() },
    cases = cases,
  }

  print("TextureSetTextureProbe captured; /reload or logout to flush SavedVariables")
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", runProbe)
