//! Bag-button workarounds for simulator gaps.
//!
//! These are narrower shims that keep startup stable but should eventually be
//! replaced by proper addon loading or a more faithful replay of the missing
//! Blizzard logic.

use super::WowLuaEnv;

/// `Blizzard_TokenUI` is an on-demand addon that creates `BackpackTokenFrame`.
/// `ContainerFrameSettingsManager:SetTokenTrackerOwner()` crashes if
/// `self.TokenTracker` is nil. Try to demand-load the real addon first and
/// only fall back to a stub frame if that still leaves no token tracker.
///
/// Some focused harnesses intentionally do not preload `Blizzard_TokenUI`, but
/// they still populate `addon_base_paths`, so runtime `LoadAddOn` can recover
/// the real `BackpackTokenFrame`. The stub remains as a last resort for unit
/// tests or minimal envs where on-demand addon loading is unavailable.
pub fn init_bag_token_tracker(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if ContainerFrameSettingsManager and not ContainerFrameSettingsManager.TokenTracker then
            if not BackpackTokenFrame and LoadAddOn then
                pcall(LoadAddOn, "Blizzard_TokenUI")
            end
            if BackpackTokenFrame then
                ContainerFrameSettingsManager.TokenTracker = BackpackTokenFrame
            else
                local f = CreateFrame("Frame", "BackpackTokenFrame", UIParent)
                f.ShouldShow = function() return false end
                f.MarkDirty = function() end
                f.CleanDirty = function() end
                f.SetIsCombinedInventory = function() end
                ContainerFrameSettingsManager.TokenTracker = f
            end
        end
    "#,
    );
}
