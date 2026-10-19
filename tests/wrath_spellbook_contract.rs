//! Authenticated Wrath legacy spellbook binding contracts.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_no_errors(env: &WowLuaEnv, boundary: &str) {
    let state = env.state().borrow();
    assert!(
        state.lua_errors.is_empty(),
        "{boundary}: {:?}",
        state.lua_errors
    );
}

#[test]
fn wrath_spell_tab_info_exposes_full_and_compact_counts_and_exact_arguments() {
    let env = WowLuaEnv::new().unwrap();
    // Ebon Crown ranks 1/4, 2/4, 4/4: the highest rank has the LOWEST ID.
    env.state().borrow_mut().known_spells = [1248891, 1248890, 1248888, 6603].into();
    env.exec(
        r#"
        local function check(index)
            local values = {GetSpellTabInfo(index, 'ignored')}
            assert(select('#', GetSpellTabInfo(index)) == 6)
            assert(type(values[1]) == 'string' and type(values[2]) == 'string')
            assert(values[3] == 0 and values[4] == 4)
            assert(values[5] == 0 and values[6] == 2)
        end
        check(1)
        check('1')
        check(1.9)
        for _, mode in ipairs({'0', '1'}) do
            SetCVar('ShowAllSpellRanks', mode)
            check(1)
        end
        for _, index in ipairs({-1, 0, 2, 100}) do
            local a,b,c,d,e,f = GetSpellTabInfo(index)
            assert(select('#', GetSpellTabInfo(index)) == 6)
            assert(a == nil and b == nil and c == 0 and d == 0 and e == 0 and f == 0)
        end
        local function bad(...)
            local ok, message = pcall(GetSpellTabInfo, ...)
            assert(not ok, 'bad type must fail')
            assert(string.find(tostring(message), 'Usage: GetSpellTabInfo(index)', 1, true))
        end
        bad()
        bad(nil)
        bad(false)
        bad({})
        bad('not a number')
    "#,
    )
    .unwrap();
    assert_no_errors(&env, "tab-info API");
}

#[test]
fn wrath_highest_rank_slot_maps_to_sorted_full_known_slot_or_zero() {
    let env = WowLuaEnv::new().unwrap();
    env.state().borrow_mut().known_spells = [1248891, 1248890, 1248888, 6603].into();
    env.exec(
        r#"
        assert(select('#', GetKnownSlotFromHighestRankSlot(1)) == 1)
        assert(GetKnownSlotFromHighestRankSlot(1) == 1)
        assert(GetKnownSlotFromHighestRankSlot(2) == 4)
        assert(GetKnownSlotFromHighestRankSlot('2', 'ignored') == 4)
        for _, value in ipairs({-1, 0, 3, 100, false, {}, 'invalid'}) do
            assert(GetKnownSlotFromHighestRankSlot(value) == 0)
        end
        assert(GetKnownSlotFromHighestRankSlot() == 0)
        assert(GetKnownSlotFromHighestRankSlot(nil) == 0)
    "#,
    )
    .unwrap();
    env.state().borrow_mut().known_spells.clear();
    env.exec("assert(GetKnownSlotFromHighestRankSlot(1) == 0)")
        .unwrap();
    assert_no_errors(&env, "compact-to-full-slot API");
}

#[test]
fn wrath_update_spells_emits_argless_changed_synchronously_even_without_changes() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local trace = {}
        local frame = CreateFrame('Frame')
        frame:RegisterEvent('SPELLS_CHANGED')
        frame:SetScript('OnEvent', function(self, event, ...)
            trace[#trace + 1] = event..':'..select('#', ...)
        end)
        trace[#trace + 1] = 'before'
        assert(select('#', UpdateSpells('ignored', false)) == 0)
        trace[#trace + 1] = 'between'
        assert(select('#', UpdateSpells()) == 0)
        trace[#trace + 1] = 'after'
        assert(table.concat(trace, ',') ==
            'before,SPELLS_CHANGED:0,between,SPELLS_CHANGED:0,after', table.concat(trace, ','))
    "#,
    )
    .unwrap();
    assert_no_errors(&env, "synchronous UpdateSpells API");
}
