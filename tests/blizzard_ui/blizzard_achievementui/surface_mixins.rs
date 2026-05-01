//! Mixin method surface for the `Blizzard_AchievementUI` lane.
//!
//! PLAN.md tasks: pin the method shape of every PLAN-named mixin in the
//! `Blizzard_AchievementUI` lane. Each mixin gets one test that probes the
//! mixin global is a table, then iterates a const tuple of PLAN-named
//! method names and asserts each is a Lua function on the table.
//!
//! **No spec/source mismatch.** Source declares all four methods at file
//! scope on `AchievementCategoryTemplateMixin`
//! (`Mainline/Blizzard_AchievementUI.lua:496-569`):
//!
//! ```lua
//! AchievementCategoryTemplateMixin = {};                                (line 496)
//!
//! function AchievementCategoryTemplateMixin:OnLoad()                    (line 498)
//!     AchievementCategoryButton_Localize(self.Button);
//!     self.Button:SetScript("OnClick", function()
//!         AchievementFrameCategories_OnCategoryClicked(self);
//!     end);
//! end
//!
//! function AchievementCategoryTemplateMixin:OnClick(buttonName, down)   (line 506)
//!     AchievementFrameCategories_OnCategoryClicked(self);
//! end
//!
//! function AchievementCategoryTemplateMixin:Init(elementData)           (line 510)
//!     -- isChild branch sets the category button width / label font / parentID;
//!     -- looks up category name + completion counts via GetCategoryInfo +
//!     -- AchievementFrame_GetCategoryTotalNumAchievements; wires
//!     -- showTooltipFunc for the feat-of-strength / status-bar tooltip
//!     -- variants; calls self:UpdateSelectionState(elementData.selected).
//! end
//!
//! function AchievementCategoryTemplateMixin:UpdateSelectionState(selected) (line 563)
//!     if selected then self.Button:LockHighlight();
//!     else self.Button:UnlockHighlight();
//!     end
//! end
//! ```
//!
//! The mixin is bound to the virtual `AchievementCategoryTemplate` Frame
//! at `Mainline/Blizzard_AchievementUI.xml:622` via the
//! `mixin="AchievementCategoryTemplateMixin"` attribute. Every category
//! button in the categories scrollbox uses this template via
//! `view:SetElementInitializer("AchievementCategoryTemplate", ...)` at
//! `Blizzard_AchievementUI.lua:588-590` — so a regression on any of the
//! four methods would surface as a runtime nil-call error the moment
//! `AchievementFrameCategories_UpdateDataProvider()` populates the
//! categories scrollbox.
//!
//! Behavior of the four methods is left for dedicated `behavior_*`
//! fixtures. This file pins only the *shape* of the mixin — that each
//! method is loaded as a Lua function on the mixin table. A behavior
//! fixture missing one of these methods would surface a confusing
//! "attempt to call a nil value" error; this shape probe surfaces the
//! deletion directly.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const CATEGORY_TEMPLATE_MIXIN_NAME: &str = "AchievementCategoryTemplateMixin";
const CATEGORY_TEMPLATE_LUA_SITE: &str = "Mainline/Blizzard_AchievementUI.lua:496";
const CATEGORY_TEMPLATE_XML_SITE: &str = "Mainline/Blizzard_AchievementUI.xml:622";

/// PLAN-named methods on `AchievementCategoryTemplateMixin`. Each tuple is
/// `(method_name, declared_at_line_number)` so a missing-method failure
/// message points directly at the source line that should declare it.
const CATEGORY_TEMPLATE_PLAN_METHODS: &[(&str, u32)] = &[
    ("OnLoad", 498),
    ("OnClick", 506),
    ("Init", 510),
    ("UpdateSelectionState", 563),
];

