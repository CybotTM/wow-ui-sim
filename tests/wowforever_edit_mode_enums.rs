#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn load_source(env: &WowLuaEnv, path: &str) {
    let root = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    env.exec(&std::fs::read_to_string(root.join(path)).unwrap())
        .unwrap();
}

#[test]
fn forever_edit_mode_source_enum_values() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        documentedEnums = {}
        APIDocumentation = { AddDocumentationTable = function(_, doc)
            for _, entry in ipairs(doc.Tables or {}) do
                if entry.Type == "Enumeration" then documentedEnums[entry.Name] = entry end
            end
        end }
    "#,
    )
    .unwrap();
    load_source(
        &env,
        "Blizzard_APIDocumentationGenerated/EditModeManagerConstantsDocumentation.lua",
    );
    env.exec(
        r#"
        local missing = {}
        for name, doc in pairs(documentedEnums) do
            for _, field in ipairs(doc.Fields) do
                if not Enum[name] or Enum[name][field.Name] ~= field.EnumValue then
                    table.insert(missing, name .. "." .. field.Name .. "=" .. field.EnumValue)
                end
            end
        end
        for _, name in ipairs({"EditModeAccountSetting", "EditModeGroupFinderSetting",
            "EditModeLossOfControlSetting", "EditModeMainActionBarEndCapSetting",
            "EditModeMainActionBarEndCapSystemIndices", "EditModeMicroMenuSetting",
            "EditModeMinimapSetting", "EditModePresetLayouts", "EditModeRaidWarningSetting",
            "EditModeSwingTimerSetting", "EditModeSwingTimerSystemIndices",
            "EditModeSwingTimerVisibility", "EditModeSystem", "EditModeUnitFrameSetting",
            "RaidDispelOverlayType"}) do
            for _, key in ipairs({"MinValue", "MaxValue", "NumValues"}) do
                local meta = Enum[name .. "Meta"]
                if not meta or meta[key] ~= documentedEnums[name][key] then
                    table.insert(missing, name .. "Meta." .. key)
                end
            end
        end
        table.sort(missing)
        assert(#missing == 0, table.concat(missing, ", "))
    "#,
    )
    .unwrap();
}

#[test]
fn forever_edit_mode_preset_consumer() {
    let env = WowLuaEnv::new().unwrap();
    load_source(
        &env,
        "Blizzard_EditMode/Camelot/EditModePresetLayoutConstants.lua",
    );
    load_source(&env, "Blizzard_EditMode/Mainline/EditModePresetLayouts.lua");
    env.exec(
        r#"
        local timers = EDIT_MODE_MODERN_SYSTEM_MAP[Enum.EditModeSystem.SwingTimer]
        local mainHand = timers[Enum.EditModeSwingTimerSystemIndices.MainHand].settings
        assert(mainHand[Enum.EditModeSwingTimerSetting.Width] == 213)
        assert(mainHand[Enum.EditModeSwingTimerSetting.Visibility] == Enum.EditModeSwingTimerVisibility.Always)
        local caps = EDIT_MODE_MODERN_SYSTEM_MAP[Enum.EditModeSystem.MainActionBarEndCap]
        assert(caps[Enum.EditModeMainActionBarEndCapSystemIndices.EndCapLeft].settings[Enum.EditModeMainActionBarEndCapSetting.Hidden] == 0)
    "#,
    )
    .unwrap();
}

