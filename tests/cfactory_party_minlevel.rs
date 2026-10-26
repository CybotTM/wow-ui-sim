//! Supplied party minimum through the original selected XML/Lua consumer.
#![cfg(any(feature = "client-retail", feature = "client-ptr",))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::game_data::default_party;
use wow_ui_sim::lua_api::state::LfdDungeonInfo;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn scenario_record(id: i32, min_level: i32, max_level: i32, random: bool) -> LfdDungeonInfo {
    LfdDungeonInfo {
        dungeon_id: id,
        name: format!("CF2 scenario {id}"),
        type_id: if random { 6 } else { 1 },
        subtype_id: 4, // Original Constants.lua: LFG_SUBTYPEID_SCENARIO.
        min_level,
        max_level,
        rec_level: min_level,
        min_rec_level: min_level,
        max_rec_level: max_level,
        expansion_level: 0,
        group_id: 0,
        texture_filename: String::new(),
        difficulty: 0,
        max_players: 3,
        description: String::new(),
        is_holiday: false,
        min_players: 1,
        map_name: String::new(),
        min_gear: 0,
        is_scaling_dungeon: false,
        is_random: random,
        is_follower_dungeon: false,
        training_ground_kind: None,
    }
}

fn supply_catalog(env: &WowLuaEnv, random: bool, lower_bound: i32, id_offset: i32) {
    let mut ordinary = scenario_record(910003 + id_offset, 1, 100, random);
    ordinary.subtype_id = 1; // Ordinary dungeon cannot open the scenario cover.
    env.state().borrow_mut().lfd_dungeons = vec![
        scenario_record(910002 + id_offset, 80, 90, random),
        scenario_record(910001 + id_offset, lower_bound, 70, random),
        ordinary,
    ];
}

fn supply_party(env: &WowLuaEnv, random: bool, id_offset: i32) {
    let mut member = default_party()
        .into_iter()
        .next()
        .expect("supplied party row");
    member.level = 49;
    member.connected = true;
    {
        let mut state = env.state().borrow_mut();
        state.player.level = 60;
        state.party_members = vec![member];
        state.party_group_active = true;
    }
    supply_catalog(env, random, 50, id_offset);
}

fn assert_selected_toc(ui: &std::path::Path) {
    let toc_path =
        find_toc_file(&ui.join("Blizzard_GroupFinder")).expect("selected GroupFinder TOC");
    let expected = match ACTIVE {
        ClientProfile::Ptr => "Blizzard_GroupFinder.toc",
        _ => "Blizzard_GroupFinder_Mainline.toc",
    };
    assert_eq!(toc_path.file_name().unwrap(), expected);
    let toc = TocFile::from_file(&toc_path).expect("parse selected GroupFinder TOC");
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    for suffix in [
        "Shared/ScenarioFinder_Shared.lua",
        "Shared/ScenarioFinder.xml",
    ] {
        assert!(toc.files.iter().any(|file| file.ends_with(suffix)));
    }
    eprintln!("CF-2 selected GroupFinder TOC: {}", toc_path.display());
}

fn load_selected_scenario(random: bool, id_offset: i32) -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    assert_selected_toc(&ui);
    let env = new_blizzard_addon_env(&ui);
    supply_party(&env, random, id_offset);
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
    retain_setup_diagnostics(&env);
    assert_original_frame(&env);
    env
}

fn retain_setup_diagnostics(env: &WowLuaEnv) {
    let state = env.state().borrow();
    for error in &state.lua_errors {
        eprintln!("CF-2 retained setup diagnostic: {error}");
    }
    if ACTIVE == ClientProfile::Mists {
        let lfr_blocked = state
            .lua_errors
            .iter()
            .any(|error| error.contains("IsLFREnabled"));
        assert!(
            !lfr_blocked,
            "Mists BLOCKED in original GroupFinder setup at IsLFREnabled; not producer RED"
        );
    }
}

