use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const PASSIVE_ONE_SLOT: i32 = 1;
const MILESTONE_ID: i32 = 50;

#[test]
fn blizzard_azerite_essence_ui_unlock_milestone_updates_slot_frame() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_unlock_milestone_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local unlockLog = {}

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        local listener = CreateFrame("Frame")
                        listener:RegisterEvent("AZERITE_ESSENCE_MILESTONE_UNLOCKED")
                        listener:SetScript("OnEvent", function(_, _, milestoneID)
                            table.insert(unlockLog, milestoneID)
                        end)

                        AzeriteEssenceUI:RefreshMilestones()
                        local milestone = AzeriteEssenceUI.Milestones[1]
                        requireCondition("milestone frame exists", type(milestone) == "table")
                        if milestone then
                            requireCondition("initial milestone id", milestone.milestoneID == 50)
                            requireCondition("initial available", milestone.AvailableState:IsShown())
                            requireCondition("initial unlocked hidden", not milestone.UnlockedState:IsShown())
                        end

                        local ok = C_AzeriteEssence.UnlockMilestone(50)
                        requireCondition("unlock returned true", ok == true)
                        requireCondition("unlock event count", #unlockLog == 1)
                        requireCondition("unlock event id", unlockLog[1] == 50)

                        AzeriteEssenceUI:RefreshMilestones()

                        requireCondition("milestone unlocked data", milestone.unlocked == true)
                        requireCondition("milestone can unlock cleared", milestone.canUnlock == false)
                        requireCondition("unlocked state shown", milestone.UnlockedState:IsShown())
                        requireCondition("available state hidden", not milestone.AvailableState:IsShown())
                        requireCondition("locked state hidden", not milestone.LockedState:IsShown())
                        requireCondition(
                            "unlocked empty icon atlas",
                            milestone.UnlockedState.EmptyIcon:GetAtlas() == "heartofazeroth-slot-minor-background"
                        )
                        requireCondition("milestone info unlocked", C_AzeriteEssence.GetMilestoneInfo(50).unlocked == true)
                        requireCondition("milestone info can unlock false", C_AzeriteEssence.GetMilestoneInfo(50).canUnlock == false)

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("unlock milestone UI check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` did not update the unlocked milestone frame:\n{failures}"
                );

                let sim = env.state().borrow();
                let milestone = sim
                    .azerite_essence
                    .milestones
                    .iter()
                    .find(|milestone| milestone.id == MILESTONE_ID)
                    .expect("seeded milestone");
                assert!(milestone.unlocked);
                assert!(!milestone.can_unlock);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking milestone unlock:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_unlock_milestone_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![locked_minor_milestone()],
        has_neck_equipped: true,
        neck_power_level: 50,
        is_at_forge: true,
        ..AzeriteEssenceState::default()
    };
}

fn locked_minor_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: MILESTONE_ID,
        required_level: 50,
        slot: Some(PASSIVE_ONE_SLOT),
        unlocked: false,
        can_unlock: true,
        is_major_slot: false,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 50_050,
        rank: None,
        active_essence_id: None,
    }
}
