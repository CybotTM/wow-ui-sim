#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn talent_count_original_267_native_cached_and_pet_rules() {
    let env = WowLuaEnv::new().unwrap();
    let source = original_lines("Blizzard_TalentUI", 267, 267);
    env.exec(&format!("{source}\nassert(numTalentGroups == 1)"))
        .unwrap();
    env.exec("assert(select('#', GetNumTalentGroups()) == 1); assert(GetNumTalentGroups(false, true) == 0)").unwrap();
    {
        let mut sim = env.state().borrow_mut();
        let counts = &mut sim.wrath_legacy.talent_group_counts;
        counts.player = 2;
        counts.inspected = 3;
        counts.pet_tab_present = true;
        counts.pet_tab_count = 7;
    }
    env.exec(
        r#"
        local events = 0
        local watcher = CreateFrame('Frame')
        watcher:RegisterEvent('ACTIVE_TALENT_GROUP_CHANGED')
        watcher:SetScript('OnEvent', function() events = events + 1 end)
        assert(GetNumTalentGroups() == 2)
        assert(GetNumTalentGroups(false, true) == 1)
        assert(GetNumTalentGroups(true, true) == 3)
        for _, value in ipairs({false, 0, 0.9, -0.9, '', '0', 'OFF', 'disabled', 'other', {}}) do
            assert(GetNumTalentGroups(value) == 2, tostring(value))
            assert(GetNumTalentGroups(false, value) == 2, tostring(value))
        end
        for _, value in ipairs({true, 1, -1, 1.9, '1', 'True', 'Yes', 'ON', 'Enabled'}) do
            assert(GetNumTalentGroups(value) == 3, tostring(value))
            assert(GetNumTalentGroups(false, value) == 1, tostring(value))
        end
        assert(GetNumTalentGroups(nil, true) == 1)
        assert(GetNumTalentGroups(function() end) == 2)
        assert(events == 0)
    "#,
    )
    .unwrap();
    {
        let sim = env.state().borrow();
        let counts = &sim.wrath_legacy.talent_group_counts;
        assert_eq!(
            (
                counts.player,
                counts.inspected,
                counts.pet_tab_present,
                counts.pet_tab_count
            ),
            (2, 3, true, 7)
        );
    }
    env.state()
        .borrow_mut()
        .wrath_legacy
        .talent_group_counts
        .pet_tab_present = false;
    assert_eq!(
        env.eval::<i32>("return GetNumTalentGroups(false, true)")
            .unwrap(),
        0
    );
    {
        let mut sim = env.state().borrow_mut();
        let counts = &mut sim.wrath_legacy.talent_group_counts;
        counts.pet_tab_present = true;
        counts.pet_tab_count = 0;
        counts.player = 0;
        counts.inspected = 0;
    }
    env.exec("assert(GetNumTalentGroups(false, true) == 0); assert(GetNumTalentGroups() == 0); assert(GetNumTalentGroups(true, true) == 0)").unwrap();
}
