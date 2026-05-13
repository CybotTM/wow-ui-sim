use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const PASSIVE_ONE_SLOT: i32 = 1;
const PASSIVE_TWO_SLOT: i32 = 2;
const PASSIVE_THREE_SLOT: i32 = 3;

#[test]
fn blizzard_azerite_essence_ui_setup_milestones_uses_seeded_shapes() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_canonical_milestones(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        AzeriteEssenceUI:Show()

                        local expected = {
                            { kind = "major", slot = Enum.AzeriteEssenceSlot.MainSlot },
                            { kind = "minor", slot = Enum.AzeriteEssenceSlot.PassiveOneSlot },
                            { kind = "minor", slot = Enum.AzeriteEssenceSlot.PassiveTwoSlot },
                            { kind = "minor", slot = Enum.AzeriteEssenceSlot.PassiveThreeSlot },
                            { kind = "minor", slot = Enum.AzeriteEssenceSlot.PassiveOneSlot },
                            { kind = "minor", slot = Enum.AzeriteEssenceSlot.PassiveTwoSlot },
                            { kind = "ranked", rank = 1 },
                            { kind = "ranked", rank = 2 },
                            { kind = "ranked", rank = 3 },
                            { kind = "stamina" },
                            { kind = "major", slot = Enum.AzeriteEssenceSlot.MainSlot },
                        }

                        local failures = {}
                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        requireCondition("Milestones count", #AzeriteEssenceUI.Milestones == #expected)
                        requireCondition("Slots count", #AzeriteEssenceUI.Slots == 10)
                        requireCondition("Lines count", #AzeriteEssenceUI.Lines == #expected - 1)
                        requireCondition("DependencyLines count", #AzeriteEssenceUI.DependencyLines == #expected - 1)

                        local slotIndex = 1
                        for index, spec in ipairs(expected) do
                            local milestone = AzeriteEssenceUI.Milestones[index]
                            requireCondition("milestone " .. index .. " exists", type(milestone) == "table")
                            if milestone then
                                requireCondition("milestone " .. index .. " id", milestone.milestoneID == 2000 + index)
                                if spec.kind == "major" then
                                    requireCondition("milestone " .. index .. " major slot", milestone.slot == spec.slot)
                                    requireCondition("milestone " .. index .. " major flag", milestone:IsMajorSlot())
                                    requireCondition("milestone " .. index .. " major draggable", milestone.isDraggable == true)
                                    requireCondition("slot " .. slotIndex .. " major", AzeriteEssenceUI.Slots[slotIndex] == milestone)
                                    slotIndex = slotIndex + 1
                                elseif spec.kind == "minor" then
                                    requireCondition("milestone " .. index .. " minor slot", milestone.slot == spec.slot)
                                    requireCondition("milestone " .. index .. " minor flag", not milestone:IsMajorSlot())
                                    requireCondition("milestone " .. index .. " minor no rank", milestone.rank == nil)
                                    requireCondition("milestone " .. index .. " minor state", type(milestone.UnlockedState) == "table")
                                    requireCondition("slot " .. slotIndex .. " minor", AzeriteEssenceUI.Slots[slotIndex] == milestone)
                                    slotIndex = slotIndex + 1
                                elseif spec.kind == "ranked" then
                                    requireCondition("milestone " .. index .. " ranked slot nil", milestone.slot == nil)
                                    requireCondition("milestone " .. index .. " ranked value", milestone.rank == spec.rank)
                                    requireCondition("milestone " .. index .. " ranked state", type(milestone.AvailableState) == "table")
                                    requireCondition("milestone " .. index .. " ranked text", type(milestone.AvailableState.RankText) == "table")
                                    requireCondition("slot " .. slotIndex .. " ranked", AzeriteEssenceUI.Slots[slotIndex] == milestone)
                                    slotIndex = slotIndex + 1
                                elseif spec.kind == "stamina" then
                                    requireCondition("milestone " .. index .. " stamina slot nil", milestone.slot == nil)
                                    requireCondition("milestone " .. index .. " stamina rank nil", milestone.rank == nil)
                                    requireCondition("milestone " .. index .. " stamina no slot state", milestone.UnlockedState == nil)
                                    requireCondition("milestone " .. index .. " stamina glow", type(milestone.Glow) == "table")
                                end
                            end
                        end
                        requireCondition("all slot-backed milestones appended", slotIndex == 11)

                        for index, line in ipairs(AzeriteEssenceUI.Lines) do
                            requireCondition("line " .. index .. " exists", type(line) == "table")
                            if line then
                                requireCondition("dependency line " .. index .. " parent array", AzeriteEssenceUI.DependencyLines[index] == line)
                                requireCondition("line " .. index .. " from", line.fromButton == AzeriteEssenceUI.Milestones[index])
                                requireCondition("line " .. index .. " to", line.toButton == AzeriteEssenceUI.Milestones[index + 1])
                                requireCondition("line " .. index .. " endpoint method", type(line.SetEndPoints) == "function")
                            end
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("SetupMilestones shape check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` SetupMilestones shape did not match seeded milestones:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking SetupMilestones:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_canonical_milestones(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: (0..11).map(canonical_milestone).collect(),
        has_neck_equipped: true,
        neck_power_level: 70,
        ..AzeriteEssenceState::default()
    };
}

fn canonical_milestone(index: i32) -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 2_001 + index,
        required_level: 50 + index,
        slot: milestone_slot(index),
        unlocked: index == 0 || index == 10,
        can_unlock: false,
        is_major_slot: index == 0 || index == 10,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 20_001 + index,
        rank: milestone_rank(index),
        active_essence_id: None,
    }
}

fn milestone_slot(index: i32) -> Option<i32> {
    match index {
        0 | 10 => Some(MAIN_SLOT),
        1 | 4 => Some(PASSIVE_ONE_SLOT),
        2 | 5 => Some(PASSIVE_TWO_SLOT),
        3 => Some(PASSIVE_THREE_SLOT),
        _ => None,
    }
}

fn milestone_rank(index: i32) -> Option<i32> {
    match index {
        6 => Some(1),
        7 => Some(2),
        8 => Some(3),
        _ => None,
    }
}
