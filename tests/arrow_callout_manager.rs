use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn arrow_callout_manager_tracks_show_hide_lifecycle() {
    let env = env();
    let (shown, active_after_show, active_after_hide, acknowledged): (bool, bool, bool, bool) = env
        .eval(
            r#"
            local calloutInfo = {
                calloutID = 101,
                calloutType = Enum.ArrowCalloutType.Generic,
                calloutDirection = Enum.ArrowCalloutDirection.Up,
                calloutFrame = "UIParent",
                calloutText = "Test",
                offsetX = 0,
                offsetY = 0,
            }
            local shown = C_ArrowCalloutManager.ShowCallout(calloutInfo)
            local activeAfterShow = C_ArrowCalloutManager.IsCalloutActive(101)
            C_ArrowCalloutManager.HideCallout(101)
            local activeAfterHide = C_ArrowCalloutManager.IsCalloutActive(101)
            local acknowledged = C_ArrowCalloutManager.IsCalloutAcknowledged(101)
            return shown, activeAfterShow, activeAfterHide, acknowledged
            "#,
        )
        .unwrap();

    assert!(shown, "ShowCallout should accept valid callout info");
    assert!(
        active_after_show,
        "shown callout should be tracked as active"
    );
    assert!(
        !active_after_hide,
        "HideCallout should clear active callout state"
    );
    assert!(
        !acknowledged,
        "HideCallout alone should not mark callouts acknowledged"
    );
}

#[test]
fn arrow_callout_manager_acknowledge_hides_and_marks() {
    let env = env();
    let (active_before, active_after, acknowledged): (bool, bool, bool) = env
        .eval(
            r#"
            C_ArrowCalloutManager.ShowCallout({
                calloutID = 202,
                calloutType = Enum.ArrowCalloutType.Tutorial,
                calloutDirection = Enum.ArrowCalloutDirection.Right,
                calloutFrame = "UIParent",
                calloutText = "Ack",
                offsetX = 0,
                offsetY = 0,
            })
            local activeBefore = C_ArrowCalloutManager.IsCalloutActive(202)
            C_ArrowCalloutManager.AcknowledgeCallout(202)
            local activeAfter = C_ArrowCalloutManager.IsCalloutActive(202)
            local acknowledged = C_ArrowCalloutManager.IsCalloutAcknowledged(202)
            return activeBefore, activeAfter, acknowledged
            "#,
        )
        .unwrap();

    assert!(
        active_before,
        "callout should be active before acknowledgement"
    );
    assert!(!active_after, "acknowledgement should hide active callout");
    assert!(acknowledged, "acknowledgement should be tracked");
}

#[test]
fn arrow_callout_manager_dispatches_show_hide_events() {
    let env = env();
    let (show_count, hide_count, last_hide_id): (i32, i32, i32) = env
        .eval(
            r#"
            local showCount = 0
            local hideCount = 0
            local lastHideID = 0
            ArrowCalloutFrameManager = {
                OnEvent = function(self, eventName, payload)
                    if eventName == "SHOW_ARROW_CALLOUT" then
                        showCount = showCount + 1
                    elseif eventName == "HIDE_ARROW_CALLOUT" then
                        hideCount = hideCount + 1
                        lastHideID = payload
                    end
                end
            }

            C_ArrowCalloutManager.ShowCallout({
                calloutID = 303,
                calloutType = Enum.ArrowCalloutType.Generic,
                calloutDirection = Enum.ArrowCalloutDirection.Left,
                calloutFrame = "UIParent",
                calloutText = "Dispatch",
                offsetX = 0,
                offsetY = 0,
            })
            C_ArrowCalloutManager.AcknowledgeCallout(303)
            return showCount, hideCount, lastHideID
            "#,
        )
        .unwrap();

    assert_eq!(
        show_count, 1,
        "ShowCallout should dispatch SHOW_ARROW_CALLOUT"
    );
    assert_eq!(
        hide_count, 1,
        "AcknowledgeCallout should dispatch HIDE_ARROW_CALLOUT via HideCallout"
    );
    assert_eq!(last_hide_id, 303, "hide dispatch should carry callout ID");
}
