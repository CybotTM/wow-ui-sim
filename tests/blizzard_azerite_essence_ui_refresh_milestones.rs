use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};
use wow_ui_sim::lua_api::{AzeriteItemState, ItemLocationData};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const PASSIVE_ONE_SLOT: i32 = 1;
const PASSIVE_TWO_SLOT: i32 = 2;
const PASSIVE_THREE_SLOT: i32 = 3;

#[test]
fn blizzard_azerite_essence_ui_refresh_milestones_updates_slot_states() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_refresh_milestones(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local updateCalls = {}

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        requireCondition("Milestones count", #AzeriteEssenceUI.Milestones == 6)
                        requireCondition("Slots count", #AzeriteEssenceUI.Slots == 6)

                        for index, milestone in ipairs(AzeriteEssenceUI.Milestones) do
                            local originalUpdate = milestone.UpdateMilestoneInfo
                            requireCondition("milestone " .. index .. " UpdateMilestoneInfo", type(originalUpdate) == "function")
                            milestone.UpdateMilestoneInfo = function(self)
                                updateCalls[index] = (updateCalls[index] or 0) + 1
                                return originalUpdate(self)
                            end
                        end

                        AzeriteEssenceUI:RefreshMilestones()

                        for index, milestone in ipairs(AzeriteEssenceUI.Milestones) do
                            requireCondition("milestone " .. index .. " update call", updateCalls[index] == 1)
                            requireCondition("slot " .. index .. " same frame", AzeriteEssenceUI.Slots[index] == milestone)

                            if index <= 3 then
                                requireCondition("milestone " .. index .. " unlocked", milestone.unlocked == true)
                                requireCondition("milestone " .. index .. " unlocked state shown", milestone.UnlockedState:IsShown())
                                if milestone.AvailableState then
                                    requireCondition("milestone " .. index .. " available state hidden", not milestone.AvailableState:IsShown())
                                end
                                if milestone.LockedState then
                                    requireCondition("milestone " .. index .. " locked state hidden", not milestone.LockedState:IsShown())
                                end

                                if milestone:IsMajorSlot() then
                                    requireCondition(
                                        "milestone " .. index .. " major glow atlas",
                                        milestone.UnlockedState.Glow:GetAtlas() == "heartofazeroth-slot-major-glow"
                                    )
                                else
                                    requireCondition(
                                        "milestone " .. index .. " minor empty atlas",
                                        milestone.UnlockedState.EmptyIcon:GetAtlas() == "heartofazeroth-slot-minor-background"
                                    )
                                end
                            else
                                requireCondition("milestone " .. index .. " locked", milestone.unlocked == false)
                                requireCondition("milestone " .. index .. " can unlock data refreshed", milestone.canUnlock == true)
                                requireCondition("milestone " .. index .. " locked state shown", milestone.LockedState:IsShown())
                                requireCondition("milestone " .. index .. " available state hidden", not milestone.AvailableState:IsShown())
                                requireCondition("milestone " .. index .. " unlocked state hidden", not milestone.UnlockedState:IsShown())
                                requireCondition(
                                    "milestone " .. index .. " locked rune atlas",
                                    type(milestone.LockedState.Rune:GetAtlas()) == "string"
                                        and milestone.LockedState.Rune:GetAtlas():match("^heartofazeroth")
                                )
                                requireCondition(
                                    "milestone " .. index .. " locked level text",
                                    milestone.LockedState.UnlockLevelText:GetText() == tostring(90 + index)
                                )
                            end
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("RefreshMilestones slot-state check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` RefreshMilestones did not refresh slot states:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking RefreshMilestones:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_refresh_milestones(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: (0..6).map(refresh_milestone).collect(),
        has_neck_equipped: true,
        neck_power_level: 1,
        ..AzeriteEssenceState::default()
    };
    state.azerite_item = Some(AzeriteItemState {
        item_location: ItemLocationData::default(),
        current_xp: 0,
        max_xp: 1_000,
        power_level: 1,
        unlimited_power_level: 0,
        unlimited_unlocked: false,
        at_max_level: false,
        enabled: true,
    });
}

fn refresh_milestone(index: i32) -> AzeriteEssenceMilestoneInfo {
    let unlocked = index < 3;
    AzeriteEssenceMilestoneInfo {
        id: 3_001 + index,
        required_level: 91 + index,
        slot: Some(milestone_slot(index)),
        unlocked,
        can_unlock: true,
        is_major_slot: index < 3,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 30_001 + index,
        rank: None,
        active_essence_id: None,
    }
}

fn milestone_slot(index: i32) -> i32 {
    match index {
        0..=2 => MAIN_SLOT,
        3 => PASSIVE_ONE_SLOT,
        4 => PASSIVE_TWO_SLOT,
        _ => PASSIVE_THREE_SLOT,
    }
}
