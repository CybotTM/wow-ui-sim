//! Behavior pin: assisted combat highlights the recommended action button.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const SLOT: u32 = 1;
const SPELL_ID: u32 = 1234;

#[test]
fn assisted_combat_next_cast_shows_matching_button_highlight() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_next_cast_button(env);
        register_action_button_as_rotation_spell(env);

        update_assisted_highlight_for_next_cast(env);

        assert_action_button_highlight_shown(env);
    });
    }
}

fn seed_next_cast_button(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.action_bars.clear();
    state.action_bars.insert(SLOT, SPELL_ID);
    state.assisted_combat.next_cast_spell_id = Some(SPELL_ID);
}

fn register_action_button_as_rotation_spell(env: &WowLuaEnv) {
    let registered: bool = env
        .eval(&format!(
            r#"
            ActionButton1:UpdateAction(true)
            AssistedCombatManager.rotationSpells[{SPELL_ID}] = true
            AssistedCombatManager:BuildAssistedHighlightCandidateActionButtonsList()
            return AssistedCombatManager.assistedHighlightCandidateActionButtons[ActionButton1] == {SPELL_ID}
            "#
        ))
        .expect("assisted highlight candidate registration must run cleanly");
    assert!(registered, "ActionButton1 must be a highlight candidate");
}

fn update_assisted_highlight_for_next_cast(env: &WowLuaEnv) {
    let updated: bool = env
        .eval(
            r#"
            local spellID = C_AssistedCombat.GetNextCastSpell()
            AssistedCombatManager:UpdateAllAssistedHighlightFramesForSpell(spellID)
            return spellID == 1234
            "#,
        )
        .expect("assisted highlight update must run cleanly");
    assert!(updated, "next assisted combat spell must be 1234");
}

fn assert_action_button_highlight_shown(env: &WowLuaEnv) {
    let shown: bool = env
        .eval(
            r#"
            local frame = ActionButton1.AssistedCombatHighlightFrame
            return frame ~= nil and frame:IsShown() == true
            "#,
        )
        .expect("assisted highlight visibility probe must run cleanly");
    assert!(
        shown,
        "UpdateAllAssistedHighlightFramesForSpell must show ActionButton1 highlight"
    );
}
