//! Wrath-only publication must not leak onto Retail.
#![cfg(feature = "client-retail")]
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn wrath_lod_globals_remain_unpublished_on_retail() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        for _, name in ipairs({
            'GetActiveTalentGroup', 'CreateMiniWorldMapArrowFrame',
            'CalendarEventGetTypes', 'CalendarDefaultGuildFilter', 'ArenaTeamRoster',
            'SetTradeSkillSubClassFilter', 'SetTradeSkillInvSlotFilter', 'random',
            'GetAuctionItemClasses', 'GetAuctionItemSubClasses', 'GetAuctionInvTypes',
            'GetNumTalentGroups', 'CalendarEventGetRepeatOptions', 'CalendarCanSendInvite',
            'GetTradeSkillSubClasses', 'GetTradeSkillInvSlots',
            'GetTradeSkillSubClassFilter', 'GetTradeSkillInvSlotFilter',
        }) do
            assert(rawget(_G, name) == nil, name)
        end
        assert(type(math.random) == 'function')
        assert(math.random(1, 1) == 1)
    "#,
    )
    .unwrap();
}
