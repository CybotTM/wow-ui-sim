//! Integration tests for `src/lua_api/globals/talent_spec_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── Talent tab / talent info ──────────────────────────────────────────────────

#[test]
fn get_num_talent_tabs_returns_zero() {
    let env = env();
    let n: i32 = env.eval("return GetNumTalentTabs()").unwrap();
    assert_eq!(n, 0);
}

#[test]
fn get_talent_info_returns_nil() {
    let env = env();
    let v: Option<String> = env.eval("return GetTalentInfo(1, 1)").unwrap();
    assert_eq!(v, None);
}

#[test]
fn get_talent_info_by_specialization_returns_nil() {
    let env = env();
    let v: Option<String> = env
        .eval("return GetTalentInfoBySpecialization(1, 1, 1)")
        .unwrap();
    assert_eq!(v, None);
}

// ── Spellbook tabs ────────────────────────────────────────────────────────────

#[test]
fn get_num_spell_tabs_returns_one() {
    let env = env();
    let n: i32 = env.eval("return GetNumSpellTabs()").unwrap();
    assert_eq!(n, 1);
}

#[test]
fn get_spell_tab_info_returns_class_name_and_spec_id() {
    let env = env();
    // Seeded player is Paladin (class_index 2), Retribution spec
    // (active_spec_index 2) by default — set Retribution specifically.
    env.state().borrow_mut().player.active_spec_index = 70;
    let (name, _icon, offset, num_spells, is_guild, spec_id): (
        String,
        String,
        i32,
        i32,
        bool,
        i32,
    ) = env.eval("return GetSpellTabInfo(1)").unwrap();
    assert_eq!(name, "Paladin");
    assert_eq!(offset, 0);
    assert_eq!(num_spells, 0);
    assert!(!is_guild);
    assert_eq!(spec_id, 70);
}

#[test]
fn get_spell_tab_info_nil_for_out_of_range_index() {
    let env = env();
    let v: Option<String> = env.eval("return GetSpellTabInfo(5)").unwrap();
    assert_eq!(v, None);
}

// ── PvP talents ───────────────────────────────────────────────────────────────

#[test]
fn get_pvp_talent_slot_info_returns_table_for_valid_slots() {
    let env = env();
    let (slot_index, enabled, locked, selected): (i32, bool, bool, i32) = env
        .eval(
            r#"
            local t = GetPvpTalentSlotInfo(2)
            return t.slotIndex, t.enabled, t.locked, t.selectedTalentID
            "#,
        )
        .unwrap();
    assert_eq!(slot_index, 2);
    assert!(enabled);
    assert!(!locked);
    assert_eq!(selected, 0);
}

#[test]
fn get_pvp_talent_slot_info_nil_for_out_of_range_slot() {
    let env = env();
    let v: Option<String> = env.eval("return GetPvpTalentSlotInfo(4)").unwrap();
    assert_eq!(v, None);
}

// ── Arena opponent spec ───────────────────────────────────────────────────────

#[test]
fn get_arena_opponent_spec_zero() {
    let env = env();
    let spec_id: i32 = env.eval("return GetArenaOpponentSpec(1)").unwrap();
    assert_eq!(spec_id, 0);
}

// ── Skill lines (legacy) ──────────────────────────────────────────────────────

#[test]
fn get_num_skill_lines_zero() {
    let env = env();
    let n: i32 = env.eval("return GetNumSkillLines()").unwrap();
    assert_eq!(n, 0);
}

#[test]
fn get_skill_line_info_nil() {
    let env = env();
    let v: Option<String> = env.eval("return GetSkillLineInfo(1)").unwrap();
    assert_eq!(v, None);
}

#[test]
fn get_selected_skill_zero() {
    let env = env();
    let n: i32 = env.eval("return GetSelectedSkill()").unwrap();
    assert_eq!(n, 0);
}
