#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn trade_original_463_508_reset_and_empty_catalog_validation() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        filterUpdates = 0
        local frame = CreateFrame('Frame')
        frame:RegisterEvent('TRADE_SKILL_UPDATE')
        frame:SetScript('OnEvent', function() filterUpdates = filterUpdates + 1 end)
    "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .wrath_legacy
        .trade_skill_filters
        .inventory_mask = 0x80;
    env.exec(&original_lines("Blizzard_TradeSkillUI", 463, 463))
        .unwrap();
    {
        let sim = env.state().borrow();
        let filters = &sim.wrath_legacy.trade_skill_filters;
        assert_eq!(
            (filters.subclass_mask, filters.selected_subclass),
            (u32::MAX, u32::MAX)
        );
        assert_eq!(filters.inventory_mask, 0x80); // subclass rebuild preserves it
    }
    env.exec(&original_lines("Blizzard_TradeSkillUI", 508, 508))
        .unwrap();
    assert_eq!(
        env.state()
            .borrow()
            .wrath_legacy
            .trade_skill_filters
            .inventory_mask,
        u32::MAX
    );
    env.exec(
        r#"
        assert(filterUpdates == 2)
        for _, name in ipairs({'SetTradeSkillSubClassFilter', 'SetTradeSkillInvSlotFilter'}) do
            local f = _G[name]
            for _, reset in ipairs({0, -1, '0', 'not a number'}) do
                assert(select('#', f(reset)) == 0)
            end
            for _, bad in ipairs({false, true, {}}) do
                local ok, err = pcall(f, bad)
                local expected = 'Usage: '..name..'(index [, on\\off, exclusive])'
                assert(not ok and string.find(err, expected, 1, true), tostring(err))
            end
            local ok, err = pcall(f, 1)
            local expected = name == 'SetTradeSkillSubClassFilter'
                and 'Bad sub class in SetTradeSkillSubClassFilter'
                or 'Bad inv slot in SetTradeSkillInvSlotFilter'
            assert(not ok and string.find(err, expected, 1, true), tostring(err))
        end
        assert(filterUpdates == 10)
    "#,
    )
    .unwrap();
    let sim = env.state().borrow();
    let filters = &sim.wrath_legacy.trade_skill_filters;
    assert_eq!(
        (
            filters.subclass_mask,
            filters.selected_subclass,
            filters.inventory_mask
        ),
        (u32::MAX, u32::MAX, u32::MAX)
    );
}
