use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_onshow_registers_and_onhide_clears_events() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_minimal_azerite_essence_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let missing_after_show: String = env
                    .eval(
                        r#"
                        local expectedEvents = {
                            "AZERITE_ESSENCE_ACTIVATED",
                            "AZERITE_ESSENCE_ACTIVATION_FAILED",
                            "AZERITE_ESSENCE_UPDATE",
                            "AZERITE_ESSENCE_FORGE_OPEN",
                            "AZERITE_ESSENCE_FORGE_CLOSE",
                            "AZERITE_ESSENCE_MILESTONE_UNLOCKED",
                            "AZERITE_ITEM_POWER_LEVEL_CHANGED",
                            "AZERITE_ITEM_ENABLED_STATE_CHANGED",
                        }

                        AzeriteEssenceUI:OnShow()

                        local missing = {}
                        for _, eventName in ipairs(expectedEvents) do
                            if not AzeriteEssenceUI:IsEventRegistered(eventName) then
                                table.insert(missing, eventName)
                            end
                        end

                        return table.concat(missing, "\n")
                    "#,
                    )
                    .expect("OnShow event registration check should run");
                assert!(
                    missing_after_show.is_empty(),
                    "`{ROOT}` did not register expected OnShow events:\n{missing_after_show}"
                );

                let still_registered_after_hide: String = env
                    .eval(
                        r#"
                        local expectedEvents = {
                            "AZERITE_ESSENCE_ACTIVATED",
                            "AZERITE_ESSENCE_ACTIVATION_FAILED",
                            "AZERITE_ESSENCE_UPDATE",
                            "AZERITE_ESSENCE_FORGE_OPEN",
                            "AZERITE_ESSENCE_FORGE_CLOSE",
                            "AZERITE_ESSENCE_MILESTONE_UNLOCKED",
                            "AZERITE_ITEM_POWER_LEVEL_CHANGED",
                            "AZERITE_ITEM_ENABLED_STATE_CHANGED",
                        }

                        AzeriteEssenceUI:OnHide()

                        local stillRegistered = {}
                        for _, eventName in ipairs(expectedEvents) do
                            if AzeriteEssenceUI:IsEventRegistered(eventName) then
                                table.insert(stillRegistered, eventName)
                            end
                        end

                        return table.concat(stillRegistered, "\n")
                    "#,
                    )
                    .expect("OnHide event unregister check should run");
                assert!(
                    still_registered_after_hide.is_empty(),
                    "`{ROOT}` left events registered after OnHide:\n{still_registered_after_hide}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking event lifecycle:\n{}",
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
