//! Bag-button workarounds for simulator gaps.
//!
//! These are narrower shims that keep startup stable but should eventually be
//! replaced by proper addon loading or a more faithful replay of the missing
//! Blizzard logic.

use super::WowLuaEnv;

/// `Blizzard_TokenUI` is an on-demand addon that creates `BackpackTokenFrame`.
/// `ContainerFrameSettingsManager:SetTokenTrackerOwner()` crashes if
/// `self.TokenTracker` is nil. Create a stub frame to avoid the nil index.
///
/// This is a simulator-gap shim. The real fix is to ensure the actual token
/// tracker owner exists through normal addon loading, not to keep a fake frame.
pub fn init_bag_token_tracker(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if ContainerFrameSettingsManager and not ContainerFrameSettingsManager.TokenTracker then
            local f = CreateFrame("Frame", "BackpackTokenFrame", UIParent)
            f.ShouldShow = function() return false end
            f.MarkDirty = function() end
            f.CleanDirty = function() end
            f.SetIsCombinedInventory = function() end
            ContainerFrameSettingsManager.TokenTracker = f
        end
    "#,
    );
}

/// Re-anchor `BagsBar` to `MicroButtonAndBagsBar` once both frames exist.
///
/// This is intentional load-order recovery. The bar should end up in this
/// position; the workaround only reapplies the anchor after the parent exists.
pub fn fix_bags_bar_anchor(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if BagsBar and MicroButtonAndBagsBar then
            BagsBar:ClearAllPoints()
            BagsBar:SetPoint("TOPRIGHT", MicroButtonAndBagsBar, "TOPRIGHT", 0, 0)
        end
    "#,
    );
}

