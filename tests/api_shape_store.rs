//! Shape preservation only; catalog contents and purchase policy are native-unknown.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]

// Retail/PTR/Forever: Blizzard_APIDocumentationGenerated/StorePublicUIDocumentation.lua:11-24.
// Mists/Era/Anniversary: same file:11-23. Required number -> one non-nil boolean.
// Selected Blizzard_SharedXML/InterfaceUtil.lua:44,48 (Retail/PTR/Forever),
// :32,36 (Mists/Era/Anniversary), via Blizzard_SharedXML TOC's InterfaceUtil.lua.
// Do not force-load the cached secure StoreUI consumer, unselected on five profiles.
#[test]
fn store_original_interface_util_contract() {
    crate::common::with_timeout(90, || {
        crate::common::blizzard_addon_harness::with_blizzard_addon_closure(
            &["Blizzard_FrameXML"],
            &[],
            |env, _| {
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "SETUP: original InterfaceUtil dependencies: {:?}",
                    env.state().borrow().lua_errors
                );
                env.exec(
                    r#"
                    assert(type(C_StorePublic.DoesGroupHavePurchaseableProducts) == 'function',
                        'SHAPE BLOCKER: missing DoesGroupHavePurchaseableProducts')
                    local function checkGroup(groupID)
                        assert(type(groupID) == 'number', 'SETUP: original numeric category ID')
                        local function checkResult(...)
                            assert(select('#', ...) == 1, 'store query return arity')
                            assert(type((...)) == 'boolean', 'non-nil boolean required')
                        end
                        checkResult(C_StorePublic.DoesGroupHavePurchaseableProducts(groupID))
                    end
                    checkGroup(WOW_SUBSCRIPTION_CATEGORY_ID)
                    checkGroup(WOW_GAME_TIME_CATEGORY_ID)
                    assert(type(StoreInterfaceUtil.OpenToSubscriptionProduct) == 'function',
                        'SETUP: selected original subscription consumer')
                    -- This is a reachability prerequisite, not an asserted native shop default.
                    -- If Shop2 is selected, this legacy group-query subcheck is blocked.
                    assert(not C_CatalogShop.IsShop2Enabled(),
                        'SETUP BLOCKED: original Shop2 branch bypasses group query')
                    StoreInterfaceUtil.OpenToSubscriptionProduct()
                    -- No assertion about opening a category, return choice, ownership or policy.
                    -- The second original query is conditional on the first actual result;
                    -- direct shape checks above cover both IDs without manufacturing a branch.
                    "#,
                )
                .expect("selected original store consumer must accept actual query results");
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "original store consumer errors: {:?}",
                    env.state().borrow().lua_errors
                );
            },
        );
    });
}
