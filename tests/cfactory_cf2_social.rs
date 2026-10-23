//! CF-2 original-cache consumers. Source-staged; runtime RED is not yet observed.

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::game_data::default_party;
use wow_ui_sim::lua_api::state::LfdDungeonInfo;

fn exec_original(env: &WowLuaEnv, relative: &str) {
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir().join(relative);
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read original {}: {error}", path.display()));
    env.exec_named(&source, &format!("@{}", path.display()))
        .unwrap_or_else(|error| panic!("execute original {}: {error}", path.display()));
}

crate::post_wrath_ui_fixture! {
    #[test]
    fn original_guild_microbutton_has_no_unseen_invitations_with_empty_clubs() {
        let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
        let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
            &ui,
            &["Blizzard_MicroMenu"],
            &[],
        );
        env.state().borrow_mut().clubs.clubs.clear();
        env.exec(
            r#"
            DISPLAYED_COMMUNITIES_INVITATIONS = {}
            local button = CreateFrame("Button", nil, UIParent)
            Mixin(button, GuildMicroButtonMixin)
            assert(button:HasUnseenInvitations() == false,
                'empty supplied club state has no invitation to notify')
            DISPLAYED_COMMUNITIES_INVITATIONS['previously-displayed-club'] = true
            assert(button:HasUnseenInvitations() == false,
                'display history cannot manufacture an incoming invitation')
            local invitations = C_Club.GetInvitationsForSelf()
            assert(type(invitations) == 'table' and next(invitations) == nil,
                'empty supplied club state projects an empty invitation list')
            "#,
        )
        .expect("original microbutton empty-invitation consumer");
    }
}

fn has_scenario_consumer() -> bool {
    matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::Mists
    )
}

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

fn seed_scenario_state(env: &WowLuaEnv, random: bool) {
    let mut state = env.state().borrow_mut();
    state.player.level = 60;
    let mut member = default_party().into_iter().next().expect("party seed");
    member.level = 49;
    member.connected = true;
    state.party_members = vec![member];
    state.party_group_active = true;
    let mut dungeon = scenario_record(910003, 1, 100, random);
    dungeon.subtype_id = 1; // Ordinary dungeon must not make scenarios available.
    state.lfd_dungeons = vec![
        scenario_record(910002, 80, 90, random),
        scenario_record(910001, 50, 70, random),
        dungeon,
    ];
}

fn load_scenario_widgets_and_consumer(env: &WowLuaEnv) {
    env.exec(
        r#"
        -- Presentation fixtures only; no API or consumer functions replaced.
        NO_SCENARIO_AVAILABLE_WITH_NEXT_LEVEL = 'Next scenario at level %d'
        NO_SCENARIO_AVAILABLE = 'No scenarios at this level'
        ScenarioFinderFrame = CreateFrame('Frame', nil, UIParent)
        ScenarioFinderFrame.NoScenariosCover = CreateFrame('Frame', nil, ScenarioFinderFrame)
        ScenarioFinderFrame.NoScenariosCover.Label =
            ScenarioFinderFrame.NoScenariosCover:CreateFontString(nil, 'OVERLAY')
        "#,
    )
    .expect("scenario presentation fixture");
    exec_original(env, "Blizzard_GroupFinder/Shared/ScenarioFinder_Shared.lua");
}

fn exercise_scenario_party_transitions(env: &WowLuaEnv) {
    env.exec(
        r#"
        ScenarioFinderFrame_UpdateAvailability()
        assert(ScenarioFinderFrame.NoScenariosCover:IsShown(),
            'level-49 party member blocks the level-60 player scenario')
        assert(ScenarioFinderFrame.NoScenariosCover.Label:GetText() == 'Next scenario at level 50',
            'next available level is the minimum across the supplied catalog')
        assert(C_PartyInfo.GetMinLevel() == 49, 'party minimum, not player level')
        "#,
    )
    .expect("original scenario consumer below minimum");
    env.state().borrow_mut().party_members[0].level = 50;
    env.exec(
        r#"
        ScenarioFinderFrame_UpdateAvailability()
        assert(not ScenarioFinderFrame.NoScenariosCover:IsShown(),
            'inclusive lower boundary removes the scenario cover')
        assert(C_PartyInfo.GetMinLevel() == 50)
        "#,
    )
    .expect("original scenario consumer at minimum");
    env.state().borrow_mut().party_members.clear();
    env.state().borrow_mut().party_group_active = false;
    env.exec(
        r#"
        ScenarioFinderFrame.NoScenariosCover:Show()
        ScenarioFinderFrame_UpdateAvailability()
        assert(not ScenarioFinderFrame.NoScenariosCover:IsShown(),
            'leaving party restores level-60 player availability')
        assert(C_PartyInfo.GetMinLevel() == 60, 'solo uses player level')
        "#,
    )
    .expect("original scenario consumer after leaving party");
}

#[test]
fn original_scenario_specific_catalog_uses_lowest_party_level() {
    if !has_scenario_consumer() {
        eprintln!("N/A: CF-2 scenario residual is Retail/PTR/Mists only");
        return;
    }
    let env = WowLuaEnv::new().expect("scenario environment");
    seed_scenario_state(&env, false);
    load_scenario_widgets_and_consumer(&env);
    exercise_scenario_party_transitions(&env);
}

#[test]
fn original_scenario_random_catalog_uses_lowest_party_level() {
    if !has_scenario_consumer() {
        eprintln!("N/A: CF-2 scenario residual is Retail/PTR/Mists only");
        return;
    }
    let env = WowLuaEnv::new().expect("scenario environment");
    seed_scenario_state(&env, true);
    load_scenario_widgets_and_consumer(&env);
    exercise_scenario_party_transitions(&env);
}

#[test]
fn original_scenario_empty_catalog_keeps_no_scenarios_cover() {
    if !has_scenario_consumer() {
        eprintln!("N/A: CF-2 scenario residual is Retail/PTR/Mists only");
        return;
    }
    let env = WowLuaEnv::new().expect("scenario environment");
    env.state().borrow_mut().lfd_dungeons.clear();
    env.state().borrow_mut().party_members.clear();
    env.state().borrow_mut().party_group_active = false;
    load_scenario_widgets_and_consumer(&env);
    env.exec(
        r#"
        ScenarioFinderFrame.NoScenariosCover:Hide()
        ScenarioFinderFrame_UpdateAvailability()
        assert(ScenarioFinderFrame.NoScenariosCover:IsShown(), 'empty catalog shows cover')
        assert(ScenarioFinderFrame.NoScenariosCover.Label:GetText() == 'No scenarios at this level',
            'empty catalog has no invented next level')
        "#,
    )
    .expect("original scenario consumer with no catalog");
}
