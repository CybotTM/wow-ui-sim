//! Debug helpers for screenshot testing (game menu inspection, etc.).

use crate::lua_api::WowLuaEnv;
use crate::widget::WidgetType;

/// Debug: open game menu via micro button click for screenshot testing.
pub fn debug_show_game_menu(env: &WowLuaEnv) {
    if std::env::var("WOW_SIM_SHOW_GAME_MENU").is_err() {
        return;
    }
    if let Err(e) = env.exec(
        r#"
        local btn = MainMenuMicroButton
        if btn then
            local onclick = btn:GetScript("OnClick")
            if onclick then onclick(btn, "LeftButton", false) end
        end
    "#,
    ) {
        eprintln!("[debug_game_menu] click error: {e}");
    }
    // Check what SetText resolves to for a game menu button
    if let Err(e) = env.exec(r#"if GameMenuFrame and GameMenuFrame.buttonPool then
        for button in GameMenuFrame.buttonPool:EnumerateActive() do
            local text, st = button:GetText() or "(nil)", button.SetText
            io.stderr:write(("[lua_debug] text=%q type(SetText)=%s\n"):format(text, type(st)))
            if type(st) == "function" then
                local info = debug.getinfo(st, "S")
                io.stderr:write(("[lua_debug] SetText source=%s\n"):format(info and info.source or "unknown"))
            end
            break
        end end"#) { eprintln!("[debug_game_menu] lua debug error: {e}"); }
    dump_game_menu_buttons(env);
}

fn dump_game_menu_buttons(env: &WowLuaEnv) {
    let state = env.state().borrow();
    let gmf_id = state.widgets.get_id_by_name("GameMenuFrame");
    eprintln!("[debug] GameMenuFrame id={gmf_id:?}");
    let Some(gmf_id) = gmf_id else { return };
    let Some(gmf) = state.widgets.get(gmf_id) else {
        return;
    };
    log_game_menu_frame(gmf);
    for (i, &cid) in gmf.children.iter().enumerate() {
        log_game_menu_child(&state.widgets, i, cid);
    }
}

fn log_game_menu_frame(gmf: &crate::widget::Frame) {
    eprintln!(
        "  vis={} strata={:?} lvl={} {}x{} children={}",
        gmf.visible,
        gmf.frame_strata,
        gmf.frame_level,
        gmf.width,
        gmf.height,
        gmf.children.len()
    );
}

fn log_game_menu_child(state: &crate::widget::WidgetRegistry, i: usize, cid: u64) {
    let Some(c) = state.get(cid) else {
        return;
    };
    let nm = c.name.as_deref().unwrap_or("(anon)");
    eprintln!(
        "  [{i}] {cid} {nm} [{:?}] {}x{} strata={:?} lvl={} vis={} text={:?}",
        c.widget_type, c.width, c.height, c.frame_strata, c.frame_level, c.visible, c.text
    );
    if c.widget_type == WidgetType::Button {
        log_game_menu_button(state, c);
    }
}

fn log_game_menu_button(state: &crate::widget::WidgetRegistry, button: &crate::widget::Frame) {
    eprintln!(
        "      font={:?} fsz={} color={:?}",
        button.font, button.font_size, button.text_color
    );
    if let Some(&tid) = button.children_keys.get("Text")
        && let Some(tf) = state.get(tid)
    {
        eprintln!(
            "      TextFS {tid}: text={:?} {}x{} vis={} strata={:?} lvl={} draw={:?} anch={}",
            tf.text,
            tf.width,
            tf.height,
            tf.visible,
            tf.frame_strata,
            tf.frame_level,
            tf.draw_layer,
            tf.anchors.len()
        );
    }
}
