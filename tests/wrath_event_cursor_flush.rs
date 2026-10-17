//! Original hidden DebugTools consumer across completed event listeners.
#![cfg(feature = "client-wrath")]

use std::cell::RefCell;
use std::rc::Rc;

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::font::{DEFAULT_WOW_FONT, WowFontSystem};

#[test]
fn original_hidden_debugtools_cursor_flushes_between_event_listeners() {
    let fonts = WowFontSystem::new_without_casc();
    assert!(fonts.font_system.db().faces().next().is_some());
    assert!(fonts.family_name(Some(DEFAULT_WOW_FONT)).is_some());
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    env.exec(include_str!("wrath_event_cursor_flush.lua"))
        .expect("cold hidden original DebugTools fixture");

    // No tick, setter flush, seeded fields, or replacement Blizzard handlers.
    env.fire_event("CURSOR_FLUSH_TEST").unwrap();
    env.exec("debug.sethook() ").unwrap();
    let errors = env.state().borrow().lua_errors.clone();
    eprintln!("event-listener errors: {errors:#?}");
    env.exec(
        r#"
        assert(CursorFlushConsumer ~= nil, 'original direct consumer never reached')
        assert(type(CursorFlushConsumer.offset) == 'number',
            'UIPanelTemplates.lua:365 reached with cursorOffset='..tostring(CursorFlushConsumer.offset))
        assert(type(CursorFlushConsumer.height) == 'number' and CursorFlushConsumer.height > 0)
        assert(table.concat(CursorFlushTrace, ',') == 'first,cursor,second,consumer',
            table.concat(CursorFlushTrace, ','))
        assert(CursorFlushConsumer.offset == CursorFlushGeometry.y)
        assert(CursorFlushConsumer.height == CursorFlushGeometry.h)
        assert(not ScriptErrorsFrame:IsShown())
        assert(not ScriptErrorsFrameScrollFrameText:IsVisible())
        assert(ScriptErrorsFrame.index == 1 and #ScriptErrorsFrame.order == 2)
        assert(string.find(ScriptErrorsFrame.messages[1], 'cursor-flush-first', 1, true))
        assert(ScriptErrorsFrameScrollFrameText:GetText() == CursorFlushFirstText)
        assert(__wow_sim_wrath_scrolling_edit_wrapped ~= true)
        "#,
    )
    .unwrap();
    assert!(errors.iter().all(|error| !error.contains("cursorOffset")));
    assert_eq!(errors.len(), 2, "only the two deliberate listener errors");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("cursor-flush-first"))
    );
    assert!(
        errors
            .iter()
            .any(|error| error.contains("cursor-flush-second"))
    );
}
