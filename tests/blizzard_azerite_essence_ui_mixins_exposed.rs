use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_exposes_mixins_after_load() {
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
                        local function requireTable(name)
                            if type(_G[name]) ~= "table" then
                                table.insert(missing, name)
                            end
                        end

                        requireTable("AzeriteEssenceUIMixin")
                        requireTable("AzeriteEssenceDependencyLineMixin")
                        requireTable("AzeriteEssenceListMixin")
                        requireTable("AzeriteEssenceButtonMixin")
                        requireTable("AzeriteEssenceHeaderButtonMixin")
                        requireTable("AzeriteMilestoneBaseMixin")
                        requireTable("AzeriteMilestoneSlotMixin")
                        requireTable("AzeriteMilestoneStaminaMixin")
                        requireTable("AzeriteMilestoneRankedMixin")
                        requireTable("AzeriteEssenceLearnAnimFrameMixin")

                        if type(AzeriteEssenceUIMixin.RegisterCallback) ~= "function" then
                            table.insert(missing, "AzeriteEssenceUIMixin.RegisterCallback")
                        end
                        if type(AzeriteEssenceDependencyLineMixin.SetEndPoints) ~= "function" then
                            table.insert(missing, "AzeriteEssenceDependencyLineMixin.SetEndPoints")
                        end

                        return table.concat(missing, "\n")
                    "#,
                    )
                    .expect("mixin surface check should run");
                assert!(
                    missing.is_empty(),
                    "`{ROOT}` missing expected mixin surface:\n{missing}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking mixins:\n{}",
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
