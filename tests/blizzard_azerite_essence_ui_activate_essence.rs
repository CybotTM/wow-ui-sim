use crate::common;

use std::collections::HashMap;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{
    AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState,
};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const MILESTONE_ID: i32 = 100;
const ESSENCE_ID: i32 = 200;

#[test]
fn blizzard_azerite_essence_ui_activate_essence_updates_slot_and_events() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_activation_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local eventLog = {}

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        local listener = CreateFrame("Frame")
                        listener:RegisterEvent("AZERITE_ESSENCE_ACTIVATED")
                        listener:RegisterEvent("AZERITE_ESSENCE_CHANGED")
                        listener:SetScript("OnEvent", function(_, event, first, second)
                            table.insert(eventLog, { event = event, first = first, second = second })
                        end)

                        C_AzeriteEssence.SetPendingActivationEssence(200)
                        local ok = C_AzeriteEssence.ActivateEssence(200, 100)

                        requireCondition("activation returned true", ok == true)
                        requireCondition("event count", #eventLog == 2)
                        if eventLog[1] then
                            requireCondition("first event name", eventLog[1].event == "AZERITE_ESSENCE_ACTIVATED")
                            requireCondition("first event essence", eventLog[1].first == 200)
                            requireCondition("first event milestone", eventLog[1].second == 100)
                        end
                        if eventLog[2] then
                            requireCondition("second event name", eventLog[2].event == "AZERITE_ESSENCE_CHANGED")
                            requireCondition("second event essence", eventLog[2].first == 200)
                            requireCondition("second event rank", eventLog[2].second == 3)
                        end

                        requireCondition("active essence API", C_AzeriteEssence.GetMilestoneEssence(100) == 200)
                        requireCondition("pending cleared API", C_AzeriteEssence.GetPendingActivationEssence() == nil)

                        local milestoneInfo = C_AzeriteEssence.GetMilestoneInfo(100)
                        requireCondition("milestone info exists", type(milestoneInfo) == "table")
                        if milestoneInfo then
                            requireCondition("milestone id unchanged", milestoneInfo.ID == 100)
                            requireCondition("milestone still unlocked", milestoneInfo.unlocked == true)
                            requireCondition("milestone still major slot", milestoneInfo.slot == Enum.AzeriteEssenceSlot.MainSlot)
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("activation event check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` did not activate the pending essence correctly:\n{failures}"
                );

                let sim = env.state().borrow();
                assert_eq!(
                    sim.azerite_essence.milestones[0].active_essence_id,
                    Some(ESSENCE_ID)
                );
                assert_eq!(sim.azerite_essence.pending_activation_essence, None);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking essence activation:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_activation_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![major_milestone()],
        essences: essence_map(),
        essence_order: vec![ESSENCE_ID],
        has_neck_equipped: true,
        neck_power_level: 50,
        ..AzeriteEssenceState::default()
    };
}

fn essence_map() -> HashMap<i32, AzeriteEssenceInfo> {
    [AzeriteEssenceInfo {
        id: ESSENCE_ID,
        name: "Rank Three Essence".to_string(),
        rank: 3,
        icon: 200_000,
        unlocked: true,
        valid: true,
        access_rank: 3,
        has_never_activated: false,
    }]
    .into_iter()
    .map(|essence| (essence.id, essence))
    .collect()
}

fn major_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: MILESTONE_ID,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 100_200,
        rank: None,
        active_essence_id: None,
    }
}