#[test]
fn forever_edit_mode_default_layout_consumer() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("assert(C_EditMode.GetEditModeDefaultLayout() == 0)")
        .unwrap();
    load_source(&env, "Blizzard_SharedXMLBase/TableUtil.lua");
    load_source(
        &env,
        "Blizzard_EditMode/Camelot/EditModePresetLayoutConstants.lua",
    );
    load_source(&env, "Blizzard_EditMode/Mainline/EditModePresetLayouts.lua");
    load_source(
        &env,
        "Blizzard_EditMode/Shared/EditModePresetLayoutSystemUtils.lua",
    );
    load_source(
        &env,
        "Blizzard_EditMode/Shared/EditModePresetLayoutsManager.lua",
    );
    env.exec(r#"
        local system = Enum.EditModeSystem.SwingTimer
        local index = Enum.EditModeSwingTimerSystemIndices.MainHand
        local expected = EDIT_MODE_MODERN_SYSTEM_MAP[system][index].anchorInfo
        local actual = EditModePresetLayoutManager:GetDefaultSystemAnchorInfo(system, index)
        assert(actual ~= expected)
        assert(actual.point == expected.point and actual.relativePoint == expected.relativePoint)
        assert(actual.offsetX == expected.offsetX and actual.offsetY == expected.offsetY)
        actual.offsetX = 9999
        assert(EditModePresetLayoutManager:GetDefaultSystemAnchorInfo(system, index).offsetX == expected.offsetX)
    "#).unwrap();
    env.state().borrow_mut().set_input_interface_style(
        wow_ui_sim::c_api::c_input_interface_style::InputInterfaceStyle::Gamepad,
    );
    env.exec("assert(C_EditMode.GetEditModeDefaultLayout() == 0)")
        .unwrap();
}

#[test]
fn forever_edit_mode_display_consumer() {
    let env = WowLuaEnv::new().unwrap();
    load_source(&env, "Blizzard_SharedXMLBase/TableUtil.lua");
    load_source(
        &env,
        "Blizzard_EditMode/Shared/EditModeSettingDisplayInfo.lua",
    );
    env.exec(r#"
        local settings = EditModeSettingDisplayInfoManager.systemSettingDisplayInfo[Enum.EditModeSystem.SwingTimer]
        assert(settings[1].setting == Enum.EditModeSwingTimerSetting.Scale)
        assert(settings[1].minValue == 50 and settings[1].maxValue == 200)
        assert(settings[2].setting == Enum.EditModeSwingTimerSetting.Opacity)
        assert(settings[2].minValue == 50 and settings[2].maxValue == 100)
    "#).unwrap();
}

#[test]
fn forever_edit_mode_gamepad_consumers() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(Enum.EditModeSystem.GamepadMainActionBar == 30, "missing gamepad system")
        local setting = Enum.EditModeGamepadMainActionBarSetting
        assert(type(setting) == "table", "missing gamepad settings")
        assert(setting.Compact == 0 and setting.ShowEmptyBars == 1)
        assert(setting.ShowButtonPrompts == 2 and setting.ShowHighlight == 3)
        assert(setting.ShouldExpand == 4)
        local meta = Enum.EditModeGamepadMainActionBarSettingMeta
        assert(meta.MinValue == 0 and meta.MaxValue == 4 and meta.NumValues == 5)
        local systemMeta = Enum.EditModeSystemMeta
        assert(systemMeta.MinValue == 0 and systemMeta.MaxValue == 30)
        assert(systemMeta.NumValues == 31)
    "#,
    )
    .unwrap();
    load_source(&env, "Blizzard_SharedXMLBase/TableUtil.lua");
    load_source(
        &env,
        "Blizzard_EditMode/Camelot/EditModePresetLayoutConstants.lua",
    );
    load_source(&env, "Blizzard_EditMode/Mainline/EditModePresetLayouts.lua");
    load_source(
        &env,
        "Blizzard_EditMode/Shared/EditModeSettingDisplayInfo.lua",
    );
    env.exec(
        r#"
        local gamepad = EDIT_MODE_GAMEPAD_SYSTEM_MAP[30]
        local settings = gamepad.settings
        assert(settings[0] == 0 and settings[1] == 1 and settings[2] == 0)
        assert(settings[3] == 1 and settings[4] == 1)
        local count = 0
        for _ in pairs(settings) do count = count + 1 end
        assert(count == 5)
        assert(gamepad.anchorInfo.point == "BOTTOM")
        assert(gamepad.anchorInfo.relativeTo == "UIParent")
        assert(gamepad.anchorInfo.offsetY == 110)
        local display = EditModeSettingDisplayInfoManager.systemSettingDisplayInfo[30]
        assert(#display == 5)
        for index, entry in ipairs(display) do
            assert(entry.setting == index - 1)
            assert(entry.type == Enum.EditModeSettingDisplayType.Checkbox)
            assert(settings[entry.setting] ~= nil)
        end
    "#,
    )
    .unwrap();
}
