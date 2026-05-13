use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_registers_and_instantiates_templates() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_minimal_azerite_essence_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let missing: String = env
                    .eval(
                        r#"
                        local templates = {
                            "AzeriteMilestoneBaseTemplate",
                            "AzeriteMilestoneMinorSlotTemplate",
                            "AzeriteMilestoneRankedTemplate",
                            "AzeriteMilestoneMajorSlotTemplate",
                            "AzeriteMilestoneStaminaTemplate",
                            "AzeriteEssenceDependencyLineTemplate",
                            "AzeriteEssenceButtonTemplate",
                            "AzeriteEssenceHeaderButtonTemplate",
                            "AzeriteEssenceStarsAnimationFrameTemplate",
                        }

                        local missing = {}
                        for _, templateName in ipairs(templates) do
                            if C_XMLUtil.GetTemplateInfo(templateName) == nil then
                                table.insert(missing, templateName .. " registry")
                            else
                                local ok, frameOrError = pcall(CreateFrame, "Frame", nil, UIParent, templateName)
                                if not ok then
                                    table.insert(missing, templateName .. " instantiate: " .. tostring(frameOrError))
                                elseif frameOrError == nil then
                                    table.insert(missing, templateName .. " instantiate nil")
                                elseif frameOrError.OnLoad ~= nil and type(frameOrError.OnLoad) ~= "function" then
                                    table.insert(missing, templateName .. " OnLoad")
                                end
                            end
                        end

                        return table.concat(missing, "\n")
                    "#,
                    )
                    .expect("template registry check should run");
                assert!(
                    missing.is_empty(),
                    "`{ROOT}` missing expected template registry surface:\n{missing}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking templates:\n{}",
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
        spell_id: 100_100,
        rank: None,
        active_essence_id: None,
    }
}
