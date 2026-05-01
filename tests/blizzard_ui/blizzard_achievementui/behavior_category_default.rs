//! Behavior pin: `AchievementFrameCategories_SelectDefaultElementData()`
//! ensures a data provider is set, asks the ScrollBox for the element
//! at index 1 (`ScrollToElementDataIndex(1, ScrollBoxConstants.AlignCenter)`),
//! and — only if that returns non-nil — forwards to
//! `AchievementFrameCategories_SelectElementData(elementData)`.
//!
//! PLAN's wording "selects the first non-restricted category on first
//! show" collapses three independent edges. This test pins the routing
//! shape (index 1, AlignCenter, conditional forward) without seeding a
//! full categories data provider — the actual filtering of "restricted"
//! categories happens upstream in `_UpdateDataProvider`.
//!
//! Source map:
//!
//! ```lua
//! -- lua:707-716 (the function under test)
//! function AchievementFrameCategories_SelectDefaultElementData()
//!     if not AchievementFrameCategories.ScrollBox:HasDataProvider() then
//!         AchievementFrameCategories_UpdateDataProvider();         -- line 709
//!     end
//!
//!     local elementData = AchievementFrameCategories.ScrollBox
//!         :ScrollToElementDataIndex(1, ScrollBoxConstants.AlignCenter);  -- line 712
//!     if elementData then
//!         AchievementFrameCategories_SelectElementData(elementData);     -- line 714
//!     end
//! end
//! ```
//!
//! ```lua
//! -- lua:718-734 (the data-provider builder; runs only on first call)
//! function AchievementFrameCategories_UpdateDataProvider ()
//!     local restrictedCategoryID = AchievementFrame.restrictedCategoryID;
//!     local newDataProvider = CreateDataProvider();
//!     for index, category in ipairs(achievementFunctions.categories) do
//!         if not category.hidden then
//!             if restrictedCategoryID then
//!                 if (category.id == restrictedCategoryID) or
//!                    (category.parent == restrictedCategoryID) then
//!                     newDataProvider:Insert(category);
//!                 end
//!             else
//!                 newDataProvider:Insert(category);                  -- default branch
//!             end
//!         end
//!     end
//!     AchievementFrameCategories.ScrollBox:SetDataProvider(newDataProvider);
//! end
//! ```
//!
//! ```lua
//! -- lua:111-142 (the categories list builder; ACHIEVEMENT_FUNCTIONS
//! -- prepends a "summary" pseudo-entry at index 1)
//! local function AchievementFrameCategories_MakeCategoryList(source, fakeSummaryId)
//!     local categories = {};
//!     if fakeSummaryId then
//!         tinsert(categories, { id = fakeSummaryId });               -- always at idx 1
//!     end
//!     for i, id in next, source do
//!         local _, parent = GetCategoryInfo(id);
//!         if ( parent == -1 or parent == GUILD_CATEGORY_ID ) then
//!             tinsert(categories, { id = id });
//!         end
//!     end
//!     ...
//! end
//! ```
//!
//! ```lua
//! -- lua:144-147 (the default achievement-functions table)
//! ACHIEVEMENT_FUNCTIONS = {
//!     categoryIndex = AchievementCategoryIndex,
//!     categories = AchievementFrameCategories_MakeCategoryList(GetCategoryList(), "summary"),
//! }
//! achievementFunctions = ACHIEVEMENT_FUNCTIONS;                       -- lua:169
//! ```
//!
//! ```lua
//! -- lua:171-178 (the call-site: only fires when nothing is selected)
//! local function AchievementFrame_GetOrSelectCurrentCategory()
//!     local category = GetSelectedCategory();
//!     if category == 0 then
//!         AchievementFrameCategories_SelectDefaultElementData();      -- line 174
//!         return GetSelectedCategory();
//!     end
//!     return category;
//! end
//! ```
//!
//! ```lua
//! -- lua:702-705 (the OnShow chain that reaches the call-site)
//! function AchievementFrameCategories_OnShow (self)
//!     AchievementFrameCategories_UpdateDataProvider();
//!     AchievementFrame_GetOrSelectCurrentCategory();                  -- line 704
//! end
//! ```
//!
//! ScrollBoxListMixin contract at
//! `Blizzard_SharedXML/Shared/Scroll/ScrollBox.lua:817-826`:
//! `:ScrollToElementDataIndex(dataIndex, alignment, ...)` returns
//! `nil` if `:GetView()` is nil OR if `:FindElementData(dataIndex)`
//! returns nil. `ScrollBoxConstants.AlignCenter == 0.5` (declared at
//! `ScrollBox.lua:20`).
//!
//! **Spec/source mismatch on FOUR axes:**
//!
//! 1. **"non-restricted" is misleading.** `_UpdateDataProvider` reads
//!    `AchievementFrame.restrictedCategoryID`. When that field is SET,
//!    the data provider is filtered to ONLY include matches (id or
//!    parent — restricted IS the inclusion filter, not the exclusion).
//!    When unset (default), all non-hidden categories pass. PLAN's
//!    "non-restricted" phrasing inverts the polarity: there is no
//!    per-category "restricted" attribute that gets filtered out.
//! 2. **"first category" is actually the summary pseudo-entry.** For
//!    `ACHIEVEMENT_FUNCTIONS` (the default `achievementFunctions`),
//!    `_MakeCategoryList(..., "summary")` prepends `{id = "summary"}`
//!    at index 1 (lua:113-115). So the "first" element returned by
//!    `ScrollToElementDataIndex(1, ...)` is the summary tab, not a
//!    real category. For `STAT_FUNCTIONS` (no `fakeSummaryId`), it is
//!    a real category.
//! 3. **"on first show" elides the no-selection gate.** The function
//!    is reached from `_GetOrSelectCurrentCategory` at lua:171-178
//!    ONLY when `GetSelectedCategory() == 0`. If a category was
//!    already selected (e.g. via `g_categorySelections`), the default
//!    selection is skipped — `OnShow` does not unconditionally
//!    re-default.
//! 4. **The forward to `_SelectElementData` is conditional.** The
//!    `if elementData then` guard at lua:713 short-circuits when the
//!    ScrollBox returns nil (no view, or empty data provider). PLAN's
//!    "selects the first ..." wording elides this guard.
//!
//! Eight assertions split presence/absence/behavior:
//!
//! - **Presence half** (5): `_SelectDefaultElementData`,
//!   `_UpdateDataProvider`, `_SelectElementData` are functions; the
//!   ScrollBox's `HasDataProvider` and `ScrollToElementDataIndex`
//!   methods are functions. Together these prove the gate (lua:708),
//!   the rebuild fallback (lua:709), the index-1 fetch (lua:712),
//!   and the conditional forward (lua:714) are all reachable.
//! - **Absence half** (1): `AchievementFrame.restrictedCategoryID` is
//!   `nil` in the default smoke state. This pins the unrestricted
//!   default branch of `_UpdateDataProvider` (lua:728); a non-nil
//!   reading would prove the smoke harness now seeds the restriction
//!   filter, in which case the test should split into restricted /
//!   unrestricted variants.
//! - **Behavior half** (2): a Lua spy installs over
//!   `ScrollBox.ScrollToElementDataIndex` (returning a sentinel
//!   `{ stub_marker = "STUB_DEFAULT_ELEMENT_DATA" }`) and over
//!   `_G.AchievementFrameCategories_SelectElementData` (capturing
//!   the marker). Driving `_SelectDefaultElementData()` once
//!   produces the compact signature
//!   `"called=1 index=1 align=0.5"` (proves the args at lua:712)
//!   AND the select-side signature `"called=1 marker=STUB..."`
//!   (proves the literal forward of the ScrollBox return value to
//!   `_SelectElementData` at lua:714). `:HasDataProvider()` is also
//!   stubbed to `true` so the test does not re-enter
//!   `_UpdateDataProvider` (which would need a populated
//!   `achievementFunctions.categories` table).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_REFERENCED_RESTRICTION_FIELD: &str = "AchievementFrame.restrictedCategoryID";
const STUB_MARKER: &str = "STUB_DEFAULT_ELEMENT_DATA";
const EXPECTED_SCROLL_SIGNATURE: &str = "called=1 index=1 align=0.5";
const EXPECTED_SELECT_SIGNATURE: &str = "called=1 marker=STUB_DEFAULT_ELEMENT_DATA";

