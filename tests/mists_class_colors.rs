#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

const CLUB_FINDER_LUA: &str =
    include_str!("../Interface/BlizzardUI/Mists/AddOns/Blizzard_Communities/ClubFinder.lua");

fn load_club_finder_lua(env: &WowLuaEnv) {
    let loader = format!(
        r#"
        CLUB_FINDER_TANK = "Tank"
        CLUB_FINDER_HEALER = "Healer"
        CLUB_FINDER_DAMAGE = "Damage"
        CLUB_FINDER_ANY_FLAG = "Any"
        CLUB_FINDER_MULTIPLE_ROLES = "Multiple"
        CHECK_ALL = "Check All"
        UNCHECK_ALL = "Uncheck All"
        TALENT_SPEC_AND_CLASS = "%s %s"
        MenuResponse = {{ Refresh = "refresh" }}

        local source = [==[
{}
        ]==]
        local chunk = assert(loadstring(source))
        chunk("Blizzard_Communities", {{}})
        "#,
        CLUB_FINDER_LUA
    );

    env.exec(&loader)
        .expect("Mists ClubFinder Lua should define dropdown mixins");
}

#[test]
fn club_finder_setup_menu_reproduces_missing_class_color() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    load_club_finder_lua(&env);

    let (ok, err): (bool, String) = env
        .eval(
            r#"
            DropdownButtonMixin = {
                SetupMenu = function(_self, initializer)
                    local submenu = {
                        CreateButton = function() end,
                        CreateCheckbox = function() end,
                    }
                    local rootDescription = {
                        SetTag = function() end,
                        CreateCheckbox = function()
                            return submenu
                        end,
                    }
                    initializer({}, rootDescription)
                end,
            }
            C_SpecializationInfo = {
                GetNumSpecializationsForClassID = function()
                    return 1
                end,
            }
            GetNumClasses = function()
                return 1
            end
            GetClassInfo = function()
                return "Broken Class", "BROKENCLASS", 99
            end
            GetClassColorObj = function()
                return nil
            end
            GetSpecializationInfoForClassID = function()
                return 9901, "Broken Spec", nil, nil, "TANK"
            end
            UnitSex = function()
                return 2
            end

            local dropdown = {
                SetSelectionText = function() end,
            }
            setmetatable(dropdown, { __index = ClubLookingForDropdownMixin })

            local ok, err = pcall(dropdown.SetupMenu, dropdown)
            return ok, tostring(err)
            "#,
        )
        .expect("ClubLookingForDropdownMixin:SetupMenu pcall should return a status");

    assert!(!ok, "ClubFinder setup should reproduce the nil class color");
    assert!(
        err.contains("classColor") || err.contains("WrapTextInColorCode"),
        "expected nil classColor failure, got: {err}"
    );
}
