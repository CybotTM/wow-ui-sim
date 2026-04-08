use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn managed_frame_mixin_onload_applies_initial_visibility() {
    let env = env();
    let is_shown: bool = env
        .eval(
            r#"
            C_FrameManager.GetFrameVisibilityState = function(frameType)
                return frameType == "alwaysVisible"
            end

            local frame = CreateFrame("Frame", "ManagedFrameVisibilityTest", UIParent)
            frame.frameType = "alwaysVisible"
            Mixin(frame, UIFrameManager_ManagedFrameMixin)
            frame:OnLoad()
            return frame:IsShown()
            "#,
        )
        .unwrap();
    assert!(
        is_shown,
        "managed frame should apply C_FrameManager visibility during OnLoad"
    );
}

#[test]
fn ui_frame_manager_update_frame_event_updates_registered_frames() {
    let env = env();
    let (shown_before, shown_after): (bool, bool) = env
        .eval(
            r#"
            C_FrameManager.GetFrameVisibilityState = function()
                return false
            end

            local frame = CreateFrame("Frame", "ManagedFrameUpdateEventTest", UIParent)
            frame.frameType = "tutorial"
            Mixin(frame, UIFrameManager_ManagedFrameMixin)
            frame:OnLoad()
            local shown_before = frame:IsShown()

            UIFrameManager:OnEvent("FRAME_MANAGER_UPDATE_FRAME", "tutorial", true)
            return shown_before, frame:IsShown()
            "#,
        )
        .unwrap();
    assert!(
        !shown_before,
        "frame should start hidden when C_FrameManager says false"
    );
    assert!(
        shown_after,
        "FRAME_MANAGER_UPDATE_FRAME should propagate visibility to registered frames"
    );
}

#[test]
fn ui_frame_manager_registers_each_frame_only_once() {
    let env = env();
    let (registered_count, update_calls): (i32, i32) = env
        .eval(
            r#"
            local updateCalls = 0
            C_FrameManager.GetFrameVisibilityState = function()
                return false
            end

            local frame = CreateFrame("Frame", "ManagedFrameDuplicateRegistrationTest", UIParent)
            frame.frameType = "duplicateTest"
            Mixin(frame, UIFrameManager_ManagedFrameMixin)
            frame.UpdateFrameState = function(self, show)
                updateCalls = updateCalls + 1
                self:SetShown(show)
            end

            frame:OnLoad()
            frame:OnLoad()

            local registeredCount = 0
            for _ in pairs(UIFrameManager.registeredFrameTypeToFrames.duplicateTest) do
                registeredCount = registeredCount + 1
            end

            return registeredCount, updateCalls
            "#,
        )
        .unwrap();
    assert_eq!(
        registered_count, 1,
        "a managed frame should only be registered once for its frame type"
    );
    assert_eq!(
        update_calls, 1,
        "duplicate registration should not re-apply initial frame state"
    );
}
