BootstrapOrderProbeDB = BootstrapOrderProbeDB or {}
BootstrapOrderProbeDB.events = BootstrapOrderProbeDB.events or {}

local function record(label)
    table.insert(BootstrapOrderProbeDB.events, label)
    print("[BootstrapOrderProbe] " .. label)
end

record("A eager file")
