//! Frame-shape surface pins for the `Blizzard_ActionBar` lane —
//! `MainActionBar` identity and its 12 action-button surface.
//!
//! PLAN.md task: `MainActionBar` exists, parent UIParent, has 12 button
//! children (`MainActionBarButton1`..`12`).
//!
//! **Spec/source mismatch — the PLAN line is wrong on TWO axes.** The
//! MainActionBar XML at `Mainline/MainActionBar.xml:29` declares the
//! frame correctly:
//!
//! ```xml
//! <Frame name="MainActionBar" inherits="EditModeActionBarTemplate"
//!        enableMouse="true" parent="UIParent" frameLevel="50"
//!        mixin="MainActionBarMixin">
//!     <Size x="454" y="35"/>
//!     ...
//!     <KeyValues>
//!         <KeyValue key="numButtons" value="12" type="number"/>
//!         <KeyValue key="buttonTemplate"
//!                   value="MainBarActionBarButtonTemplate" type="string"/>
//!         ...
//!     </KeyValues>
//!     ...
//! </Frame>
//! ```
//!
//! The 12 buttons are NOT direct children of `MainActionBar`. Each
//! button is wrapped in an intermediate container Frame, and the buttons
//! themselves use a special-case name. Both behaviors come from the
//! shared `ActionBarMixin:ActionBar_OnLoad` at
//! `Shared/ActionBar.lua:3-55`:
//!
//! ```lua
//! for i=1, self.numButtons do
//!     local buttonContainer = CreateFrame("Frame",
//!         actionBarName.."ButtonContainer"..i, self,
//!         "ActionBarButtonContainerTemplate", i);
//!
//!     local buttonName;
//!     if self == MainActionBar then
//!         buttonName = "ActionButton"..i;        -- <-- special case!
//!     elseif self == StanceBar then
//!         buttonName = "StanceButton"..i;
//!     elseif self == PetActionBar then
//!         buttonName = "PetActionButton"..i;
//!     elseif self == PossessActionBar then
//!         buttonName = "PossessButton"..i;
//!     else
//!         buttonName = actionBarName.."Button"..i;
//!     end
//!
//!     local actionButton = CreateFrame("CheckButton", buttonName,
//!         buttonContainer, self.buttonTemplate, i);
//!     ...
//!     table.insert(self.actionButtons, actionButton);
//! end
//! ```
//!
//! So the actual 12-deep surface is:
//!
//! - `MainActionBar` (parent: `UIParent`)
//!   - `MainActionBarButtonContainer<N>` (parent: `MainActionBar`,
//!     N = 1..12) — the layer the PLAN line would have meant by "12
//!     button children" if it were checking direct children of the bar.
//!     - `ActionButton<N>` (parent: `MainActionBarButtonContainer<N>`)
//!       — the CheckButton the player actually clicks. Named per the
//!       comment at `Shared/ActionBar.lua:16-18`: *"Different naming
//!       for these bars is to avoid errors with legacy code"* —
//!       Bindings.xml entries that reference `ActionButton1` directly,
//!       plus the legacy `ActionButton<N>:Click()` call sites in
//!       Blizzard's own `SecureActionButton_OnClick` plumbing, all
//!       pre-date the multi-bar refactor and would break under any
//!       other naming scheme.
//!
//! Conclusion: `_G.MainActionBarButton1`..`MainActionBarButton12` do
//! NOT exist. They cannot exist at the same time as `_G.ActionButton1`
//! ..`ActionButton12` (the special-case branch is mutually exclusive
//! with the fall-through that uses `actionBarName.."Button"..i`).
//!
//! Test split along the spec/source boundary, modeled on the same
//! pattern as
//! `tests/blizzard_ui/blizzard_achievementui/surface_frames.rs`'s
//! `achievement_frame_search_progress_bar_split_presence_absence`:
//!
//! - **Presence half** (`main_action_bar_publishes_expected_panel_identity`):
//!   pin the three PLAN facts that ARE true — `MainActionBar` exists
//!   as a table global, has `parent == UIParent`, and
//!   `MainActionBar.actionButtons` is a 12-entry array (the internal
//!   contract every consumer of the bar reads, set at
//!   `Shared/ActionBar.lua:7` and populated at `:41`). The frameLevel
//!   declared in XML (50) is intentionally NOT pinned: the
//!   `EditModeSystemMixin:OnSystemLoad` chain triggered by
//!   `EditModeActionBar_OnLoad` at `Shared/ActionBar.lua:256-264`
//!   bumps the runtime level by +2 (the bar reports 52 post-load),
//!   so a strict `frameLevel == 50` would fail despite the XML being
//!   correct. The PLAN line doesn't ask for frameLevel — pinning a
//!   derived runtime value would conflate XML correctness with
//!   EditMode plumbing.
//!
//! - **Actual button publishing half**
//!   (`main_action_bar_publishes_action_button_globals_and_button_containers`):
//!   pin that the 12 PLAN-meant slots ARE published, just not under
//!   the PLAN names. Each slot N has `_G.ActionButton<N>` as the
//!   CheckButton (special-case name from
//!   `Shared/ActionBar.lua:19-20`), `_G.MainActionBarButtonContainer<N>`
//!   as the wrapping Frame (line 14 — every multi-bar uses the same
//!   `ButtonContainer` suffix on the bar's own name), and the parent
//!   chain `ActionButton<N> → MainActionBarButtonContainer<N> →
//!   MainActionBar` resolves cleanly via `:GetParent()` on each step.
//!
//! - **Absence half**
//!   (`main_action_bar_does_not_publish_plan_named_button_globals`):
//!   pin that `_G.MainActionBarButton<N>` is nil for N = 1..12. A
//!   non-nil reading would prove either Blizzard removed the
//!   special-case branch at `Shared/ActionBar.lua:19-20` (so
//!   MainActionBar's buttons fell through to
//!   `actionBarName.."Button"..i` and got the PLAN-named global) or
//!   the simulator double-published the buttons under both names
//!   (which would shadow `ActionButton<N>` consumers via stale
//!   references). Either way the spec needs revisiting before the
//!   test is "fixed" by mutating the assertion.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBar";
const FRAME_NAME: &str = "MainActionBar";
const XML_SITE: &str = "Mainline/MainActionBar.xml:29";
const ACTION_BAR_LUA_SITE: &str = "Shared/ActionBar.lua:3-55";
const NUM_BUTTONS: usize = 12;