/// Fix ItemContextOverlay showing on bag buttons after startup events.
///
/// `ItemButton`'s `PostOnShow` calls `UpdateItemContextMatching()` which references
/// `ItemButtonUtil` (from `Blizzard_FrameXMLUtil`). But bag buttons load before
/// `Blizzard_FrameXMLUtil`, so `PostOnShow` errors out and `itemContextMatchResult`
/// stays nil. When `PLAYER_ENTERING_WORLD` later triggers `SetMatchesSearch` →
/// `GetItemContextOverlayMode`, `nil ~= DoesNotApply` evaluates to true, showing
/// a black 80% opacity overlay on each bag icon. Re-run `UpdateItemContextMatching`
/// after events so `itemContextMatchResult` is properly set to `DoesNotApply`.
///
/// This is still a simulator-gap shim because it patches the final state instead
/// of replaying the original `PostOnShow` path once `ItemButtonUtil` exists.
pub fn fix_bag_item_context_overlay(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local dna = ItemButtonUtil and ItemButtonUtil.ItemContextMatchResult
            and ItemButtonUtil.ItemContextMatchResult.DoesNotApply
        if not dna then return end
        local function fixBtn(btn)
            if not btn then return end
            btn.itemContextMatchResult = dna
            if btn.UpdateItemContextOverlay then
                pcall(btn.UpdateItemContextOverlay, btn)
            end
        end
        if MainMenuBarBagManager and MainMenuBarBagManager.allBagButtons then
            for _, btn in ipairs(MainMenuBarBagManager.allBagButtons) do
                fixBtn(btn)
            end
        end
        fixBtn(MainMenuBarBackpackButton)
    "#,
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> WowLuaEnv {
        WowLuaEnv::new().expect("Failed to create Lua environment")
    }

    #[test]
    fn bag_bar_anchor_is_only_reapplied_when_both_frames_exist() {
        let env = env();
        env.exec(
            r#"
            BagsBar = {
                clear_calls = 0,
                anchor_target = nil,
                ClearAllPoints = function(self)
                    self.clear_calls = self.clear_calls + 1
                end,
                SetPoint = function(self, _, target)
                    self.anchor_target = target
                end,
            }
            "#,
        )
        .unwrap();

        fix_bags_bar_anchor(&env);
        let clear_calls_without_parent: i32 = env.eval("return BagsBar.clear_calls").unwrap();
        assert_eq!(clear_calls_without_parent, 0);

        env.exec("MicroButtonAndBagsBar = { marker = 'parent' }")
            .unwrap();
        fix_bags_bar_anchor(&env);
        let (clear_calls, anchored_to_parent): (i32, bool) = env
            .eval("return BagsBar.clear_calls, BagsBar.anchor_target == MicroButtonAndBagsBar")
            .unwrap();
        assert_eq!(clear_calls, 1);
        assert!(anchored_to_parent);
    }

    #[test]
    fn token_tracker_stub_installs_only_when_missing() {
        let env = env();
        env.exec(
            r#"
            local existing = { marker = "real" }
            ContainerFrameSettingsManager = { TokenTracker = existing }
            UIParent = {}
            CreateFrame = function()
                error("should not create replacement tracker")
            end
            "#,
        )
        .unwrap();

        init_bag_token_tracker(&env);
        let marker: String = env
            .eval("return ContainerFrameSettingsManager.TokenTracker.marker")
            .unwrap();
        assert_eq!(marker, "real");

        let fresh_env = crate::lua_api::WowLuaEnv::new().expect("Failed to create Lua environment");
        fresh_env
            .exec(
                r#"
            created = 0
            ContainerFrameSettingsManager = {}
            UIParent = {}
            CreateFrame = function(_, name, parent)
                created = created + 1
                return {
                    name = name,
                    parent = parent,
                }
            end
            "#,
            )
            .unwrap();

        init_bag_token_tracker(&fresh_env);
        let (created, is_backpack_token_frame, parent_matches, should_show): (
            i32,
            bool,
            bool,
            bool,
        ) = fresh_env
            .eval(
                r#"
                local tracker = ContainerFrameSettingsManager.TokenTracker
                return created,
                    tracker.name == "BackpackTokenFrame",
                    tracker.parent == UIParent,
                    tracker:ShouldShow()
                "#,
            )
            .unwrap();
        assert_eq!(created, 1);
        assert!(is_backpack_token_frame);
        assert!(parent_matches);
        assert!(!should_show);
    }

    #[test]
    fn item_context_overlay_fix_only_runs_when_match_result_is_available() {
        let env = env();
        env.exec(
            r#"
            MainMenuBarBagManager = {
                allBagButtons = {
                    { touched = false, UpdateItemContextOverlay = function(self) self.touched = true end },
                },
            }
            MainMenuBarBackpackButton = {
                touched = false,
                UpdateItemContextOverlay = function(self) self.touched = true end,
            }
            "#,
        )
        .unwrap();

        fix_bag_item_context_overlay(&env);
        let (bag_touched, backpack_touched): (bool, bool) = env
            .eval(
                r#"
                return MainMenuBarBagManager.allBagButtons[1].touched,
                    MainMenuBarBackpackButton.touched
                "#,
            )
            .unwrap();
        assert!(!bag_touched);
        assert!(!backpack_touched);

        env.exec(
            r#"
            ItemButtonUtil = {
                ItemContextMatchResult = {
                    DoesNotApply = "dna",
                },
            }
            "#,
        )
        .unwrap();
        fix_bag_item_context_overlay(&env);

        let (bag_result, bag_touched, backpack_result, backpack_touched): (
            String,
            bool,
            String,
            bool,
        ) = env
            .eval(
                r#"
                local bag = MainMenuBarBagManager.allBagButtons[1]
                return bag.itemContextMatchResult,
                    bag.touched,
                    MainMenuBarBackpackButton.itemContextMatchResult,
                    MainMenuBarBackpackButton.touched
                "#,
            )
            .unwrap();
        assert_eq!(bag_result, "dna");
        assert!(bag_touched);
        assert_eq!(backpack_result, "dna");
        assert!(backpack_touched);
    }
}
