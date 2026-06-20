local DB = SecureEnvProbeDB
local env = getfenv(1)
local primitiveKey = DB and DB.primitiveKey

DB.useSecure = {
    envEqualsGlobalName = env == _G,
    envEqualsWriterEnv = env == SecureEnvProbe_GlobalEnv,
    envMarker = env.SecureEnvProbe_GlobalEnvMarker,
    writerEnvMarker = SecureEnvProbe_GlobalEnv and SecureEnvProbe_GlobalEnv.SecureEnvProbe_GlobalEnvMarker or nil,
    lateOwnSlotBefore = rawget(env, "SecureEnvProbe_UseSecureLate") ~= nil,
    beforeRead = SecureEnvProbe_UseSecureLate,
    primitiveOwnSlot = primitiveKey and rawget(env, primitiveKey) or nil,
    primitiveSeen = primitiveKey and _G[primitiveKey] or nil,
    mathBefore = math.SecureEnvProbeMarker,
}

SecureEnvProbe_UseSecureLate = 17
math.SecureEnvProbeMarker = "from-use-secure"

DB.useSecure.afterSecureWrite = SecureEnvProbe_UseSecureLate
DB.useSecure.ownSlotAfter = rawget(env, "SecureEnvProbe_UseSecureLate")
DB.useSecure.mathAfter = math.SecureEnvProbeMarker
