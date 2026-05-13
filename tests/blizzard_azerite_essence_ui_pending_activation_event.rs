use crate::common;

use std::collections::HashMap;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{
    AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState,
};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const PENDING_ESSENCE_ID: i32 = 123;

#[test]
fn blizzard_azerite_essence_ui_pending_activation_event_refreshes_list() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_pending_activation_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        pendingLog = {}

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        local listener = CreateFrame("Frame")
                        listener:RegisterEvent("PENDING_AZERITE_ESSENCE_CHANGED")
                        listener:SetScript("OnEvent", function(_, _, previousID, newID)
                            table.insert(pendingLog, { previousID = previousID, newID = newID })
                        end)

                        AzeriteEssenceUI.EssenceList:OnShow()

                        local originalRefresh = AzeriteEssenceUI.EssenceList.Refresh
                        AzeriteEssenceUI.EssenceList._testRefreshCalls = 0
                        AzeriteEssenceUI.EssenceList.Refresh = function(self)
                            self._testRefreshCalls = self._testRefreshCalls + 1
                            return originalRefresh(self)
                        end

                        C_AzeriteEssence.SetPendingActivationEssence(123)

                        requireCondition("set event count", #pendingLog == 1)
                        if pendingLog[1] then
                            requireCondition("set event previous nil", pendingLog[1].previousID == nil)
                            requireCondition("set event new", pendingLog[1].newID == 123)
                        end
                        requireCondition("set refreshed list", AzeriteEssenceUI.EssenceList._testRefreshCalls == 1)
                        requireCondition("pending getter", C_AzeriteEssence.GetPendingActivationEssence() == 123)

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("pending activation set check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` did not handle pending activation set event:\n{failures}"
                );

                let pending_after_set = env
                    .state()
                    .borrow()
                    .azerite_essence
                    .pending_activation_essence;
                assert_eq!(pending_after_set, Some(PENDING_ESSENCE_ID));

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        C_AzeriteEssence.ClearPendingActivationEssence()

                        requireCondition("clear event count", #pendingLog == 2)
                        if pendingLog[2] then
                            requireCondition("clear event previous", pendingLog[2].previousID == 123)
                            requireCondition("clear event new nil", pendingLog[2].newID == nil)
                        end
                        requireCondition("clear refreshed list", AzeriteEssenceUI.EssenceList._testRefreshCalls == 2)
                        requireCondition("pending cleared getter", C_AzeriteEssence.GetPendingActivationEssence() == nil)

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("pending activation clear check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` did not handle pending activation clear event:\n{failures}"
                );

                let pending_after_clear = env
                    .state()
                    .borrow()
                    .azerite_essence
                    .pending_activation_essence;
                assert_eq!(pending_after_clear, None);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking pending activation events:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_pending_activation_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        essences: essence_map(),
        essence_order: vec![PENDING_ESSENCE_ID],
        has_neck_equipped: true,
        neck_power_level: 50,
        ..AzeriteEssenceState::default()
    };
}

fn essence_map() -> HashMap<i32, AzeriteEssenceInfo> {
    [AzeriteEssenceInfo {
        id: PENDING_ESSENCE_ID,
        name: "Pending Essence".to_string(),
        rank: 1,
        icon: 123_000,
        unlocked: true,
        valid: true,
        access_rank: 1,
        has_never_activated: false,
    }]
    .into_iter()
    .map(|essence| (essence.id, essence))
    .collect()
}

fn main_slot_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 6_001,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 60_001,
        rank: None,
        active_essence_id: None,
    }
}
