//! Integration tests for `src/lua_api/globals/spell_macro_verbs.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{CursorInfo, MacroInfo};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── PickupSpell ───────────────────────────────────────────────────────────────

#[test]
fn pickup_spell_sets_cursor_to_spell_variant() {
    let env = env();
    env.exec("PickupSpell(12345)").unwrap();
    assert!(matches!(
        env.state().borrow().cursor_item,
        Some(CursorInfo::Spell { spell_id: 12345 })
    ));
}

// ── PickupTalent / PickupPvpTalent ────────────────────────────────────────────

#[test]
fn pickup_talent_sets_cursor_to_talent_non_pvp() {
    let env = env();
    env.exec("PickupTalent(42)").unwrap();
    assert!(matches!(
        env.state().borrow().cursor_item,
        Some(CursorInfo::Talent {
            talent_id: 42,
            pvp: false
        })
    ));
}

#[test]
fn pickup_pvp_talent_marks_pvp_true() {
    let env = env();
    env.exec("PickupPvpTalent(99)").unwrap();
    assert!(matches!(
        env.state().borrow().cursor_item,
        Some(CursorInfo::Talent {
            talent_id: 99,
            pvp: true
        })
    ));
}

// ── PickupPetAction ───────────────────────────────────────────────────────────

#[test]
fn pickup_pet_action_synthesizes_spell_id_from_slot() {
    let env = env();
    env.exec("PickupPetAction(3)").unwrap();
    assert!(matches!(
        env.state().borrow().cursor_item,
        Some(CursorInfo::PetAction {
            slot: 3,
            spell_id: 1_000_003
        })
    ));
}

// ── PickupMacro ───────────────────────────────────────────────────────────────

#[test]
fn pickup_macro_sets_cursor_to_macro_variant() {
    let env = env();
    env.exec("PickupMacro(7)").unwrap();
    assert!(matches!(
        env.state().borrow().cursor_item,
        Some(CursorInfo::Macro { macro_index: 7 })
    ));
}

// ── RunMacro ──────────────────────────────────────────────────────────────────

#[test]
fn run_macro_by_index_sets_running_macro_one_based() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.macros.push(MacroInfo {
            name: "Alpha".into(),
            icon: String::new(),
            body: "/say hi".into(),
        });
        st.macros.push(MacroInfo {
            name: "Beta".into(),
            icon: String::new(),
            body: "/say bye".into(),
        });
    }
    env.exec("RunMacro(2)").unwrap();
    assert_eq!(env.state().borrow().running_macro, Some(2));
}

#[test]
fn run_macro_by_name_resolves_to_slot() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.macros.push(MacroInfo {
            name: "Alpha".into(),
            ..MacroInfo::default()
        });
        st.macros.push(MacroInfo {
            name: "Beta".into(),
            ..MacroInfo::default()
        });
    }
    env.exec(r#"RunMacro("Beta")"#).unwrap();
    assert_eq!(env.state().borrow().running_macro, Some(2));
}

#[test]
fn run_macro_out_of_range_is_noop() {
    let env = env();
    env.exec("RunMacro(99)").unwrap();
    assert!(env.state().borrow().running_macro.is_none());
}

// ── StopMacro ─────────────────────────────────────────────────────────────────

#[test]
fn stop_macro_clears_running_macro() {
    let env = env();
    env.state().borrow_mut().running_macro = Some(5);
    env.exec("StopMacro()").unwrap();
    assert!(env.state().borrow().running_macro.is_none());
}

// ── EditMacro ─────────────────────────────────────────────────────────────────

#[test]
fn edit_macro_updates_existing_slot_fields() {
    let env = env();
    env.state().borrow_mut().macros.push(MacroInfo {
        name: "Old".into(),
        icon: "old-icon".into(),
        body: "/old".into(),
    });
    env.exec(r#"EditMacro(1, "New", "new-icon", "/new")"#)
        .unwrap();
    let st = env.state().borrow();
    let entry = &st.macros[0];
    assert_eq!(entry.name, "New");
    assert_eq!(entry.icon, "new-icon");
    assert_eq!(entry.body, "/new");
}

#[test]
fn edit_macro_grows_table_on_out_of_range_index() {
    let env = env();
    assert_eq!(env.state().borrow().macros.len(), 0);
    env.exec(r#"EditMacro(3, "Gamma", "icon", "/g")"#).unwrap();
    let st = env.state().borrow();
    assert_eq!(st.macros.len(), 3, "table must grow to cover slot 3");
    assert_eq!(st.macros[2].name, "Gamma");
    assert_eq!(st.macros[2].body, "/g");
    // Intervening slots are default-empty.
    assert_eq!(st.macros[0].name, "");
    assert_eq!(st.macros[1].name, "");
}

#[test]
fn edit_macro_by_name_updates_matching_slot() {
    let env = env();
    env.state().borrow_mut().macros.push(MacroInfo {
        name: "Fireball".into(),
        icon: "i".into(),
        body: "/cast Fireball".into(),
    });
    env.exec(r#"EditMacro("Fireball", nil, nil, "/cast Pyroblast")"#)
        .unwrap();
    let st = env.state().borrow();
    let entry = &st.macros[0];
    assert_eq!(entry.name, "Fireball", "name unchanged when passed nil");
    assert_eq!(entry.body, "/cast Pyroblast");
}
