//! Supplied LFR availability through the selected original PVEFrame.
#![cfg(feature = "client-mists")]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn assert_selected_group_finder(ui: &std::path::Path) {
    assert_eq!(ACTIVE, ClientProfile::Mists);
    let path = find_toc_file(&ui.join("Blizzard_GroupFinder")).unwrap();
    assert_eq!(
        path.file_name().unwrap(),
        "Blizzard_GroupFinder_Classic.toc"
    );
    let toc = TocFile::from_file(&path).unwrap();
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    for suffix in ["Classic/PVEFrame.lua", "Classic/PVEFrame.xml"] {
        assert!(toc.files.iter().any(|file| file.ends_with(suffix)));
    }
    eprintln!(
        "selected original GroupFinder: {} {:?}",
        path.display(),
        toc.metadata
    );
}

pub(super) fn load_original_pve_frame() -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    assert_selected_group_finder(&ui);
    let env = new_blizzard_addon_env(&ui);
    wow_ui_sim::xml::register_intrinsic_templates();
    let loaded = load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &[
            "Blizzard_SharedXMLBase",
            "Blizzard_Colors",
            "Blizzard_SharedXML",
            "Blizzard_SharedXMLGame",
            "Blizzard_LoadLocale",
            "Blizzard_Fonts_Shared",
            "Blizzard_FrameXMLBase",
            "Blizzard_GroupFinder",
        ],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_GroupFinder"));
    eprintln!(
        "unfiltered load diagnostics: {:?}",
        env.state().borrow().lua_errors
    );
    assert_original_frame(&env);
    env
}

fn assert_original_frame(env: &WowLuaEnv) {
    env.exec(r#"
        assert(PVEFrame and GroupFinderFrame, 'original XML frames')
        assert(GroupFinderFrame:GetParent() == PVEFrame, 'original frame parent')
        local button = GroupFinderFrame.groupButton2
        assert(button == GroupFinderFrameGroupButton2, 'original LFR XML button')
        assert(button:GetParent() == GroupFinderFrame and button:GetID() == 2)
        assert(button.icon and button.bg and button.name, 'original button template children')
        assert(GroupFinderFrame:GetScript('OnEvent') == GroupFinderFrame_OnEvent,
            'original XML event handler')
        for _, callback in ipairs({GroupFinderFrame_EvaluateButtonVisibility, GroupFinderFrame_OnEvent}) do
            local info = debug.getinfo(callback, 'S')
            local suffix = 'Blizzard_GroupFinder/Classic/PVEFrame.lua'
            local source = info.source:gsub('\\', '/')
            assert(info.what == 'Lua' and source:sub(-#suffix) == suffix, 'original consumer provenance')
            print('LFR consumer provenance', source, info.linedefined, info.lastlinedefined)
        end
    "#).expect("original selected PVEFrame identity and provenance");
}

fn assert_original_visibility(env: &WowLuaEnv, available: bool, level: i32) {
    {
        let mut sim = env.state().borrow_mut();
        sim.lfr_availability = Some(available);
        sim.scenario_finder_availability = Some((true, String::new()));
        sim.player.level = level;
    }
    let errors_before = env.state().borrow().lua_errors.len();
    observe_original_lfr_before_successor(env, available);
    let shown: bool = env
        .eval("return GroupFinderFrame.groupButton2:IsShown()")
        .unwrap();
    assert_eq!(
        shown, available,
        "original LFR button with supplied {available}, level {level}"
    );
    if !available {
        env.exec(
            r#"
            local point, relative, relativePoint, x, y = GroupFinderFrame.groupButton3:GetPoint(1)
            assert(point == 'TOP' and relative == GroupFinderFrame.groupButton1)
            assert(relativePoint == 'BOTTOM' and x == 0 and y == -23, 'original gap-closing anchor')
        "#,
        )
        .expect("original hidden-LFR gap-closing branch");
    }
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        errors_before,
        "no hidden callback errors"
    );
    eprintln!("original LFR observation: supplied={available}, level={level}, shown={shown}");
}

fn observe_original_lfr_before_successor(env: &WowLuaEnv, available: bool) {
    let script = format!("local expected = {available}\n")
        + r#"
        local observed = false
        debug.sethook(function(_, line)
            local info = debug.getinfo(2, 'S')
            local suffix = 'Blizzard_GroupFinder/Classic/PVEFrame.lua'
            local source = info.source:gsub('\\', '/')
            if line ~= 270 or source:sub(-#suffix) ~= suffix then return end
            debug.sethook()
            assert(GroupFinderFrame.groupButton2:IsShown() == expected, 'LFR state before scenario query')
            if not expected then
                local point, relative, relativePoint, x, y = GroupFinderFrame.groupButton3:GetPoint(1)
                assert(point == 'TOP' and relative == GroupFinderFrame.groupButton1)
                assert(relativePoint == 'BOTTOM' and x == 0 and y == -23, 'gap closed before scenario query')
            end
            observed = true
            print('LFR observation before original scenario boundary', expected)
        end, 'l')
        local ok, reason = xpcall(function()
            GroupFinderFrame:GetScript('OnEvent')(GroupFinderFrame, 'PLAYER_LEVEL_CHANGED')
        end, debug.traceback)
        debug.sethook()
        if not observed then error(reason or 'original LFR observation not reached', 0) end
        assert(ok, tostring(reason))
        assert(not GroupFinderFrame.groupButton4:IsShown(), 'empty catalog remains hidden after scenario query')
    "#;
    env.exec(&script)
        .expect("original PVEFrame.lua:229 must consume supplied LFR availability and complete past scenario query");
}

#[test]
fn stock_lfr_default_reaches_original_pve_frame() {
    let env = load_original_pve_frame();
    env.state().borrow_mut().scenario_finder_availability = Some((true, String::new()));
    observe_original_lfr_before_successor(&env, true);
    assert!(env.eval::<bool>("return C_LFGInfo.IsLFREnabled()").unwrap());
}

#[test]
fn original_pve_frame_tracks_supplied_lfr_availability_and_isolation() {
    let first = load_original_pve_frame();
    assert_original_visibility(&first, true, 1);
    assert_original_visibility(&first, false, 90);
    assert_original_visibility(&first, true, 90);
    let second = load_original_pve_frame();
    assert_original_visibility(&second, false, 1);
    assert_original_visibility(&second, true, 1);
    assert_original_visibility(&second, false, 90);
    first
        .exec("assert(GroupFinderFrame.groupButton2:IsShown())")
        .unwrap();
    assert_original_visibility(&first, false, 1);
    assert_original_visibility(&first, true, 1);
    second
        .exec("assert(not GroupFinderFrame.groupButton2:IsShown())")
        .unwrap();

    first.state().borrow_mut().lfr_availability = None;
    let error = first
        .exec("GroupFinderFrame:GetScript('OnEvent')(GroupFinderFrame, 'PLAYER_LEVEL_CHANGED')")
        .expect_err("unset input must not silently become false");
    assert!(error.to_string().contains("native-unknown: C_LFGInfo.IsLFREnabled has no supplied availability or documented default"), "{error}");
    assert_original_visibility(&first, false, 90);
    assert_original_visibility(&first, true, 90);
}
