//! Integration tests for the `C_ArrowCalloutManager` surface registered
//! in `src/c_api/c_arrow_callout_manager.rs`. Drives the live show/hide
//! flow in `Blizzard_ArrowCalloutFrame/ArrowCalloutFrame.lua` and the
//! close-button OnClick path at lua:174 that calls
//! `C_ArrowCalloutManager.AcknowledgeCallout`.

use wow_ui_sim::lua_api::WowLuaEnv;

const SAMPLE_TUTORIAL_INFO: &str = r#"
    return {
        calloutID = 42,
        calloutFrame = "UIParent",
        calloutType = 3, -- Enum.ArrowCalloutType.Tutorial
        calloutDirection = 0, -- Up
        offsetX = 4,
        offsetY = -8,
        calloutText = "Click here to continue.",
    }
"#;

#[test]
fn c_arrow_callout_manager_globals_are_registered() {
    let env = WowLuaEnv::new().expect("env");
    let namespace_kind: String = env.eval("return type(C_ArrowCalloutManager)").unwrap();
    assert_eq!(namespace_kind, "table");

    for fn_name in [
        "ShowCallout",
        "HideCallout",
        "AcknowledgeCallout",
        "IsCalloutActive",
        "IsCalloutAcknowledged",
    ] {
        let kind: String = env
            .eval(&format!("return type(C_ArrowCalloutManager.{fn_name})"))
            .unwrap();
        assert_eq!(kind, "function", "{fn_name} must be a Rust-bound function");
    }
}

#[test]
fn show_callout_populates_active_set_and_returns_true() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(&format!(
        "ARROW_CALLOUT_INFO = (function() {SAMPLE_TUTORIAL_INFO} end)()"
    ))
    .unwrap();

    let returned: bool = env
        .eval("return C_ArrowCalloutManager.ShowCallout(ARROW_CALLOUT_INFO)")
        .unwrap();
    assert!(returned, "ShowCallout should return true on success");

    let active = env.state().borrow().arrow_callouts.active.clone();
    assert_eq!(active.len(), 1);
    let entry = active.get(&42).expect("callout 42 should be active");
    assert_eq!(entry.callout_frame, "UIParent");
    assert_eq!(entry.callout_text, "Click here to continue.");
    assert_eq!(entry.callout_type, 3);
    assert_eq!(entry.callout_direction, 0);
    assert_eq!(entry.offset_x, 4.0);
    assert_eq!(entry.offset_y, -8.0);
    assert_eq!(entry.ui_widget_set_id, None);
}

#[test]
fn show_callout_without_callout_id_returns_false() {
    let env = WowLuaEnv::new().expect("env");
    let returned: bool = env
        .eval(
            "return C_ArrowCalloutManager.ShowCallout({calloutFrame=\"UIParent\", calloutText=\"x\"})",
        )
        .unwrap();
    assert!(!returned, "ShowCallout must reject info without calloutID");
    assert!(env.state().borrow().arrow_callouts.active.is_empty());
}

#[test]
fn show_callout_fires_show_arrow_callout_event_with_payload() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        EVENTS_SEEN = {}
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("SHOW_ARROW_CALLOUT")
        listener:SetScript("OnEvent", function(_, event, payload)
            local id = (type(payload) == "table") and payload.calloutID or payload
            table.insert(EVENTS_SEEN, event .. ":" .. tostring(id))
        end)
        C_ArrowCalloutManager.ShowCallout({
            calloutID = 7,
            calloutFrame = "UIParent",
            calloutType = 3,
            calloutDirection = 1,
            offsetX = 0,
            offsetY = 0,
            calloutText = "hi",
        })
        "#,
    )
    .unwrap();

    let first: String = env.eval("return EVENTS_SEEN[1]").unwrap();
    assert_eq!(first, "SHOW_ARROW_CALLOUT:7");
}

#[test]
fn hide_callout_clears_entry_and_fires_event() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        EVENTS_SEEN = {}
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("HIDE_ARROW_CALLOUT")
        listener:SetScript("OnEvent", function(_, event, calloutID)
            table.insert(EVENTS_SEEN, event .. ":" .. tostring(calloutID))
        end)
        C_ArrowCalloutManager.ShowCallout({calloutID = 11, calloutFrame = "UIParent", calloutText = ""})
        C_ArrowCalloutManager.HideCallout(11)
        "#,
    )
    .unwrap();

    assert!(env.state().borrow().arrow_callouts.active.is_empty());
    let first: String = env.eval("return EVENTS_SEEN[1]").unwrap();
    assert_eq!(first, "HIDE_ARROW_CALLOUT:11");
}

