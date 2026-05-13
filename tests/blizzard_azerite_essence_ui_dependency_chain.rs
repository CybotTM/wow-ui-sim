use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const DEPENDENCY: &str = "Blizzard_Colors";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_loads_colors_dependency_and_color_mixins() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_minimal_azerite_essence_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let colors_loaded: bool = env
                    .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_Colors")"#)
                    .expect("C_AddOns.IsAddOnLoaded should return for Blizzard_Colors");
                assert!(
                    colors_loaded,
                    "`{DEPENDENCY}` should auto-load before `{ROOT}` finishes"
                );

                let colors_are_mixins: bool = env
                    .eval(
                        r#"
                        local function isColorMixinInstance(color)
                            return type(color) == "table"
                                and type(color.GetRGB) == "function"
                                and type(color.GetRGBA) == "function"
                                and type(color.IsRGBEqualTo) == "function"
                                and type(color.WrapTextInColorCode) == "function"
                                and color:IsRGBEqualTo(color)
                        end
                        return isColorMixinInstance(HEIRLOOM_BLUE_COLOR)
                            and isColorMixinInstance(WHITE_FONT_COLOR)
                    "#,
                    )
                    .expect("ColorMixin instance check should run");
                assert!(
                    colors_are_mixins,
                    "`HEIRLOOM_BLUE_COLOR` and `WHITE_FONT_COLOR` should be ColorMixin instances"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors during dependency-chain load:\n{}",
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
