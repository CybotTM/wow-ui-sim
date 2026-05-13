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

const CONNECTED_LINES_LUA: &str = r#"
    Item.CreateFromItemLocation = function()
        return {
            ContinueOnItemLoad = function(_, callback)
                callback()
            end,
            GetItemIcon = function()
                return 0
            end,
            GetItemName = function()
                return "Heart of Azeroth"
            end,
        }
    end

    AzeriteEssenceUI:Show()
    AzeriteEssenceUI:RefreshMilestones()

    local failures = {}
    local function requireCondition(label, condition)
        if not condition then
            table.insert(failures, label)
        end
    end

    requireCondition("line count", #AzeriteEssenceUI.DependencyLines == #AzeriteEssenceUI.Milestones - 1)

    for index, line in ipairs(AzeriteEssenceUI.DependencyLines) do
        local from = AzeriteEssenceUI.Milestones[index]
        local to = AzeriteEssenceUI.Milestones[index + 1]
        requireCondition("line " .. index .. " fromButton", line.fromButton == from)
        requireCondition("line " .. index .. " toButton", line.toButton == to)
        requireCondition("line " .. index .. " shown", line:IsShown())
        requireCondition("line " .. index .. " connected state", line.lineState == PowerDependencyLineMixin.LINE_STATE_CONNECTED)
        requireCondition("line " .. index .. " connected anim", line.animType == PowerDependencyLineMixin.LINE_FADE_ANIM_TYPE_CONNECTED)

        local fillStartPoint, fillStartTarget = line.Fill:GetStartPoint()
        local fillEndPoint, fillEndTarget = line.Fill:GetEndPoint()
        local backgroundStartPoint, backgroundStartTarget = line.Background:GetStartPoint()
        local backgroundEndPoint, backgroundEndTarget = line.Background:GetEndPoint()

        requireCondition("line " .. index .. " fill start point", fillStartPoint == "CENTER")
        requireCondition("line " .. index .. " fill start target", fillStartTarget == from)
        requireCondition("line " .. index .. " fill end point", fillEndPoint == "CENTER")
        requireCondition("line " .. index .. " fill end target", fillEndTarget == to)
        requireCondition("line " .. index .. " background start point", backgroundStartPoint == "CENTER")
        requireCondition("line " .. index .. " background start target", backgroundStartTarget == from)
        requireCondition("line " .. index .. " background end point", backgroundEndPoint == "CENTER")
        requireCondition("line " .. index .. " background end target", backgroundEndTarget == to)
    end

    return table.concat(failures, "\n")
"#;

const DISCONNECTED_FIRST_LINE_LUA: &str = r#"
    AzeriteEssenceUI:RefreshMilestones()

    local failures = {}
    local function requireCondition(label, condition)
        if not condition then
            table.insert(failures, label)
        end
    end

    local line = AzeriteEssenceUI.DependencyLines[1]
    local red, green, blue = line.FillScroll1:GetVertexColor()

    requireCondition("first line exists", type(line) == "table")
    requireCondition("first line disconnected state", line.lineState == PowerDependencyLineMixin.LINE_STATE_DISCONNECTED)
    requireCondition("first line disconnected anim", line.animType == PowerDependencyLineMixin.LINE_FADE_ANIM_TYPE_UNLOCKED)
    requireCondition("locked color red", math.abs(red - 0.486) <= 0.001)
    requireCondition("locked color green", math.abs(green - 0.486) <= 0.001)
    requireCondition("locked color blue", math.abs(blue - 0.486) <= 0.001)

    return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_azerite_essence_ui_dependency_lines_connect_and_disconnect() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_unlocked_milestone_graph(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                assert_connected_lines(env);
                lock_second_milestone(&mut env.state().borrow_mut());
                assert_disconnected_first_line(env);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking dependency lines:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn assert_connected_lines(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let failures: String = env
        .eval(CONNECTED_LINES_LUA)
        .expect("connected dependency-line check should run");
    assert!(
        failures.is_empty(),
        "`{ROOT}` dependency lines did not connect all seeded milestones:\n{failures}"
    );
}

fn assert_disconnected_first_line(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let failures: String = env
        .eval(DISCONNECTED_FIRST_LINE_LUA)
        .expect("disconnected dependency-line check should run");
    assert!(
        failures.is_empty(),
        "`{ROOT}` dependency line did not flip to locked disconnected state:\n{failures}"
    );
}

fn seed_unlocked_milestone_graph(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: (0..11).map(unlocked_milestone).collect(),
        has_neck_equipped: true,
        neck_power_level: 70,
        ..AzeriteEssenceState::default()
    };
    state.azerite_item = Some(AzeriteItemState {
        item_location: ItemLocationData {
            bag_id: Some(0),
            slot_index: Some(1),
            equipment_slot_index: None,
        },
        current_xp: 0,
        max_xp: 1_000,
        power_level: 70,
        unlimited_power_level: 0,
        unlimited_unlocked: false,
        at_max_level: false,
        enabled: true,
    });
}

fn lock_second_milestone(state: &mut SimState) {
    let Some(milestone) = state.azerite_essence.milestones.get_mut(1) else {
        return;
    };
    milestone.unlocked = false;
    milestone.can_unlock = false;
}

fn unlocked_milestone(index: i32) -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 4_000 + index,
        required_level: 50 + index,
        slot: milestone_slot(index),
        unlocked: true,
        can_unlock: false,
        is_major_slot: index == 0 || index == 10,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 116,
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
