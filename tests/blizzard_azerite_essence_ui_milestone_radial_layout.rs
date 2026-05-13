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
fn blizzard_azerite_essence_ui_milestone_centers_match_radial_layout() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_radial_milestones(&mut env.state().borrow_mut());
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

                        local tolerance = 1
                        local failures = {}
                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        local orbLeft = AzeriteEssenceUI.OrbBackground:GetLeft()
                        local orbTop = AzeriteEssenceUI.OrbBackground:GetTop()
                        requireCondition("orb left resolved", type(orbLeft) == "number")
                        requireCondition("orb top resolved", type(orbTop) == "number")
                        requireCondition("milestone count", #AzeriteEssenceUI.Milestones == #expected)

                        for index, point in ipairs(expected) do
                            local milestone = AzeriteEssenceUI.Milestones[index]
                            requireCondition("milestone " .. index .. " exists", type(milestone) == "table")
                            if milestone and type(orbLeft) == "number" and type(orbTop) == "number" then
                                local centerX, centerY = milestone:GetCenter()
                                requireCondition("milestone " .. index .. " center x resolved", type(centerX) == "number")
                                requireCondition("milestone " .. index .. " center y resolved", type(centerY) == "number")
                                if type(centerX) == "number" and type(centerY) == "number" then
                                    local actualX = centerX - orbLeft
                                    local actualY = centerY - orbTop
                                    requireCondition("milestone " .. index .. " radial x", math.abs(actualX - point[1]) <= tolerance)
                                    requireCondition("milestone " .. index .. " radial y", math.abs(actualY - point[2]) <= tolerance)
                                end
                            end
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("milestone radial layout check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` milestone centers did not match the radial layout:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking milestone radial layout:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_radial_milestones(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: (0..11).map(radial_milestone).collect(),
        has_neck_equipped: true,
        neck_power_level: 70,
        ..AzeriteEssenceState::default()
    };
}

fn radial_milestone(index: i32) -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 3_000 + index,
        required_level: 50 + index,
        slot: milestone_slot(index),
        unlocked: index == 0,
        can_unlock: false,
        is_major_slot: index == 0,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 30_000 + index,
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
