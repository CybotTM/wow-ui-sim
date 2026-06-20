local DB = SecureEnvProbeDB
local env = getfenv(1)
local primitiveKey = DB and DB.primitiveKey

DB.loadInto = {
    envEqualsGlobalName = env == _G,
    envEqualsWriterEnv = env == SecureEnvProbe_GlobalEnv,
    envMarker = env.SecureEnvProbe_GlobalEnvMarker,
    writerEnvMarker = SecureEnvProbe_GlobalEnv and SecureEnvProbe_GlobalEnv.SecureEnvProbe_GlobalEnvMarker or nil,
    lateOwnSlotBefore = rawget(env, "SecureEnvProbe_LoadIntoLate") ~= nil,
    beforeRead = SecureEnvProbe_LoadIntoLate,
    primitiveOwnSlot = primitiveKey and rawget(env, primitiveKey) or nil,
    primitiveSeen = primitiveKey and _G[primitiveKey] or nil,
    mathBefore = math.SecureEnvProbeMarker,
}

SecureEnvProbe_LoadIntoLate = 7
math.SecureEnvProbeMarker = "from-load-into-secure"

DB.loadInto.afterSecureWrite = SecureEnvProbe_LoadIntoLate
DB.loadInto.ownSlotAfter = rawget(env, "SecureEnvProbe_LoadIntoLate")
DB.loadInto.mathAfter = math.SecureEnvProbeMarker
