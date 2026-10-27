//! F10 native fixtures, separate from the unchanged modern PlayerSpells rows.
//! Cached layout/selectors are INFERRED, not captured 12340 runtime contracts.

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_spellbook_fixture() -> WowLuaEnv {
    let env = super::load_original_startup();
    env.state().borrow_mut().known_spells = [6603, 635, 19750].into();
    env.exec(
        r#"
        F10Errors = {}
        seterrorhandler(function(message)
            F10Errors[#F10Errors + 1] = tostring(message)
        end)
        HideUIPanel(SpellBookFrame)
        SetCVar('ShowAllSpellRanks', '1')
        assert(GetNumSpellTabs() > 0, 'fixture needs supplied known spells')
        "#,
    )
    .expect("prepare supplied spellbook state without replacing consumer functions");
    env
}

fn assert_no_errors(env: &WowLuaEnv) {
    super::assert_no_errors(env, "F10 original consumer");
    env.exec(r"assert(#F10Errors == 0, table.concat(F10Errors, '\n'))")
        .expect("no caught callback errors");
}

fn exec_consumer(env: &WowLuaEnv, code: &str) {
    let result = env.exec(code);
    assert_no_errors(env);
    result.expect("unchanged native consumer and capability assertions");
}

fn open_spellbook(env: &WowLuaEnv) {
    exec_consumer(
        env,
        "ToggleSpellBook(BOOKTYPE_SPELL); assert(SpellBookFrame:IsVisible())",
    );
}

fn load_talent_fixture() -> WowLuaEnv {
    let env = load_spellbook_fixture();
    let addons = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    let directory = addons.join("Blizzard_TalentUI");
    assert_original_talent_sources(&directory);
    let toc = wow_ui_sim::loader::find_toc_file(&directory).expect("native TalentUI TOC");
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
        .expect("load original native talent addon");
    assert_no_errors(&env);
    env
}

fn assert_original_talent_sources(directory: &std::path::Path) {
    for (name, expected) in TALENT_SOURCES {
        let bytes = std::fs::read(directory.join(name)).expect("pinned TalentUI source");
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), *expected, "{name}");
    }
}

const TALENT_SOURCES: &[(&str, &str)] = &[
    // Digests bind the original consumer; they are not a native runtime oracle.
    (
        "Blizzard_TalentUI.toc",
        "b02ff0b66a81a573cc6c3e7a69a51f37fc5e0b4807e9dc415804c791e2a72cba",
    ),
    (
        "Blizzard_TalentUI.lua",
        "32ae6dcbd0c801b59d2d71a73cec68a963beacabb823cedbb97f11e7f0a968e3",
    ),
    (
        "Blizzard_TalentUI.xml",
        "73ef8ae6411ea556ee05a5eb0cc1f693bc0a00695375875296f253d47175a064",
    ),
    (
        "Localization.lua",
        "835100e9c62e81bde9181d26349865ae5cdea459b04929ca019182859d9dd02d",
    ),
];

#[test]
fn glyph_socket_consumer_tracks_supplied_records() {
    let env = load_talent_fixture();
    env.exec("GlyphFrame_LoadUI()")
        .expect("load original GlyphUI");
    exec_consumer(
        &env,
        r#"
        PlayerTalentFrame.talentGroup = 1
        A_Admin.SetWrathGlyphSocket(1, 1, true, 1, 101, 'Interface\\Icons\\GlyphOne')
        GlyphFrameGlyph_UpdateSlot(GlyphFrameGlyph1)
        assert(GlyphFrameGlyph1.spell == 101)
        assert(GlyphFrameGlyph1.glyph:GetTexture() == 'Interface\\Icons\\GlyphOne')
        A_Admin.SetWrathGlyphSocket(1, 1, true, 2, 202, 'Interface\\Icons\\GlyphTwo')
        GlyphFrameGlyph_UpdateSlot(GlyphFrameGlyph1)
        assert(GlyphFrameGlyph1.spell == 202)
        assert(GlyphFrameGlyph1.glyph:GetTexture() == 'Interface\\Icons\\GlyphTwo')
        A_Admin.SetWrathGlyphSocket(1, 1, true, 1, nil, nil)
        local enabled, kind, spell, icon = GetGlyphSocketInfo(1, 1)
        assert(enabled == 1 and kind == 1 and spell == nil and icon == nil)
        A_Admin.SetWrathGlyphSocket(1, 1, false, 2, nil, nil)
        enabled, kind, spell, icon = GetGlyphSocketInfo(1, 1)
        assert(enabled == nil and kind == 2 and spell == nil and icon == nil)
        A_Admin.SetWrathGlyphSocket(1, 1, false, 2, 202, 'Interface\\Icons\\GlyphTwo')
        enabled, kind, spell, icon = GetGlyphSocketInfo(1, 1)
        assert(enabled == nil and kind == 2 and spell == 202 and icon == 'Interface\\Icons\\GlyphTwo', 'locked populated socket must retain glyph')
        assert(select('#', GetGlyphSocketInfo(2, 1)) == 0)
        assert(select('#', GetGlyphSocketInfo(0, 1)) == 0)
        assert(GetNumGlyphSockets() == 6)
        A_Admin.SetWrathGlyphSocket(2, 1, true, 2, 303, 'Interface\\Icons\\GlyphThree')
        local groupEnabled, groupType, groupSpell = GetGlyphSocketInfo(1, 2)
        assert(groupEnabled == 1 and groupType == 2 and groupSpell == 303)
        enabled, kind, spell, icon = GetGlyphSocketInfo(1, 1)
        assert(enabled == nil and kind == 2 and spell == 202 and icon == 'Interface\\Icons\\GlyphTwo')
        "#,
    );
}

