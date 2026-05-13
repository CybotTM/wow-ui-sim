use crate::common;

use std::collections::HashMap;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{
    AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState,
};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const MILESTONE_ID: i32 = 100;
const ESSENCE_ID: i32 = 200;

#[test]
fn blizzard_azerite_essence_ui_can_activate_essence_respects_combat_lockout() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_can_activate_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                env.state().borrow_mut().player.in_combat = true;
                let blocked: bool = env
                    .eval("return C_AzeriteEssence.CanActivateEssence(200, 100)")
                    .expect("combat-blocked CanActivateEssence should run");
                assert!(!blocked);

                env.state().borrow_mut().player.in_combat = false;
                let allowed: bool = env
                    .eval("return C_AzeriteEssence.CanActivateEssence(200, 100)")
                    .expect("out-of-combat CanActivateEssence should run");
                assert!(allowed);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking CanActivateEssence combat gate:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_can_activate_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![major_milestone()],
        essences: essence_map(),
        essence_order: vec![ESSENCE_ID],
        has_neck_equipped: true,
        neck_power_level: 50,
        ..AzeriteEssenceState::default()
    };
}

fn essence_map() -> HashMap<i32, AzeriteEssenceInfo> {
    [AzeriteEssenceInfo {
        id: ESSENCE_ID,
        name: "Combat Gate Essence".to_string(),
        rank: 1,
        icon: 200_100,
        unlocked: true,
        valid: true,
        access_rank: 1,
        has_never_activated: false,
    }]
    .into_iter()
    .map(|essence| (essence.id, essence))
    .collect()
}

fn major_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: MILESTONE_ID,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 100_200,
        rank: None,
        active_essence_id: None,
    }
}
