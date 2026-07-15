use super::load_game_ui_without_player_choice;

/// Proves the proposed removals are reversed: PTR retains the UI geometry globals.
#[test]
fn ui_geometry_helpers_remain_global() {
    let env = load_game_ui_without_player_choice();

    let (intersect_type, notch_type, offset_type): (String, String, String) = env
        .eval(
            r#"
            return type(UIDoFramesIntersect), type(GetNotchHeight), type(GetUIParentOffset)
            "#,
        )
        .expect("UI geometry publication probe succeeds");

    assert_eq!(intersect_type, "function");
    assert_eq!(notch_type, "function");
    assert_eq!(offset_type, "function");

    let (overlaps, separated, edge_touch): (bool, bool, bool) = env
        .eval(
            r#"
            local function Rect(left, right, bottom, top)
                return {
                    GetLeft = function() return left end,
                    GetRight = function() return right end,
                    GetBottom = function() return bottom end,
                    GetTop = function() return top end,
                }
            end

            local base = Rect(0, 10, 0, 10)
            return UIDoFramesIntersect(base, Rect(5, 15, 5, 15)),
                UIDoFramesIntersect(base, Rect(20, 30, 20, 30)),
                UIDoFramesIntersect(base, Rect(10, 20, 0, 10))
            "#,
        )
        .expect("frame intersection behavior probe succeeds");

    assert!(overlaps);
    assert!(!separated);
    assert!(!edge_touch);

    let (plain_notch, scaled_notch, debug_offset, notch_offset): (f64, f64, f64, f64) = env
        .eval(
            r#"
            local originalShouldAvoidNotch = C_UI.ShouldUIParentAvoidNotch
            local originalSafeRegion = C_UI.GetTopLeftNotchSafeRegion
            local originalPhysicalSize = GetPhysicalScreenSize
            local originalGetSize = UIParent.GetSize
            local originalDebugHeight = DebugBarManager.GetTotalHeight

            C_UI.ShouldUIParentAvoidNotch = function() return false end
            local plainNotch = GetNotchHeight()

            C_UI.ShouldUIParentAvoidNotch = function() return true end
            C_UI.GetTopLeftNotchSafeRegion = function() return 0, 0, 0, 120 end
            GetPhysicalScreenSize = function() return 1920, 1080 end
            UIParent.GetSize = function() return 1024, 720 end
            local scaledNotch = GetNotchHeight()

            DebugBarManager.GetTotalHeight = function() return 100 end
            local debugOffset = GetUIParentOffset()
            DebugBarManager.GetTotalHeight = function() return 40 end
            local notchOffset = GetUIParentOffset()

            C_UI.ShouldUIParentAvoidNotch = originalShouldAvoidNotch
            C_UI.GetTopLeftNotchSafeRegion = originalSafeRegion
            GetPhysicalScreenSize = originalPhysicalSize
            UIParent.GetSize = originalGetSize
            DebugBarManager.GetTotalHeight = originalDebugHeight

            return plainNotch, scaledNotch, debugOffset, notchOffset
            "#,
        )
        .expect("notch and UI parent offset behavior probe succeeds");

    assert_eq!(plain_notch, 0.0);
    assert_eq!(scaled_notch, 80.0);
    assert_eq!(debug_offset, 100.0);
    assert_eq!(notch_offset, 80.0);
}
