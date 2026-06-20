local DB = SecureEnvProbeDB

DB.loadIntoAfter = {
    insecureSees = rawget(_G, "SecureEnvProbe_LoadIntoLate"),
    mathMarker = math.SecureEnvProbeMarker,
}
