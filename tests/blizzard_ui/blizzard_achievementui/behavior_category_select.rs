//! Behavior pin: `AchievementCategoryTemplateMixin:OnClick(buttonName,
//! down)` forwards immediately to the bare global
//! `AchievementFrameCategories_OnCategoryClicked(self)`. The reload of
//! the achievement list via `AchievementFrameAchievements_UpdateDataProvider`
//! is NOT direct — it's reached transitively through a four-hop chain
//! (`:OnClick → _OnCategoryClicked → _SelectElementData →
//! _OnCategoryChanged → _UpdateDataProvider`), gated on `categoryChanged`,
//! the non-summary branch, and `achievementFunctions` being one of the
//! ACHIEVEMENT/GUILD constants.
//!
//! There is also no field `AchievementFrameCategories.elementData` — the
//! selection state lives in TWO places, neither of which match PLAN's
//! wording: `g_categorySelections[categoryIndex]` (a module-local map
//! keyed by tab index, written at lua:681) and `elementData.selected`
//! (a flag on the element-data table itself, written at lua:680).
//!
//! Source map:
//!
//! ```lua
//! -- lua:506-508 (the mixin click)
//! function AchievementCategoryTemplateMixin:OnClick(buttonName, down)
//!     AchievementFrameCategories_OnCategoryClicked(self);     -- line 507
//! end
//! ```
//!
//! ```lua
//! -- lua:698-700 (the bare-global hop)
//! function AchievementFrameCategories_OnCategoryClicked(button)
//!     AchievementFrameCategories_SelectElementData(button:GetElementData());  -- line 699
//! end
//! ```
//!
//! ```lua
//! -- lua:634-695 (the actual selection-state writer)
//! function AchievementFrameCategories_SelectElementData(elementData, ignoreCollapse)
//!     ...
//!     elementData.selected = true;                            -- line 680  (per-element flag)
//!     g_categorySelections[categoryIndex] = elementData;      -- line 681  (module-local map)
//!     ...
//!     if categoryChanged then                                 -- line 693  (the gate)
//!         AchievementFrameCategories_OnCategoryChanged(category);  -- line 694
//!     end
//! end
//! ```
//!
//! ```lua
//! -- lua:759-787 (the per-tab dispatch)
//! function AchievementFrameCategories_OnCategoryChanged(category)
//!     if ( category == "summary" ) then
//!         ...
//!     else
//!         if ( achievementFunctions == ACHIEVEMENT_FUNCTIONS or
//!              achievementFunctions == GUILD_ACHIEVEMENT_FUNCTIONS ) then
//!             AchievementFrame_ShowSubFrame(AchievementFrameAchievements);
//!             AchievementFrameAchievements_UpdateDataProvider();   -- line 768
//!             ...
//!         elseif ( achievementFunctions == COMPARISON_ACHIEVEMENT_FUNCTIONS ) then
//!             ...  -- comparison-side reload
//!         end
//!     end
//! end
//! ```
//!
//! XML chain: `<Frame parentKey="Categories" name="$parentCategories"
//! inherits="AchivementGoldBorderBackdrop">` at xml:1729 — accessed
//! globally as `AchievementFrameCategories`. The frame is a Categories
//! container, NOT a per-row element. Per-row state belongs on the row's
//! `elementData` (a plain Lua table on the data provider), not on the
//! container frame.
//!
//! **Spec/source mismatch on TWO axes:**
//!
//! 1. **`AchievementFrameCategories.elementData` is not a real field.**
//!    No call site in the entire addon (Mainline or Cata) writes
//!    `AchievementFrameCategories.elementData = ...`. PLAN's wording
//!    invents a container-level field that does not exist; the actual
//!    selection state is split across `g_categorySelections` (module
//!    local) and `elementData.selected` (per-row). A reader of PLAN
//!    looking for `AchievementFrameCategories.elementData` to debug a
//!    selection issue would find nothing.
//! 2. **The reload via `_UpdateDataProvider` is transitive, gated, and
//!    conditional.** The chain `:OnClick → _OnCategoryClicked →
//!    _SelectElementData → _OnCategoryChanged → _UpdateDataProvider`
//!    requires `categoryChanged == true` (lua:638 / lua:693) AND the
//!    category is not "summary" (lua:760) AND
//!    `achievementFunctions == ACHIEVEMENT_FUNCTIONS` or
//!    `GUILD_ACHIEVEMENT_FUNCTIONS` (lua:766). PLAN's "reloads the
//!    achievement list via ..." wording elides four hops and three
//!    conditions, making the contract sound unconditional.
//!
//! Eight assertions split presence/absence/behavior:
//!
//! - **Presence half** (5): `AchievementCategoryTemplateMixin` is a
//!   table; `AchievementCategoryTemplateMixin.OnClick`,
//!   `AchievementFrameCategories_OnCategoryClicked`,
//!   `AchievementFrameCategories_SelectElementData`, and
//!   `AchievementFrameAchievements_UpdateDataProvider` are all
//!   functions. Together these prove the four-hop chain is reachable.
//! - **Absence half** (1): `AchievementFrameCategories.elementData` is
//!   nil. PLAN's named field does not exist; a non-nil reading would
//!   prove Blizzard added a container-level cache and the absence half
//!   should flip to a behavior probe asserting the field gets written
//!   on click.
//! - **Behavior half** (2): driving
//!   `AchievementCategoryTemplateMixin.OnClick(stub_self, "LeftButton",
//!   true)` invokes the spy on
//!   `AchievementFrameCategories_OnCategoryClicked` exactly once, AND
//!   the spy receives the SAME `stub_self` that was passed to OnClick
//!   (proves the `self` value at lua:507 is forwarded literally as the
//!   `button` arg of `_OnCategoryClicked`). The spy short-circuits the
//!   chain so we don't need a fully-populated elementData and selection
//!   behavior — the test pins ONLY the routing edge that PLAN's wording
//!   collapses.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_NAMED_BUT_ABSENT_FIELD: &str = "AchievementFrameCategories.elementData";
const STUB_MARKER: &str = "STUB_CATEGORY_BUTTON";

