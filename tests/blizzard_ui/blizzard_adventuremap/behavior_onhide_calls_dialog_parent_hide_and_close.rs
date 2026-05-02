//! `AdventureMapMixin:OnHide` dialog-abort and close behavior.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const EXPECTED_CALL_ORDER: &str = "parent_hide,close";

#[test]
fn adventure_map_onhide_aborts_dialog_then_closes_adventure_map() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.state().borrow_mut().adventure_map.last_closed = None;

        let surface: OnHideSurface = env
            .eval(
                r#"
                local originalMapCanvasOnHide = MapCanvasMixin.OnHide
                local originalOnParentHide = AdventureMapQuestChoiceDialog.OnParentHide
                local originalClose = C_AdventureMap.Close

                local calls = {}
                local parentHideCount = 0
                local closeCount = 0
                local parentMatches = false

                MapCanvasMixin.OnHide = function() end
                AdventureMapQuestChoiceDialog:SetParent(AdventureMapFrame)
                AdventureMapQuestChoiceDialog.result = nil

                AdventureMapQuestChoiceDialog.OnParentHide = function(self, parent)
                    parentHideCount = parentHideCount + 1
                    parentMatches = parent == AdventureMapFrame
                    table.insert(calls, "parent_hide")
                    return originalOnParentHide(self, parent)
                end

                C_AdventureMap.Close = function()
                    closeCount = closeCount + 1
                    table.insert(calls, "close")
                    return originalClose()
                end

                AdventureMapMixin.OnHide(AdventureMapFrame)

                MapCanvasMixin.OnHide = originalMapCanvasOnHide
                AdventureMapQuestChoiceDialog.OnParentHide = originalOnParentHide
                C_AdventureMap.Close = originalClose

                return table.concat(calls, ","),
                       parentHideCount,
                       closeCount,
                       parentMatches,
                       AdventureMapQuestChoiceDialog.result
                "#,
            )
            .expect("AdventureMap OnHide dialog-close probe must run cleanly");

        assert_onhide_surface(surface);

        assert!(
            env.state().borrow().adventure_map.last_closed.is_some(),
            "`AdventureMapMixin:OnHide` must call the simulator-backed `C_AdventureMap.Close`"
        );
    });
}

type OnHideSurface = (String, i64, i64, bool, i64);

fn assert_onhide_surface(surface: OnHideSurface) {
    let (call_order, parent_hide_count, close_count, parent_matches, dialog_result) = surface;

    assert_eq!(
        call_order, EXPECTED_CALL_ORDER,
        "`AdventureMapMixin:OnHide` must abort the dialog before closing the adventure map"
    );
    assert_eq!(
        parent_hide_count, 1,
        "`AdventureMapMixin:OnHide` must call `AdventureMapQuestChoiceDialog:OnParentHide` once"
    );
    assert_eq!(
        close_count, 1,
        "`AdventureMapMixin:OnHide` must call `C_AdventureMap.Close` once"
    );
    assert!(
        parent_matches,
        "`AdventureMapMixin:OnHide` must pass itself to `OnParentHide`"
    );
    assert_eq!(
        dialog_result, 3,
        "`AdventureMapQuestChoiceDialog:OnParentHide` must decline-abort the dialog"
    );
}
