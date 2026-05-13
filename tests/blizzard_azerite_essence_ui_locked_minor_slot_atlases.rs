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
fn blizzard_azerite_essence_ui_locked_minor_slots_use_canonical_rune_atlases() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_locked_minor_milestones(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        AzeriteEssenceUI:Show()

                        local expectedAtlases = {
                            "heartofazeroth-slot-minor-unlearned-bottomleft",
                            "heartofazeroth-slot-minor-unlearned-topright",
                            "heartofazeroth-slot-minor-unlearned-3",
                        }

                        local failures = {}
                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        requireCondition("four milestones seeded", #AzeriteEssenceUI.Milestones == 4)

                        for index, atlas in ipairs(expectedAtlases) do
                            local milestone = AzeriteEssenceUI.Milestones[index + 1]
                            requireCondition("minor milestone " .. index .. " exists", type(milestone) == "table")
                            if milestone then
                                requireCondition("minor milestone " .. index .. " locked state", type(milestone.LockedState) == "table")
                                requireCondition("minor milestone " .. index .. " locked state shown", milestone.LockedState:IsShown())
                                requireCondition("minor milestone " .. index .. " rune atlas", milestone.LockedState.Rune:GetAtlas() == atlas)
                            end
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("locked minor-slot atlas check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` locked minor slot atlases did not match Blizzard constants:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking locked minor slot atlases:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_locked_minor_milestones(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![
            main_slot_milestone(),
            locked_minor_milestone(1, PASSIVE_ONE_SLOT),
            locked_minor_milestone(2, PASSIVE_TWO_SLOT),
            locked_minor_milestone(3, PASSIVE_THREE_SLOT),
        ],
        has_neck_equipped: true,
        neck_power_level: 1,
        ..AzeriteEssenceState::default()
    };
}

fn main_slot_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 100,
        required_level: 1,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 10_000,
        rank: None,
        active_essence_id: None,
    }
}

fn locked_minor_milestone(index: i32, slot: i32) -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 100 + index,
        required_level: 70 + index,
        slot: Some(slot),
        unlocked: false,
        can_unlock: false,
        is_major_slot: false,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 10_000 + index,
        rank: None,
        active_essence_id: None,
    }
}
