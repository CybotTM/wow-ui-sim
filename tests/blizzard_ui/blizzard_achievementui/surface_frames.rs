//! Frame-shape surface pins for the `Blizzard_AchievementUI` lane.
//!
//! PLAN.md task: pin that `AchievementFrame` exists, has `frameStrata` of
//! `MEDIUM`, has parent `UIParent`, and is hidden by default. All four
//! facts come from a single XML declaration at
//! `Mainline/Blizzard_AchievementUI.xml:1505`:
//!
//! ```xml
//! <Frame name="AchievementFrame" toplevel="true" parent="UIParent"
//!        frameStrata="MEDIUM" hidden="true" enableMouse="true"
//!        inherits="BackdropTemplate">
//! ```
//!
//! Each fact has its own assertion so a regression touches the smallest
//! possible test surface. The four together pin the panel's identity in
//! the WoW window manager:
//!
//! - **Existence as a global table.** XML `name="AchievementFrame"`
//!   registers the frame in `_G` at XML-load time. Without this the
//!   `TOGGLEACHIEVEMENT` keybind handler at `Blizzard_AchievementUI.lua:195`
//!   (`AchievementFrame_ToggleAchievementFrame` — pinned in
//!   `surface_globals.rs`) would surface a nil-table-method error on
//!   `ShowUIPanel(AchievementFrame)`.
//!
//! - **`frameStrata == "MEDIUM"`.** This is the default UIPanel stratum.
//!   The achievement panel deliberately renders at the same level as
//!   character / spellbook / inventory frames so the standard UIPanel
//!   layout system (`UIPanelWindows["AchievementFrame"]`, registered at
//!   `Blizzard_AchievementUI.lua:151`) can manage its anchor and the
//!   panel-stack push/pop without crossing strata boundaries. A regression
//!   to `LOW` would push it below world chrome; a regression to `HIGH`
//!   would let it cover dialogs.
//!
//! - **`parent == "UIParent"`.** The standard UI root. `parent="UIParent"`
//!   on the XML keeps the frame inside the user-scaled UI (UIParent is
//!   what `SetUIScale` and the resolution-aware reparenting drive against),
//!   not the world frame which renders 3D content. A regression that
//!   reparents this onto `WorldFrame` or some intermediate would break
//!   user-set UI scaling and detach the panel from `UIParent.Hide()`-style
//!   global toggles.
//!
//! - **Hidden by default.** XML `hidden="true"` makes the frame hidden
//!   on creation; `ToggleAchievementFrame` flips it visible via
//!   `ShowUIPanel`. A regression dropping `hidden="true"` would put the
//!   panel on screen at game start, blocking the player's view.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const FRAME_NAME: &str = "AchievementFrame";
const XML_SITE: &str = "Mainline/Blizzard_AchievementUI.xml:1505";

#[test]
fn achievement_frame_publishes_expected_panel_identity() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let frame_type: String = env
            .eval(&format!("return type(_G[{FRAME_NAME:?}])"))
            .expect("AchievementFrame global probe must run cleanly");

        assert_eq!(
            frame_type, "table",
            "Expected `_G[{FRAME_NAME:?}]` to be a table after `{ROOT}` loads, got \
             `{frame_type}`. The frame is declared at `{XML_SITE}` with \
             `name=\"AchievementFrame\"` and `parent=\"UIParent\"`, so the named-frame \
             registration runs at XML load time. A nil reading means either the XML did not \
             execute (a regression in the load pipeline) or the frame failed to register its \
             name (a regression in the named-frame routing inside `CreateFrame`). Either way, \
             every downstream consumer that reaches `AchievementFrame.X` would surface a \
             nil-table-index error — including the keybind handler \
             `AchievementFrame_ToggleAchievementFrame` (`Blizzard_AchievementUI.lua:195`) \
             which calls `ShowUIPanel(AchievementFrame)` / `HideUIPanel(AchievementFrame)`."
        );

        let frame_strata: String = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:GetFrameStrata()"))
            .expect("`GetFrameStrata` must run cleanly on AchievementFrame");

        assert_eq!(
            frame_strata, "MEDIUM",
            "Expected `AchievementFrame:GetFrameStrata()` to return `MEDIUM` after `{ROOT}` \
             loads, got `{frame_strata}`. The XML at `{XML_SITE}` declares \
             `frameStrata=\"MEDIUM\"` literally. MEDIUM is the default UIPanel stratum so the \
             achievement panel renders at the same level as character / spellbook / inventory \
             frames — the standard UIPanel layout system manages its anchor and the \
             panel-stack push/pop without crossing strata boundaries. A regression to `LOW` \
             would push it below world chrome; a regression to `HIGH` would let it cover \
             dialogs / tooltip text from other panels."
        );

        let parent_name: String = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:GetParent():GetName()"))
            .expect("`GetParent():GetName()` must run cleanly on AchievementFrame");

        assert_eq!(
            parent_name, "UIParent",
            "Expected `AchievementFrame:GetParent():GetName()` to return `UIParent` after \
             `{ROOT}` loads, got `{parent_name}`. The XML at `{XML_SITE}` declares \
             `parent=\"UIParent\"` literally. `UIParent` is the standard scaled-UI root — \
             `SetUIScale` and the resolution-aware reparenting drive against it, and \
             `UIParent.Hide()`-style global toggles cascade to it. A regression that \
             reparents this onto `WorldFrame` (the 3D world root) or some intermediate would \
             break user-set UI scaling and detach the panel from the global UI toggle."
        );

        let is_shown: bool = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:IsShown()"))
            .expect("`IsShown` must run cleanly on AchievementFrame");

        assert!(
            !is_shown,
            "Expected `AchievementFrame:IsShown()` to return false after `{ROOT}` loads. The \
             XML at `{XML_SITE}` declares `hidden=\"true\"` literally — the frame is hidden on \
             creation, and `ToggleAchievementFrame` flips it visible via `ShowUIPanel` only \
             when the player presses the achievement keybind or clicks the micro menu button. \
             A true reading here means a regression dropped `hidden=\"true\"` from the XML or \
             the loader failed to honour the attribute, putting the panel on screen at game \
             start and blocking the player's view."
        );
    });
}
