use wow_ui_sim::lua_api::WowLuaEnv;

const NAMEPLATES_TBC_LUA: &str = include_str!(
    "../Interface/BlizzardUI/Mists/AddOns/Blizzard_NamePlates/TBC/Blizzard_NamePlates.lua"
);

#[test]
fn nameplate_options_reproduce_nil_vertical_scale_arithmetic() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    env.exec(NAMEPLATES_TBC_LUA)
        .expect("TBC/Mists NamePlates Lua should define option helpers");

    let (ok, err): (bool, String) = env
        .eval(
            r#"
            DefaultCompactNamePlateEnemyFrameOptions = {}
            DefaultCompactNamePlateFriendlyFrameOptions = {}
            DefaultCompactNamePlateFrameSetUpOptions = {}
            C_NamePlate = {
                SetNamePlateFriendlySize = function() end,
                SetNamePlateEnemySize = function() end,
                SetNamePlateSelfSize = function() end,
                GetNamePlates = function() return {} end,
            }

            local previousGetCVar = GetCVar
            GetCVar = function(name)
                if name == "NamePlateVerticalScale" then
                    return nil
                end
                if name == "NamePlateHorizontalScale" then
                    return "1"
                end
                return previousGetCVar(name)
            end

            local driver = { baseNamePlateWidth = 128, baseNamePlateHeight = 32 }
            local ok, err = pcall(NamePlateDriverMixin.UpdateNamePlateOptions, driver)
            return ok, tostring(err)
            "#,
        )
        .expect("NamePlateDriverMixin.UpdateNamePlateOptions pcall should return a status");

    assert!(
        !ok,
        "nil NamePlateVerticalScale should fail during arithmetic"
    );
    assert!(
        err.contains("namePlateVerticalScale") || err.contains("arithmetic") || err.contains("nil"),
        "expected nil vertical scale arithmetic failure, got: {err}"
    );
}
