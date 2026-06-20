local addonName = ...

SecureEnvProbeDB = SecureEnvProbeDB or {}

local DB = SecureEnvProbeDB
DB.addonName = addonName
DB.build = { GetBuildInfo() }
DB.writer = DB.writer or {}

SecureEnvProbe_GlobalEnv = getfenv(1)
SecureEnvProbe_GlobalEnvMarker = "__normal_writer_env__"

local PRIMITIVE_SENTINEL = "__SecureEnvProbe_writer_rebind__"
local TABLE_SENTINEL = "__SecureEnvProbe_table_marker__"

local primitiveCandidates = {
    "UNKNOWN",
    "OKAY",
    "CANCEL",
    "YES",
    "NO",
    "NONE",
}

local function choosePrimitiveKey()
    for _, key in ipairs(primitiveCandidates) do
        local value = rawget(_G, key)
        local valueType = type(value)
        if valueType == "string" or valueType == "number" or valueType == "boolean" then
            return key, value, valueType
        end
    end
end

local function secureVariable(name)
    if type(issecurevariable) ~= "function" then
        return "missing"
    end

    local ok, secure, owner = pcall(issecurevariable, name)
    if not ok then
        return "error", tostring(secure)
    end

    return secure and true or false, owner
end

local primitiveKey, originalPrimitiveValue, originalPrimitiveType = choosePrimitiveKey()

DB.primitiveKey = primitiveKey
DB.writer.originalPrimitiveValue = originalPrimitiveValue
DB.writer.originalPrimitiveType = originalPrimitiveType
DB.writer.env = {
    chunkEnvEqualsStoredGlobalEnv = getfenv(1) == SecureEnvProbe_GlobalEnv,
    storedGlobalEnvMarker = SecureEnvProbe_GlobalEnv.SecureEnvProbe_GlobalEnvMarker,
}

if primitiveKey then
    rawset(_G, primitiveKey, PRIMITIVE_SENTINEL)
    DB.writer.reboundPrimitiveValue = rawget(_G, primitiveKey)
end

SecureEnvProbe_LoadIntoLate = 5
SecureEnvProbe_LoadIntoLate = 6
SecureEnvProbe_UseSecureLate = 15
SecureEnvProbe_UseSecureLate = 16

math.SecureEnvProbeMarker = TABLE_SENTINEL

DB.writer.afterWrites = {
    loadIntoLate = SecureEnvProbe_LoadIntoLate,
    useSecureLate = SecureEnvProbe_UseSecureLate,
    mathMarker = math.SecureEnvProbeMarker,
}

local secure, owner = secureVariable("SecureEnvProbe_LoadIntoLate")
DB.writer.taintAfterAddonWrite = {
    secure = secure,
    owner = owner,
}

local function summarize()
    local loadInto = DB.loadInto or {}
    local loadIntoAfter = DB.loadIntoAfter or {}
    local useSecure = DB.useSecure or {}
    local final = DB.writer.afterAllAddons or {}

    print("SecureEnvProbe: results saved to SecureEnvProbeDB")
    print(string.format(
        "  primitive key %s: writer=%s loadInto=%s useSecure=%s",
        tostring(DB.primitiveKey),
        tostring(DB.writer.reboundPrimitiveValue),
        tostring(loadInto.primitiveSeen),
        tostring(useSecure.primitiveSeen)
    ))
    print(string.format(
        "  late globals: loadInto before=%s afterSecure=%s insecureAfter=%s final=%s",
        tostring(loadInto.beforeRead),
        tostring(loadInto.afterSecureWrite),
        tostring(loadIntoAfter.insecureSees),
        tostring(final.loadIntoLate)
    ))
    print(string.format(
        "  useSecure late: before=%s afterSecure=%s final=%s",
        tostring(useSecure.beforeRead),
        tostring(useSecure.afterSecureWrite),
        tostring(final.useSecureLate)
    ))
    print(string.format(
        "  math marker: loadIntoBefore=%s useSecureBefore=%s final=%s",
        tostring(loadInto.mathBefore),
        tostring(useSecure.mathBefore),
        tostring(final.mathMarker)
    ))
end

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", function()
    DB.writer.afterAllAddons = {
        loadIntoLate = rawget(_G, "SecureEnvProbe_LoadIntoLate"),
        useSecureLate = rawget(_G, "SecureEnvProbe_UseSecureLate"),
        primitiveValue = primitiveKey and rawget(_G, primitiveKey) or nil,
        mathMarker = math.SecureEnvProbeMarker,
    }

    if primitiveKey then
        rawset(_G, primitiveKey, originalPrimitiveValue)
    end
    math.SecureEnvProbeMarker = nil

    summarize()
end)

SLASH_SECUREENVPROBE1 = "/seprobe"
SlashCmdList.SECUREENVPROBE = summarize
