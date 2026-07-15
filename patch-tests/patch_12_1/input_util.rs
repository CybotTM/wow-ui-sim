use super::load_game_ui_without_player_choice;

/// Proves the cursor helpers remain globals rather than InputUtil methods.
#[test]
fn cursor_helpers_remain_global() {
    let env = load_game_ui_without_player_choice();

    let (
        cursor_on_update,
        cursor_update,
        scaled_delta,
        mouse_is_over,
        show_inspect,
        namespaced_on_update,
        namespaced_update,
        namespaced_delta,
        namespaced_mouse_over,
        namespaced_inspect,
    ): (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            return type(CursorOnUpdate),
                type(CursorUpdate),
                type(GetScaledCursorDelta),
                type(MouseIsOver),
                type(ShowInspectCursor),
                type(InputUtil.CursorOnUpdate),
                type(InputUtil.CursorUpdate),
                type(InputUtil.GetCursorDelta),
                type(InputUtil.IsMouseOver),
                type(InputUtil.ShowInspectCursor)
            "#,
        )
        .expect("input utility publication probe succeeds");

    assert_eq!(cursor_on_update, "function");
    assert_eq!(cursor_update, "function");
    assert_eq!(scaled_delta, "function");
    assert_eq!(mouse_is_over, "function");
    assert_eq!(show_inspect, "function");
    assert_eq!(namespaced_on_update, "nil");
    assert_eq!(namespaced_update, "nil");
    assert_eq!(namespaced_delta, "nil");
    assert_eq!(namespaced_mouse_over, "nil");
    assert_eq!(namespaced_inspect, "nil");

    let (frame_x, frame_y, parent_x, parent_y, delta_x, delta_y): (f64, f64, f64, f64, f64, f64) =
        env.eval(
            r#"
            GetCursorPosition = function() return 100, 200 end
            GetCursorDelta = function() return 10, 20 end
            UIParent:SetScale(2)
            local frameX, frameY = GetScaledCursorPositionForFrame(UIParent)
            local parentX, parentY = GetScaledCursorPosition()
            local deltaX, deltaY = GetScaledCursorDelta()
            return frameX, frameY, parentX, parentY, deltaX, deltaY
            "#,
        )
        .expect("scaled cursor behavior probe succeeds");

    assert_eq!((frame_x, frame_y), (50.0, 100.0));
    assert_eq!((parent_x, parent_y), (50.0, 100.0));
    assert_eq!((delta_x, delta_y), (5.0, 10.0));

    let (mouse_result, offsets, cursor): (bool, String, String) = env
        .eval(
            r#"
            local region = CreateFrame("Frame")
            region.IsMouseOver = function(_, top, bottom, left, right)
                return top == 1 and bottom == 2 and left == 3 and right == 4
            end
            local mouseResult = MouseIsOver(region, 1, 2, 3, 4)

            local cursor
            SetCursor = function(value) cursor = value end
            ShowInspectCursor()
            return mouseResult, "1,2,3,4", cursor
            "#,
        )
        .expect("cursor wrapper behavior probe succeeds");

    assert!(mouse_result);
    assert_eq!(offsets, "1,2,3,4");
    assert_eq!(cursor, "INSPECT_CURSOR");
}
