use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_show_hide_callbacks_fire_once_per_transition() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_minimal_azerite_essence_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local showCount = 0
                        local hideCount = 0

                        local function requireCount(label, expectedShow, expectedHide)
                            if showCount ~= expectedShow or hideCount ~= expectedHide then
                                table.insert(
                                    failures,
                                    string.format("%s show=%d hide=%d", label, showCount, hideCount)
                                )
                            end
                        end

                        AzeriteEssenceUI:RegisterCallback(AzeriteEssenceUIMixin.Event.OnShow, function()
                            showCount = showCount + 1
                        end)
                        AzeriteEssenceUI:RegisterCallback(AzeriteEssenceUIMixin.Event.OnHide, function()
                            hideCount = hideCount + 1
                        end)

                        AzeriteEssenceUI:Show()
                        requireCount("after first show", 1, 0)

                        AzeriteEssenceUI:Hide()
                        requireCount("after hide", 1, 1)

                        AzeriteEssenceUI:Show()
                        requireCount("after second show", 2, 1)

                        AzeriteEssenceUI:Hide()
                        requireCount("after second hide", 2, 2)

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("callback lifecycle check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` callback lifecycle counts were wrong:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking callback lifecycle:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
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
        spell_id: 100_100,
        rank: None,
        active_essence_id: None,
    }
}
