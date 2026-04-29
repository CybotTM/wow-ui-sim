#![cfg(feature = "client-retail")]
mod common;

use std::collections::HashSet;

use common::blizzard_addon_manifest::{
    BLIZZARD_ADDON_SMOKE_TARGETS, BlizzardAddonSmokeShape, BlizzardAddonSmokeTarget,
};

const EXPECTED_SMOKE_TARGET_COUNT: usize = 4;

fn assert_loaded_target_shape(target: &BlizzardAddonSmokeTarget<'static>, loaded: &[String]) {
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
}

fn assert_no_lua_errors(env: &wow_ui_sim::lua_api::WowLuaEnv, target_name: &str) {
    let errors = common::panel_fixtures::recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "{} smoke target should settle without Lua errors:\n  {}",
        target_name,
        errors.join("\n  ")
    );
}

fn probe_target_presence(
    target: &BlizzardAddonSmokeTarget<'static>,
    env: &wow_ui_sim::lua_api::WowLuaEnv,
) -> (String, bool) {
    let presence_probe = format!(
        "return type({}), {} ~= nil",
        target.expected_global, target.expected_frame,
    );
    env.eval(&presence_probe)
        .unwrap_or_else(|error| panic!("{} presence probe should return: {error}", target.name))
}

fn assert_target_presence(
    target: &BlizzardAddonSmokeTarget<'static>,
    global_type: &str,
    frame_exists: bool,
) {
    assert_eq!(
        global_type, "function",
        "{} should expose {} as a function",
        target.name, target.expected_global,
    );
    assert!(
        frame_exists,
        "{} should create {}",
        target.name, target.expected_frame,
    );
}

fn assert_target_behavior_probe(
    target: &BlizzardAddonSmokeTarget<'static>,
    env: &wow_ui_sim::lua_api::WowLuaEnv,
) {
    let behavior_result: String = env
        .eval(target.behavior_probe_lua)
        .unwrap_or_else(|error| panic!("{} behavior probe should run: {error}", target.name));
    assert_eq!(
        behavior_result, "ok",
        "{} startup behavior probe should return ok, got {}",
        target.name, behavior_result,
    );
}

fn assert_startup_shape_for_target(
    target: &BlizzardAddonSmokeTarget<'static>,
    env: &wow_ui_sim::lua_api::WowLuaEnv,
) {
    let (global_type, frame_exists) = probe_target_presence(target, env);
    assert_target_presence(target, &global_type, frame_exists);
    assert_target_behavior_probe(target, env);
    assert_no_lua_errors(env, target.name);
}

fn assert_smoke_target_startup_shape(target: &BlizzardAddonSmokeTarget<'static>) {
    common::blizzard_addon_harness::with_blizzard_addon_smoke_shape(
        target.roots,
        target.overrides,
        |env, loaded| {
            assert_loaded_target_shape(target, loaded);
            assert_startup_shape_for_target(target, env);
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
fn blizzard_addon_smoke_targets_assert_startup_shape_after_loading_the_closure() {
    common::with_perf_lock(|| {
        common::with_timeout(600, move || {
            for target in BLIZZARD_ADDON_SMOKE_TARGETS {
                assert_smoke_target_startup_shape(target);
            }
        })
    });
}
