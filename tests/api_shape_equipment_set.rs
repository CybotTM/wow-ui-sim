//! Modern documented shape only; locked-item criteria and unknown-set policy are native-unknown.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists"
))]

// Retail/PTR/Forever: Blizzard_APIDocumentationGenerated/EquipmentManagerDocumentation.lua:56-69.
// Mists: same file:53-65. Required numeric equipmentSetID -> one non-nil bool.
// Selected Blizzard_FrameXML/Mainline/EquipmentManager.lua:260 (Retail/PTR/Forever),
// Blizzard_FrameXML/Wrath/EquipmentManager.lua:308 (Mists).
// FrameXML TOCs select [Family]/EquipmentManager.lua (Retail/PTR:99, Forever:105),
// Blizzard_FrameXML_Mists.toc:70 selects Wrath/EquipmentManager.lua.
// Request #7 confirms only modern shape, not native semantics. Historical Wrath's
// string-input API is a different contract and does not authorize Wrath coverage.
#[test]
fn equipment_set_original_equip_set_contract() {
    crate::common::with_timeout(90, || {
        crate::common::blizzard_addon_harness::with_blizzard_addon_closure(
            &["Blizzard_FrameXML"],
            &[],
            |env, _| {
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "SETUP: original equipment manager dependencies: {:?}",
                    env.state().borrow().lua_errors
                );
                env.exec(
                    r#"
                    assert(type(C_EquipmentSet.EquipmentSetContainsLockedItems) == 'function',
                        'SHAPE BLOCKER: missing EquipmentSetContainsLockedItems')
                    local name = 'Shape contract equipment set'
                    C_EquipmentSet.CreateEquipmentSet(name, '12345')
                    local setID = C_EquipmentSet.GetEquipmentSetID(name)
                    assert(type(setID) == 'number', 'SETUP: saved set must exist')
                    assert(UnitCastingInfo('player') == nil, 'SETUP: not-casting prerequisite')
                    local function checkResult(...)
                        assert(select('#', ...) == 1, 'locked-items query return arity')
                        assert(type((...)) == 'boolean', 'non-nil boolean required')
                    end
                    checkResult(C_EquipmentSet.EquipmentSetContainsLockedItems(setID))
                    assert(type(EquipmentManager_EquipSet) == 'function',
                        'SETUP: runtime-selected original equipment manager')
                    EquipmentManager_EquipSet(setID)
                    -- Either real result is accepted. No assertion about lock decision,
                    -- equipment mutation, combat restrictions or unknown/empty-set policy.
                    "#,
                )
                .expect("original equipment manager must accept actual boolean query result");
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "original equipment manager errors: {:?}",
                    env.state().borrow().lua_errors
                );
            },
        );
    });
}
