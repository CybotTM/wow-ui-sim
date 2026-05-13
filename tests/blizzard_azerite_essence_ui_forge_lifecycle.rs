use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_forge_lifecycle_opens_and_closes_panel() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_forge_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local forgeOpenCount = 0
                        local forgeCloseCount = 0

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        local listener = CreateFrame("Frame")
                        listener:RegisterEvent("AZERITE_ESSENCE_FORGE_OPEN")
                        listener:RegisterEvent("AZERITE_ESSENCE_FORGE_CLOSE")
                        listener:SetScript("OnEvent", function(_, event)
                            if event == "AZERITE_ESSENCE_FORGE_OPEN" then
                                forgeOpenCount = forgeOpenCount + 1
                            elseif event == "AZERITE_ESSENCE_FORGE_CLOSE" then
                                forgeCloseCount = forgeCloseCount + 1
                            end
                        end)

                        requireCondition("starts at forge", C_AzeriteEssence.IsAtForge())
                        FireEvent("AZERITE_ESSENCE_FORGE_OPEN")
                        requireCondition("forge open event", forgeOpenCount == 1)

                        local shown = AzeriteEssenceUI:TryShow()
                        requireCondition("TryShow succeeded", shown == true)
                        requireCondition("panel shown", AzeriteEssenceUI:IsShown())

                        C_AzeriteEssence.CloseForge()

                        requireCondition("forge close event", forgeCloseCount == 1)
                        requireCondition("forge state cleared", not C_AzeriteEssence.IsAtForge())
                        requireCondition("panel hidden after close", not AzeriteEssenceUI:IsShown())

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("forge lifecycle check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` forge lifecycle did not open and close cleanly:\n{failures}"
                );

                assert!(!env.state().borrow().azerite_essence.is_at_forge);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking forge lifecycle:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_forge_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![major_milestone()],
        has_neck_equipped: true,
        neck_power_level: 50,
        is_at_forge: true,
        ..AzeriteEssenceState::default()
    };
}

fn major_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 7_001,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 70_001,
        rank: None,
        active_essence_id: None,
    }
}
