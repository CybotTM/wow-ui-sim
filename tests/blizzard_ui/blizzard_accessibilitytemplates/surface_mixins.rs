//! Mixin-method surface pinned by `Blizzard_AccessibilityTemplates`.
//!
//! `UIThemeContainerMixin` is defined in
//! `Blizzard_AccessibilityTemplates/Mainline/AccessibilityTemplates.lua` as
//! a free-form Lua table that grows methods via `function Mixin:Method() ... end`
//! syntax (which is just sugar for assigning a function to
//! `UIThemeContainerMixin.Method`). Frames using the mixin are produced
//! via `mixin="UIThemeContainerMixin"` on the `UIThemeContainerFrame`
//! intrinsic (see `AccessibilityIntrinsics.xml`), and inherit the full
//! method bag through the standard `Mixin(self, UIThemeContainerMixin)`
//! pattern that runs from XML mixin attributes.
//!
//! If any of these methods disappears (or, more subtly, gets shadowed by
//! a non-function value during the file's load), every dialog that uses
//! a `UIThemeContainerFrame`-rooted theme container starts erroring at
//! the first event tick — the four intrinsic-script entry points
//! (`UIThemeContainerFrame_OnPreLoad/PreShow/PostHide/PostEvent`) all
//! call into the methods listed here. Pinning the function-shaped
//! surface is the minimum guard against that class of regression.
//!
//! The four `..._OnPre*`/`..._OnPost*` intrinsic-script entry points are
//! intentionally NOT pinned here — they're exercised end-to-end by the
//! load smoke (which would catch any nil-call from the script chain),
//! so duplicating that coverage at the function-table level would just
//! be redundant.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AccessibilityTemplates";

const EXPECTED_METHODS: &[&str] = &[
    "UpdateTheme",
    "CheckUpdateTheme",
    "GetCVarValue",
    "IsDarkMode",
    "UpdateFontStrings",
    "UpdateFrames",
    "UpdateBackground",
    "RegisterObject",
    "RegisterObjects",
    "RegisterFontString",
    "RegisterFontStrings",
    "RegisterFrame",
    "RegisterFrames",
    "RegisterBackgroundTexture",
];

#[test]
fn ui_theme_container_mixin_exposes_expected_methods() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for method in EXPECTED_METHODS {
            let probe = format!("return type(UIThemeContainerMixin[{method:?}])");
            let actual_type: String = env.eval(&probe).unwrap_or_else(|error| {
                panic!("failed to probe `UIThemeContainerMixin.{method}` type: {error}")
            });
            assert_eq!(
                actual_type, "function",
                "Expected `UIThemeContainerMixin.{method}` to be a function after `{ROOT}` loads, \
                 got `{actual_type}`. Defined in `Mainline/AccessibilityTemplates.lua` via \
                 `function UIThemeContainerMixin:{method}(...) ... end`. If this regresses, the \
                 `Mixin(self, UIThemeContainerMixin)` call wired up by the \
                 `mixin=\"UIThemeContainerMixin\"` attribute on `UIThemeContainerFrame` will \
                 silently leave the method nil on every theme-container instance, and the \
                 intrinsic-script entry points (`OnPreLoad/PreShow/PostHide/PostEvent`) will \
                 nil-call at the first event tick."
            );
        }
    });
}
