//! Original Classic group-pin visibility consumers with supplied map state.
#![cfg(any(feature = "client-era", feature = "client-anniversary",))]

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
    // WorldMap declares both MapCanvas and SharedMapDataProviders as dependencies.
    let env = load_original_consumer("Blizzard_WorldMap");
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