const CATEGORY_BUTTON_MIXIN_NAME: &str = "AchievementCategoryTemplateButtonMixin";
const CATEGORY_BUTTON_LUA_SITE: &str = "Mainline/Blizzard_AchievementUI.lua:571";
const CATEGORY_BUTTON_XML_SITE: &str = "Mainline/Blizzard_AchievementUI.xml:625";

/// PLAN-named methods on `AchievementCategoryTemplateButtonMixin`. Each
/// tuple is `(method_name, declared_at_line_number)`. The button mixin is
/// a sibling of `AchievementCategoryTemplateMixin` — bound to the inner
/// `<Button parentKey="Button">` child of the `AchievementCategoryTemplate`
/// virtual Frame at xml:625 — and only carries the tooltip wiring.
const CATEGORY_BUTTON_PLAN_METHODS: &[(&str, u32)] = &[("OnEnter", 573), ("OnLeave", 579)];

const ACHIEVEMENT_TEMPLATE_MIXIN_NAME: &str = "AchievementTemplateMixin";
const ACHIEVEMENT_TEMPLATE_LUA_SITE: &str = "Mainline/Blizzard_AchievementUI.lua:1039";
const ACHIEVEMENT_TEMPLATE_XML_SITE: &str = "Mainline/Blizzard_AchievementUI.xml:733";

/// PLAN-named methods on `AchievementTemplateMixin`. Each tuple is
/// `(method_name, declared_at_line_number)` so a missing-method failure
/// message points directly at the source line that should declare it.
/// The mixin drives every individual achievement row in the right-hand
/// achievements scrollbox: the row's collapse/expand toggle, hover/click
/// behavior, tracking checkbox state, and the saturated-vs-desaturated
/// styling that distinguishes earned from unearned achievements.
const ACHIEVEMENT_TEMPLATE_PLAN_METHODS: &[(&str, u32)] = &[
    ("OnLoad", 1041),
    ("OnClick", 1089),
    ("OnEnter", 1093),
    ("OnLeave", 1103),
    ("Init", 1158),
    ("Collapse", 1308),
    ("Expand", 1332),
    ("Saturate", 1359),
    ("Desaturate", 1392),
    ("DisplayObjectives", 1541),
    ("ToggleTracking", 1580),
    ("SetAsTracked", 1609),
    ("SetSelected", 1141),
];

