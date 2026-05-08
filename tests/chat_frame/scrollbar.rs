use super::*;

struct ChatScrollbarSurface {
    chat_x: f64,
    chat_w: f64,
    scroll_x: f64,
    scroll_w: f64,
    edit_w: f64,
    points: String,
}

#[test]
fn chat_scrollbar_stays_attached_to_chat_frame_right_edge() {
    test_timeout! {
        let env = setup_env();
        let layout = chat_layout_debug(&env);
        let surface = read_chat_scrollbar_surface(&env);

        assert_chat_scrollbar_surface(surface, &layout);
    }
}

fn read_chat_scrollbar_surface(env: &WowLuaEnv) -> ChatScrollbarSurface {
    let (chat_x, _chat_y, chat_w, _chat_h): (f64, f64, f64, f64) = env
        .eval("local x, y, w, h = ChatFrame1:GetRect(); return x, y, w, h")
        .expect("ChatFrame1:GetRect failed");
    let (scroll_x, _scroll_y, scroll_w, _scroll_h): (f64, f64, f64, f64) = env
        .eval("local x, y, w, h = ChatFrame1.ScrollBar:GetRect(); return x, y, w, h")
        .expect("ChatFrame1.ScrollBar:GetRect failed");
    let (_edit_x, _edit_y, edit_w, _edit_h): (f64, f64, f64, f64) = env
        .eval("local x, y, w, h = ChatFrame1EditBox:GetRect(); return x, y, w, h")
        .expect("ChatFrame1EditBox:GetRect failed");

    ChatScrollbarSurface {
        chat_x,
        chat_w,
        scroll_x,
        scroll_w,
        edit_w,
        points: read_chat_scrollbar_points(env),
    }
}

fn read_chat_scrollbar_points(env: &WowLuaEnv) -> String {
    env.eval(
        r#"
            local out = {}
            for i = 1, ChatFrame1.ScrollBar:GetNumPoints() do
                local point, rel, relPoint, x, y = ChatFrame1.ScrollBar:GetPoint(i)
                local relName = rel and rel:GetName() or "$parent"
                table.insert(out, string.format("%s->%s:%s(%.0f,%.0f)", point, relName, relPoint, x, y))
            end
            table.sort(out)
            return table.concat(out, " | ")
        "#,
    )
    .expect("ChatFrame1.ScrollBar:GetPoint failed")
}

fn assert_chat_scrollbar_surface(surface: ChatScrollbarSurface, layout: &str) {
    let chat_right = surface.chat_x + surface.chat_w;
    assert!(
        (surface.scroll_x - chat_right).abs() <= 30.0,
        "ChatFrame1.ScrollBar should stay near ChatFrame1 right edge. chat=({:.0}, w={:.0}) scroll=({:.0}, w={:.0}) anchors={}\n{}",
        surface.chat_x,
        surface.chat_w,
        surface.scroll_x,
        surface.scroll_w,
        surface.points,
        layout
    );
    assert!(
        (4.0..=32.0).contains(&surface.scroll_w),
        "ChatFrame1.ScrollBar width should stay sane, got {}. anchors={}\n{}",
        surface.scroll_w,
        surface.points,
        layout
    );
    assert!(
        (350.0..=600.0).contains(&surface.edit_w),
        "ChatFrame1EditBox width should stay sane, got {}. scrollbar anchors={}\n{}",
        surface.edit_w,
        surface.points,
        layout
    );
    assert!(
        !surface.points.contains("TOP->$parent:TOP")
            && !surface.points.contains("BOTTOM->$parent:BOTTOM"),
        "ChatFrame1.ScrollBar should not keep inner Track anchors after startup: {}\n{}",
        surface.points,
        layout
    );
}
