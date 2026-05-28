//! Integration tests for the glyph cursor globals registered in
//! `src/lua_api/globals/real/glyph_state.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn arm_cursor_glyph(env: &WowLuaEnv, name: &str) {
    let mut state = env.state().borrow_mut();
    state.glyph.pending_glyph_name = Some(name.to_string());
    state.glyph.pending_glyph_removal = false;
}

#[test]
fn glyph_globals_live_under_real_globals_boundary() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/glyph_state.rs").exists(),
        "glyph globals are modeled through SimState and belong under globals::real",
    );
    assert!(
        std::path::Path::new("src/lua_api/globals/real/glyph_state.rs").exists(),
        "glyph globals should stay classified as real modeled Lua globals",
    );
}

#[test]
fn has_pending_glyph_cast_tracks_cursor_state() {
    let env = WowLuaEnv::new().expect("env");
    let before: bool = env.eval("return HasPendingGlyphCast()").unwrap();
    assert!(!before);

    arm_cursor_glyph(&env, "Glyph of Flash of Light");
    let after: bool = env.eval("return HasPendingGlyphCast()").unwrap();
    assert!(after);
}

#[test]
fn get_pending_glyph_name_round_trips() {
    let env = WowLuaEnv::new().expect("env");
    let nil_first: bool = env.eval("return GetPendingGlyphName() == nil").unwrap();
    assert!(nil_first);

    arm_cursor_glyph(&env, "Glyph of Holy Shock");
    let name: String = env.eval("return GetPendingGlyphName()").unwrap();
    assert_eq!(name, "Glyph of Holy Shock");
}

#[test]
fn is_pending_glyph_removal_reads_flag() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.glyph.pending_glyph_name = Some("Remove Glyph".to_string());
        state.glyph.pending_glyph_removal = true;
    }
    let removal: bool = env.eval("return IsPendingGlyphRemoval()").unwrap();
    assert!(removal);
}

#[test]
fn has_attached_glyph_checks_map() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .glyph
        .attached_glyphs
        .insert(635, "Glyph of Holy Light".to_string());

    let attached: bool = env.eval("return HasAttachedGlyph(635)").unwrap();
    assert!(attached);

    let missing: bool = env.eval("return HasAttachedGlyph(19750)").unwrap();
    assert!(!missing);
}

#[test]
fn get_current_glyph_name_for_spell_returns_inscribed_name() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .glyph
        .attached_glyphs
        .insert(635, "Glyph of Holy Light".to_string());

    let name: String = env.eval("return GetCurrentGlyphNameForSpell(635)").unwrap();
    assert_eq!(name, "Glyph of Holy Light");

    let missing: bool = env
        .eval("return GetCurrentGlyphNameForSpell(99999) == nil")
        .unwrap();
    assert!(missing);
}

#[test]
fn is_spell_valid_for_pending_glyph_requires_cursor_glyph() {
    let env = WowLuaEnv::new().expect("env");
    let no_cursor: bool = env.eval("return IsSpellValidForPendingGlyph(635)").unwrap();
    assert!(!no_cursor);

    arm_cursor_glyph(&env, "Glyph of Holy Light");
    let with_cursor: bool = env.eval("return IsSpellValidForPendingGlyph(635)").unwrap();
    assert!(with_cursor);
}

#[test]
fn attach_glyph_to_spell_inscribes_and_clears_pending() {
    let env = WowLuaEnv::new().expect("env");
    arm_cursor_glyph(&env, "Glyph of Holy Light");

    env.exec(
        "events_seen = {}\n\
         local f = CreateFrame('Frame')\n\
         f:RegisterEvent('GLYPH_ADDED')\n\
         f:SetScript('OnEvent', function(_, _, sid) table.insert(events_seen, sid) end)\n\
         AttachGlyphToSpell(635)",
    )
    .expect("AttachGlyphToSpell");

    let state = env.state().borrow();
    assert_eq!(
        state.glyph.attached_glyphs.get(&635),
        Some(&"Glyph of Holy Light".to_string()),
    );
    assert!(state.glyph.pending_glyph_name.is_none());
    assert!(!state.glyph.pending_glyph_removal);
    drop(state);

    let count: i32 = env.eval("return #events_seen").unwrap();
    assert_eq!(count, 1);
    let seen: f64 = env.eval("return events_seen[1]").unwrap();
    assert!((seen - 635.0).abs() < 1e-6);
}

#[test]
fn attach_glyph_to_spell_with_removal_pending_erases_entry() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state
            .glyph
            .attached_glyphs
            .insert(635, "Glyph of Holy Light".to_string());
        state.glyph.pending_glyph_name = Some("Remove Glyph".to_string());
        state.glyph.pending_glyph_removal = true;
    }

    env.exec(
        "events_seen = {}\n\
         local f = CreateFrame('Frame')\n\
         f:RegisterEvent('GLYPH_REMOVED')\n\
         f:SetScript('OnEvent', function(_, _, sid) table.insert(events_seen, sid) end)\n\
         AttachGlyphToSpell(635)",
    )
    .expect("AttachGlyphToSpell removal");

    let state = env.state().borrow();
    assert!(!state.glyph.attached_glyphs.contains_key(&635));
    assert!(state.glyph.pending_glyph_name.is_none());
    assert!(!state.glyph.pending_glyph_removal);
    drop(state);

    let count: i32 = env.eval("return #events_seen").unwrap();
    assert_eq!(count, 1);
}

#[test]
fn attach_glyph_to_spell_is_noop_without_pending() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        "events_seen = {}\n\
         local f = CreateFrame('Frame')\n\
         f:RegisterEvent('GLYPH_ADDED')\n\
         f:RegisterEvent('GLYPH_REMOVED')\n\
         f:SetScript('OnEvent', function(_, _, sid) table.insert(events_seen, sid) end)\n\
         AttachGlyphToSpell(635)",
    )
    .expect("AttachGlyphToSpell noop");

    let count: i32 = env.eval("return #events_seen").unwrap();
    assert_eq!(count, 0);
    assert!(env.state().borrow().glyph.attached_glyphs.is_empty());
}
