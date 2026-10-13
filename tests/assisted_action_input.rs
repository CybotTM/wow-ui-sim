//! Input and tests staged; query producers and behavioral proof remain pending.
//! Direct semantic spellID slots only. Ascending order and nil-on-miss
//! are explicit simulator policies, not native runtime parity claims.

#![cfg(feature = "retail-12-0-0")]

use wow_ui_sim::lua_api::WowLuaEnv;

const ACTION_SPELL: u32 = 7001;
const OTHER_ACTION_SPELL: u32 = 7002;
const NEXT_CAST_SPELL: u32 = 9001;

fn fixture_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create explicit assisted-input environment");
    {
        let mut state = env.state().borrow_mut();
        state.action_bars.clear();
        state.action_macros.clear();
        state.action_outfits.clear();
    }
    env.exec(
        r#"
        function AssertAssistedSlots(expected)
            local actual = C_ActionBar.FindAssistedCombatActionButtons()
            if expected == nil then
                assert(actual == nil, 'simulator policy: nil, not empty table')
                return
            end
            assert(type(actual) == 'table', 'matching slots return a table')
            assert(#actual == #expected, 'exact matching slot count')
            for index, slot in ipairs(expected) do
                assert(actual[index] == slot, 'simulator policy: ascending slot order')
            end
            local count = 0
            for index, slot in pairs(actual) do
                count = count + 1
                assert(expected[index] == slot, 'no extra result entries')
            end
            assert(count == #expected, 'dense list of sparse slot IDs')
        end
        "#,
    )
    .expect("install assertions without replacing public APIs");
    env
}

#[test]
fn assisted_action_input_get_action_spell_reads_live_optional_identity() {
    let env = fixture_env();
    assert_eq!(env.state().borrow().assisted_combat.action_spell_id, None);
    env.state().borrow_mut().assisted_combat.next_cast_spell_id = Some(NEXT_CAST_SPELL);
    env.exec("assert(C_AssistedCombat.GetActionSpell() == nil)")
        .expect("unconfigured action identity is absent even with a recommendation");
    for spell_id in [Some(ACTION_SPELL), Some(OTHER_ACTION_SPELL), None] {
        env.state().borrow_mut().assisted_combat.action_spell_id = spell_id;
        let expected = spell_id.map_or_else(|| "nil".to_owned(), |id| id.to_string());
        env.exec(&format!(
            "assert(C_AssistedCombat.GetActionSpell() == {expected}, 'live action identity')"
        ))
        .expect("read each explicit action identity without recreating environment");
        assert_eq!(
            env.state().borrow().assisted_combat.next_cast_spell_id,
            Some(NEXT_CAST_SPELL)
        );
    }
    env.exec("assert(C_AssistedCombat.GetNextCastSpell() == 9001)")
        .expect("action input leaves the existing next-cast query unchanged");
}

#[test]
fn assisted_action_input_identity_and_slot_queries_are_environment_local() {
    let first = fixture_env();
    let second = fixture_env();
    {
        let mut state = first.state().borrow_mut();
        state.assisted_combat.action_spell_id = Some(ACTION_SPELL);
        state.action_bars.insert(5, ACTION_SPELL);
    }
    first
        .exec("assert(C_AssistedCombat.GetActionSpell() == 7001); AssertAssistedSlots({5})")
        .expect("first environment sees its explicit identity and slots");
    second
        .exec("assert(C_AssistedCombat.GetActionSpell() == nil); AssertAssistedSlots(nil)")
        .expect("second environment does not inherit first environment inputs");
    {
        let mut state = second.state().borrow_mut();
        state.assisted_combat.action_spell_id = Some(OTHER_ACTION_SPELL);
        state.action_bars.insert(12, OTHER_ACTION_SPELL);
    }
    first
        .exec("assert(C_AssistedCombat.GetActionSpell() == 7001); AssertAssistedSlots({5})")
        .expect("second environment updates leave first environment unchanged");
    second
        .exec("assert(C_AssistedCombat.GetActionSpell() == 7002); AssertAssistedSlots({12})")
        .expect("second environment sees only its own identity and slots");
}

#[test]
fn assisted_action_input_sparse_matches_use_ascending_simulator_policy() {
    let env = fixture_env();
    {
        let mut state = env.state().borrow_mut();
        state.assisted_combat.action_spell_id = Some(ACTION_SPELL);
        state.action_bars.extend([
            (145, ACTION_SPELL),
            (8, OTHER_ACTION_SPELL),
            (37, ACTION_SPELL),
            (2, ACTION_SPELL),
            (14, NEXT_CAST_SPELL),
        ]);
    }
    env.exec("AssertAssistedSlots({2, 37, 145})")
        .expect("dense ascending list of exactly the matching semantic spellID slots");
}

#[test]
fn assisted_action_input_absent_identity_and_no_matches_return_nil_policy() {
    let env = fixture_env();
    {
        let mut state = env.state().borrow_mut();
        state.assisted_combat.next_cast_spell_id = Some(NEXT_CAST_SPELL);
        state.action_bars.insert(7, NEXT_CAST_SPELL);
    }
    env.exec("AssertAssistedSlots(nil)")
        .expect("absent action identity does not match a populated next-cast slot");
    env.state().borrow_mut().assisted_combat.action_spell_id = Some(ACTION_SPELL);
    env.exec("AssertAssistedSlots(nil)")
        .expect("configured action identity with only unrelated bindings returns nil");
    env.state().borrow_mut().action_bars.clear();
    env.exec("AssertAssistedSlots(nil)")
        .expect("configured identity with empty action bars returns nil");
}

#[test]
fn assisted_action_input_results_are_fresh_and_reflect_live_bindings() {
    let env = fixture_env();
    {
        let mut state = env.state().borrow_mut();
        state.assisted_combat.action_spell_id = Some(ACTION_SPELL);
        state
            .action_bars
            .extend([(19, ACTION_SPELL), (3, ACTION_SPELL)]);
    }
    env.exec(
        r#"
        local first = C_ActionBar.FindAssistedCombatActionButtons()
        local second = C_ActionBar.FindAssistedCombatActionButtons()
        assert(type(first) == 'table' and type(second) == 'table')
        assert(not rawequal(first, second), 'each query returns a fresh table')
        assert(first[1] == 3 and first[2] == 19)
        first[1] = 999
        first[2] = nil
        first.extra = 'caller mutation'
        assert(second[1] == 3 and second[2] == 19 and second.extra == nil)
        AssertAssistedSlots({3, 19})
        retainedAssistedSlots = second
        "#,
    )
    .expect("caller mutations cannot affect another result or the backing model");
    assert_eq!(
        env.state().borrow().action_bars.get(&3),
        Some(&ACTION_SPELL)
    );
    {
        let mut state = env.state().borrow_mut();
        state.action_bars.remove(&3);
        state.action_bars.insert(19, OTHER_ACTION_SPELL);
        state.action_bars.insert(41, ACTION_SPELL);
    }
    env.exec(
        r#"
        AssertAssistedSlots({41})
        assert(retainedAssistedSlots[1] == 3 and retainedAssistedSlots[2] == 19,
            'previous result remains a snapshot after binding updates')
        "#,
    )
    .expect("next call reflects removals and rebinding, not a cached slot list");
}

#[test]
fn assisted_action_input_matching_never_uses_next_cast_recommendation() {
    let env = fixture_env();
    {
        let mut state = env.state().borrow_mut();
        state.assisted_combat.action_spell_id = Some(ACTION_SPELL);
        state.assisted_combat.next_cast_spell_id = Some(NEXT_CAST_SPELL);
        state.action_bars.extend([
            (23, ACTION_SPELL),
            (4, NEXT_CAST_SPELL),
            (31, OTHER_ACTION_SPELL),
        ]);
    }
    env.exec("AssertAssistedSlots({23})")
        .expect("different next-cast spell does not supply a matching slot");
    for next_cast in [Some(OTHER_ACTION_SPELL), None, Some(ACTION_SPELL)] {
        env.state().borrow_mut().assisted_combat.next_cast_spell_id = next_cast;
        env.exec("AssertAssistedSlots({23}); assert(C_AssistedCombat.GetActionSpell() == 7001)")
            .expect("recommendation changes cannot change action identity or its matches");
    }
    env.state().borrow_mut().assisted_combat.action_spell_id = Some(OTHER_ACTION_SPELL);
    env.exec("AssertAssistedSlots({31}); assert(C_AssistedCombat.GetActionSpell() == 7002)")
        .expect("live action identity changes matching independently of next-cast");
}
