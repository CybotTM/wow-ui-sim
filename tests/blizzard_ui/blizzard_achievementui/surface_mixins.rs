//! Mixin method surface for the `Blizzard_AchievementUI` lane.
//!
//! PLAN.md task: pin that `AchievementCategoryTemplateMixin` exposes
//! `OnLoad`, `OnClick`, `Init`, `UpdateSelectionState` as Lua functions
//! on the mixin table.
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
