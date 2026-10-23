//! Shape preservation only; upgrade eligibility and invalid-input semantics are native-unknown.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]

// Retail/PTR/Forever: Blizzard_APIDocumentationGenerated/ItemUpgradeDocumentation.lua:11-24.
// Mists/Era/Anniversary: same file:11-23. Required ItemLocation -> one non-nil bool.
// Selected generic Blizzard_FrameXMLUtil/ItemUtil.lua:147 (Retail/PTR), :214 (Forever),
// :142 (Mists/Era/Anniversary); NOT Forever Camelot/ItemUtil.lua.
// Blizzard_FrameXMLUtil TOCs select generic ItemUtil.lua. ItemContextMatchResult is
// a consumer-local enum, not an extra generated API return contract.
#[test]
fn item_upgrade_boolean_contract() {
    crate::common::with_timeout(90, || {
        crate::common::blizzard_addon_harness::with_blizzard_addon_closure(
            &["Blizzard_ObjectAPI"],
            &[],
            |env, _| {
                env.exec(
                    r#"
                    assert(type(C_ItemUpgrade.CanUpgradeItem) == 'function',
                        'SHAPE BLOCKER: missing CanUpgradeItem')
                    local location = ItemLocation:CreateFromEquipmentSlot(1)
                    assert(location:IsEquipmentSlot(), 'SETUP: normal ItemLocation')
                    local function checkResult(...)
                        assert(select('#', ...) == 1, 'CanUpgradeItem return arity')
                        assert(type((...)) == 'boolean', 'non-nil boolean required')
                    end
                    checkResult(C_ItemUpgrade.CanUpgradeItem(location))
                    "#,
                )
                .expect("documented ItemLocation -> bool shape, not eligibility policy");
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "shape/setup errors: {:?}",
                    env.state().borrow().lua_errors
                );
            },
        );
    });
}

// Era/Anniversary select ItemUtil but no loadable upgrade window: the Mists UI
// TOC restricts AllowLoadGameType to mists. Their context-specific consumer
// subcheck is BLOCKED, not a runtime N/A PASS; direct documented shape stays covered.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists"
))]
#[test]
fn item_upgrade_original_item_util_contract() {
    crate::common::with_timeout(90, || {
        crate::common::blizzard_addon_harness::with_blizzard_addon_closure(
            &["Blizzard_FrameXML", "Blizzard_ItemUpgradeUI"],
            &[],
            |env, _| {
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "SETUP: original upgrade UI errors: {:?}",
                    env.state().borrow().lua_errors
                );
                consume_upgrade_result(env);
            },
        );
    });
}

#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists"
))]
fn consume_upgrade_result(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        assert(ItemUpgradeFrame, 'SETUP: runtime-selected original upgrade frame')
        ItemUpgradeFrame:Show()
        assert(ItemButtonUtil.GetItemContext() == ItemButtonUtil.ItemContextEnum.UpgradableItem,
            'SETUP: normal UI state did not reach upgrade context')
        local location = ItemLocation:CreateFromEquipmentSlot(1)
        assert(C_Item.DoesItemExist(location), 'SETUP: equipped item required for original branch')
        local function checkResult(...)
            assert(select('#', ...) == 1, 'CanUpgradeItem return arity')
            assert(type((...)) == 'boolean', 'non-nil boolean required')
        end
        checkResult(C_ItemUpgrade.CanUpgradeItem(location))
        local result = ItemButtonUtil.GetItemContextMatchResultForItem(location)
        assert(type(result) == 'number', 'consumer result type')
        local results = ItemButtonUtil.ItemContextMatchResult
        assert(result == results.Match or result == results.Mismatch,
            'original upgrade branch must accept actual boolean')
        "#,
    )
    .expect("original ItemUtil must consume actual upgrade query without Lua error");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "original upgrade consumer errors: {:?}",
        env.state().borrow().lua_errors
    );
}