/// Pin every PLAN-named method on `AchievementCategoryTemplateMixin` as a
/// Lua function on the mixin table.
///
/// Five assertions: one precondition probe that the mixin global itself
/// exists, then one per PLAN-named method (4) confirming
/// `type(AchievementCategoryTemplateMixin.<method>) == "function"`. The
/// precondition probe surfaces a missing mixin global with a precise
/// message instead of a confusing nil-index error inside the loop.
#[test]
fn achievement_category_template_mixin_exposes_plan_named_methods() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let mixin_type: String = env
            .eval(&format!(
                "return type(_G[{CATEGORY_TEMPLATE_MIXIN_NAME:?}])"
            ))
            .expect("AchievementCategoryTemplateMixin global probe must run cleanly");

        assert_eq!(
            mixin_type, "table",
            "Expected `_G[{CATEGORY_TEMPLATE_MIXIN_NAME:?}]` to be a table after `{ROOT}` \
             loads, got `{mixin_type}`. The Lua source at `{CATEGORY_TEMPLATE_LUA_SITE}` \
             declares `AchievementCategoryTemplateMixin = {{}};` at file scope, then \
             attaches the four method declarations to it. The mixin is bound to the \
             virtual `AchievementCategoryTemplate` Frame at `{CATEGORY_TEMPLATE_XML_SITE}` \
             via `mixin=\"AchievementCategoryTemplateMixin\"`, and \
             `view:SetElementInitializer(\"AchievementCategoryTemplate\", ...)` at \
             `Blizzard_AchievementUI.lua:588-590` instantiates one per category in the \
             categories scrollbox. A nil reading means either the table assignment \
             failed (Lua chunk crashed before reaching line 496) or Blizzard refactored \
             the mixin onto a different namespace. Every method probe below depends on \
             this table existing, so a missing mixin here means the rest of the test \
             is moot."
        );

        for (method, line_number) in CATEGORY_TEMPLATE_PLAN_METHODS {
            let method_type: String = env
                .eval(&format!(
                    "return type(_G[{CATEGORY_TEMPLATE_MIXIN_NAME:?}].{method})"
                ))
                .unwrap_or_else(|err| {
                    panic!("`{CATEGORY_TEMPLATE_MIXIN_NAME}.{method}` probe raised: {err}")
                });

            assert_eq!(
                method_type, "function",
                "Expected `{CATEGORY_TEMPLATE_MIXIN_NAME}.{method}` to be a function \
                 after `{ROOT}` loads, got `{method_type}`. The Lua source declares \
                 `function AchievementCategoryTemplateMixin:{method}(...)` at line \
                 {line_number}. A nil reading means either the method declaration was \
                 removed, the Lua chunk failed before reaching line {line_number}, or \
                 Blizzard refactored the method onto a child widget's mixin (e.g. \
                 onto `AchievementCategoryTemplateButtonMixin` at lua:571 — which is \
                 currently a sibling mixin holding only `OnEnter`/`OnLeave` for the \
                 button's tooltip wiring). Every category button in the categories \
                 scrollbox is initialised through this mixin's `Init` (line 510) \
                 which also calls `:UpdateSelectionState` (line 560), and clicking \
                 the button dispatches via the Button's `OnClick` script wired in \
                 `:OnLoad` (line 501-503) which calls \
                 `AchievementFrameCategories_OnCategoryClicked(self)` — a missing \
                 method here means category buttons fail to populate or fail to \
                 react to clicks at runtime."
            );
        }
    });
}

