use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_setup_model_scene_uses_canonical_scene_ids() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_minimal_azerite_essence_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);
                install_model_scene_entry_spy(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local createdEntries = __azerite_model_scene_entries or {}
                        local setFromSceneCalls = {}

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        local originalSetFromModelSceneID = AzeriteEssenceUI.ItemModelScene.SetFromModelSceneID

                        AzeriteEssenceUI.ItemModelScene.SetFromModelSceneID = function(self, modelSceneID, forceUpdate)
                            table.insert(setFromSceneCalls, {
                                modelSceneID = modelSceneID,
                                forceUpdate = forceUpdate,
                            })
                            return originalSetFromModelSceneID(self, modelSceneID, forceUpdate)
                        end

                        AzeriteEssenceUIMixin.SetupModelScene(AzeriteEssenceUI)
                        AzeriteEssenceUI.Milestones[1]:CheckAndSetUpRevealEffect()

                        AzeriteEssenceUI.ItemModelScene.SetFromModelSceneID = originalSetFromModelSceneID

                        requireCondition("heart scene id", setFromSceneCalls[1] and setFromSceneCalls[1].modelSceneID == 256)
                        requireCondition("heart force update", setFromSceneCalls[1] and setFromSceneCalls[1].forceUpdate == true)

                        local expectedEntries = {
                            [256] = {1962885, nil},
                            [259] = {2101299, nil},
                            [269] = {1983548, nil},
                            [270] = {1983548, 2924332},
                            [286] = {1983548, 2924332},
                            [287] = {165995, nil},
                            [288] = {166008, nil},
                            [289] = {166008, nil},
                            [316] = {1983524, nil},
                        }

                        for _, entry in ipairs(createdEntries) do
                            local expected = expectedEntries[entry.modelSceneID]
                            if expected then
                                expected.seen = true
                                requireCondition("entry " .. entry.modelSceneID .. " effect 1", entry.effectFileID1 == expected[1])
                                requireCondition("entry " .. entry.modelSceneID .. " effect 2", entry.effectFileID2 == expected[2])
                            end
                        end

                        for modelSceneID in pairs(expectedEntries) do
                            requireCondition("created model scene entry " .. modelSceneID, expectedEntries[modelSceneID].seen == true)
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("model-scene ID check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` model-scene IDs did not match Blizzard constants:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking model-scene IDs:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn install_model_scene_entry_spy(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        __azerite_model_scene_entries = {}
        local originalCreateModelSceneEntry = StaticModelInfo.CreateModelSceneEntry

        StaticModelInfo.CreateModelSceneEntry = function(modelSceneID, effectFileID1, effectFileID2)
            table.insert(__azerite_model_scene_entries, {
                modelSceneID = modelSceneID,
                effectFileID1 = effectFileID1,
                effectFileID2 = effectFileID2,
            })
            return originalCreateModelSceneEntry(modelSceneID, effectFileID1, effectFileID2)
        end
    "#,
    )
    .expect("model-scene entry spy should install before addon load");
}

fn seed_minimal_azerite_essence_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        has_neck_equipped: true,
        neck_power_level: 50,
        ..AzeriteEssenceState::default()
    };
}

fn main_slot_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 100,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 116,
        rank: None,
        active_essence_id: None,
    }
}