#[test]
fn hide_callout_for_inactive_id_is_a_noop() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        EVENTS_SEEN = {}
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("HIDE_ARROW_CALLOUT")
        listener:SetScript("OnEvent", function(_, event, id)
            table.insert(EVENTS_SEEN, event .. ":" .. tostring(id))
        end)
        C_ArrowCalloutManager.HideCallout(999)
        "#,
    )
    .unwrap();

    let count: f64 = env.eval("return #EVENTS_SEEN").unwrap();
    assert_eq!(
        count, 0.0,
        "HideCallout for inactive id must not fire event"
    );
}

#[test]
fn hide_callout_with_nil_argument_is_a_noop() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("C_ArrowCalloutManager.HideCallout(nil)").unwrap();
    assert!(env.state().borrow().arrow_callouts.active.is_empty());
}

#[test]
fn acknowledge_callout_marks_id_clears_active_and_writes_cvar() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        C_ArrowCalloutManager.ShowCallout({calloutID = 5, calloutFrame = "UIParent", calloutText = ""})
        C_ArrowCalloutManager.ShowCallout({calloutID = 9, calloutFrame = "UIParent", calloutText = ""})
        C_ArrowCalloutManager.AcknowledgeCallout(5)
        "#,
    )
    .unwrap();

    let st = env.state().borrow();
    assert!(st.arrow_callouts.acknowledged.contains(&5));
    assert!(!st.arrow_callouts.active.contains_key(&5));
    assert!(st.arrow_callouts.active.contains_key(&9));
    assert_eq!(
        st.cvars.get("acknowledgedArrowCallouts").as_deref(),
        Some("5")
    );
}

#[test]
fn acknowledged_cvar_is_comma_separated_in_ascending_order() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        C_ArrowCalloutManager.AcknowledgeCallout(7)
        C_ArrowCalloutManager.AcknowledgeCallout(2)
        C_ArrowCalloutManager.AcknowledgeCallout(13)
        "#,
    )
    .unwrap();

    let cvar = env
        .state()
        .borrow()
        .cvars
        .get("acknowledgedArrowCallouts")
        .unwrap_or_default();
    assert_eq!(cvar, "2,7,13");
}

#[test]
fn is_callout_active_reflects_show_and_hide() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        C_ArrowCalloutManager.ShowCallout({calloutID = 21, calloutFrame = "UIParent", calloutText = ""})
        "#,
    )
    .unwrap();

    let active: bool = env
        .eval("return C_ArrowCalloutManager.IsCalloutActive(21)")
        .unwrap();
    assert!(active);

    env.exec("C_ArrowCalloutManager.HideCallout(21)").unwrap();
    let active_after: bool = env
        .eval("return C_ArrowCalloutManager.IsCalloutActive(21)")
        .unwrap();
    assert!(!active_after);

    let other: bool = env
        .eval("return C_ArrowCalloutManager.IsCalloutActive(999)")
        .unwrap();
    assert!(!other);
}

#[test]
fn is_callout_acknowledged_reflects_acknowledge() {
    let env = WowLuaEnv::new().expect("env");
    let before: bool = env
        .eval("return C_ArrowCalloutManager.IsCalloutAcknowledged(33)")
        .unwrap();
    assert!(!before);

    env.exec("C_ArrowCalloutManager.AcknowledgeCallout(33)")
        .unwrap();
    let after: bool = env
        .eval("return C_ArrowCalloutManager.IsCalloutAcknowledged(33)")
        .unwrap();
    assert!(after);
}

#[test]
fn close_button_onclick_path_acknowledges_tutorial_callout() {
    // Mirrors `ArrowCalloutCloseButtonMixin:OnClick` at
    // `Blizzard_ArrowCalloutFrame/ArrowCalloutFrame.lua:174`. The close
    // button looks up `self:GetParent().calloutInfo.calloutID` and
    // calls `C_ArrowCalloutManager.AcknowledgeCallout(id)`.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        local info = {
            calloutID = 100,
            calloutFrame = "UIParent",
            calloutType = 3,
            calloutDirection = 2,
            offsetX = 0,
            offsetY = 0,
            calloutText = "Tutorial step",
        }
        assert(C_ArrowCalloutManager.ShowCallout(info))

        local container = CreateFrame("Frame")
        container.calloutInfo = info
        local closeButton = CreateFrame("Button", nil, container)
        function closeButton:OnClick()
            C_ArrowCalloutManager.AcknowledgeCallout(self:GetParent().calloutInfo.calloutID)
        end
        closeButton:OnClick()
        "#,
    )
    .unwrap();

    let st = env.state().borrow();
    assert!(st.arrow_callouts.acknowledged.contains(&100));
    assert!(!st.arrow_callouts.active.contains_key(&100));
    assert_eq!(
        st.cvars.get("acknowledgedArrowCallouts").as_deref(),
        Some("100")
    );
}
