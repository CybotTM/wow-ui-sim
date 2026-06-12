local PREFIX = "PRP"
ProtectedRetailProbeDB = ProtectedRetailProbeDB or {}
ProtectedRetailProbeDB.runs = ProtectedRetailProbeDB.runs or {}

local currentRun

local function safe(label, fn)
  local ok, a, b, c = pcall(fn)
  if currentRun then
    currentRun[label] = {
      ok = ok,
      a = tostring(a),
      b = tostring(b),
      c = tostring(c),
    }
  end
  if ok then
    print(PREFIX, label, tostring(a), tostring(b), tostring(c))
  else
    print(PREFIX, label, "ERROR", tostring(a))
  end
end

local function callMethod(frame, method)
  if type(frame) ~= "table" or type(frame[method]) ~= "function" then
    return "missing"
  end
  return frame[method](frame)
end

local function frameState(label, frame)
  safe(label .. ".type", function() return type(frame) end)
  safe(label .. ".IsProtected", function() return callMethod(frame, "IsProtected") end)
  safe(label .. ".IsForbidden", function() return callMethod(frame, "IsForbidden") end)
  safe(label .. ".ProtectType", function() return type(frame and frame.Protect) end)
  safe(label .. ".SetProtectedType", function() return type(frame and frame.SetProtected) end)
end

local function protectionSetters(label, frame)
  safe(label .. ".ProtectCall", function()
    return frame:Protect()
  end)
  safe(label .. ".SetProtectedTrueCall", function()
    return frame:SetProtected(true)
  end)
  safe(label .. ".SetProtectedFalseCall", function()
    return frame:SetProtected(false)
  end)
  safe(label .. ".AfterSetterAttempts.IsProtected", function()
    return frame:IsProtected()
  end)
end

local function runProbe()
  currentRun = {
    build = { GetBuildInfo() },
    time = time(),
  }
  table.insert(ProtectedRetailProbeDB.runs, currentRun)

  print(PREFIX, "BEGIN")

  local plain = CreateFrame("Frame", nil, UIParent)
  frameState("plain", plain)
  protectionSetters("plain", plain)

  frameState("xml_protected_attr", _G.ProtectedRetailProbeXMLProtected)
  protectionSetters("xml_protected_attr", _G.ProtectedRetailProbeXMLProtected)

  safe("secure_template.create", function()
    return CreateFrame("Button", nil, UIParent, "SecureActionButtonTemplate")
  end)

  local secureButton = CreateFrame("Button", nil, UIParent, "SecureActionButtonTemplate")
  frameState("secure_template", secureButton)
  protectionSetters("secure_template", secureButton)

  local child = CreateFrame("Frame", nil, secureButton)
  frameState("secure_template_child", child)

  safe("LoadAddOn.Blizzard_StoreUI", function()
    if C_AddOns and C_AddOns.LoadAddOn then
      return C_AddOns.LoadAddOn("Blizzard_StoreUI")
    end
    return LoadAddOn("Blizzard_StoreUI")
  end)

  frameState("StoreFrame", _G.StoreFrame)
  frameState("StoreVASValidationFrame", _G.StoreVASValidationFrame)

  print(PREFIX, "END")
  currentRun = nil
end

SLASH_PROTECTEDRETAILPROBE1 = "/prp"
SlashCmdList.PROTECTEDRETAILPROBE = runProbe

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", function()
  C_Timer.After(1, runProbe)
end)
