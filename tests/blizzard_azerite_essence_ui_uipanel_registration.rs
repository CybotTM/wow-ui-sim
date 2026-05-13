use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_registers_uipanel_window() {
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
                        local missing = {}
                        local panelInfo = UIPanelWindows and UIPanelWindows.AzeriteEssenceUI
                        if type(panelInfo) ~= "table" then
                            table.insert(missing, "UIPanelWindows.AzeriteEssenceUI")
                        else
                            if panelInfo.area ~= "left" then
                                table.insert(missing, "area=" .. tostring(panelInfo.area))
                            end
                            if panelInfo.pushable ~= 1 then
                                table.insert(missing, "pushable=" .. tostring(panelInfo.pushable))
                            end
                        end
                        return table.concat(missing, "\n")
                    "#,
                    )
                    .expect("UIPanelWindows registration check should run");
                assert!(
                    missing.is_empty(),
                    "`{ROOT}` UIPanelWindows registration did not match addon contract:\n{missing}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking UIPanelWindows registration:\n{}",
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
