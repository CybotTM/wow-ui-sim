use super::WowLuaEnv;

/// Pre-event objective tracker setup: hide empty frames and configure the
/// tracker frame container.
///
/// Module registration happens automatically when PLAYER_ENTERING_WORLD and
/// VARIABLES_LOADED fire (via EventUtil.ContinueAfterAllEvents → Init).
/// Quest titles populate via QUEST_LOG_UPDATE fired in startup events.
pub(crate) fn init_objective_tracker(env: &WowLuaEnv) {
    setup_tracker_frame(env);
}

fn setup_tracker_frame(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local otf = ObjectiveTrackerFrame
        if not otf then return end
        -- Ensure layoutIndex is set (should come from XML KeyValue but may need fallback)
        if not otf.layoutIndex then otf.layoutIndex = 50 end
        -- AddManagedFrame checks IsInDefaultPosition() and skips frames not in
        -- default position. Since EditMode isn't initialized, the mixin's
        -- IsInDefaultPosition() returns false. Override so the container accepts it.
        otf.IsInDefaultPosition = function() return true end
        otf:Show()
        -- Explicitly add to the managed frame container. The OnShow handler
        -- may not fire correctly, so call AddManagedFrame directly.
        -- This reparents OTF into the container and calls Layout() to set anchors.
        local lp = otf.layoutParent
        if lp and lp.AddManagedFrame then
            pcall(lp.AddManagedFrame, lp, otf)
        end
        -- Compute height from container height minus OTF's vertical offset.
        -- UpdateHeight() does parentHeight + offsetY, but calling it triggers
        -- layout cycles. Compute it directly instead.
        local _, _, _, _, offsetY = otf:GetPoint(1)
        if offsetY and lp then
            local h = lp:GetHeight() + offsetY
            if h < 100 then h = 400 end
            otf:SetHeight(h)
        end
    "#,
    );
}