#[test]
fn main_action_bar_publishes_expected_panel_identity() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let frame_type: String = env
            .eval(&format!("return type(_G[{FRAME_NAME:?}])"))
            .expect("MainActionBar global probe must run cleanly");

        assert_eq!(
            frame_type, "table",
            "Expected `_G[{FRAME_NAME:?}]` to be a table after `{ROOT}` loads, got \
             `{frame_type}`. The XML at `{XML_SITE}` declares this frame with \
             `name=\"MainActionBar\"` and `parent=\"UIParent\"`, so the named-frame \
             registration runs at XML-load time. A nil reading means either the XML did not \
             execute (a regression in the load pipeline) or the frame failed to register its \
             name. Every downstream consumer that names `MainActionBar` directly would \
             surface a nil-table-method error — including `IsNormalActionBarState()` at \
             `Shared/MultiActionBars.lua:54` (`return MainActionBar:IsShown()`), \
             `MainActionBarMixin:OnLoad`/`OnEvent`/`OnShow` at `Shared/MainActionBar.lua`, \
             every page-cycle handler that calls `MainActionBar.ActionBarPageNumber.Text:SetText` \
             at `Shared/MainActionBar.lua:10`, and the assist-combat highlight hook at \
             `Mainline/AssistedCombatManager.lua:223` (`MainActionBar:GetEndCapsFrameLevel`)."
        );

        let parent_name: String = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:GetParent():GetName()"))
            .expect("`GetParent():GetName()` must run cleanly on MainActionBar");

        assert_eq!(
            parent_name, "UIParent",
            "Expected `MainActionBar:GetParent():GetName()` to return `UIParent`, got \
             `{parent_name}`. The XML at `{XML_SITE}` declares `parent=\"UIParent\"` \
             literally — `UIParent` is the standard scaled-UI root that `SetUIScale` and \
             the resolution-aware reparenting drive against. A regression that reparents \
             the action bar onto `WorldFrame` (the 3D world root) or some intermediate \
             would break user-set UI scaling and detach the bar from `UIParent.Hide()` \
             cascading."
        );

        let action_buttons_count: i64 = env
            .eval(&format!("return #_G[{FRAME_NAME:?}].actionButtons"))
            .expect("`#MainActionBar.actionButtons` must run cleanly");

        let expected_count = NUM_BUTTONS as i64;
        assert_eq!(
            action_buttons_count, expected_count,
            "Expected `#MainActionBar.actionButtons` to be `{expected_count}`, got \
             `{action_buttons_count}`. The XML at `{XML_SITE}` declares \
             `<KeyValue key=\"numButtons\" value=\"12\" type=\"number\"/>`, which \
             `ActionBarMixin:ActionBar_OnLoad` at `{ACTION_BAR_LUA_SITE}` reads and uses to \
             drive a `for i=1, self.numButtons do ... table.insert(self.actionButtons, \
             actionButton) end` loop (lua:13/41). The `actionButtons` array is the \
             internal contract every consumer of the bar reads — `UpdateShownButtons()` \
             walks it (lua:200-209), `SetShowGrid()` walks it (lua:144-171), and \
             `ActionBarController` queries it via `bar.actionButtons[i]` for keybind \
             dispatch. A short array means either the OnLoad chunk failed mid-loop or \
             Blizzard moved away from the file-local table pattern."
        );
    });
}