type CategoryDefaultProbe = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

#[test]
fn select_default_element_data_calls_scroll_to_index_one_align_center_and_forwards_result() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let setup_ok: bool = env
            .eval(
                r#"
                assert(AchievementFrameCategories,
                    "AchievementFrameCategories must exist after addon load")
                assert(AchievementFrameCategories.ScrollBox,
                    "AchievementFrameCategories.ScrollBox must exist after addon load")

                local sb = AchievementFrameCategories.ScrollBox

                _G.__test_original_has_dp = sb.HasDataProvider
                sb.HasDataProvider = function() return true end

                _G.__test_scroll_count = 0
                _G.__test_scroll_index = -1
                _G.__test_scroll_align = -1
                _G.__test_original_scroll = sb.ScrollToElementDataIndex
                sb.ScrollToElementDataIndex = function(self, index, alignment)
                    _G.__test_scroll_count = _G.__test_scroll_count + 1
                    _G.__test_scroll_index = index or -1
                    _G.__test_scroll_align = alignment or -1
                    return { stub_marker = "STUB_DEFAULT_ELEMENT_DATA" }
                end

                _G.__test_select_count = 0
                _G.__test_select_marker = ""
                _G.__test_original_select = _G.AchievementFrameCategories_SelectElementData
                _G.AchievementFrameCategories_SelectElementData = function(elementData)
                    _G.__test_select_count = _G.__test_select_count + 1
                    _G.__test_select_marker = (elementData and elementData.stub_marker)
                        or "<no-marker>"
                    -- Intentionally do NOT delegate: we isolate the routing
                    -- edge and skip the rest of the chain to avoid pulling
                    -- in g_categorySelections + per-row state writes.
                end
                return true
                "#,
            )
            .expect("setup phase must run cleanly (spies install)");
        assert!(setup_ok, "setup eval must return true");

        let drive_ok: bool = env
            .eval(
                r#"
                AchievementFrameCategories_SelectDefaultElementData()
                return true
                "#,
            )
            .expect("driving _SelectDefaultElementData must run cleanly");
        assert!(drive_ok, "drive eval must return true");

        let observations: CategoryDefaultProbe = env
            .eval(
                r#"
                local sb = AchievementFrameCategories.ScrollBox

                local select_default_type = type(_G.AchievementFrameCategories_SelectDefaultElementData)
                local update_dp_type = type(_G.AchievementFrameCategories_UpdateDataProvider)
                local select_elt_type = type(_G.__test_original_select)
                local has_dp_type = type(_G.__test_original_has_dp)
                local scroll_to_idx_type = type(_G.__test_original_scroll)

                local restricted_type = type(AchievementFrame.restrictedCategoryID)

                local scroll_signature = string.format(
                    "called=%d index=%d align=%s",
                    _G.__test_scroll_count,
                    _G.__test_scroll_index,
                    tostring(_G.__test_scroll_align))
                local select_signature = string.format(
                    "called=%d marker=%s",
                    _G.__test_select_count,
                    tostring(_G.__test_select_marker))

                sb.HasDataProvider = _G.__test_original_has_dp
                sb.ScrollToElementDataIndex = _G.__test_original_scroll
                _G.AchievementFrameCategories_SelectElementData = _G.__test_original_select
                _G.__test_original_has_dp = nil
                _G.__test_original_scroll = nil
                _G.__test_original_select = nil
                _G.__test_scroll_count = nil
                _G.__test_scroll_index = nil
                _G.__test_scroll_align = nil
                _G.__test_select_count = nil
                _G.__test_select_marker = nil

                return select_default_type,
                       update_dp_type,
                       select_elt_type,
                       has_dp_type,
                       scroll_to_idx_type,
                       restricted_type,
                       scroll_signature,
                       select_signature
                "#,
            )
            .expect("post-drive probe must run cleanly");

        let (
            select_default_type,
            update_dp_type,
            select_elt_type,
            has_dp_type,
            scroll_to_idx_type,
            restricted_type,
            scroll_signature,
            select_signature,
        ) = observations;

        assert_eq!(
            select_default_type, "function",
            "Expected `_G.AchievementFrameCategories_SelectDefaultElementData` to be a function \
             (declared at `Mainline/Blizzard_AchievementUI.lua:707`). Got \
             `{select_default_type}`. A `nil` reading means the call-site at lua:174 \
             (`AchievementFrameCategories_SelectDefaultElementData()` inside \
             `AchievementFrame_GetOrSelectCurrentCategory`) would crash whenever \
             `GetSelectedCategory()` returns 0 — i.e. on every fresh show with no prior \
             category selection."
        );

        assert_eq!(
            update_dp_type, "function",
            "Expected `_G.AchievementFrameCategories_UpdateDataProvider` to be a function \
             (declared at `Mainline/Blizzard_AchievementUI.lua:718`, the data-provider builder \
             that filters by `category.hidden` and `restrictedCategoryID`). Got `{update_dp_type}`. \
             A `nil` reading means the fallback at lua:709 (called when \
             `:HasDataProvider()` returns false) would crash, breaking the very-first-show \
             path through `_OnShow` at lua:702-703."
        );

        assert_eq!(
            select_elt_type, "function",
            "Expected the original `_G.AchievementFrameCategories_SelectElementData` (captured \
             pre-spy as `__test_original_select`) to be a function (declared at \
             `Mainline/Blizzard_AchievementUI.lua:634`). Got `{select_elt_type}`. A `nil` reading \
             means the conditional forward at lua:714 would crash whenever \
             `ScrollToElementDataIndex` returns non-nil — i.e. on any successful default \
             selection."
        );

        assert_eq!(
            has_dp_type, "function",
            "Expected the original `AchievementFrameCategories.ScrollBox.HasDataProvider` \
             (captured pre-spy as `__test_original_has_dp`) to be a function (the gate at \
             lua:708 `if not AchievementFrameCategories.ScrollBox:HasDataProvider() then`). \
             Got `{has_dp_type}`. A `nil` reading means the ScrollBoxList view did not \
             initialize on the Categories frame — `ScrollBoxBaseMixin:Init` (called from XML \
             on the `<Frame inherits=\"WowScrollBoxList\">` template) didn't run."
        );

        assert_eq!(
            scroll_to_idx_type, "function",
            "Expected the original \
             `AchievementFrameCategories.ScrollBox.ScrollToElementDataIndex` (captured pre-spy \
             as `__test_original_scroll`) to be a function (declared at \
             `Blizzard_SharedXML/Shared/Scroll/ScrollBox.lua:817`). Got `{scroll_to_idx_type}`. \
             A `nil` reading means the index-1 fetch at lua:712 would crash, severing the \
             entire default-selection path."
        );

        assert_eq!(
            restricted_type, "nil",
            "Expected `{PLAN_REFERENCED_RESTRICTION_FIELD}` to be `nil` in default smoke (no \
             restricted-category mode active). Got `{restricted_type}`. \
             `_UpdateDataProvider` at lua:719 reads this field and, when set, FILTERS the \
             data provider to ONLY include categories whose id or parent matches — the \
             OPPOSITE polarity of PLAN's \"non-restricted\" wording. A non-nil reading here \
             means the smoke harness now seeds a restriction filter and the test should \
             split into restricted-mode and unrestricted-mode variants — only the \
             unrestricted variant pins the contract this test asserts."
        );

        assert_eq!(
            scroll_signature, EXPECTED_SCROLL_SIGNATURE,
            "Expected `ScrollToElementDataIndex` to be called exactly once with `index=1` and \
             `alignment=0.5` (i.e. `ScrollBoxConstants.AlignCenter` from \
             `ScrollBox.lua:20`). Got `{scroll_signature}` (expected \
             `{EXPECTED_SCROLL_SIGNATURE}`). A `called=0` reading means lua:712 was no \
             longer reached (likely the `:HasDataProvider()` gate at lua:708 routed into a \
             different branch, or the function was inlined/renamed). An `index=` other than \
             1 means PLAN's \"first\" wording diverged from the source — Blizzard would have \
             changed which slot represents the default landing element. An `align=` other \
             than 0.5 means the alignment constant was retuned (e.g. `AlignBegin == 0` for \
             top-aligned defaults), which would change the visible scroll position when a \
             user opens the achievement frame."
        );

        assert_eq!(
            select_signature, EXPECTED_SELECT_SIGNATURE,
            "Expected `_SelectElementData` to be called exactly once and to receive the SAME \
             `elementData` table that `ScrollToElementDataIndex` returned (proven by the \
             round-trip of `stub_marker = \"{STUB_MARKER}\"`). Got `{select_signature}` \
             (expected `{EXPECTED_SELECT_SIGNATURE}`). A `called=0` reading means the guard \
             at lua:713 (`if elementData then`) short-circuited unexpectedly — the spy's \
             return was nil, OR the return path was rewritten to require an extra check. A \
             `marker=<no-marker>` reading means the value passed to `_SelectElementData` is \
             no longer the literal return of `:ScrollToElementDataIndex` — e.g. it was \
             wrapped, unwrapped (`elementData.id`), or replaced with a sibling lookup. \
             Either rewrite would change the field set the downstream selection writer at \
             lua:680-681 (`elementData.selected = true`, \
             `g_categorySelections[categoryIndex] = elementData`) operates on."
        );
    });
}