#[test]
fn talent_tab_background_tracks_supplied_records_on_show() {
    let env = load_talent_fixture();
    for (background, rank, prerequisite) in [("FirstTree", 1, false), ("SecondTree", 0, true), ("SecondTree", 1, true)] {
        env.state().borrow_mut().wrath_talent_tabs.insert(
            (1, 1),
            wow_ui_sim::lua_api::state::WrathTalentTab {
                name: "Supplied talent tree".to_string(),
                icon: "Interface\\Icons\\Supplied".to_string(),
                points_spent: 3,
                background: background.to_string(),
                preview_points_spent: 1,
            },
        );
        if prerequisite {
            env.state().borrow_mut().wrath_talent_tabs.insert(
                (1, 2),
                wow_ui_sim::lua_api::state::WrathTalentTab {
                    name: "Second tree".to_string(),
                    icon: "Interface\\Icons\\SecondTree".to_string(),
                    points_spent: 0,
                    background: "AnotherTree".to_string(),
                    preview_points_spent: 0,
                },
            );
        }
        let mut rows = vec![wow_ui_sim::lua_api::state::WrathTalentRow {
            name: "First talent".to_string(),
            icon: "Interface\\Icons\\FirstTalent".to_string(),
            tier: 1,
            column: 1,
            rank,
            max_rank: 1,
            exceptional: false,
            meets_prereq: true,
            preview_rank: rank,
            meets_preview_prereq: true,
            prerequisites: vec![],
        }];
        if prerequisite {
            rows.push(wow_ui_sim::lua_api::state::WrathTalentRow {
                name: "Second talent".to_string(),
                icon: "Interface\\Icons\\SecondTalent".to_string(),
                tier: 1,
                column: 2,
                rank: 1,
                max_rank: 1,
                exceptional: false,
                meets_prereq: true,
                preview_rank: 1,
                meets_preview_prereq: true,
                prerequisites: vec![wow_ui_sim::lua_api::state::WrathTalentPrerequisite {
                    talent_index: 1,
                    required_rank: 1,
                }],
            });
        }
        env.state().borrow_mut().wrath_talent_rows.insert((1, 1), rows);
        env.exec("A_Admin.SetWrathUnspentTalentPoints(1, 3); SetCVar('previewTalents', '0')")
            .expect("supply talent points");
        env.exec("PlayerTalentFrame:Hide(); PlayerTalentFrame:Show()")
            .expect("show native talent panel");
        assert_no_errors(&env);
        let expected_tabs = if prerequisite { 2 } else { 1 };
        env.exec(&format!("assert(GetNumTalentTabs(false, false) == {expected_tabs})"))
            .expect("supplied tab count");
        env.exec(&format!(
            "assert(PlayerTalentFrameBackgroundTopLeft:GetTexture() == 'Interface\\\\TalentFrame\\\\{background}-TopLeft')"
        ))
        .expect("native consumer applies supplied background");
        env.exec("assert(PlayerTalentFrameTalent1:IsShown()); assert(PlayerTalentFrameTalent1IconTexture:GetTexture() == 'Interface\\\\Icons\\\\FirstTalent')")
            .expect("native consumer renders supplied talent");
        if prerequisite {
            let arrow = if rank == 0 { -1 } else { 1 };
            env.exec(&format!("assert(PlayerTalentFrameTalent2:IsShown()); assert(PlayerTalentFrame.TALENT_BRANCH_ARRAY[1][2].leftArrow == {arrow})"))
                .expect("native consumer reflects prerequisite rank");
        } else {
            env.exec("assert(not PlayerTalentFrameTalent2:IsShown())")
                .expect("single-talent set hides second button");
        }
    }
}

#[test]
fn talent_controls_consumer_tracks_supplied_points() {
    let env = load_talent_fixture();
    env.exec("A_Admin.SetWrathUnspentTalentPoints(1, 0)")
        .expect("supply zero points");
    env.exec(
        r#"
        assert(GetUnspentTalentPoints(false, false, 1) == 0)
        A_Admin.SetWrathUnspentTalentPoints(1, 3)
        assert(GetUnspentTalentPoints(false, false, 1) == 3)
        A_Admin.SetWrathUnspentTalentPoints(2, 9)
        assert(GetUnspentTalentPoints(false, false, 2) == 9)
        assert(GetUnspentTalentPoints(false, false, 1) == 3)
        assert(GetUnspentTalentPoints(false, false) == 3)
        assert(GetUnspentTalentPoints(true, false, 1) == 0)
        assert(GetUnspentTalentPoints(false, true, 1) == 0)
        "#,
    )
    .expect("query both supplied talent point values");
}

