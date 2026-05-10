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

#[test]
fn nameplate_options_default_vertical_scale_updates_sizes() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    env.exec(NAMEPLATES_TBC_LUA)
        .expect("TBC/Mists NamePlates Lua should define option helpers");

    let result: (
        String,
        bool,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            DefaultCompactNamePlateEnemyFrameOptions = {}
            DefaultCompactNamePlateFriendlyFrameOptions = {}
            DefaultCompactNamePlateFrameSetUpOptions = {}
            Lerp = function(startValue, endValue, amount)
                return startValue + (endValue - startValue) * amount
            end
            local sizes = {}
            C_NamePlate = {
                SetNamePlateFriendlySize = function(width, height)
                    sizes.friendlyWidth = width
                    sizes.friendlyHeight = height
                end,
                SetNamePlateEnemySize = function(width, height)
                    sizes.enemyWidth = width
                    sizes.enemyHeight = height
                end,
                SetNamePlateSelfSize = function(width, height)
                    sizes.selfWidth = width
                    sizes.selfHeight = height
                end,
                GetNamePlates = function() return {} end,
            }

            local driver = { baseNamePlateWidth = 128, baseNamePlateHeight = 32 }
            local ok, err = pcall(NamePlateDriverMixin.UpdateNamePlateOptions, driver)
            return GetCVar("NamePlateVerticalScale"),
                ok,
                tostring(err),
                tostring(DefaultCompactNamePlateFrameSetUpOptions.healthBarHeight),
                tostring(sizes.friendlyWidth),
                tostring(sizes.friendlyHeight),
                tostring(sizes.enemyWidth),
                tostring(sizes.enemyHeight),
                tostring(sizes.selfWidth),
                tostring(sizes.selfHeight)
            "#,
        )
        .expect("NamePlateVerticalScale default should drive NamePlate options");

    assert_eq!(
        (result.0.as_str(), result.1, result.2.as_str()),
        ("1", true, "nil"),
        "Mists NamePlateVerticalScale should be seeded before option sync"
    );
    assert_eq!(
        (
            result.3.as_str(),
            result.4.as_str(),
            result.5.as_str(),
            result.6.as_str(),
            result.7.as_str(),
            result.8.as_str(),
            result.9.as_str(),
        ),
        ("10", "128", "32", "128", "32", "140.8", "32")
    );
}
