//! Mixin-surface probes for `Blizzard_AnimaDiversionUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const FRAME_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnShow",
    "OnHide",
    "OnEvent",
    "HasAvailableNode",
    "UpdateTutorialTips",
    "SetExclusiveSelectionNode",
    "ClearExclusiveSelectionNode",
    "CanReinforceNode",
    "AddBolsterEffectToGem",
    "StopGemsFullSound",
    "SetupBolsterProgressBar",
    "SetupBolsterGem",
    "AddStandardDataProviders",
    "SetupTextureKits",
    "TryShow",
    "SetupCurrencyFrame",
];
const DATA_PROVIDER_MIXIN_METHODS: &[&str] = &[
    "OnShow",
    "OnHide",
    "OnEvent",
    "SetupConnectionOnPin",
    "ResetModelScene",
    "AddEffectOnPin",
    "ClearEffectOnPin",
    "ClearEffectOnAllPins",
    "RemoveAllData",
    "CanReinforceNode",
    "RefreshAllData",
    "AddNode",
    "AddOrigin",
    "AddModelScene",
];
const FRAME_MIXIN_METHODS_PROBE: &str = r#"
local methods = {
    "OnLoad",
    "OnShow",
    "OnHide",
    "OnEvent",
    "HasAvailableNode",
    "UpdateTutorialTips",
    "SetExclusiveSelectionNode",
    "ClearExclusiveSelectionNode",
    "CanReinforceNode",
    "AddBolsterEffectToGem",
    "StopGemsFullSound",
    "SetupBolsterProgressBar",
    "SetupBolsterGem",
    "AddStandardDataProviders",
    "SetupTextureKits",
    "TryShow",
    "SetupCurrencyFrame",
}

local missing = {}
for _, method in ipairs(methods) do
    if type(AnimaDiversionFrameMixin[method]) ~= "function" then
        table.insert(missing, method .. ":" .. type(AnimaDiversionFrameMixin[method]))
    end
end

return type(AnimaDiversionFrameMixin), missing
"#;
const DATA_PROVIDER_MIXIN_METHODS_PROBE: &str = r#"
local methods = {
    "OnShow",
    "OnHide",
    "OnEvent",
    "SetupConnectionOnPin",
    "ResetModelScene",
    "AddEffectOnPin",
    "ClearEffectOnPin",
    "ClearEffectOnAllPins",
    "RemoveAllData",
    "CanReinforceNode",
    "RefreshAllData",
    "AddNode",
    "AddOrigin",
    "AddModelScene",
}

local missing = {}
for _, method in ipairs(methods) do
    if type(AnimaDiversionDataProviderMixin[method]) ~= "function" then
        table.insert(missing, method .. ":" .. type(AnimaDiversionDataProviderMixin[method]))
    end
end

return type(AnimaDiversionDataProviderMixin), missing
"#;

#[test]
fn anima_diversion_frame_mixin_exposes_expected_methods() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let surface: MixinMethodSurface = env
            .eval(FRAME_MIXIN_METHODS_PROBE)
            .expect("AnimaDiversionFrameMixin method probe must run cleanly");

        assert_mixin_methods("AnimaDiversionFrameMixin", FRAME_MIXIN_METHODS, surface);
    });
}

#[test]
fn anima_diversion_data_provider_mixin_exposes_expected_methods() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let surface: MixinMethodSurface = env
            .eval(DATA_PROVIDER_MIXIN_METHODS_PROBE)
            .expect("AnimaDiversionDataProviderMixin method probe must run cleanly");

        assert_mixin_methods(
            "AnimaDiversionDataProviderMixin",
            DATA_PROVIDER_MIXIN_METHODS,
            surface,
        );
    });
}

type MixinMethodSurface = (String, Vec<String>);

fn assert_mixin_methods(mixin_name: &str, expected_methods: &[&str], surface: MixinMethodSurface) {
    let (mixin_type, missing_methods) = surface;

    assert_eq!(
        mixin_type, "table",
        "`{mixin_name}` must be exposed as a table"
    );
    assert!(
        missing_methods.is_empty(),
        "`{mixin_name}` must expose these methods as functions: {expected_methods:?}; \
         missing or wrong-type entries: {missing_methods:?}"
    );
}
