use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn level_link_locks_default_to_unlocked() {
    let env = env();
    let (action_locked, spell_locked): (bool, bool) = env
        .eval(
            r#"
            return C_LevelLink.IsActionLocked(1),
                   C_LevelLink.IsSpellLocked(133)
            "#,
        )
        .unwrap();

    assert!(
        !action_locked,
        "actions should default to unlocked when no lock state is configured"
    );
    assert!(
        !spell_locked,
        "spells should default to unlocked when no lock state is configured"
    );
}

#[test]
fn level_link_uses_state_backed_action_and_spell_locks() {
    let env = env();
    let (
        action_lock_direct,
        action_lock_struct,
        spell_lock_direct,
        spell_lock_struct,
        action_unlock_explicit,
        spell_unlock_missing,
    ): (bool, bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            C_LevelLink._state.lockedActions = {
                [2] = true,
                [3] = { locked = true },
                [4] = { locked = false },
            }
            C_LevelLink._state.lockedSpells = {
                [111] = true,
                [222] = { locked = true },
            }

            return C_LevelLink.IsActionLocked(2),
                   C_LevelLink.IsActionLocked(3),
                   C_LevelLink.IsSpellLocked(111),
                   C_LevelLink.IsSpellLocked(222),
                   not C_LevelLink.IsActionLocked(4),
                   not C_LevelLink.IsSpellLocked(333)
            "#,
        )
        .unwrap();

    assert!(
        action_lock_direct,
        "boolean action lock entries should lock"
    );
    assert!(action_lock_struct, "table action lock entries should lock");
    assert!(spell_lock_direct, "boolean spell lock entries should lock");
    assert!(spell_lock_struct, "table spell lock entries should lock");
    assert!(
        action_unlock_explicit,
        "explicit unlocked action entry should not lock"
    );
    assert!(
        spell_unlock_missing,
        "missing spell lock entry should not lock"
    );
}

#[test]
fn level_link_normalizes_numeric_input_and_rejects_invalid_ids() {
    let env = env();
    let (
        action_from_string,
        spell_from_string,
        invalid_action_unlocked,
        invalid_spell_unlocked,
        last_action_query_ok,
        last_spell_query_ok,
    ): (bool, bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            C_LevelLink._state.lockedActions = { [17] = true }
            C_LevelLink._state.lockedSpells = { [204] = true }

            local actionFromString = C_LevelLink.IsActionLocked("17")
            local spellFromString = C_LevelLink.IsSpellLocked("204")
            local invalidActionUnlocked = not C_LevelLink.IsActionLocked("bad-action")
            local invalidSpellUnlocked = not C_LevelLink.IsSpellLocked({})

            return actionFromString,
                   spellFromString,
                   invalidActionUnlocked,
                   invalidSpellUnlocked,
                   C_LevelLink._state.lastActionQuery == nil,
                   C_LevelLink._state.lastSpellQuery == nil
            "#,
        )
        .unwrap();

    assert!(
        action_from_string,
        "numeric-string action IDs should normalize and lock"
    );
    assert!(
        spell_from_string,
        "numeric-string spell IDs should normalize and lock"
    );
    assert!(
        invalid_action_unlocked,
        "invalid action IDs should be unlocked"
    );
    assert!(
        invalid_spell_unlocked,
        "invalid spell IDs should be unlocked"
    );
    assert!(
        last_action_query_ok,
        "lastActionQuery should be nil after invalid action lookup"
    );
    assert!(
        last_spell_query_ok,
        "lastSpellQuery should be nil after invalid spell lookup"
    );
}
