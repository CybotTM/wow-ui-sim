//! Original Wrath consumer boundary; simulator geometry policy, not native proof.
#![cfg(feature = "client-wrath")]

use std::cell::RefCell;
use std::rc::Rc;

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::font::{DEFAULT_WOW_FONT, WowFontSystem};
use wow_ui_sim::xml::{XmlElement, parse_xml, register_template};

const PINNED_SHA256: &str = "d975f94a6d7deb8895a8ab47a2b6975cca4fd9e45f67cc15c54c8e100b37427e";
const TICK_SECONDS: f64 = 0.016;

fn read_original_wrath_consumer() -> String {
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("SETUP FAILURE: populate the pinned Wrath runtime cache first");
    let path = toc
        .parent()
        .expect("FrameXML TOC parent")
        .join("UIPanelTemplates.lua");
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("SETUP FAILURE: read {}: {error}", path.display()));
    let digest = format!("{:x}", Sha256::digest(&bytes));
    assert_eq!(
        digest,
        PINNED_SHA256,
        "SETUP FAILURE: original Wrath bytes changed: {}",
        path.display()
    );
    eprintln!("Wrath consumer source={} sha256={digest}", path.display());
    String::from_utf8(bytes).expect("SETUP FAILURE: original Wrath source UTF-8")
}

fn assert_callback_errors_empty(env: &WowLuaEnv) {
    env.exec("assert(#C2Errors == 0, table.concat(C2Errors, '\\n'))")
        .expect("callback assertions must not be swallowed by script dispatch");
    let state = env.state().borrow();
    assert!(state.lua_errors.is_empty(), "{:?}", state.lua_errors);
}

fn exec_and_audit(env: &WowLuaEnv, source: &str) {
    let result = env.exec(source);
    assert_callback_errors_empty(env);
    result.unwrap();
}

fn tick_and_audit(env: &WowLuaEnv) {
    let result = env.fire_on_update(TICK_SECONDS);
    assert_callback_errors_empty(env);
    result.unwrap();
}

fn create_original_wrath_consumer_env() -> WowLuaEnv {
    let fonts = WowFontSystem::new_without_casc();
    assert!(
        fonts.font_system.db().faces().next().is_some(),
        "SETUP FAILURE: installed font face required"
    );
    assert!(
        fonts.family_name(Some(DEFAULT_WOW_FONT)).is_some(),
        "SETUP FAILURE: default font alias required"
    );
    let source = read_original_wrath_consumer();
    let env = WowLuaEnv::new().expect("SETUP FAILURE: Wrath environment");
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    env.exec(
        "C2Errors = {}; seterrorhandler(function(message)          C2Errors[#C2Errors + 1] = tostring(message) end)",
    )
    .unwrap();
    // Complete original file, unchanged. This public loader API compiles directly;
    // no extraction, source-patch pipeline, post-load guard or TOC startup here.
    env.loader_env()
        .exec(&source)
        .expect("SETUP FAILURE: complete original UIPanelTemplates.lua");
    assert_callback_errors_empty(&env);
    let ui = parse_xml(include_str!("wrath_scrolling_cursor/consumer.xml")).unwrap();
    let XmlElement::EditBox(template) = &ui.elements[0] else {
        panic!("SETUP FAILURE: EditBox template");
    };
    register_template(
        "C2WrathScrollingConsumerTemplate",
        "EditBox",
        template.clone(),
    );
    exec_and_audit(&env, include_str!("wrath_scrolling_cursor/fixture.lua"));
    env
}

#[test]
fn original_wrath_startup_error_panel_has_cursor_before_direct_scrolling_update() {
    let env = WowLuaEnv::new().expect("SETUP FAILURE: Wrath environment");
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(Rc::new(RefCell::new(WowFontSystem::new_without_casc())));
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("SETUP FAILURE: populate the pinned Wrath runtime cache first");

    // Actual FrameXML loading also loads the original DebugTools error consumer.
    // No tick, replacement consumer, post-load guard or seeded cursor fields.
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
        .expect("SETUP FAILURE: load pinned Wrath FrameXML");

    let cursor_errors = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| message.contains("cursorOffset"))
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        cursor_errors.is_empty(),
        "original startup error panel reached scrolling before cursor presentation: {cursor_errors:?}"
    );
}

#[test]
fn original_wrath_settext_delivers_cursor_before_synchronous_changed_consumer() {
    let env = create_original_wrath_consumer_env();
    exec_and_audit(
        &env,
        r#"
        C2Expect(7, 4, 5, -51, 48, false)
        C2Box:SetText('é\n猫\nz')
        C2Box:SetCursorPosition(7)
        C2AssertColdPending(7, 4)
        "#,
    );
    tick_and_audit(&env);
    exec_and_audit(&env, "C2AssertDelivered(true)");
}

#[test]
fn original_wrath_insert_delivers_cursor_before_synchronous_changed_consumer() {
    let env = create_original_wrath_consumer_env();
    exec_and_audit(
        &env,
        r#"
        C2Expect(8, 5, 5 + C2ZWidth:GetStringWidth(), -51, 48, true)
        C2Box:Insert('é\n猫\nz')
        C2AssertColdPending(8, 5)
        "#,
    );
    tick_and_audit(&env);
    exec_and_audit(&env, "C2AssertDelivered(true)");
}

#[test]
fn original_wrath_keyboard_crosses_newline_and_unicode_without_text_notification() {
    let env = create_original_wrath_consumer_env();
    exec_and_audit(
        &env,
        r#"
        C2Expect(8, 5, 5 + C2ZWidth:GetStringWidth(), -51, 48, true)
        C2Box:Insert('é\n猫\nz')
        C2Box:SetFocus()
        C2AssertColdPending(8, 5)
        "#,
    );
    tick_and_audit(&env);
    exec_and_audit(&env, "C2AssertDelivered(true)");
    // End of z -> third-line start -> previous-line end -> before multibyte 猫.
    for expectation in [
        "C2Expect(7, 4, 5, -51, 48, nil)",
        "C2Expect(6, 3, 5 + C2CatWidth:GetStringWidth(), -27, 24, nil)",
        "C2Expect(3, 2, 5, -27, 24, nil)",
    ] {
        exec_and_audit(&env, expectation);
        let result = env.send_key_press("LEFT", None);
        assert_callback_errors_empty(&env);
        result.unwrap();
        exec_and_audit(&env, "C2AssertPending()");
        tick_and_audit(&env);
        exec_and_audit(&env, "C2AssertDelivered(false)");
    }
}
