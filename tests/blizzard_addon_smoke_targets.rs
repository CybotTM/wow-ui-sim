mod common;

use std::collections::HashSet;

use common::blizzard_addon_manifest::{
    BLIZZARD_ADDON_SMOKE_TARGETS, BlizzardAddonSmokeShape, BlizzardAddonSmokeTarget,
};

const EXPECTED_SMOKE_TARGET_COUNT: usize = 4;

fn assert_loaded_target_shape(target: &BlizzardAddonSmokeTarget<'static>) {
    common::blizzard_addon_harness::with_blizzard_addon_closure(
        target.roots,
        target.overrides,
        |_, loaded| {
            let loaded_set: HashSet<_> = loaded.iter().cloned().collect();

            for root in target.roots {
                assert!(
                    loaded_set.contains(*root),
                    "{} smoke target should load its root addon {root}; loaded={loaded:?}",
                    target.name,
                );
            }

            for required_addon in target.required_addons {
                assert!(
                    loaded_set.contains(*required_addon),
                    "{} smoke target should include dependency {} in its explicit closure; loaded={loaded:?}",
                    target.name,
                    required_addon,
                );
            }
        },
    );
}

#[test]
fn blizzard_addon_smoke_targets_cover_each_requested_addon_shape_once() {
    let shapes: HashSet<_> = BLIZZARD_ADDON_SMOKE_TARGETS
        .iter()
        .map(|target| target.shape)
        .collect();
    let unique_names: HashSet<_> = BLIZZARD_ADDON_SMOKE_TARGETS
        .iter()
        .map(|target| target.name)
        .collect();

    assert_eq!(
        BLIZZARD_ADDON_SMOKE_TARGETS.len(),
        EXPECTED_SMOKE_TARGET_COUNT
    );
    assert_eq!(unique_names.len(), BLIZZARD_ADDON_SMOKE_TARGETS.len());
    assert!(
        BLIZZARD_ADDON_SMOKE_TARGETS
            .iter()
            .all(|target| !target.roots.is_empty() && !target.required_addons.is_empty())
    );
    assert_eq!(
        shapes,
        HashSet::from([
            BlizzardAddonSmokeShape::MostlyFunctional,
            BlizzardAddonSmokeShape::TemplateHeavy,
            BlizzardAddonSmokeShape::LayoutHeavy,
            BlizzardAddonSmokeShape::MultiAddonFlow,
        ]),
        "smoke targets should define exactly one representative target per requested addon shape",
    );
}

#[test]
fn blizzard_addon_smoke_targets_resolve_and_load_their_expected_closures() {
    common::with_perf_lock(|| {
        common::with_timeout(600, move || {
            for target in BLIZZARD_ADDON_SMOKE_TARGETS {
                assert_loaded_target_shape(target);
            }
        })
    });
}
