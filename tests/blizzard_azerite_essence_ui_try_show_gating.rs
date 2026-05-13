use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_try_show_requires_neck_equipped() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_azerite_essence_state(&mut env.state().borrow_mut(), false);
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let blocked: (bool, bool) = env
                    .eval(
                        r#"
                        local result = AzeriteEssenceUI:TryShow()
                        return result == false, AzeriteEssenceUI:IsShown()
                    "#,
                    )
                    .expect("blocked TryShow should run");
                assert_eq!(blocked, (true, false));

                env.state().borrow_mut().azerite_essence.has_neck_equipped = true;

                let allowed: (bool, bool) = env
                    .eval(
                        r#"
                        local result = AzeriteEssenceUI:TryShow()
                        return result == true, AzeriteEssenceUI:IsShown()
                    "#,
                    )
                    .expect("allowed TryShow should run");
                assert_eq!(allowed, (true, true));

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking TryShow gating:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_azerite_essence_state(state: &mut SimState, has_neck_equipped: bool) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        has_neck_equipped,
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
