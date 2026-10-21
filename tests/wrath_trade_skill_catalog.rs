#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn trade_catalog_original_470_515_fresh_empty_zero_returns() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function TradeSkillFilterFrame_LoadSubClasses(...) subclassCount = select('#', ...) end
        function TradeSkillFilterFrame_LoadInvSlots(...) inventoryCount = select('#', ...) end
        catalogEvents = 0
        local watcher = CreateFrame('Frame')
        watcher:RegisterEvent('TRADE_SKILL_UPDATE')
        watcher:SetScript('OnEvent', function() catalogEvents = catalogEvents + 1 end)
    "#,
    )
    .unwrap();
    env.exec(&original_lines("Blizzard_TradeSkillUI", 469, 471))
        .unwrap();
    env.exec(&original_lines("Blizzard_TradeSkillUI", 514, 516))
        .unwrap();
    env.exec(
        r#"
        TradeSkillSubClassDropDown_Initialize()
        TradeSkillInvSlotDropDown_Initialize()
        assert(subclassCount == 0 and inventoryCount == 0)
        assert(select('#', GetTradeSkillSubClasses(false, {})) == 0)
        assert(select('#', GetTradeSkillInvSlots(123)) == 0)
        assert(catalogEvents == 0)
        SetTradeSkillSubClassFilter(0)
        SetTradeSkillInvSlotFilter(0)
        assert(catalogEvents == 2)
        TradeSkillSubClassDropDown_Initialize()
        TradeSkillInvSlotDropDown_Initialize()
        assert(subclassCount == 0 and inventoryCount == 0)
        assert(catalogEvents == 2)
    "#,
    )
    .unwrap();
}
