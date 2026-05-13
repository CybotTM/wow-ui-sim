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
fn blizzard_azerite_essence_ui_milestones_keep_canonical_locations() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_canonical_milestones(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let missing: String = env
                    .eval(
                        r#"
                        AzeriteEssenceUI:RefreshMilestones()

                        local expected = {
                            {238, -235},
                            {101, -270},
                            {155, -349},
                            {247, -375},
                            {336, -337},
                            {377, -250},
                            {356, -156},
                            {278, -99},
                            {179, -106},
                            {111, -174},
                            {103, -191},
                        }

                        local missing = {}
                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(missing, label)
                            end
                        end

                        requireCondition("Milestones count", #AzeriteEssenceUI.Milestones == #expected)
                        requireCondition("Slots count", #AzeriteEssenceUI.Slots == #expected)

                        local previousRequiredLevel = -math.huge
                        for index, expectedPoint in ipairs(expected) do
                            local milestone = AzeriteEssenceUI.Milestones[index]
                            requireCondition("milestone " .. index .. " exists", type(milestone) == "table")
                            if milestone then
                                local point, relativeTo, relativePoint, x, y = milestone:GetPoint(1)
                                requireCondition("milestone " .. index .. " anchor point", point == "CENTER")
                                requireCondition("milestone " .. index .. " anchor target", relativeTo == AzeriteEssenceUI.OrbBackground)
                                requireCondition("milestone " .. index .. " anchor target point", relativePoint == "TOPLEFT")
                                requireCondition("milestone " .. index .. " x", type(x) == "number" and x == expectedPoint[1])
                                requireCondition("milestone " .. index .. " y", type(y) == "number" and y == expectedPoint[2])
                                requireCondition("milestone " .. index .. " requiredLevel number", type(milestone.requiredLevel) == "number")
                                if type(milestone.requiredLevel) == "number" then
                                    requireCondition(
                                        "milestone " .. index .. " requiredLevel order",
                                        milestone.requiredLevel > previousRequiredLevel
                                    )
                                    previousRequiredLevel = milestone.requiredLevel
                                end
                            end
                        end

                        return table.concat(missing, "\n")
                    "#,
                    )
                    .expect("milestone location check should run");
                assert!(
                    missing.is_empty(),
                    "`{ROOT}` milestone locations did not match XML contract:\n{missing}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking milestone locations:\n{}",
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
    let required_level = 50 + index;
    AzeriteEssenceMilestoneInfo {
        id: 1_000 + index,
        required_level,
        slot: milestone_slot(index),
        unlocked: index == 0,
        can_unlock: false,
        is_major_slot: index == 0,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 10_000 + index,
        rank: milestone_rank(index),
        active_essence_id: None,
    }
}

fn milestone_slot(index: i32) -> Option<i32> {
    match index {
        0 => Some(MAIN_SLOT),
        1 | 4 | 7 => Some(PASSIVE_ONE_SLOT),
        2 | 5 | 8 => Some(PASSIVE_TWO_SLOT),
        3 | 6 | 9 => Some(PASSIVE_THREE_SLOT),
        _ => None,
    }
}

fn milestone_rank(index: i32) -> Option<i32> {
    (index == 10).then_some(1)
}