#[test]
fn main_action_bar_publishes_action_button_globals_and_button_containers() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for slot in 1..=NUM_BUTTONS {
            let action_button_name = format!("ActionButton{slot}");
            let container_name = format!("MainActionBarButtonContainer{slot}");

            let button_object_type: String = env
                .eval(&format!(
                    "return type(_G[{action_button_name:?}]) == \"table\" and \
                            _G[{action_button_name:?}]:GetObjectType() or \
                            type(_G[{action_button_name:?}])"
                ))
                .expect("ActionButton<N> object-type probe must run cleanly");

            assert_eq!(
                button_object_type, "CheckButton",
                "Expected `_G[{action_button_name:?}]:GetObjectType()` to return `CheckButton` \
                 after `{ROOT}` loads, got `{button_object_type}`. \
                 `ActionBarMixin:ActionBar_OnLoad` at `{ACTION_BAR_LUA_SITE}` line 19-20 \
                 special-cases `MainActionBar` to name its buttons `ActionButton<N>` (NOT \
                 `MainActionBarButton<N>` — see the absence test \
                 `main_action_bar_does_not_publish_plan_named_button_globals` for the \
                 inverse pin). The XML at `{XML_SITE}` declares \
                 `<KeyValue key=\"buttonTemplate\" value=\"MainBarActionBarButtonTemplate\" \
                 type=\"string\"/>`, and `MainBarActionBarButtonTemplate` at \
                 `Mainline/MainActionBar.xml:3` inherits `ActionBarButtonTemplate` which \
                 is itself a `<CheckButton>`. A non-CheckButton or nil reading means \
                 either the special-case branch at lua:19-20 broke (so the button name \
                 fell through to the generic suffix) or the OnLoad loop failed before \
                 reaching slot {slot}. Bindings.xml `ActionButton<N>` keybinds and the \
                 legacy `ActionButton<N>:Click()` call sites all assume this name."
            );

            let button_parent_name: String = env
                .eval(&format!(
                    "return _G[{action_button_name:?}]:GetParent():GetName()"
                ))
                .expect("ActionButton<N>:GetParent():GetName() must run cleanly");

            assert_eq!(
                button_parent_name, container_name,
                "Expected `_G[{action_button_name:?}]:GetParent():GetName()` to return \
                 `{container_name}`, got `{button_parent_name}`. \
                 `ActionBarMixin:ActionBar_OnLoad` at `{ACTION_BAR_LUA_SITE}` line 14 \
                 creates each button's container as \
                 `CreateFrame(\"Frame\", actionBarName..\"ButtonContainer\"..i, self, \
                 \"ActionBarButtonContainerTemplate\", i)`, then line 31 creates the \
                 button as `CreateFrame(\"CheckButton\", buttonName, buttonContainer, \
                 self.buttonTemplate, i)` — so the button's parent IS the container, NOT \
                 `MainActionBar` directly. A different parent name means either the \
                 wrapping container was elided (so `ActionButton{slot}` would parent \
                 directly onto `MainActionBar` and break the `buttonContainer:SetSize` \
                 sizing layer at lua:43) or the container's name was changed."
            );

            let container_parent_name: String = env
                .eval(&format!(
                    "return _G[{container_name:?}]:GetParent():GetName()"
                ))
                .expect("MainActionBarButtonContainer<N>:GetParent():GetName() must run cleanly");

            assert_eq!(
                container_parent_name, FRAME_NAME,
                "Expected `_G[{container_name:?}]:GetParent():GetName()` to return \
                 `{FRAME_NAME}`, got `{container_parent_name}`. \
                 `ActionBarMixin:ActionBar_OnLoad` at `{ACTION_BAR_LUA_SITE}` line 14 \
                 creates each button container with `self` as the parent (where `self` \
                 is `MainActionBar` for this bar's invocation of the OnLoad), so the \
                 container's parent must be `MainActionBar`. A different parent name \
                 means the OnLoad ran on a different frame (e.g. the `for i=1, \
                 self.numButtons` loop self-reference broke) or the container was \
                 reparented post-load — neither has a consumer in Blizzard's own code so \
                 either would be a simulator-side regression."
            );
        }
    });
}

