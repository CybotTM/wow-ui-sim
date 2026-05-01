//! Behavior pin: PLAN-named "AchievementFrame_TryShowFilterDropdown
//! opens the dropdown only outside restricted-mode categories"
//! collapses THREE structural facts about the global at
//! `Mainline/Blizzard_AchievementUI.lua:310-315`.
//!
//! 1. **Gate is `not self.restrictedCategoryID`, not a category-id
//!    comparison.** PLAN's "outside restricted-mode categories"
//!    wording could be read as a per-category check; the actual gate
//!    is a presence check on `AchievementFrame.restrictedCategoryID`.
//!    Any non-nil value (including `false`-equivalent integers like
//!    `0`) blocks the show. Tests drive three states:
//!    - `restrictedCategoryID = nil`  → both Show calls fire
//!    - `restrictedCategoryID = 8`    → no-op (typical case)
//!    - `restrictedCategoryID = 0`    → no-op (0 is truthy in Lua,
//!      so `not 0` is false; would surprise a reader from a
//!      C-family background).
//! 2. **Two-widget surface, with one nested under `self.Header`.**
//!    lua:312 calls `self.FilterDropdown:Show()` (top-level) and
//!    lua:313 calls `self.Header.LeftDDLInset:Show()` (nested under
//!    the Header subframe). PLAN's "the dropdown" wording elides the
//!    LeftDDLInset companion. A regression that dropped either widget
//!    would still let the dropdown appear or leave the inset
//!    permanently hidden; the test pins both.
//! 3. **`AchievementFrame_HideFilterDropdown` is the unconditional
//!    counterpart.** lua:305-308 hides BOTH widgets regardless of
//!    `restrictedCategoryID`. PLAN's "only outside restricted-mode
//!    categories" wording applies to Show, NOT Hide. The Hide
//!    function is called from at least three sites (lua:770, lua:808,
//!    lua:817) where the gate would be wrong. Test pins the
//!    asymmetry: Hide fires regardless of `restrictedCategoryID`.
//!
//! Source map of the contract:
//!
//! ```lua
//! function AchievementFrame_HideFilterDropdown (self)              -- lua:305
//!     self.FilterDropdown:Hide()                                    -- lua:306
//!     self.Header.LeftDDLInset:Hide()                               -- lua:307
//! end
//!
//! function AchievementFrame_TryShowFilterDropdown (self)            -- lua:310
//!     if not self.restrictedCategoryID then                         -- lua:311
//!         self.FilterDropdown:Show()                                -- lua:312
//!         self.Header.LeftDDLInset:Show()                           -- lua:313
//!     end
//! end
//! ```
//!
//! Two tests split the gate-states for Show from the Hide-is-
//! unconditional contract so each body stays under the readability
//! budget and a regression to either side is named in the failing
//! signature.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";

const FAKE_FRAME_BUILDER: &str = r#"
    local function counter(captures, key)
        return function() captures[key] = (captures[key] or 0) + 1 end
    end
    local function build_frame(captures)
        local frame = {}
        frame.FilterDropdown = {
            Show = counter(captures, "filter_dropdown_show_calls"),
            Hide = counter(captures, "filter_dropdown_hide_calls"),
        }
        frame.Header = {
            LeftDDLInset = {
                Show = counter(captures, "left_ddl_inset_show_calls"),
                Hide = counter(captures, "left_ddl_inset_hide_calls"),
            },
        }
        return frame
    end
    local function show_signature(captures)
        return string.format(
            "filter_dropdown_show_calls=%d left_ddl_inset_show_calls=%d " ..
            "filter_dropdown_hide_calls=%d left_ddl_inset_hide_calls=%d",
            captures.filter_dropdown_show_calls or 0,
            captures.left_ddl_inset_show_calls or 0,
            captures.filter_dropdown_hide_calls or 0,
            captures.left_ddl_inset_hide_calls or 0)
    end
"#;