/// Pin every PLAN-named method on `AchievementCategoryTemplateButtonMixin`
/// as a Lua function on the mixin table.
///
/// **No spec/source mismatch.** Source declares
/// `AchievementCategoryTemplateButtonMixin = {};` at file scope at
/// `Mainline/Blizzard_AchievementUI.lua:571`, then attaches both PLAN-named
/// methods immediately below: `:OnEnter()` at line 573 (calls
/// `self.showTooltipFunc(self)` if the parent `Init` wired one — the
/// feat-of-strength / status-bar tooltip variant), and `:OnLeave()` at
/// line 579 (`GameTooltip:SetMinimumWidth(0, false); GameTooltip:Hide()`).
///
/// The mixin is bound to the inner
/// `<Button parentKey="Button" mixin="AchievementCategoryTemplateButtonMixin">`
/// at `Mainline/Blizzard_AchievementUI.xml:625`, nested inside the
/// `<Frame name="AchievementCategoryTemplate">` virtual frame at xml:622.
/// So the parent-mixin `:OnLoad` (lua:498) wires the Button's `OnClick`
/// script and the Button mixin handles `:OnEnter`/`:OnLeave` — together
/// they form the click+tooltip surface for every category row in the
/// scrollbox. A regression on either method would surface as a runtime
/// nil-call error the moment a player hovers a category button (showing
/// stale tooltip from a previous hover, or never hiding it on leave).
///
/// Three assertions: one precondition probe that the mixin global itself
/// exists, then one per PLAN-named method (2) confirming
/// `type(AchievementCategoryTemplateButtonMixin.<method>) == "function"`.
#[test]
fn achievement_category_template_button_mixin_exposes_plan_named_methods() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let mixin_type: String = env
            .eval(&format!("return type(_G[{CATEGORY_BUTTON_MIXIN_NAME:?}])"))
            .expect("AchievementCategoryTemplateButtonMixin global probe must run cleanly");

        assert_eq!(
            mixin_type, "table",
            "Expected `_G[{CATEGORY_BUTTON_MIXIN_NAME:?}]` to be a table after `{ROOT}` \
             loads, got `{mixin_type}`. The Lua source at `{CATEGORY_BUTTON_LUA_SITE}` \
             declares `AchievementCategoryTemplateButtonMixin = {{}};` at file scope, \
             then attaches `:OnEnter` (line 573) and `:OnLeave` (line 579) to it. The \
             mixin is bound to the inner `<Button parentKey=\"Button\">` of the \
             virtual `AchievementCategoryTemplate` Frame at `{CATEGORY_BUTTON_XML_SITE}` \
             via `mixin=\"AchievementCategoryTemplateButtonMixin\"`. A nil reading \
             means either the table assignment failed (Lua chunk crashed before \
             reaching line 571) or Blizzard merged the button's tooltip wiring back \
             into the parent `AchievementCategoryTemplateMixin` and removed this \
             sibling. Every method probe below depends on this table existing."
        );

        for (method, line_number) in CATEGORY_BUTTON_PLAN_METHODS {
            let method_type: String = env
                .eval(&format!(
                    "return type(_G[{CATEGORY_BUTTON_MIXIN_NAME:?}].{method})"
                ))
                .unwrap_or_else(|err| {
                    panic!("`{CATEGORY_BUTTON_MIXIN_NAME}.{method}` probe raised: {err}")
                });

            assert_eq!(
                method_type, "function",
                "Expected `{CATEGORY_BUTTON_MIXIN_NAME}.{method}` to be a function \
                 after `{ROOT}` loads, got `{method_type}`. The Lua source declares \
                 `function AchievementCategoryTemplateButtonMixin:{method}(...)` at \
                 line {line_number}. A nil reading means either the method \
                 declaration was removed, the Lua chunk failed before reaching line \
                 {line_number}, or Blizzard refactored the method onto the parent \
                 `AchievementCategoryTemplateMixin` (lua:496). Every category row's \
                 hover tooltip dispatches through these methods: `:OnEnter` calls \
                 `self.showTooltipFunc(self)` which the parent mixin's `:Init` (lua:510) \
                 wires per category, and `:OnLeave` calls `GameTooltip:Hide()` to \
                 dismiss the tooltip when the cursor leaves — a missing method here \
                 means category-button tooltips fail to show on hover or fail to \
                 hide on leave at runtime."
            );
        }
    });
}

