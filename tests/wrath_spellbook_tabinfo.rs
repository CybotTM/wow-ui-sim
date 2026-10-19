//! Original Wrath consumers; native producer contracts remain a separate gate.
#![cfg(feature = "client-wrath")]

use std::cell::RefCell;
use std::rc::Rc;

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::font::WowFontSystem;
use wow_ui_sim::screen::ScreenKind;

const SPELLBOOK_SHA256: &str = "af2d753f865f5c04d1332eb4c667b22b7f3dd828696a7d5c4587d3c530c89c32";
const SCROLLING_SHA256: &str = "d975f94a6d7deb8895a8ab47a2b6975cca4fd9e45f67cc15c54c8e100b37427e";

fn assert_original_consumers(toc: &std::path::Path) {
    for (name, digest) in [
        ("SpellBookFrame.lua", SPELLBOOK_SHA256),
        ("UIPanelTemplates.lua", SCROLLING_SHA256),
        (
            "../AddOns/Blizzard_DebugTools/Blizzard_DebugTools.lua",
            "bbacf3103785ff4609e2e55daeb73d23ae87195a27080ae3a9ce1b18e8816af8",
        ),
    ] {
        let path = toc.parent().unwrap().join(name);
        let bytes = std::fs::read(&path).expect("unchanged original consumer");
        assert_eq!(
            format!("{:x}", Sha256::digest(bytes)),
            digest,
            "{}",
            path.display()
        );
    }
}

fn load_original_startup() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Wrath environment");
    env.set_screen_size(1600.0, 1200.0);
    env.set_screen_mode(ScreenKind::Game);
    let fonts = WowFontSystem::new_without_casc();
    assert!(fonts.font_system.db().faces().next().is_some(), "font face");
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().expect("Wrath cache");
    assert_original_consumers(&toc);
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Wrath addons");
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).expect("original FrameXML");
    for addon in
        wow_ui_sim::loader::discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game)
    {
        wow_ui_sim::loader::load_startup_addon(
            &env.loader_env(),
            &addon.toc_path,
            addon.kind,
            None,
        )
        .expect("original startup addon");
    }
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::collect_lua_error_startup(&env);
    assert_no_errors(&env, "stock startup");
    env
}

fn assert_no_errors(env: &WowLuaEnv, boundary: &str) {
    let state = env.state().borrow();
    assert!(
        state.lua_errors.is_empty(),
        "{boundary}: {:?}",
        state.lua_errors
    );
}

#[test]
fn original_wrath_spellbook_show_select_skill_lines_hide_without_errors() {
    let env = load_original_startup();
    let result = env.exec(
        r#"
        SetCVar('ShowAllSpellRanks', '0')
        assert(not GetCVarBool('ShowAllSpellRanks'))
        SpellBookFrame:Hide()
        SpellBookFrame:Show()
        assert(SpellBookFrame:IsShown())
        local tabs = GetNumSpellTabs()
        assert(tabs > 0, 'fixture needs skill lines')
        for index = tabs, 1, -1 do
            SpellBookSkillLineTab_OnClick(_G['SpellBookSkillLineTab'..index])
            assert(SpellBookFrame.selectedSkillLine == index)
            assert(type(SpellBookFrame.selectedSkillLineOffset) == 'number',
                'selectedSkillLineOffset='..tostring(SpellBookFrame.selectedSkillLineOffset))
        end
        SpellBookFrame:Hide()
        assert(not SpellBookFrame:IsShown())
        "#,
    );
    assert_no_errors(&env, "original SpellBook show/select/hide");
    result.expect("original spellbook lifecycle and numeric offset");
}

fn load_original_debugtools(env: &WowLuaEnv) {
    env.exec("assert(C_AddOns.LoadAddOn('Blizzard_DebugTools'))")
        .expect("SETUP: load original demand-loaded DebugTools");
    assert_no_errors(env, "original DebugTools setup");
}

#[test]
fn original_wrath_debugtools_same_text_synchronous_scrolling_has_cursor() {
    let env = load_original_startup();
    load_original_debugtools(&env);
    let result = env.exec(
        r#"
        assert(not ScriptErrorsFrame:IsShown(), 'cold error panel')
        ScriptErrorsFrame_OnError('Wrath spellbook lane cursor fixture')
        local box = ScriptErrorsFrameScrollFrameText
        local text = box:GetText()
        assert(type(text) == 'string' and #text > 0)
        box:SetText(text)
        box:SetText(text)
        ScriptErrorsFrame_Update()
        assert(type(box.cursorOffset) == 'number', 'cursorOffset='..tostring(box.cursorOffset))
        assert(type(box.cursorHeight) == 'number', 'cursorHeight='..tostring(box.cursorHeight))
        ScriptErrorsFrame:Hide()
        "#,
    );
    assert_no_errors(&env, "original DebugTools repeated-text scrolling");
    result.expect("original synchronous scrolling must see cursor presentation");
}
