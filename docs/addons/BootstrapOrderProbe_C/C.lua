BootstrapOrderProbeDB = BootstrapOrderProbeDB or {}
BootstrapOrderProbeDB.events = BootstrapOrderProbeDB.events or {}

local function record(label)
    table.insert(BootstrapOrderProbeDB.events, label)
    print("[BootstrapOrderProbe] " .. label)
end

record("C eager file")

SLASH_BOOTSTRAPORDERPROBE1 = "/boprobe"
SlashCmdList.BOOTSTRAPORDERPROBE = function(msg)
    msg = msg or ""
    if msg == "load" then
        local ok, reason = C_AddOns.LoadAddOn("BootstrapOrderProbe_B")
        record("LoadAddOn B returned " .. tostring(ok) .. " " .. tostring(reason))
    elseif msg == "reset" then
        BootstrapOrderProbeDB.events = {}
        print("[BootstrapOrderProbe] reset")
        return
    end

    print("[BootstrapOrderProbe] IsAddOnLoaded(B)=", C_AddOns.IsAddOnLoaded("BootstrapOrderProbe_B"))
    print("[BootstrapOrderProbe] B bootstrap seen=", BootstrapOrderProbe_B_BootstrapSeen, "normal seen=", BootstrapOrderProbe_B_NormalSeen)
    print("[BootstrapOrderProbe] order=", table.concat(BootstrapOrderProbeDB.events, " -> "))
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", function()
    record("PLAYER_LOGIN")
    print("[BootstrapOrderProbe] IsAddOnLoaded(B)=", C_AddOns.IsAddOnLoaded("BootstrapOrderProbe_B"))
    print("[BootstrapOrderProbe] B bootstrap seen=", BootstrapOrderProbe_B_BootstrapSeen, "normal seen=", BootstrapOrderProbe_B_NormalSeen)
    print("[BootstrapOrderProbe] order=", table.concat(BootstrapOrderProbeDB.events, " -> "))
    print("[BootstrapOrderProbe] commands: /boprobe, /boprobe load, /boprobe reset")
end)