/// Pin every PLAN-named method on `AchievementTemplateMixin` as a Lua
/// function on the mixin table.
///
/// **No spec/source mismatch.** Source declares
/// `AchievementTemplateMixin = {};` at file scope at
/// `Mainline/Blizzard_AchievementUI.lua:1039`, then attaches a much larger
/// surface than this test pins (the mixin also defines non-PLAN-named
/// methods `:ProcessClick` at lua:1060, `:UpdatePlusMinusTexture` at
/// lua:1110, `:IsSelected` at lua:1147, `:GetObjectiveFrame` at lua:1151,
/// `:OnCheckClicked` at lua:1621, `:OnShieldClicked` at lua:1625, plus the
/// static helper `.CalculateSelectedHeight` at lua:1422). This test pins
/// only the 13 PLAN-named methods.
///
/// The mixin is bound to the virtual EventButton `AchievementTemplate` at
/// `Mainline/Blizzard_AchievementUI.xml:733` via `mixin="AchievementTemplateMixin"`.
/// Every individual achievement row in the right-hand achievements scrollbox
/// uses this template — `view:SetElementInitializer("AchievementTemplate", ...)`
/// in `AchievementFrameAchievements_OnLoad` (lua:843+) wires `:Init` per row.
/// The mixin governs collapse/expand state (`:Collapse` lua:1308 / `:Expand`
/// lua:1332), the selected-row height calculation (`.CalculateSelectedHeight`
/// at lua:1422 — used by the scrollbox's element-extent function at lua:856),
/// the saturated-vs-desaturated styling that distinguishes earned from
/// unearned achievements (`:Saturate` lua:1359 / `:Desaturate` lua:1392),
/// the objectives panel rendering (`:DisplayObjectives` lua:1541), and the
/// tracking checkbox state (`:ToggleTracking` lua:1580 / `:SetAsTracked`
/// lua:1609 / `:SetSelected` lua:1141).
///
/// Fourteen assertions: one precondition probe that the mixin global itself
/// exists, then one per PLAN-named method (13) confirming
/// `type(AchievementTemplateMixin.<method>) == "function"`. The precondition
/// probe surfaces a missing mixin global with a precise message instead of
/// a confusing nil-index error inside the loop.
#[test]
fn achievement_template_mixin_exposes_plan_named_methods() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let mixin_type: String = env
            .eval(&format!(
                "return type(_G[{ACHIEVEMENT_TEMPLATE_MIXIN_NAME:?}])"
            ))
            .expect("AchievementTemplateMixin global probe must run cleanly");

        assert_eq!(
            mixin_type, "table",
            "Expected `_G[{ACHIEVEMENT_TEMPLATE_MIXIN_NAME:?}]` to be a table after `{ROOT}` \
             loads, got `{mixin_type}`. The Lua source at `{ACHIEVEMENT_TEMPLATE_LUA_SITE}` \
             declares `AchievementTemplateMixin = {{}};` at file scope, then attaches \
             13 PLAN-named methods plus several non-PLAN-named helpers (ProcessClick, \
             UpdatePlusMinusTexture, IsSelected, GetObjectiveFrame, OnCheckClicked, \
             OnShieldClicked, and the static `.CalculateSelectedHeight`). The mixin is \
             bound to the virtual EventButton `AchievementTemplate` at \
             `{ACHIEVEMENT_TEMPLATE_XML_SITE}` via `mixin=\"AchievementTemplateMixin\"`, \
             and every individual achievement row in the right-hand achievements \
             scrollbox uses this template via `view:SetElementInitializer(\"AchievementTemplate\", ...)` \
             which calls `:Init` per row. A nil reading means either the table \
             assignment failed (Lua chunk crashed before reaching line 1039) or \
             Blizzard refactored the mixin onto a different namespace. Every method \
             probe below depends on this table existing."
        );

        for (method, line_number) in ACHIEVEMENT_TEMPLATE_PLAN_METHODS {
            let method_type: String = env
                .eval(&format!(
                    "return type(_G[{ACHIEVEMENT_TEMPLATE_MIXIN_NAME:?}].{method})"
                ))
                .unwrap_or_else(|err| {
                    panic!("`{ACHIEVEMENT_TEMPLATE_MIXIN_NAME}.{method}` probe raised: {err}")
                });

            assert_eq!(
                method_type, "function",
                "Expected `{ACHIEVEMENT_TEMPLATE_MIXIN_NAME}.{method}` to be a function \
                 after `{ROOT}` loads, got `{method_type}`. The Lua source declares \
                 `function AchievementTemplateMixin:{method}(...)` at line \
                 {line_number}. A nil reading means either the method declaration was \
                 removed, the Lua chunk failed before reaching line {line_number}, or \
                 Blizzard refactored the method onto a sibling/child mixin. The \
                 method drives core achievement-row behavior — `:Init` per-row, \
                 `:Collapse`/`:Expand` for accordion state, `:Saturate`/`:Desaturate` \
                 for earned-vs-unearned styling, `:DisplayObjectives` for criterion \
                 list rendering, `:ToggleTracking`/`:SetAsTracked`/`:SetSelected` for \
                 the tracking checkbox + selected-row state, and `:OnLoad`/`:OnClick`/\
                 `:OnEnter`/`:OnLeave` for the script wiring. A missing method here \
                 means achievement rows fail to populate, react to clicks, render \
                 their objectives, or track the player's progress at runtime."
            );
        }
    });
}
