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

#[test]
fn supplied_spellbook_slot_records_can_be_set() {
    let env = load_spellbook_fixture();
    env.exec(r#"A_Admin.SetWrathSpellBookSlot('spell', 1, 6603, 'Auto Attack', '', 'Interface\\Icons\\INV_Sword_04')"#)
        .expect("supply a Wrath spellbook slot record");
    let state = env.state().borrow();
    let record = state.wrath_spell_slots.get(&("spell".to_string(), 1)).unwrap();
    assert_eq!(record.spell_id, 6603);
    assert_eq!(record.name, "Auto Attack");
    assert_eq!(record.rank, "");
    assert_eq!(record.icon, "Interface\\Icons\\INV_Sword_04");
}

#[test]
fn original_spellbook_reads_supplied_slot_names() {
    let env = load_spellbook_fixture();
    exec_consumer(&env, r#"
        -- Isolate the name consumer from the separate texture producer.
        GetSpellTexture = function(slot, book)
            if book == 'spell' and slot >= 1 and slot <= 3 then
                return 'Interface\\Icons\\INV_Sword_04'
            end
        end
        A_Admin.SetWrathSpellBookSlot('spell', 1, 6603, 'Auto Attack', '', 'Interface\\Icons\\INV_Sword_04')
        A_Admin.SetWrathSpellBookSlot('spell', 2, 635, 'Holy Light', '', 'Interface\\Icons\\Spell_Holy_HolyBolt')
        A_Admin.SetWrathSpellBookSlot('spell', 3, 19750, 'Flash of Light', '', 'Interface\\Icons\\Spell_Holy_FlashHeal')
        SpellBookFrame:Show()
        assert(SpellButton1SpellName:GetText() == 'Auto Attack', 'SpellBookFrame.lua:495 slot 1')
        assert(SpellButton2SpellName:GetText() == 'Holy Light', 'SpellBookFrame.lua:495 slot 2')
        SpellBookFrame:Hide()
        A_Admin.SetWrathSpellBookSlot('spell', 1, 19750, 'Flash of Light', '', 'Interface\\Icons\\Spell_Holy_FlashHeal')
        A_Admin.SetWrathSpellBookSlot('spell', 2, 6603, 'Auto Attack', '', 'Interface\\Icons\\INV_Sword_04')
        SpellBookFrame:Show()
        assert(SpellButton1SpellName:GetText() == 'Flash of Light')
        assert(SpellButton2SpellName:GetText() == 'Auto Attack')
    "#);
}

#[test]
fn original_spellbook_displays_supplied_slot_layouts() {
    let env = load_spellbook_fixture();
    exec_consumer(&env, r#"
        A_Admin.SetWrathSpellBookSlot('spell', 1, 6603, 'Auto Attack', '', 'Interface\\Icons\\INV_Sword_04')
        A_Admin.SetWrathSpellBookSlot('spell', 2, 635, 'Holy Light', '', 'Interface\\Icons\\Spell_Holy_HolyBolt')
        A_Admin.SetWrathSpellBookSlot('spell', 3, 19750, 'Flash of Light', '', 'Interface\\Icons\\Spell_Holy_FlashHeal')
        SpellBookFrame:Show()
        assert(SpellButton1IconTexture:IsVisible(), 'slot 1 icon must be visible at SpellBookFrame.lua:447')
        assert(SpellButton1IconTexture:GetTexture() == 'Interface\\Icons\\INV_Sword_04')
        assert(SpellButton1SpellName:GetText() == 'Auto Attack', 'slot 1 name at SpellBookFrame.lua:495')
        assert(SpellButton2SpellName:GetText() == 'Holy Light', 'slot 2 name at SpellBookFrame.lua:495')
        SpellBookFrame:Hide()
        A_Admin.SetWrathSpellBookSlot('spell', 1, 19750, 'Flash of Light', '', 'Interface\\Icons\\Spell_Holy_FlashHeal')
        A_Admin.SetWrathSpellBookSlot('spell', 2, 6603, 'Auto Attack', '', 'Interface\\Icons\\INV_Sword_04')
        A_Admin.SetWrathSpellBookSlot('spell', 3, 635, 'Holy Light', '', 'Interface\\Icons\\Spell_Holy_HolyBolt')
        SpellBookFrame:Show()
        assert(SpellButton1IconTexture:IsVisible(), 'changed slot 1 icon must be visible')
        assert(SpellButton1IconTexture:GetTexture() == 'Interface\\Icons\\Spell_Holy_FlashHeal')
        assert(SpellButton1SpellName:GetText() == 'Flash of Light')
        assert(SpellButton2SpellName:GetText() == 'Auto Attack')
    "#);
}

#[test]
fn supplied_spellbook_slot_queries_validate_and_return_unresolved_values() {
    let env = load_spellbook_fixture();
    env.exec(r#"
        assert(select('#', GetSpellTexture(1024, 'spell')) == 1)
        assert(GetSpellTexture(1024, 'spell') == nil)
        assert(select('#', GetSpellName(1024, 'spell')) == 2)
        local name, rank = GetSpellName(1024, 'spell')
        assert(name == nil and rank == nil)
        assert(IsSelectedSpell(1, 'spell') == nil)
        A_Admin.SetWrathSpellBookSlot('spell', 1, 6603, 'Auto Attack', '', 'icon', true)
        assert(IsSelectedSpell(1, 'spell') == true)
        A_Admin.SetWrathSpellBookSlot('spell', 1, 6603, 'Auto Attack', '', 'icon', false)
        assert(IsSelectedSpell(1, 'spell') == nil)
    "#).expect("unresolved in-range slots return nil values");
    for expression in [
        "GetSpellTexture(0, 'spell')", "GetSpellTexture(1025, 'spell')",
        "GetSpellTexture(true, 'spell')", "GetSpellName(0, 'spell')",
        "GetSpellName(1025, 'spell')", "GetSpellName({}, 'spell')",
        "GetSpellName(1, nil)",
    ] {
        assert!(env.exec(expression).is_err(), "invalid input must error: {expression}");
    }
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
