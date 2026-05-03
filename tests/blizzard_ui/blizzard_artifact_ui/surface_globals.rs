//! Global surface probes for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArtifactUI";
const MIXIN_GLOBALS: &[&str] = &[
    "ArtifactUIMixin",
    "ArtifactFrameUnderlayMixin",
    "ArtifactPerksMixin",
    "ArtifactTitleTemplateMixin",
    "ArtifactPowerButtonMixin",
    "ArtifactAppearancesMixin",
    "ArtifactAppearanceSlotMixin",
];
const ARTIFACT_UI_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnShow",
    "OnHide",
    "OnEvent",
    "OnTraitsRefunded",
    "OnAppearanceChanging",
    "EvaulateForgeState",
    "SetTab",
    "SetupPerArtifactData",
    "RefreshKnowledgeRanks",
    "OnKnowledgeEnter",
    "OnKnowledgeLeave",
    "OnInventoryItemMouseEnter",
    "OnInventoryItemMouseLeave",
];

#[test]
fn artifact_ui_exports_expected_mixin_globals_after_load_addon() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(&surface_probe())
            .expect("ArtifactUI global surface probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must expose expected mixin globals and ArtifactUIMixin methods; \
             mismatches: {mismatches:?}"
        );
    });
}

fn load_artifact_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, error): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
        .expect("C_AddOns.LoadAddOn probe should run cleanly");
    assert!(
        loaded,
        "`{ROOT}` must load before surface probes; error={error:?}"
    );
}

fn surface_probe() -> String {
    let mixin_list = lua_array_literal(MIXIN_GLOBALS);
    let method_list = lua_array_literal(ARTIFACT_UI_MIXIN_METHODS);

    format!(
        r#"
        local mixinGlobals = {{{mixin_list}}}
        local methodNames = {{{method_list}}}
        local mismatches = {{}}

        for _, mixinName in ipairs(mixinGlobals) do
            if type(_G[mixinName]) ~= "table" then
                table.insert(mismatches, mixinName .. ":" .. type(_G[mixinName]))
            end
        end

        for _, methodName in ipairs(methodNames) do
            local methodType = type(ArtifactUIMixin) == "table"
                and type(ArtifactUIMixin[methodName])
                or "nil"
            if methodType ~= "function" then
                table.insert(mismatches, "ArtifactUIMixin." .. methodName .. ":" .. methodType)
            end
        end

        return mismatches
        "#
    )
}

fn lua_array_literal(values: &[&str]) -> String {
    values
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>()
        .join(", ")
}
