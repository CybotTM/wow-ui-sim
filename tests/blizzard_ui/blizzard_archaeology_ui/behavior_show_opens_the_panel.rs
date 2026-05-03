//! Show/hide routing behavior for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ArchaeologyUI";

#[test]
fn archaeology_frame_show_and_hide_route_through_ui_panel_system() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let result: ShowHideResult = env
            .eval(
                r#"
                local originalShowUIPanel = ShowUIPanel
                local originalHideUIPanel = HideUIPanel
                local transitions = {}

                ShowUIPanel = function(frame, ...)
                    table.insert(transitions, "show:" .. (frame and frame:GetName() or "nil"))
                    return originalShowUIPanel(frame, ...)
                end
                HideUIPanel = function(frame, ...)
                    table.insert(transitions, "hide:" .. (frame and frame:GetName() or "nil"))
                    return originalHideUIPanel(frame, ...)
                end

                ArchaeologyFrame:Hide()
                ArchaeologyFrame_Show()
                local shownAfterShow = ArchaeologyFrame:IsShown()
                ArchaeologyFrame_Hide()
                local shownAfterHide = ArchaeologyFrame:IsShown()

                ShowUIPanel = originalShowUIPanel
                HideUIPanel = originalHideUIPanel

                return table.concat(transitions, "|"), shownAfterShow, shownAfterHide
                "#,
            )
            .expect("ArchaeologyFrame show/hide routing probe must run cleanly");

        assert_show_hide_result(result);
    });
}

type ShowHideResult = (String, bool, bool);

fn assert_show_hide_result(result: ShowHideResult) {
    let (transitions, shown_after_show, shown_after_hide) = result;

    assert_eq!(
        transitions, "show:ArchaeologyFrame|hide:ArchaeologyFrame",
        "`ArchaeologyFrame_Show` and `ArchaeologyFrame_Hide` must route through the UI panel system"
    );
    assert!(
        shown_after_show,
        "`ArchaeologyFrame_Show` must leave `ArchaeologyFrame` shown"
    );
    assert!(
        !shown_after_hide,
        "`ArchaeologyFrame_Hide` must leave `ArchaeologyFrame` hidden"
    );
}
