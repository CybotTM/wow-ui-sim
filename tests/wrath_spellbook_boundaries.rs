//! Approved rank-zero/missing-metadata policy, not native missing-record parity.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn unranked_and_missing_spells_remain_separate_and_views_follow_membership() {
    let first = wow_ui_sim::spells::get_spell(2575).unwrap();
    let second = wow_ui_sim::spells::get_spell(2576).unwrap();
    assert_eq!((first.name, first.subtext), ("Mining", ""));
    assert_eq!((second.name, second.subtext), ("Mining", ""));
    assert!(wow_ui_sim::spells::get_spell(u32::MAX).is_none());

    let env = WowLuaEnv::new().unwrap();
    env.state().borrow_mut().known_spells = [2575, 2576, u32::MAX].into();
    env.exec(
        r#"
        assert(GetNumSpellTabs() == 1)
        local _, _, fullOffset, fullCount, compactOffset, compactCount = GetSpellTabInfo(1)
        assert(fullOffset == 0 and compactOffset == 0)
        assert(fullCount == 3 and compactCount == 3, 'rank-zero names must not collapse')
        for slot = 1, 3 do
            assert(GetKnownSlotFromHighestRankSlot(slot) == slot)
        end
        "#,
    )
    .unwrap();

    env.state().borrow_mut().known_spells.remove(&2575);
    env.exec(
        r#"
        local _, _, _, fullCount, _, compactCount = GetSpellTabInfo(1)
        assert(fullCount == 2 and compactCount == 2, 'no stale parallel view')
        "#,
    )
    .unwrap();

    env.state().borrow_mut().known_spells.clear();
    env.exec(
        r#"
        local _, _, _, fullCount, _, compactCount = GetSpellTabInfo(1)
        assert(fullCount == 0 and compactCount == 0)
        assert(GetKnownSlotFromHighestRankSlot(1) == 0)
        "#,
    )
    .unwrap();
    assert!(env.state().borrow().lua_errors.is_empty());
}
