use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_should_open_bags_tracks_unlocked_essence_count() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_azerite_essence_state(&mut env.state().borrow_mut(), 0);
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let should_open_empty: bool = env
                    .eval("return AzeriteEssenceUI:ShouldOpenBagsOnShow()")
                    .expect("empty essence count bag gate should run");
                assert!(!should_open_empty);

                env.state().borrow_mut().azerite_essence.num_unlocked = 1;

                let should_open_with_unlocked: bool = env
                    .eval("return AzeriteEssenceUI:ShouldOpenBagsOnShow()")
                    .expect("non-empty essence count bag gate should run");
                assert!(should_open_with_unlocked);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking bag-open gating:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_azerite_essence_state(state: &mut SimState, num_unlocked: i32) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        has_neck_equipped: true,
        neck_power_level: 50,
        num_unlocked,
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