fn assert_original_frame(env: &WowLuaEnv) {
    env.exec(
        r#"
        local frame = assert(ScenarioFinderFrame, 'original XML frame must exist')
        assert(frame:GetName() == 'ScenarioFinderFrame')
        assert(frame:GetParent() == GroupFinderFrame, 'original XML parent')
        assert(frame.NoScenariosCover:GetParent() == frame, 'original XML cover')
        assert(frame.NoScenariosCover.Label:GetParent() == frame.NoScenariosCover)
        assert(frame:GetScript('OnShow') == ScenarioFinderFrame_OnShow,
            'original XML script binding')
        assert(frame:GetScript('OnEvent') == ScenarioFinderFrame_OnEvent)
        local suffix = 'Blizzard_GroupFinder/Shared/ScenarioFinder_Shared.lua'
        for _, fn in ipairs({ScenarioFinderFrame_UpdateAvailability,
                frame:GetScript('OnShow'), frame:GetScript('OnEvent')}) do
            local info = debug.getinfo(fn, 'S')
            local source = info.source:gsub('\\', '/')
            assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
                'runtime-selected original Lua provenance')
            print('CF-2 original consumer', source, info.linedefined, info.lastlinedefined)
        end
        "#,
    )
    .expect("original XML-created ScenarioFinder frame and function provenance before RED");
}

fn assert_availability(env: &WowLuaEnv, minimum: i32, next_level: Option<i32>, available: bool) {
    let next = next_level.map_or_else(|| "nil".into(), |level| level.to_string());
    eprintln!("CF-2 consumer observation: minimum={minimum}, next={next}, available={available}");
    env.exec(&format!(
        r#"
        ScenarioFinderFrame_UpdateAvailability()
        assert(C_PartyInfo.GetMinLevel() == {minimum}, 'supplied party minimum')
        assert(ScenarioFinderFrame.NoScenariosCover:IsShown() == {covered},
            'original consumer cover follows supplied minimum and catalog')
        local nextLevel = {next}
        if {covered} then
            local expected = NO_SCENARIO_AVAILABLE
            if nextLevel then
                expected = string.format(NO_SCENARIO_AVAILABLE_WITH_NEXT_LEVEL, nextLevel)
            end
            assert(ScenarioFinderFrame.NoScenariosCover.Label:GetText() == expected,
                'original localized label follows replacement catalog threshold')
        end
        "#,
        covered = !available,
    ))
    .expect("original ScenarioFinder availability through live producers");
}

fn exercise_party_minimum(env: &WowLuaEnv) {
    assert_availability(env, 49, Some(50), false);
    env.state().borrow_mut().party_members[0].level = 50;
    assert_availability(env, 50, None, true);
    env.state().borrow_mut().player.level = 48;
    assert_availability(env, 48, Some(50), false);
    env.state().borrow_mut().player.level = 100;
    env.state().borrow_mut().party_members[0].level = 71;
    assert_availability(env, 71, Some(80), false);
    env.state().borrow_mut().party_members[0].level = 80;
    assert_availability(env, 80, None, true);
    env.state().borrow_mut().party_members[0].level = 90;
    assert_availability(env, 90, None, true);
    env.state().borrow_mut().party_members[0].level = 91;
    assert_availability(env, 91, None, false);
}

fn exercise_catalog_replacement(env: &WowLuaEnv, random: bool, id_offset: i32) {
    env.state().borrow_mut().player.level = 60;
    env.state().borrow_mut().party_members[0].level = 50;
    supply_catalog(env, random, 55, id_offset);
    assert_availability(env, 50, Some(55), false);
    env.state().borrow_mut().party_members[0].level = 55;
    assert_availability(env, 55, None, true);
    supply_catalog(env, random, 65, id_offset);
    assert_availability(env, 55, Some(65), false);
    if random {
        env.state()
            .borrow_mut()
            .lfd_dungeons
            .retain(|row| row.dungeon_id != 910002 + id_offset);
        assert_availability(env, 55, Some(65), false);
    }
    exercise_supplied_solo_recovery(env, random, id_offset);
}

fn exercise_supplied_solo_recovery(env: &WowLuaEnv, random: bool, id_offset: i32) {
    // Supplied solo policy only, not native ungrouped/category edge parity.
    {
        let mut state = env.state().borrow_mut();
        state.party_members.clear();
        state.party_group_active = false;
    }
    assert_availability(env, 60, Some(65), false);
    supply_catalog(env, random, 50, id_offset);
    assert_availability(env, 60, None, true);
}

#[test]
fn original_scenario_specific_catalog_uses_lowest_party_level() {
    for id_offset in [0, 10000] {
        let env = load_selected_scenario(false, id_offset);
        exercise_party_minimum(&env);
        exercise_catalog_replacement(&env, false, id_offset);
    }
}

#[test]
fn original_scenario_random_catalog_uses_lowest_party_level() {
    for id_offset in [0, 10000] {
        let env = load_selected_scenario(true, id_offset);
        exercise_party_minimum(&env);
        exercise_catalog_replacement(&env, true, id_offset);
    }
}