/// F10: blizzard_player_spells_loads::blizzard_player_spells_panel_root_is_hidden_after_load.
#[test]
fn talent_root_remains_hidden_after_native_addon_load() {
    let env = load_talent_fixture();
    exec_consumer(
        &env,
        "assert(PlayerTalentFrame and not PlayerTalentFrame:IsShown())",
    );
}

/// F10: blizzard_player_spells_loads::blizzard_player_spells_is_addon_loaded_after_explicit_load.
#[test]
fn talent_addon_reports_loaded_after_explicit_native_load() {
    let env = load_talent_fixture();
    exec_consumer(&env, "assert(IsAddOnLoaded('Blizzard_TalentUI'))");
}

/// F10: spellbook::spellbook_spells_visible_on_first_open.
#[test]
fn spellbook_shows_supplied_spell_on_first_native_open() {
    let env = load_spellbook_fixture();
    open_spellbook(&env);
    exec_consumer(
        &env,
        r#"
        assert(SpellButton1:IsVisible())
        local expected = GetSpellName(SpellBook_GetSpellID(SpellButton1:GetID()), BOOKTYPE_SPELL)
        assert(type(expected) == 'string' and expected ~= '')
        assert(SpellButton1SpellName:GetText() == expected)
        "#,
    );
}

/// F10: spellbook::spellbook_icon_textures_in_ancestor_visible.
#[test]
fn first_spell_icon_is_ancestor_visible() {
    let env = load_spellbook_fixture();
    open_spellbook(&env);
    let state = env.state().borrow();
    let icon = state
        .widgets
        .get_id_by_name("SpellButton1IconTexture")
        .unwrap();
    assert!(state.widgets.is_ancestor_visible(icon));
}

/// F10: spellbook_layout::spellbook_spell_items_have_nonzero_rect.
#[test]
fn first_spell_button_has_nonzero_resolved_geometry() {
    let env = load_spellbook_fixture();
    open_spellbook(&env);
    exec_consumer(
        &env,
        r#"
        local left, bottom, width, height = SpellButton1:GetRect()
        assert(left and bottom and width > 0 and height > 0)
        "#,
    );
}

/// F10: test_keybindings_spellbook_talents::spellbook_first_visible_item_icon_matches_spellbook_texture.
#[test]
fn first_spell_icon_matches_native_spellbook_texture() {
    let env = load_spellbook_fixture();
    open_spellbook(&env);
    exec_consumer(
        &env,
        r#"
        local slot = SpellBook_GetSpellID(SpellButton1:GetID())
        local expected = GetSpellTexture(slot, BOOKTYPE_SPELL)
        assert(expected ~= nil)
        assert(SpellButton1IconTexture:GetTexture() == expected)
        "#,
    );
}

/// F10: test_keybindings_spellbook_talents::spellbook_paging_label_is_formatted_on_first_open.
#[test]
fn first_open_formats_native_spellbook_page_label() {
    let env = load_spellbook_fixture();
    open_spellbook(&env);
    exec_consumer(
        &env,
        "assert(SpellBookPageText:GetText() == string.format(PAGE_NUMBER, 1))",
    );
}

/// F10: spellbook_layout::spellbook_hover_shows_spell_tooltip.
#[test]
fn first_spell_hover_populates_native_tooltip() {
    let env = load_spellbook_fixture();
    open_spellbook(&env);
    exec_consumer(
        &env,
        r#"
        SpellButton1:GetScript('OnEnter')(SpellButton1)
        assert(GameTooltip:IsShown() and GameTooltip:NumLines() > 0)
        SpellButton1:GetScript('OnLeave')(SpellButton1)
        assert(not GameTooltip:IsShown())
        "#,
    );
}

/// F10: test_keybindings_spellbook_talents::keybind_s_toggles_spellbook_closed_on_second_press.
/// Shared intent is toggle lifecycle; this tests the original free function, not modern key routing.
#[test]
fn native_spellbook_toggle_closes_on_second_call() {
    let env = load_spellbook_fixture();
    open_spellbook(&env);
    exec_consumer(
        &env,
        "ToggleSpellBook(BOOKTYPE_SPELL); assert(not SpellBookFrame:IsShown())",
    );
}

/// F10: test_keybindings_spellbook_talents::talent_panel_has_at_least_one_visible_talent_node_frame.
#[test]
fn native_talent_panel_has_visible_talent_button() {
    let env = load_talent_fixture();
    exec_consumer(
        &env,
        r#"
        PlayerTalentFrame_Toggle(false, 1)
        assert(PlayerTalentFrame:IsVisible())
        local visible = false
        for index = 1, MAX_NUM_TALENTS do
            local button = _G['PlayerTalentFrameTalent'..index]
            if button and button:IsVisible() then visible = true end
        end
        assert(visible, 'native talent consumer needs a visible modeled talent')
        "#,
    );
}
