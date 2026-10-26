//! Supplied availability through selected Mists InterfaceOverrides.lua:4.
#![cfg(feature = "client-mists")]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn load_original_display_settings() -> WowLuaEnv {
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
    assert!(
        toc.files
            .iter()
            .any(|file| file.ends_with("Classic/InterfaceOverrides.lua"))
    );
    let env = new_blizzard_addon_env(&ui);
    assert_eq!(env.state().borrow().mists_transmog.enabled, None);
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
    eprintln!(
        "unfiltered load diagnostics: {:?}",
        env.state().borrow().lua_errors
    );
    env.exec(
        r#"
        local info = debug.getinfo(InterfaceOverrides.AdjustDisplaySettings, 'S')
        local suffix = 'Blizzard_SettingsDefinitions_Frame/Classic/InterfaceOverrides.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and info.linedefined == 3)
        assert(source:sub(-#suffix) == suffix, 'original selected consumer')
        print('transmog consumer provenance', source, info.linedefined)
        function ObserveTransmogDisplay()
            local category, layout = Settings.RegisterVerticalLayoutCategory('Transmog input probe')
            local reached = false
            debug.sethook(function(_, line)
                local caller = debug.getinfo(2, 'S')
                local path = caller.source:gsub('\\', '/')
                if line == 38 and path:sub(-#suffix) == suffix then
                    reached = true
                    debug.sethook()
                    error('transmog-branch-observed')
                end
            end, 'l')
            local ok, err = pcall(InterfaceOverrides.AdjustDisplaySettings, category)
            debug.sethook()
            if not reached then error(err or 'original boundary not reached') end
            assert(not ok and tostring(err):find('transmog-branch-observed', 1, true))
            local controls = layout:GetInitializers()
            if #controls == 2 then
                assert(controls[1]:GetSetting():GetVariable() == 'PROXY_SHOW_HELM')
                assert(controls[2]:GetSetting():GetVariable() == 'PROXY_SHOW_CLOAK')
                assert(controls[1]:GetSetting():GetVariableType() == 'boolean')
                assert(controls[2]:GetSetting():GetVariableType() == 'boolean')
            end
            return #controls
        end
    "#,
    )
    .expect("original consumer and real Settings controls");
    env
}

fn assert_display_controls(env: &WowLuaEnv, enabled: bool) {
    env.state().borrow_mut().mists_transmog.enabled = Some(enabled);
    let errors_before = env.state().borrow().lua_errors.len();
    let controls: usize = env
        .eval("return ObserveTransmogDisplay()")
        .expect("original AdjustDisplaySettings:4 must consume supplied availability");
    assert_eq!(controls, if enabled { 0 } else { 2 });
    let (arity, supplied): (i32, bool) = env
        .eval("local function pack(...) return select('#', ...), ... end; return pack(C_TransmogOutfitInfo.IsTransmogEnabled())")
        .unwrap();
    assert_eq!((arity, supplied), (1, enabled));
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

fn exercise_display_transitions(initial: bool) {
    let first = load_original_display_settings();
    assert_display_controls(&first, initial);
    let second = load_original_display_settings();
    assert_display_controls(&second, !initial);
    assert_display_controls(&first, !initial);
    assert_display_controls(&second, initial);
    if initial {
        assert_display_controls(&first, true);
    }
    // Each environment registers the false-branch variables only once, as the
    // original SettingsPanel rejects duplicate registration. No sink is replaced.
    assert_eq!(first.state().borrow().mists_transmog.enabled, Some(true));
    let retained: usize = first.eval("return ObserveTransmogDisplay()").unwrap();
    assert_eq!(retained, 0);
    first.state().borrow_mut().mists_transmog.enabled = None;
    let error = first
        .eval::<usize>("return ObserveTransmogDisplay()")
        .expect_err("unset availability must not choose a UI branch");
    assert!(
        error.to_string().contains(
            "C_TransmogOutfitInfo.IsTransmogEnabled: transmog availability has not been supplied"
        ),
        "{error}"
    );
    assert_display_controls(&first, true);
    assert_eq!(
        second.state().borrow().mists_transmog.enabled,
        Some(initial)
    );
}

#[test]
fn original_display_enabled_then_transitions_isolation_and_unset_recovery() {
    exercise_display_transitions(true);
}

#[test]
fn original_display_disabled_then_transitions_isolation_and_unset_recovery() {
    exercise_display_transitions(false);
}
