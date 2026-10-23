//! CF-2 original cached consumers. Authored only; runtime RED awaits admission.

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

const HIDDEN_MAP: i32 = 84;
const VISIBLE_MAP: i32 = 85;

fn load_original_consumer(addon: &str) -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, loaded) =
        crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(&ui, &[addon], &[]);
    assert!(loaded.iter().any(|name| name == addon));
    eprintln!("CF-2 original consumer cache: {}", ui.display());
    env
}

fn has_match_results_consumer() -> bool {
    let applicable = matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    );
    if !applicable {
        eprintln!("CF-2 match-results consumer N/A for {ACTIVE:?}; not passing proof");
    }
    applicable
}

fn create_match_event_fixture(env: &WowLuaEnv) {
    env.exec(
        r#"
        local info = debug.getinfo(PVPMatchResultsMixin.OnEvent, 'S')
        local suffix = 'Blizzard_PVPMatch/PVPMatchResults.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must invoke the original cached event consumer')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        -- Real event-registration output fixture; no substituted handler or API.
        Cf2MatchResults = CreateFrame('Frame', nil, UIParent)
        Mixin(Cf2MatchResults, PVPMatchResultsMixin)
        Cf2MatchResults:SetScript('OnEvent', Cf2MatchResults.OnEvent)
        Cf2MatchResults:RegisterEvent('PLAYER_ENTERING_WORLD')
        "#,
    )
    .expect("original match event provenance and frame fixture");
}

fn assert_enter_world_registration(env: &WowLuaEnv, expected: bool) {
    let errors_before = env.state().borrow().lua_errors.len();
    // Invoke the unchanged original handler directly so swallowed dispatch errors
    // cannot masquerade as a successful condition branch.
    env.exec("Cf2MatchResults:OnEvent('PLAYER_ENTERING_WORLD')")
        .expect("original entering-world condition must consume supplied match state");
    let registered: bool = env
        .eval("return Cf2MatchResults:IsEventRegistered('PVP_MATCH_COMPLETE')")
        .unwrap();
    assert_eq!(registered, expected, "post-match event subscription");
    let rewards_registered: bool = env
        .eval("return Cf2MatchResults:IsEventRegistered('POST_MATCH_ITEM_REWARD_UPDATE')")
        .unwrap();
    assert_eq!(rewards_registered, expected, "reward event subscription");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

#[test]
fn original_match_results_enter_world_ignores_explicit_inactive_match() {
    if !has_match_results_consumer() {
        return;
    }
    let env = load_original_consumer("Blizzard_PVPMatch");
    {
        let mut sim = env.state().borrow_mut();
        sim.pvp_match_active = false;
        sim.pvp_catalog.match_completed = false;
    }
    create_match_event_fixture(&env);
    assert_enter_world_registration(&env, false);
}

#[test]
fn original_match_results_enter_world_subscribes_for_active_and_completed_matches() {
    if !has_match_results_consumer() {
        return;
    }
    let env = load_original_consumer("Blizzard_PVPMatch");
    {
        let mut sim = env.state().borrow_mut();
        sim.pvp_match_active = true;
        sim.pvp_catalog.match_completed = false;
    }
    create_match_event_fixture(&env);
    assert_enter_world_registration(&env, true);
    env.exec("Cf2MatchResults:UnregisterAllEvents()")
        .expect("reset real registration output between match snapshots");
    {
        let mut sim = env.state().borrow_mut();
        sim.pvp_match_active = false;
        sim.pvp_catalog.match_completed = true;
    }
    assert_enter_world_registration(&env, true);
}

fn create_group_pin_fixture(env: &WowLuaEnv) {
    env.exec(
        r#"
        local info = debug.getinfo(GroupMembersPinMixin.OnMapChanged, 'S')
        local suffix = 'Blizzard_SharedMapDataProviders/GroupMembersDataProvider.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must invoke the original cached group-members consumer')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        Cf2Map = CreateFrame('Frame', nil, UIParent)
        Mixin(Cf2Map, MapCanvasMixin)
        Cf2GroupProvider = CreateFromMixins(GroupMembersDataProviderMixin)
        -- Player ping is unrelated to icon visibility. Supply its normal size input.
        Cf2GroupProvider.unitPinSizes = {player = 0, party = 16, raid = 16}
        Cf2GroupPin = CreateFrame('UnitPositionFrame', nil, UIParent)
        Mixin(Cf2GroupPin, GroupMembersPinMixin)
        Cf2GroupPin.owningMap = Cf2Map
        Cf2GroupPin.dataProvider = Cf2GroupProvider
        "#,
    )
    .expect("original group-pin provenance and real visibility output fixture");
}

fn assert_original_group_pin(env: &WowLuaEnv, map: i32, shown: bool) {
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(&format!("Cf2Map.mapID = {map}; Cf2GroupPin:OnMapChanged()"))
        .expect("original group-pin consumer must read supplied map display input");
    let actual: bool = env.eval("return Cf2GroupPin:IsShown()").unwrap();
    assert_eq!(actual, shown, "group icon visibility on supplied map {map}");
    if shown {
        let pin_map: i32 = env.eval("return Cf2GroupPin:GetUiMapID()").unwrap();
        assert_eq!(pin_map, map, "visible pin binds selected map");
    }
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

#[test]
fn original_group_pin_tracks_map_display_visibility_and_live_updates() {
    if !matches!(ACTIVE, ClientProfile::Era | ClientProfile::Anniversary) {
        eprintln!("CF-2 group-pin consumer N/A for {ACTIVE:?}; not passing proof");
        return;
    }
    let env = load_original_consumer("Blizzard_SharedMapDataProviders");
    {
        let mut sim = env.state().borrow_mut();
        sim.map_display_hide_icons.insert(HIDDEN_MAP, true);
        sim.map_display_hide_icons.insert(VISIBLE_MAP, false);
    }
    create_group_pin_fixture(&env);
    assert_original_group_pin(&env, HIDDEN_MAP, false);
    assert_original_group_pin(&env, VISIBLE_MAP, true);
    assert_original_group_pin(&env, HIDDEN_MAP, false);
    env.state()
        .borrow_mut()
        .map_display_hide_icons
        .insert(HIDDEN_MAP, false);
    assert_original_group_pin(&env, HIDDEN_MAP, true);
    env.state()
        .borrow_mut()
        .map_display_hide_icons
        .insert(VISIBLE_MAP, true);
    assert_original_group_pin(&env, VISIBLE_MAP, false);
    assert_original_group_pin(&env, HIDDEN_MAP, true);
}
