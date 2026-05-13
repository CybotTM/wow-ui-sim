use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const PENDING_ESSENCE_ID: i32 = 123;

#[test]
fn blizzard_azerite_essence_ui_right_mouse_up_clears_pending_essence() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_pending_azerite_essence_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local pendingLog = {}
                        local listener = CreateFrame("Frame")
                        listener:RegisterEvent("PENDING_AZERITE_ESSENCE_CHANGED")
                        listener:SetScript("OnEvent", function(_, _, previousID, newID)
                            table.insert(pendingLog, { previousID = previousID, newID = newID })
                        end)

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        requireCondition("pending before", C_AzeriteEssence.HasPendingActivationEssence())

                        local onMouseUp = AzeriteEssenceUI:GetScript("OnMouseUp")
                        requireCondition("OnMouseUp script", type(onMouseUp) == "function")
                        if onMouseUp then
                            onMouseUp(AzeriteEssenceUI, "RightButton")
                        end

                        requireCondition("pending after", not C_AzeriteEssence.HasPendingActivationEssence())
                        requireCondition("pending event count", #pendingLog == 1)
                        if pendingLog[1] then
                            requireCondition("pending event previous", pendingLog[1].previousID == 123)
                            requireCondition("pending event new nil", pendingLog[1].newID == nil)
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("OnMouseUp pending-clear check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` RightButton OnMouseUp did not clear pending essence:\n{failures}"
                );

                let pending_after = env
                    .state()
                    .borrow()
                    .azerite_essence
                    .pending_activation_essence;
                assert_eq!(pending_after, None);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking OnMouseUp pending clear:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_pending_azerite_essence_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        has_neck_equipped: true,
        neck_power_level: 50,
        pending_activation_essence: Some(PENDING_ESSENCE_ID),
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
