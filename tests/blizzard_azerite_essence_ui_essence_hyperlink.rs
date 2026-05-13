use crate::common;

use std::collections::HashMap;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{
    AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState,
};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const ESSENCE_ID: i32 = 200;
const ESSENCE_NAME: &str = "Anima of Life and Death";
const EXPECTED_LINK: &str = "|cffa335ee|Hazessence:200:3|h[Anima of Life and Death]|h|r";

#[test]
fn blizzard_azerite_essence_ui_essence_hyperlink_uses_canonical_bytes() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_hyperlink_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let link: String = env
                    .eval("return C_AzeriteEssence.GetEssenceHyperlink(200, 3)")
                    .expect("GetEssenceHyperlink should return a string for seeded essence");
                assert_eq!(link, EXPECTED_LINK);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking essence hyperlink:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_hyperlink_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        essences: essence_map(),
        essence_order: vec![ESSENCE_ID],
        has_neck_equipped: true,
        ..AzeriteEssenceState::default()
    };
}

fn essence_map() -> HashMap<i32, AzeriteEssenceInfo> {
    [AzeriteEssenceInfo {
        id: ESSENCE_ID,
        name: ESSENCE_NAME.to_string(),
        rank: 3,
        icon: 200_300,
        unlocked: true,
        valid: true,
        access_rank: 3,
        has_never_activated: false,
    }]
    .into_iter()
    .map(|essence| (essence.id, essence))
    .collect()
}

fn main_slot_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 8_001,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 80_001,
        rank: None,
        active_essence_id: None,
    }
}
