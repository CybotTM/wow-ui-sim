//! No-artifact show-failure behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArtifactUI";

#[test]
fn show_panel_with_no_viewed_artifact_invokes_show_failed_clear() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        install_clear_probe_before_artifact_ui_load(env);
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(NO_ARTIFACT_SHOW_FAILURE_PROBE)
            .expect("ArtifactUI no-artifact show failure probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must route no-artifact ShowUIPanel attempts through showFailedFunc; \
             mismatches: {mismatches:?}"
        );
    });
}

#[test]
fn can_view_artifact_allows_no_viewed_artifact_at_forge_or_with_multiple_artifacts() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);

        assert_can_view_artifact(
            env,
            false,
            "default no-artifact state should not be viewable",
        );

        {
            let mut state = env.state().borrow_mut();
            state.viewed_artifact.is_at_forge = true;
        }
        assert_can_view_artifact(
            env,
            true,
            "being at the forge should unlock ArtifactUI view",
        );

        {
            let mut state = env.state().borrow_mut();
            state.viewed_artifact.is_at_forge = false;
            state.viewed_artifact.num_obtained_artifacts = 2;
        }
        assert_can_view_artifact(
            env,
            true,
            "having more than one obtained artifact should unlock ArtifactUI view",
        );
    });
}

fn install_clear_probe_before_artifact_ui_load(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        __artifact_ui_clear_calls = 0
        local originalClear = C_ArtifactUI.Clear
        C_ArtifactUI.Clear = function(...)
            __artifact_ui_clear_calls = __artifact_ui_clear_calls + 1
            return originalClear(...)
        end
        "#,
    )
    .expect("C_ArtifactUI.Clear probe should install before ArtifactUI load");
}

fn load_artifact_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, error): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
        .expect("C_AddOns.LoadAddOn probe should run cleanly");
    assert!(
        loaded,
        "`{ROOT}` must load before no-artifact ShowUIPanel probe; error={error:?}"
    );
}

fn assert_can_view_artifact(env: &wow_ui_sim::lua_api::WowLuaEnv, expected: bool, message: &str) {
    let can_view: bool = env
        .eval("return ArtifactUI_CanViewArtifact()")
        .expect("ArtifactUI_CanViewArtifact probe should run cleanly");
    assert_eq!(can_view, expected, "{message}");
}

const NO_ARTIFACT_SHOW_FAILURE_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

expect(ArtifactUI_CanViewArtifact() == false, "ArtifactUI_CanViewArtifact should be false")
expect(C_ArtifactUI.IsAtForge() == false, "IsAtForge should default false")
expect(C_ArtifactUI.GetNumObtainedArtifacts() <= 1, "GetNumObtainedArtifacts should not unlock view")
expect(ArtifactFrame:IsShown() == false, "ArtifactFrame should start hidden")

ShowUIPanel(ArtifactFrame)

expect(__artifact_ui_clear_calls == 1, "C_ArtifactUI.Clear calls:" .. tostring(__artifact_ui_clear_calls))
expect(ArtifactFrame:IsShown() == false, "ArtifactFrame should remain hidden")

return mismatches
"#;
