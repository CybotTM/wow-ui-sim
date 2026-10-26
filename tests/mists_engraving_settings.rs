//! Complete selected Mists display-settings construction with supplied inputs.
#![cfg(feature = "client-mists")]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn load_original_settings(engraving: Option<bool>, transmog: bool) -> WowLuaEnv {
    assert_eq!(ACTIVE, ClientProfile::Mists);
    let ui = blizzard_ui_dir();
    let toc_path = find_toc_file(&ui.join("Blizzard_SettingsDefinitions_Frame")).unwrap();
    assert_eq!(
        toc_path.file_name().unwrap(),
        "Blizzard_SettingsDefinitions_Frame_Classic.toc"
    );
    let toc = TocFile::from_file(&toc_path).unwrap();
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    assert_eq!(toc.files[1], "Classic/InterfaceOverrides.lua");
    let env = new_blizzard_addon_env(&ui);
    {
        let mut state = env.state().borrow_mut();
        state.engraving.enabled = engraving;
        state.mists_transmog.enabled = Some(transmog);
    }
    load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &[
            "Blizzard_SharedXMLBase",
            "Blizzard_SharedXML",
            "Blizzard_SharedXMLGame",
            "Blizzard_LoadLocale",
            "Blizzard_SettingsDefinitions_Frame",
        ],
        &[],
    );
    let errors = env.state().borrow().lua_errors.clone();
    eprintln!("unfiltered load diagnostics: {errors:?}");
    assert!(errors.is_empty(), "{errors:#?}");
    install_settings_observer(&env);
    env
}

fn install_settings_observer(env: &WowLuaEnv) {
    env.exec(
        r#"
        local function original(func, suffix, line)
            local info = debug.getinfo(func, 'S')
            local source = info.source:gsub('\\', '/')
            assert(info.what == 'Lua' and info.linedefined == line, source)
            assert(source:sub(-#suffix) == suffix, source)
        end
        original(InterfaceOverrides.AdjustDisplaySettings,
            'Blizzard_SettingsDefinitions_Frame/Classic/InterfaceOverrides.lua', 3)
        original(Settings.SetupCVarCheckbox, 'Blizzard_Settings_Shared/Blizzard_Settings.lua', 561)
        original(CVarSettingMixin.Init, 'Blizzard_Settings_Shared/Blizzard_Setting.lua', 273)
        function ObserveEngravingSettings()
            local category, layout = Settings.RegisterVerticalLayoutCategory('Engraving settings probe')
            InterfaceOverrides.AdjustDisplaySettings(category)
            local runeCount, previewCount, proxyCount = 0, 0, 0
            for _, initializer in ipairs(layout:GetInitializers()) do
                local setting = initializer:GetSetting()
                local variable = setting:GetVariable()
                if variable == 'alwaysShowRuneIcons' then
                    runeCount = runeCount + 1
                    assert(setting == Settings.GetSetting(variable))
                    assert(setting:GetVariableType() == Settings.VarType.Boolean)
                    EngravingRuneSetting = setting
                elseif variable == 'previewTalentsOption' then
                    previewCount = previewCount + 1
                elseif variable == 'PROXY_SHOW_HELM' or variable == 'PROXY_SHOW_CLOAK' then
                    proxyCount = proxyCount + 1
                end
            end
            assert(previewCount == 1, 'complete original constructor past engraving branch')
            return runeCount, proxyCount
        end
        "#,
    )
    .expect("original selected consumer and real Settings provenance");
}

fn assert_constructed_controls(env: &WowLuaEnv, enabled: bool, transmog: bool) {
    let controls: (i32, i32) = env
        .eval("return ObserveEngravingSettings()")
        .expect("complete original AdjustDisplaySettings must return past line 138");
    assert_eq!(controls, (i32::from(enabled), if transmog { 0 } else { 2 }));
    assert!(env.state().borrow().lua_errors.is_empty());
}

fn assert_typed_rune_cvar(env: &WowLuaEnv) {
    env.exec(
        r#"
        local setting = EngravingRuneSetting
        assert(setting ~= nil, 'real alwaysShowRuneIcons checkbox setting')
        C_CVar.SetCVar('alwaysShowRuneIcons', '0')
        assert(setting:GetValue() == false)
        C_CVar.SetCVar('alwaysShowRuneIcons', '1')
        assert(setting:GetValue() == true)
        setting:SetValue(false)
        assert(setting:GetValue() == false and C_CVar.GetCVar('alwaysShowRuneIcons') == '0')
        setting:SetValue(true)
        assert(setting:GetValue() == true and C_CVar.GetCVar('alwaysShowRuneIcons') == '1')
        local ok, message = pcall(setting.SetValue, setting, '0')
        assert(not ok and message:find("requires 'boolean' type", 1, true), tostring(message))
        assert(C_CVar.GetCVar('alwaysShowRuneIcons') == '1')
        "#,
    )
    .expect("real boolean CVar setting reads and writes typed values");
}

fn exercise_fresh_settings(initial: bool) {
    let first = load_original_settings(Some(initial), !initial);
    let second = load_original_settings(Some(initial), initial);
    // Change only the second environment before either constructor is invoked.
    second.state().borrow_mut().engraving.enabled = Some(!initial);
    first
        .exec("C_CVar.SetCVar('alwaysShowRuneIcons', '0')")
        .unwrap();
    second
        .exec("C_CVar.SetCVar('alwaysShowRuneIcons', '1')")
        .unwrap();
    assert_constructed_controls(&first, initial, !initial);
    assert_constructed_controls(&second, !initial, initial);
    let (enabled, disabled) = if initial {
        (&first, &second)
    } else {
        (&second, &first)
    };
    let disabled_cvar: String = disabled
        .eval("return C_CVar.GetCVar('alwaysShowRuneIcons')")
        .unwrap();
    assert_typed_rune_cvar(enabled);
    assert_eq!(
        disabled
            .eval::<String>("return C_CVar.GetCVar('alwaysShowRuneIcons')")
            .unwrap(),
        disabled_cvar
    );
    assert!(
        disabled
            .eval::<bool>("return EngravingRuneSetting == nil")
            .unwrap()
    );
    assert_eq!(first.state().borrow().engraving.enabled, Some(initial));
    assert_eq!(second.state().borrow().engraving.enabled, Some(!initial));
    assert!(first.state().borrow().lua_errors.is_empty());
    assert!(second.state().borrow().lua_errors.is_empty());
}

#[test]
fn original_settings_enabled_first_projects_availability_and_cvar() {
    exercise_fresh_settings(true);
}

#[test]
fn original_settings_disabled_first_isolated_supplied_inputs() {
    exercise_fresh_settings(false);
}

#[test]
fn original_settings_requires_supplied_engraving() {
    let unsupplied = load_original_settings(None, true);
    let error = unsupplied
        .eval::<(i32, i32)>("return ObserveEngravingSettings()")
        .expect_err("unsupplied engraving must not choose a settings branch")
        .to_string();
    assert!(
        error.contains("InterfaceOverrides.lua:138:")
            && error.contains("C_Engraving.IsEngravingEnabled: unsupplied input"),
        "{error}"
    );
    // Earlier CVar variables are already registered in the failed environment.
    // Recovery is deliberately a fresh environment, never a registry reset.
    let supplied = load_original_settings(Some(true), true);
    assert_constructed_controls(&supplied, true, true);
    assert_typed_rune_cvar(&supplied);
    assert_eq!(unsupplied.state().borrow().engraving.enabled, None);
}
