use crate::lua_api::WowLuaEnv;

#[test]
fn test_patch_12_0_0_force_allow_aero_cvar_removed() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                if GetCVar("ForceAllowAero") ~= nil then
                    return "value=" .. tostring(GetCVar("ForceAllowAero"))
                end
                if GetCVarDefault("ForceAllowAero") ~= nil then
                    return "default=" .. tostring(GetCVarDefault("ForceAllowAero"))
                end
                return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "retail 12.0.0 removed ForceAllowAero CVar should have no value or default"
    );
}

#[test]
fn test_patch_12_0_0_removed_nameplate_cvars() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local names = {
                    "NamePlateClassificationScale",
                    "NamePlateHorizontalScale",
                    "NamePlateMaximumClassificationScale",
                    "NamePlateVerticalScale",
                    "NameplatePersonalClickThrough",
                    "NameplatePersonalHideDelayAlpha",
                    "NameplatePersonalHideDelaySeconds",
                    "NameplatePersonalShowAlways",
                    "NameplatePersonalShowInCombat",
                    "NameplatePersonalShowWithTarget",
                    "ShowClassColorInFriendlyNameplate",
                    "ShowClassColorInNameplate",
                    "ShowNamePlateLoseAggroFlash",
                    "TerrainBlendBakeEnable",
                    "TerrainUnlitShaderEnable",
                }
                for _, name in ipairs(names) do
                    local value = GetCVar(name)
                    if value ~= nil then
                        return name .. ":GetCVar=" .. tostring(value)
                    end
                    local default = GetCVarDefault(name)
                    if default ~= nil then
                        return name .. ":GetCVarDefault=" .. tostring(default)
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "retail 12.0.0 removed nameplate CVars should have no value or default: {result}"
    );
}
