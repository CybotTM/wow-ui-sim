//! `ArtifactUI_CanViewArtifact` predicate behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArtifactUI";

#[test]
fn can_view_artifact_is_the_or_of_forge_purchase_disabled_and_multi_artifact_state() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);

        for (case_index, case) in can_view_cases().into_iter().enumerate() {
            seed_predicate_case(env, case);

            let actual: PredicateResult = env
                .eval("return ArtifactUI_CanViewArtifact(), ArtifactUI_HasPurchasedAnything()")
                .expect("ArtifactUI predicate probe should run cleanly");

            assert_eq!(
                actual,
                case.expected_result(),
                "`{ROOT}` predicate case {case_index} should match Blizzard's OR chain: {case:?}"
            );
        }
    });
}

fn load_artifact_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, error): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
        .expect("C_AddOns.LoadAddOn probe should run cleanly");
    assert!(
        loaded,
        "`{ROOT}` must load before can-view predicate probe; error={error:?}"
    );
}

fn seed_predicate_case(env: &wow_ui_sim::lua_api::WowLuaEnv, case: PredicateCase) {
    let mut state = env.state().borrow_mut();
    state.viewed_artifact.is_at_forge = case.is_at_forge;
    state.viewed_artifact.total_purchased_ranks = if case.has_purchased_ranks { 1 } else { 0 };
    state.viewed_artifact.is_maxed_by_rules = case.is_maxed_by_rules;
    state.viewed_artifact.is_disabled = case.is_disabled;
    state.viewed_artifact.num_obtained_artifacts = if case.has_multiple_artifacts { 2 } else { 1 };
}

fn can_view_cases() -> Vec<PredicateCase> {
    (0..32)
        .map(|bits| PredicateCase {
            is_at_forge: bit_is_set(bits, 0),
            has_purchased_ranks: bit_is_set(bits, 1),
            is_maxed_by_rules: bit_is_set(bits, 2),
            is_disabled: bit_is_set(bits, 3),
            has_multiple_artifacts: bit_is_set(bits, 4),
        })
        .collect()
}

fn bit_is_set(bits: u8, index: u8) -> bool {
    bits & (1 << index) != 0
}

#[derive(Clone, Copy, Debug)]
struct PredicateCase {
    is_at_forge: bool,
    has_purchased_ranks: bool,
    is_maxed_by_rules: bool,
    is_disabled: bool,
    has_multiple_artifacts: bool,
}

impl PredicateCase {
    fn expected_result(self) -> PredicateResult {
        let has_purchased_anything = self.has_purchased_ranks || self.is_maxed_by_rules;
        let can_view_artifact = self.is_at_forge
            || has_purchased_anything
            || self.is_disabled
            || self.has_multiple_artifacts;
        (can_view_artifact, has_purchased_anything)
    }
}

type PredicateResult = (bool, bool);