#[test]
fn main_action_bar_does_not_publish_plan_named_button_globals() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for slot in 1..=NUM_BUTTONS {
            let plan_named_button = format!("MainActionBarButton{slot}");

            let plan_named_type: String = env
                .eval(&format!("return type(_G[{plan_named_button:?}])"))
                .expect("MainActionBarButton<N> nil-probe must run cleanly");

            assert_eq!(
                plan_named_type, "nil",
                "Expected `_G[{plan_named_button:?}]` to be nil after `{ROOT}` loads, got \
                 `{plan_named_type}`. The PLAN.md line for this task names the bar's 12 \
                 buttons as `MainActionBarButton1`..`MainActionBarButton12`, but \
                 `ActionBarMixin:ActionBar_OnLoad` at `{ACTION_BAR_LUA_SITE}` line 19-20 \
                 special-cases `MainActionBar` to use the legacy name `ActionButton<N>` \
                 instead. The branch is mutually exclusive with the fall-through at \
                 lua:28 (`buttonName = actionBarName..\"Button\"..i`) that would have \
                 produced the PLAN names — so a non-nil reading proves either Blizzard \
                 removed the special case (and MainActionBar's buttons fell through to \
                 the generic name, which would simultaneously null `_G.ActionButton<N>` — \
                 the sibling presence test \
                 `main_action_bar_publishes_action_button_globals_and_button_containers` \
                 would also fail), OR the simulator double-published the buttons under \
                 both names (which would shadow `_G.ActionButton<N>` consumers — every \
                 Bindings.xml entry, every `ActionButton1:Click()` call site — via a \
                 stale reference left over from an aborted OnLoad rerun). Either way the \
                 PLAN spec needs revisiting before this assertion is mutated. Note: \
                 StanceBar / PetActionBar / PossessActionBar follow the same pattern \
                 (special-case names `StanceButton<N>`, `PetActionButton<N>`, \
                 `PossessButton<N>` per lua:21-26) — only the Multi-bars (`MultiBarBottomLeft` \
                 etc.) and EditMode-only bars (MultiBar5/6/7) fall through to the \
                 `actionBarName..\"Button\"..i` suffix and thus DO publish a \
                 `<BarName>Button<N>` global."
            );
        }
    });
}
