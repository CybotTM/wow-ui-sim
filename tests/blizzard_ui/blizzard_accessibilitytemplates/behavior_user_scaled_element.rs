//! Behavior pin for `UserScaledElementMixin:OnTextScaleUpdated` —
//! the mixin entry point that `TextSizeManagerBase:UpdateRegisteredObjects`
//! calls on every registered element when the user font-scale CVar changes.
//!
//! `OnTextScaleUpdated(scale, registrationInfo)` resolves a per-axis scaled
//! dimension and calls `SetWidth` / `SetHeight` only when that dimension is
//! non-nil:
//!
//! - **width**  = `(self.desiredWidth or registrationInfo.baseWidth)
//!                  * GetWeightedScale("width", scale, registrationInfo)`
//! - **height** = `registrationInfo.baseHeight
//!                  * GetWeightedScale("height", scale, registrationInfo)`
//!
//! `UserScaledElementMixin:GetWeightedScale(scaleContext, scale, regInfo)`
//! delegates to `TextSizeManager:GetWeightedScale(scale, regInfo)` only when
//! `scaleContext == "width"` OR `scaleContext == "height" AND
//! regInfo.useScaleWeightForHeight`. Otherwise it returns `scale` verbatim
//! (linear scaling). `TextSizeManagerBase:GetWeightedScale(scale, regInfo)`
//! returns `1 + (scale-1) * weight`, where `weight` is `regInfo.scaleWeight`
//! (or `defaultScaleWeight = 0.5`) when `regInfo.useScaleWeight` is set, and
//! `1` otherwise — so an unweighted registrationInfo always produces linear
//! scaling on both axes.
//!
//! Three scenarios cover the dispatch matrix:
//!
//! 1. **Linear scaling** (no `useScaleWeight`): both axes scale by `scale`.
//! 2. **Weighted scaling** (`useScaleWeight=true`, `scaleWeight=0.25`,
//!    `useScaleWeightForHeight=true`): both axes scale by `1 + (scale-1)*0.25`.
//! 3. **Skip-when-nil**: no `desiredWidth` and no `baseWidth/baseHeight` on
//!    the regInfo → `GetScaledDesiredWidth/Height` return nil, the
//!    `if scaledX then self:SetX(...)` guards drop the dispatch, and the
//!    pre-existing `SetSize` value is preserved.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_AccessibilityTemplates";
const FRAME_NAME: &str = "UserScaledTestFrame";

fn create_user_scaled_frame(env: &WowLuaEnv) {
    let setup = format!(
        r#"
        local frame = CreateFrame("Frame", "{FRAME_NAME}", UIParent)
        Mixin(frame, UserScaledElementMixin)
        "#
    );
    env.exec(&setup)
        .expect("failed to seed UserScaledElement test frame");
}

fn read_frame_dimensions(env: &WowLuaEnv) -> (f64, f64) {
    let probe =
        format!("local w, h = {FRAME_NAME}:GetWidth(), {FRAME_NAME}:GetHeight(); return w, h");
    env.eval::<(f64, f64)>(&probe)
        .expect("failed to read frame dimensions")
}

fn assert_dim_close(actual: (f64, f64), expected: (f64, f64), label: &str) {
    let (aw, ah) = actual;
    let (ew, eh) = expected;
    let close = (aw - ew).abs() < 1e-4 && (ah - eh).abs() < 1e-4;
    assert!(
        close,
        "Expected `{FRAME_NAME}` ({label}) to land at \
         (w={ew:.4}, h={eh:.4}); got (w={aw:.4}, h={ah:.4}). If this regresses, \
         either OnTextScaleUpdated picked the wrong scale weight, dropped a \
         SetWidth/SetHeight call, or GetDesiredHeight stopped reading from \
         registrationInfo.baseHeight."
    );
}

#[test]
fn on_text_scale_updated_applies_linear_scale_when_unweighted() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        create_user_scaled_frame(env);
        env.exec(&format!(
            r#"
            {FRAME_NAME}.desiredWidth = 100
            {FRAME_NAME}:OnTextScaleUpdated(2.0, {{ baseHeight = 50 }})
            "#
        ))
        .expect("OnTextScaleUpdated(2.0, unweighted regInfo) raised");

        // No useScaleWeight → GetScaleWeight returns 1 → weighted scale == scale.
        // width  = desiredWidth(100) * 2.0 = 200
        // height = baseHeight(50)    * 2.0 = 100
        assert_dim_close(read_frame_dimensions(env), (200.0, 100.0), "linear");
    });
}

#[test]
fn on_text_scale_updated_applies_weighted_scale_to_both_axes() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        create_user_scaled_frame(env);
        env.exec(&format!(
            r#"
            {FRAME_NAME}.desiredWidth = 100
            {FRAME_NAME}:OnTextScaleUpdated(2.0, {{
                baseHeight = 50,
                useScaleWeight = true,
                scaleWeight = 0.25,
                useScaleWeightForHeight = true,
            }})
            "#
        ))
        .expect("OnTextScaleUpdated(2.0, weighted regInfo) raised");

        // weighted scale = 1 + (2.0 - 1) * 0.25 = 1.25
        // width  = 100 * 1.25 = 125
        // height = 50  * 1.25 = 62.5  (useScaleWeightForHeight=true unlocks weighting)
        assert_dim_close(read_frame_dimensions(env), (125.0, 62.5), "weighted");
    });
}

#[test]
fn on_text_scale_updated_preserves_existing_size_when_dimensions_resolve_nil() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        create_user_scaled_frame(env);
        env.exec(&format!(
            r#"
            {FRAME_NAME}:SetSize(80, 40)
            -- registrationInfo with no baseWidth/baseHeight, frame has no desiredWidth.
            -- GetScaledDesired{{Width,Height}} both return nil, so the
            -- `if scaledX then self:SetX(...)` guards must skip both setters.
            {FRAME_NAME}:OnTextScaleUpdated(2.0, {{}})
            "#
        ))
        .expect("OnTextScaleUpdated(2.0, empty regInfo) raised");

        assert_dim_close(read_frame_dimensions(env), (80.0, 40.0), "nil-skip");
    });
}