#[test]
fn try_show_filter_dropdown_gates_on_restricted_category_id_presence_and_drives_filter_dropdown_and_left_ddl_inset()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: String = env
            .eval(&format!(
                r#"
                assert(type(_G.AchievementFrame_TryShowFilterDropdown) == "function",
                    "AchievementFrame_TryShowFilterDropdown must be a global function (lua:310)")

                {fake_frame_builder}

                local nil_captures = {{}}
                local nil_frame = build_frame(nil_captures)
                nil_frame.restrictedCategoryID = nil
                AchievementFrame_TryShowFilterDropdown(nil_frame)
                local nil_signature = show_signature(nil_captures)

                local set_captures = {{}}
                local set_frame = build_frame(set_captures)
                set_frame.restrictedCategoryID = 8
                AchievementFrame_TryShowFilterDropdown(set_frame)
                local set_signature = show_signature(set_captures)

                local zero_captures = {{}}
                local zero_frame = build_frame(zero_captures)
                zero_frame.restrictedCategoryID = 0
                AchievementFrame_TryShowFilterDropdown(zero_frame)
                local zero_signature = show_signature(zero_captures)

                return string.format(
                    "nil_state[%s] set_state[%s] zero_state[%s]",
                    nil_signature, set_signature, zero_signature)
                "#,
                fake_frame_builder = FAKE_FRAME_BUILDER,
            ))
            .expect("AchievementFrame_TryShowFilterDropdown must exist and gate on restrictedCategoryID");

        let nil_drives_both_show = "filter_dropdown_show_calls=1 left_ddl_inset_show_calls=1 \
                                    filter_dropdown_hide_calls=0 left_ddl_inset_hide_calls=0";
        let restricted_no_ops = "filter_dropdown_show_calls=0 left_ddl_inset_show_calls=0 \
                                 filter_dropdown_hide_calls=0 left_ddl_inset_hide_calls=0";
        let expected = format!(
            "nil_state[{nil_drives_both_show}] \
             set_state[{restricted_no_ops}] \
             zero_state[{restricted_no_ops}]"
        );

        assert_eq!(
            observations, expected,
            "TryShowFilterDropdown contract pinned at lua:310-315: \
             nil restrictedCategoryID drives BOTH FilterDropdown:Show AND Header.LeftDDLInset:Show; \
             any truthy restrictedCategoryID (8 or 0) is a no-op since Lua treats 0 as truthy. \
             Regression: gate inverted, one widget dropped, or 0 mistakenly treated as falsy."
        );
    });
}

#[test]
fn hide_filter_dropdown_unconditionally_drives_both_widgets_regardless_of_restricted_category_id() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: String = env
            .eval(&format!(
                r#"
                assert(type(_G.AchievementFrame_HideFilterDropdown) == "function",
                    "AchievementFrame_HideFilterDropdown must be a global function (lua:305)")

                {fake_frame_builder}

                local nil_captures = {{}}
                local nil_frame = build_frame(nil_captures)
                nil_frame.restrictedCategoryID = nil
                AchievementFrame_HideFilterDropdown(nil_frame)
                local nil_signature = show_signature(nil_captures)

                local set_captures = {{}}
                local set_frame = build_frame(set_captures)
                set_frame.restrictedCategoryID = 8
                AchievementFrame_HideFilterDropdown(set_frame)
                local set_signature = show_signature(set_captures)

                return string.format(
                    "nil_state[%s] set_state[%s]",
                    nil_signature, set_signature)
                "#,
                fake_frame_builder = FAKE_FRAME_BUILDER,
            ))
            .expect("AchievementFrame_HideFilterDropdown must exist and unconditionally hide both widgets");

        let unconditional_hide = "filter_dropdown_show_calls=0 left_ddl_inset_show_calls=0 \
                                  filter_dropdown_hide_calls=1 left_ddl_inset_hide_calls=1";
        let expected = format!(
            "nil_state[{unconditional_hide}] \
             set_state[{unconditional_hide}]"
        );

        assert_eq!(
            observations, expected,
            "HideFilterDropdown contract pinned at lua:305-308: BOTH FilterDropdown:Hide AND \
             Header.LeftDDLInset:Hide fire regardless of restrictedCategoryID. The asymmetry \
             with TryShowFilterDropdown (which IS gated) is intentional — Hide is called from \
             unrelated lifecycle hooks (lua:770, lua:808, lua:817) where the gate would be wrong. \
             Regression: gate accidentally added to Hide path, or one widget dropped."
        );
    });
}
