//! Show-failed close handling for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ArchaeologyUI";

#[test]
fn archaeology_show_failed_calls_close_research_once() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let close_call_count: i32 = env
            .eval(
                r#"
                local originalCloseResearch = CloseResearch
                local closeCallCount = 0
                CloseResearch = function(...)
                    closeCallCount = closeCallCount + 1
                    return originalCloseResearch(...)
                end

                ArchaeologyFrame_ShowFailed(ArchaeologyFrame)
                CloseResearch = originalCloseResearch

                return closeCallCount
                "#,
            )
            .expect("ArchaeologyFrame_ShowFailed close probe must run cleanly");

        assert_eq!(
            close_call_count, 1,
            "`ArchaeologyFrame_ShowFailed` must dispatch `CloseResearch()` exactly once"
        );

        let state = env.state();
        let sim = state.borrow();
        assert!(
            sim.archaeology.last_close_request.is_some(),
            "`CloseResearch()` must bump `state.archaeology.last_close_request`"
        );
    });
}
