use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState};
use wow_ui_sim::lua_api::{AzeriteItemState, ItemLocationData};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_refreshes_power_level_badge_from_event() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_power_level_state(&mut env.state().borrow_mut(), 75);
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local originalRefreshMilestones = AzeriteEssenceUI.RefreshMilestones
                        AzeriteEssenceUI._testRefreshMilestonesCalls = 0

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        Item.CreateFromItemLocation = function()
                            return {
                                ContinueOnItemLoad = function(_, callback)
                                    callback()
                                end,
                                GetItemIcon = function()
                                    return 0
                                end,
                                GetItemName = function()
                                    return "Heart of Azeroth"
                                end,
                            }
                        end

                        AzeriteEssenceUI.RefreshMilestones = function(self)
                            self._testRefreshMilestonesCalls = self._testRefreshMilestonesCalls + 1
                            return originalRefreshMilestones(self)
                        end

                        AzeriteEssenceUI:Show()
                        requireCondition("badge shown after show", AzeriteEssenceUI.PowerLevelBadgeFrame:IsShown())
                        requireCondition(
                            "badge text after show",
                            AzeriteEssenceUI.PowerLevelBadgeFrame.Label:GetText() == "75"
                        )
                        requireCondition("show refreshed milestones", AzeriteEssenceUI._testRefreshMilestonesCalls == 1)

                        AzeriteEssenceUI._testRefreshMilestonesCalls = 0
                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("initial power-level badge check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` did not display the seeded power level:\n{failures}"
                );

                env.state()
                    .borrow_mut()
                    .azerite_item
                    .as_mut()
                    .expect("seeded azerite item")
                    .power_level = 80;
                env.exec(r#"AzeriteEssenceUI:OnEvent("AZERITE_ITEM_POWER_LEVEL_CHANGED")"#)
                    .expect("power level changed event branch should dispatch");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        requireCondition("badge shown after event", AzeriteEssenceUI.PowerLevelBadgeFrame:IsShown())
                        requireCondition(
                            "badge text after event",
                            AzeriteEssenceUI.PowerLevelBadgeFrame.Label:GetText() == "80"
                        )
                        requireCondition("event refreshed milestones", AzeriteEssenceUI._testRefreshMilestonesCalls == 1)

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("event power-level badge check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` did not refresh the power level badge from event:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking power level refresh:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_power_level_state(state: &mut SimState, power_level: i32) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![power_level_milestone()],
        has_neck_equipped: true,
        neck_power_level: power_level,
        ..AzeriteEssenceState::default()
    };
    state.azerite_item = Some(AzeriteItemState {
        item_location: ItemLocationData {
            bag_id: Some(0),
            slot_index: Some(1),
            ..ItemLocationData::default()
        },
        current_xp: 0,
        max_xp: 1_000,
        power_level,
        unlimited_power_level: 0,
        unlimited_unlocked: false,
        at_max_level: false,
        enabled: true,
    });
}

fn power_level_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 4_001,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 40_001,
        rank: None,
        active_essence_id: None,
    }
}
