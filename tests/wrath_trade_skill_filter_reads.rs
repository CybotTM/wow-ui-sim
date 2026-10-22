#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn trade_filter_original_476_520_native_masks_and_validation() {
    let env = WowLuaEnv::new().unwrap();
    for line in [476, 520] {
        let source = original_lines("Blizzard_TradeSkillUI", line, line);
        env.exec(&format!(
            "{source}\nassert(allChecked == 1 and type(allChecked) == 'number')"
        ))
        .unwrap();
    }
    env.exec(
        r#"
        filterReadEvents = 0
        local watcher = CreateFrame('Frame')
        watcher:RegisterEvent('TRADE_SKILL_UPDATE')
        watcher:SetScript('OnEvent', function() filterReadEvents = filterReadEvents + 1 end)
        local function usage(f, expected, ...)
            local ok, err = pcall(f, ...)
            assert(not ok and string.find(err, expected, 1, true), tostring(err))
        end
        for _, name in ipairs({'GetTradeSkillSubClassFilter', 'GetTradeSkillInvSlotFilter'}) do
            local getter = _G[name]
            local expected = 'Usage: '..name..'(index)'
            usage(getter, expected)
            for _, bad in ipairs({false, true, {}, 'bad', ''}) do usage(getter, expected, bad) end
            for _, aggregate in ipairs({0, -1, -20, '0', 0.9, -0.9}) do
                assert(select('#', getter(aggregate)) == 1)
                assert(getter(aggregate, false, {}) == 1)
            end
        end
        local ok, err = pcall(GetTradeSkillSubClassFilter, 1)
        assert(not ok and string.find(err, 'Bad sub class in GetTradeSkillSubClassFilter', 1, true))
        assert(select('#', GetTradeSkillInvSlotFilter(1)) == 1)
        assert(GetTradeSkillInvSlotFilter(1) == nil)
        assert(GetTradeSkillInvSlotFilter(9999) == nil) -- sentinel before range validation
        assert(filterReadEvents == 0)
    "#,
    )
    .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        let filters = &mut sim.wrath_legacy.trade_skill_filters;
        filters.subclass_mask = 0;
        filters.selected_subclass = 2;
        filters.inventory_mask = 1;
    }
    env.exec(
        r#"
        assert(GetTradeSkillSubClassFilter(0) == 1) -- empty catalog is vacuously all
        assert(GetTradeSkillInvSlotFilter(0) == nil)
        assert(select('#', GetTradeSkillInvSlotFilter(0)) == 1)
        local ok, err = pcall(GetTradeSkillInvSlotFilter, '1')
        assert(not ok and string.find(err, 'Bad inv type in GetTradeSkillInvSlotFilter', 1, true))
        assert(filterReadEvents == 0)
        SetTradeSkillSubClassFilter(0, false, false)
        assert(filterReadEvents == 1)
        assert(GetTradeSkillSubClassFilter(0) == 1)
        assert(GetTradeSkillInvSlotFilter(0) == nil) -- single-bit mask survives subclass reset
    "#,
    )
    .unwrap();
    {
        let sim = env.state().borrow();
        let filters = &sim.wrath_legacy.trade_skill_filters;
        assert_eq!(
            (
                filters.subclass_mask,
                filters.selected_subclass,
                filters.inventory_mask
            ),
            (u32::MAX, u32::MAX, 1)
        );
    }
    env.exec(
        r#"
        SetTradeSkillInvSlotFilter(0, false, false)
        assert(GetTradeSkillInvSlotFilter(0) == 1)
        assert(GetTradeSkillInvSlotFilter(9999) == nil)
        assert(filterReadEvents == 2)
    "#,
    )
    .unwrap();
}
