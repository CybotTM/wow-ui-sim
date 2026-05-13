use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_stars_animation_template_uses_parent_level_and_array() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_minimal_azerite_essence_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        local parent = CreateFrame("Frame", "AzeriteStarsTemplateTestParent", UIParent)
                        parent:SetFrameLevel(42)
                        local child = CreateFrame("Frame", "AzeriteStarsTemplateTestChild", parent, "AzeriteEssenceStarsAnimationFrameTemplate")

                        requireCondition("standalone parentArray table", type(parent.StarsAnimations) == "table")
                        requireCondition("standalone parentArray entry", parent.StarsAnimations[1] == child)
                        requireCondition("standalone uses parent level", child:IsUsingParentLevel())
                        requireCondition("standalone parent-derived level", child:GetFrameLevel() == parent:GetFrameLevel())

                        parent:SetFrameLevel(60)
                        requireCondition("standalone follows parent level change", child:GetFrameLevel() == parent:GetFrameLevel())
                        requireCondition("standalone stars texture", type(child.Stars) == "table")
                        requireCondition("standalone animation group", type(child.Anim) == "table")

                        requireCondition("panel StarsAnimations table", type(AzeriteEssenceUI.StarsAnimations) == "table")
                        requireCondition("panel StarsAnimations count", #AzeriteEssenceUI.StarsAnimations == 3)
                        requireCondition("panel star frame 1", AzeriteEssenceUI.StarsAnimations[1] == AzeriteEssenceUI.StarsAnimationFrame1)
                        requireCondition("panel star frame 2", AzeriteEssenceUI.StarsAnimations[2] == AzeriteEssenceUI.StarsAnimationFrame2)
                        requireCondition("panel star frame 3", AzeriteEssenceUI.StarsAnimations[3] == AzeriteEssenceUI.StarsAnimationFrame3)

                        for index, starFrame in ipairs(AzeriteEssenceUI.StarsAnimations) do
                            requireCondition("panel star " .. index .. " uses parent level", starFrame:IsUsingParentLevel())
                            requireCondition("panel star " .. index .. " follows panel level", starFrame:GetFrameLevel() == AzeriteEssenceUI:GetFrameLevel())
                        end

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("stars animation template check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` stars animation template wiring did not match XML contract:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking stars animation template:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_minimal_azerite_essence_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        has_neck_equipped: true,
        neck_power_level: 50,
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
        spell_id: 116,
        rank: None,
        active_essence_id: None,
    }
}
