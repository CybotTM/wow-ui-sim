//! Native 12340 evidence: handoff/laptop-labels-answers-20261018/
//! wrath-legacy-lod-apis-12340-20261020.md. Original cached consumers remain unmodified.
#![cfg(feature = "client-wrath")]
use wow_ui_sim::lua_api::WowLuaEnv;

fn original_lines(addon: &str, first: usize, last: usize) -> String {
    let path = dirs::cache_dir().unwrap().join(format!(
        "wow-ui-sim/blizzard-ui/wrath/AddOns/{addon}/{addon}.lua"
    ));
    std::fs::read_to_string(path)
        .expect("original cached consumer")
        .lines()
        .skip(first - 1)
        .take(last - first + 1)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn talent_original_266_and_native_arguments() {
    let env = WowLuaEnv::new().unwrap();
    let source = original_lines("Blizzard_TalentUI", 266, 266);
    env.exec(&format!("{source}\nassert(activeTalentGroup == 1)"))
        .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        sim.wrath_legacy.player_talent_group = 1;
        sim.wrath_legacy.inspected_talent_group = 2;
    }
    env.exec(
        r#"
        assert(select('#', GetActiveTalentGroup()) == 1)
        assert(GetActiveTalentGroup() == 2)
        assert(GetActiveTalentGroup(false, true) == 1)
        assert(GetActiveTalentGroup(true, true) == 3)
        local falses = {false, 0, 0.9, -0.9, '', '0', 'false', 'No', 'OFF', 'disabled', 'other', {}}
        local trues = {true, 1, -1, 1.9, '1', '9', 'True', 'Yes', 'ON', 'Enabled'}
        for _, value in ipairs(falses) do
            assert(GetActiveTalentGroup(value) == 2, tostring(value))
            assert(GetActiveTalentGroup(false, value) == 2, tostring(value))
        end
        for _, value in ipairs(trues) do
            assert(GetActiveTalentGroup(value) == 3, tostring(value))
            assert(GetActiveTalentGroup(false, value) == 1, tostring(value))
        end
        assert(GetActiveTalentGroup(nil, true) == 1)
        assert(GetActiveTalentGroup(function() end) == 2)
    "#,
    )
    .unwrap();
    let fresh = WowLuaEnv::new().unwrap();
    assert_eq!(
        fresh.eval::<i32>("return GetActiveTalentGroup()").unwrap(),
        1
    );
}
