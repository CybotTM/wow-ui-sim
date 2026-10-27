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

#[test]
fn original_wrath_send_mail_selects_supplied_stationery() {
    let env = crate::common::new_headless_cursor_env();
    env.set_screen_size(1024.0, 768.0);
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("SETUP FAILURE: Wrath FrameXML cache");
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
        .expect("SETUP FAILURE: load native FrameXML");
    env.exec(r#"
        A_Admin.SetStationeryCatalog({
            { name = 'Scarlet', texture = 'Scarlet' },
            { name = 'Azure', texture = 'Azure' },
        })
    "#).unwrap();
    let previous_errors = env.state().borrow().lua_errors.len();
    env.exec("SendMailFrame:Show()").expect("show original send mail frame");
    let errors = env.state().borrow().lua_errors[previous_errors..].to_vec();
    assert!(errors.is_empty(), "original send-mail OnShow failed: {errors:?}");
    env.exec(r#"
        assert(GetSelectedStationeryTexture() == 'Scarlet')
        StationeryPopupButton_OnClick(nil, 2)
        assert(GetSelectedStationeryTexture() == 'Azure')
        assert(StationeryPopupFrame.selectedIndex == 2)
        assert(SendStationeryBackgroundLeft:GetTexture() == STATIONERY_PATH..'Azure1')
        StationeryPopupButton_OnClick(nil, 1)
        assert(GetSelectedStationeryTexture() == 'Scarlet')
        assert(SendStationeryBackgroundLeft:GetTexture() == STATIONERY_PATH..'Scarlet1')
    "#).expect("original consumer selects both catalog entries");
}

#[test]
fn original_wrath_stationery_popup_renders_supplied_catalog() {
    let env = crate::common::new_headless_cursor_env();
    env.set_screen_size(1024.0, 768.0);
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("SETUP FAILURE: Wrath FrameXML cache");
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
        .expect("SETUP FAILURE: load native FrameXML");
    let previous_errors = env.state().borrow().lua_errors.len();
    env.exec("StationeryPopupFrame:Show()")
        .expect("open original stationery popup");
    let errors = env.state().borrow().lua_errors[previous_errors..].to_vec();
    assert!(
        errors.is_empty(),
        "original popup OnShow failed: {errors:?}"
    );
    env.exec(
        r#"
        A_Admin.SetMoney(100)
        A_Admin.SetStationeryCatalog({
            { name = 'Scarlet', texture = 'Interface\\Icons\\INV_Misc_Note_01', cost = 250 },
            { name = 'Azure', texture = 'Interface\\Icons\\INV_Misc_Note_02' },
        })
        StationeryPopupFrame:Hide()
        StationeryPopupFrame:Show()
        assert(StationeryPopupButton1Name:GetText() == 'Scarlet')
        assert(StationeryPopupButton2Name:GetText() == 'Azure')
        assert(StationeryPopupButton1Icon:GetTexture() == 'Interface\\Icons\\INV_Misc_Note_01')
        assert(StationeryPopupButton2Icon:GetTexture() == 'Interface\\Icons\\INV_Misc_Note_02')
        assert(StationeryPopupButton1MoneyFrame.staticMoney == 250)
        assert(StationeryPopupButton2MoneyFrame.staticMoney == 0)
        assert(not StationeryPopupButton1:IsEnabled())
        assert(StationeryPopupButton2:IsEnabled())
        local red = StationeryPopupButton1MoneyFrame.small and NumberFontNormalRightRed or NumberFontNormalLargeRightRed
        assert(StationeryPopupButton1MoneyFrameCopperButton:GetNormalFontObject() == red)
        assert(StationeryPopupButton1:IsShown() and StationeryPopupButton2:IsShown())
        assert(not StationeryPopupButton3:IsShown())
        "#,
    )
    .expect("original popup renders the supplied catalog");
    let errors = env.state().borrow().lua_errors[previous_errors..].to_vec();
    assert!(
        errors.is_empty(),
        "original popup update failed: {errors:?}"
    );
}

/// Append to tests/wrath_scrolling_cursor.rs; producer is intentionally separate.
#[test]
fn active_nested_load_addon_xml_cursor_precedes_original_synchronous_consumer() {
    let fonts = WowFontSystem::new_without_casc();
    assert!(
        fonts.font_system.db().faces().next().is_some(),
        "SETUP FAILURE: font face"
    );
    assert!(
        fonts.family_name(Some(DEFAULT_WOW_FONT)).is_some(),
        "SETUP FAILURE: font alias"
    );
    let env = WowLuaEnv::new().expect("SETUP FAILURE: Wrath environment");
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    env.exec("C2Errors = {}; seterrorhandler(function(message) C2Errors[#C2Errors + 1] = tostring(message) end); C2NestedTrace = {}")
        .unwrap();
    env.loader_env()
        .exec(&read_original_wrath_consumer())
        .expect("SETUP FAILURE: full unchanged pinned UIPanelTemplates.lua");
    assert_callback_errors_empty(&env);

    let root = tempfile::tempdir().unwrap();
    for (name, file, contents) in [
        (
            "C2NestedOuter",
            "Outer.xml",
            include_str!("wrath_scrolling_cursor/nested-outer.xml"),
        ),
        (
            "C2NestedInner",
            "Inner.xml",
            include_str!("wrath_scrolling_cursor/nested-scroll.xml"),
        ),
        (
            "C2NestedProbe",
            "Probe.lua",
            include_str!("wrath_scrolling_cursor/nested-probe.lua"),
        ),
    ] {
        let directory = root.path().join(name);
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(
            directory.join(format!("{name}.toc")),
            format!("## Interface: 30403\n## Title: {name}\n## LoadOnDemand: 1\n{file}\n"),
        )
        .unwrap();
        std::fs::write(directory.join(file), contents).unwrap();
    }
    env.state().borrow_mut().addon_base_paths = vec![root.path().to_path_buf()];
    // Real Lua -> runtime LoadAddOn -> XML OnLoad -> nested LoadAddOn. No tick.
    exec_and_audit(&env, "assert(C_AddOns.LoadAddOn('C2NestedOuter'))");
    exec_and_audit(
        &env,
        r#"
        assert(table.concat(C2NestedTrace, ',') ==
            'outer-enter,box-load,parent-load,cursor,probe,changed-enter,changed-return,sibling-enter,sibling-return,outer-return',
            table.concat(C2NestedTrace, ','))
        assert(C2NestedCursorCount == 1 and C2NestedChangedCount == 1)
        assert(C2NestedBox:GetCursorPosition() == 7)
        assert(C2NestedBox:GetUTF8CursorPosition() == 4)
        assert(C2NestedScroll:GetVerticalScroll() > 0)
        assert(C2NestedScroll:GetVerticalScroll() <= C2NestedScroll:GetVerticalScrollRange())
        assert(C2NestedBox.handleCursorChange == false)
        assert(__wow_sim_wrath_scrolling_edit_wrapped ~= true, 'guard must not mask boundary')
    "#,
    );
}

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
fn original_mail_body_first_text_delivers_cursor_and_scrolls() {
    let env = crate::common::new_headless_cursor_env();
    env.set_screen_size(1024.0, 768.0);
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("SETUP FAILURE: Wrath FrameXML cache");
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
        .expect("SETUP FAILURE: load native FrameXML");
    env.exec(
        r#"
        assert(SendMailBodyEditBox and SendMailBodyEditBox:GetScript('OnTextChanged'))
        assert(SendMailBodyEditBox:GetScript('OnCursorChanged'))
        local box = SendMailBodyEditBox
        local scroll = box:GetParent():GetParent()
        scroll:SetHeight(30)
        box:SetHeight(300)
        scroll:SetVerticalScroll(0)
        MailCursorEvents = 0
        box:HookScript('OnCursorChanged', function() MailCursorEvents = MailCursorEvents + 1 end)
        assert(box.cursorOffset == nil and box.cursorHeight == nil)
        local text = string.rep('line\n', 12) .. 'end'
        box:SetText(text)
        box:SetCursorPosition(#text)
        MailFrame:Show()
        SendMailFrame:Show()
        "#,
    )
    .expect("SETUP FAILURE: original mail body first text");
    env.fire_on_update(TICK_SECONDS).unwrap();
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        !errors.iter().any(|error| error.contains("cursorOffset")),
        "original mail consumer cursor error: {errors:?}"
    );
    env.exec(
        r#"
        local box = SendMailBodyEditBox
        local scroll = box:GetParent():GetParent()
        assert(MailCursorEvents > 0)
        assert(type(box.cursorOffset) == 'number' and type(box.cursorHeight) == 'number')
        assert(box.cursorHeight > 0 and box.cursorOffset < 0)
        assert(box.handleCursorChange == false)
        assert(scroll:GetVerticalScroll() > 0)
        assert(scroll:GetVerticalScroll() <= scroll:GetVerticalScrollRange())
        "#,
    )
    .expect("original mail consumer must receive cursor geometry and scroll");
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
