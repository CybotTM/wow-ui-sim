//! Admin remove/recreate operations must not reuse an old location observation.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]

#[test]
fn admin_reagent_count_recreation_discards_item_upgrade_observation() {
    crate::common::with_timeout(90, || {
        crate::common::blizzard_addon_harness::with_blizzard_addon_closure(
            &["Blizzard_ObjectAPI"],
            &[],
            |env, _| {
                env.exec(r#"
                    A_Admin.ClearBags()
                    A_Admin.AddBagItem(0, 1, 6948, 1)
                    local location = ItemLocation:CreateFromBagAndSlot(0, 1)
                    A_Admin.SetItemUpgradeEligibility(location, true)
                    assert(C_ItemUpgrade.CanUpgradeItem(location) == true, 'supplied observation before removal')
                    A_Admin.SetReagentCount(6948, 0)
                    assert(not C_Item.DoesItemExist(location), 'admin removed the original item')
                    A_Admin.SetReagentCount(6948, 1)
                    assert(C_Item.DoesItemExist(location) and C_Item.GetItemID(location) == 6948,
                        'admin recreated the same item ID at the same location')
                    local ok, reason = pcall(C_ItemUpgrade.CanUpgradeItem, location)
                    local diagnostic = 'native-unknown: C_ItemUpgrade.CanUpgradeItem has no supplied eligibility or documented default for this location'
                    assert(not ok and tostring(reason):find(diagnostic, 1, true),
                        'recreated item must have no supplied eligibility: '..tostring(reason))
                "#).expect("admin reagent recreation must invalidate the old item observation");
            },
        );
    });
}