type CategorySelectProbe = (String, String, String, String, String, String, i64, String);

#[test]
fn category_template_on_click_forwards_self_to_on_category_clicked_and_reload_chain_is_present() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let setup_ok: bool = env
            .eval(
                r#"
                assert(AchievementFrameCategories,
                    "AchievementFrameCategories must exist after addon load")
                assert(AchievementCategoryTemplateMixin,
                    "AchievementCategoryTemplateMixin must exist after addon load")

                _G.__test_clicked_count = 0
                _G.__test_clicked_received_marker = ""
                _G.__test_original_clicked = _G.AchievementFrameCategories_OnCategoryClicked
                _G.AchievementFrameCategories_OnCategoryClicked = function(button)
                    _G.__test_clicked_count = _G.__test_clicked_count + 1
                    _G.__test_clicked_received_marker = (button and button.stub_marker) or "<no-marker>"
                    -- Intentionally do NOT delegate: we isolate the routing edge
                    -- and skip the rest of the chain to avoid pulling in
                    -- selection-behavior + data-provider state that the
                    -- smoke harness doesn't seed.
                end
                return true
                "#,
            )
            .expect("setup phase must run cleanly (spy install)");
        assert!(setup_ok, "setup eval must return true");

        let drive_ok: bool = env
            .eval(
                r#"
                local stub = { stub_marker = "STUB_CATEGORY_BUTTON" }
                AchievementCategoryTemplateMixin.OnClick(stub, "LeftButton", true)
                return true
                "#,
            )
            .expect("driving :OnClick on a stub must run cleanly");
        assert!(drive_ok, "drive eval must return true");

        let observations: CategorySelectProbe = env
            .eval(
                r#"
                local mixin_table_type = type(_G.AchievementCategoryTemplateMixin)
                local on_click_type = type(AchievementCategoryTemplateMixin.OnClick)
                local on_clicked_type = type(_G.__test_original_clicked)
                local select_element_data_type =
                    type(_G.AchievementFrameCategories_SelectElementData)
                local update_data_provider_type =
                    type(_G.AchievementFrameAchievements_UpdateDataProvider)

                local element_data_field_type =
                    type(AchievementFrameCategories.elementData)

                local count = _G.__test_clicked_count or -1
                local received_marker = _G.__test_clicked_received_marker or ""

                _G.AchievementFrameCategories_OnCategoryClicked = _G.__test_original_clicked
                _G.__test_original_clicked = nil
                _G.__test_clicked_count = nil
                _G.__test_clicked_received_marker = nil

                return mixin_table_type,
                       on_click_type,
                       on_clicked_type,
                       select_element_data_type,
                       update_data_provider_type,
                       element_data_field_type,
                       count,
                       received_marker
                "#,
            )
            .expect("post-drive probe must run cleanly");

        let (
            mixin_table_type,
            on_click_type,
            on_clicked_type,
            select_element_data_type,
            update_data_provider_type,
            element_data_field_type,
            count,
            received_marker,
        ) = observations;

        assert_eq!(
            mixin_table_type, "table",
            "Expected `_G.AchievementCategoryTemplateMixin` to be a table (declared at \
             `Mainline/Blizzard_AchievementUI.lua:496` as `AchievementCategoryTemplateMixin = {{}}` \
             then populated with `OnLoad`, `OnClick`, `Init`, `UpdateSelectionState`). Got \
             `{mixin_table_type}`. A `nil` reading means the addon's chunk failed before line \
             496; a `function` reading means the global got overwritten with a function \
             somewhere downstream. Either way, the `mixin=\"AchievementCategoryTemplateMixin\"` \
             XML attribute on the AchievementCategoryTemplate virtual frame would no longer \
             resolve."
        );

        assert_eq!(
            on_click_type, "function",
            "Expected `AchievementCategoryTemplateMixin.OnClick` to be a function (declared at \
             `Mainline/Blizzard_AchievementUI.lua:506`). Got `{on_click_type}`. A `nil` reading \
             means the click forwarder is missing — the XML `<OnClick \
             method=\"OnClick\"/>` binding on the category template button would resolve to \
             nothing and category clicks would no-op."
        );

        assert_eq!(
            on_clicked_type, "function",
            "Expected the original `_G.AchievementFrameCategories_OnCategoryClicked` to be a \
             function (declared at `Mainline/Blizzard_AchievementUI.lua:698`). Got \
             `{on_clicked_type}`. A `nil` reading means the bare-global hop is missing — the \
             `:OnClick` body at lua:507 (`AchievementFrameCategories_OnCategoryClicked(self)`) \
             would crash. Note: the same global is also called from `OnLoad`'s inline closure at \
             lua:501-503 (a duplicate click path on `self.Button:SetScript(\"OnClick\", ...)`), \
             so a nil reading would break BOTH click paths."
        );

        assert_eq!(
            select_element_data_type, "function",
            "Expected `_G.AchievementFrameCategories_SelectElementData` to be a function \
             (declared at `Mainline/Blizzard_AchievementUI.lua:634`, the actual selection-state \
             writer). Got `{select_element_data_type}`. A `nil` reading means \
             `_OnCategoryClicked`'s lua:699 call \
             (`AchievementFrameCategories_SelectElementData(button:GetElementData())`) would \
             crash. This function is also called from `_SelectDefaultElementData` at lua:714 and \
             from the comparison-side select-and-scroll path at lua:2618 — a nil reading breaks \
             every category selection edge."
        );

        assert_eq!(
            update_data_provider_type, "function",
            "Expected `_G.AchievementFrameAchievements_UpdateDataProvider` to be a function \
             (declared at `Mainline/Blizzard_AchievementUI.lua:960`, the achievement-list \
             reload). Got `{update_data_provider_type}`. A `nil` reading means the reload \
             at lua:768 (the non-summary, ACHIEVEMENT_FUNCTIONS/GUILD_ACHIEVEMENT_FUNCTIONS \
             branch of `_OnCategoryChanged`) would crash. PLAN's claim 'reloads the achievement \
             list via ...' depends on this function being reachable, but note that the chain \
             is transitive (4 hops) and gated (categoryChanged + non-summary + \
             achievementFunctions guards) — the reload does not fire on every click."
        );

        assert_eq!(
            element_data_field_type, "nil",
            "Expected `{PLAN_NAMED_BUT_ABSENT_FIELD}` to be nil — no call site in the addon \
             writes this field. The actual selection state lives on \
             `g_categorySelections[categoryIndex]` (module-local, written at lua:681) and on \
             `elementData.selected` (per-row flag, written at lua:680). Got \
             `{element_data_field_type}`. A non-nil reading would prove Blizzard added a \
             container-level cache and the absence half of this test should flip to a behavior \
             probe asserting the field gets written on click. Note: reading a missing field on a \
             frame goes through `__index`; the simulator returns nil for unset fields, matching \
             the WoW client semantics."
        );

        assert_eq!(
            count, 1,
            "Expected the spy on `_G.AchievementFrameCategories_OnCategoryClicked` to fire \
             exactly once when driving \
             `AchievementCategoryTemplateMixin.OnClick(stub_self, \"LeftButton\", true)`. Got \
             `{count}`. A count of 0 means the body at lua:507 is no longer a forwarder (e.g. \
             the global got renamed or the call was inlined into `:OnClick` directly); a count \
             > 1 means `:OnClick` now fans out to multiple click handlers, which would surface \
             as duplicate selection behavior on a real click."
        );

        assert_eq!(
            received_marker, STUB_MARKER,
            "Expected the spy to receive the SAME `self` that was passed to \
             `AchievementCategoryTemplateMixin.OnClick(stub_self, ...)` — proving the body at \
             lua:507 forwards `self` literally (`AchievementFrameCategories_OnCategoryClicked(self)`) \
             without wrapping or substitution. Got `{received_marker:?}` (expected \
             `{STUB_MARKER:?}`). A `\"<no-marker>\"` reading means the spy received a \
             different table (e.g. someone changed the forward to pass `self.Button` instead of \
             `self`); any other string means the stub got replaced with a wholly different \
             object. Either way, the click semantic would no longer match the docstring \
             contract \"OnClick forwards self to OnCategoryClicked\"."
        );
    });
}
