use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{
    AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState,
};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const PASSIVE_ONE_SLOT: i32 = 1;
const ESSENCE_ID: i32 = 200;

#[test]
fn blizzard_azerite_essence_ui_reveal_starts_once_and_cancels() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_reveal_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local milestoneBeginCalls = 0
                        local lineBeginCalls = 0
                        local swirlCalls = 0

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        AzeriteEssenceUI:Show()

                        requireCondition("ShouldPlayReveal seeded", AzeriteEssenceUI:ShouldPlayReveal() == true)
                        requireCondition("not initially in progress", AzeriteEssenceUI:IsRevealInProgress() == false)
                        requireCondition("two milestones seeded", #AzeriteEssenceUI.Milestones == 2)
                        requireCondition("one dependency line seeded", #AzeriteEssenceUI.Lines == 1)

                        local firstSlot = AzeriteEssenceUI.Milestones[1]
                        firstSlot.PlayRevealEffect = function() end

                        local originalApplyRevealSwirl = AzeriteEssenceUI.ApplyRevealSwirl
                        AzeriteEssenceUI.ApplyRevealSwirl = function(self, milestoneFrame, delay)
                            swirlCalls = swirlCalls + 1
                            return originalApplyRevealSwirl(self, milestoneFrame, delay)
                        end

                        for _, milestone in ipairs(AzeriteEssenceUI.Milestones) do
                            local originalBeginReveal = milestone.BeginReveal
                            milestone.BeginReveal = function(self, delay)
                                milestoneBeginCalls = milestoneBeginCalls + 1
                                return originalBeginReveal(self, delay)
                            end
                        end

                        for _, line in ipairs(AzeriteEssenceUI.Lines) do
                            local originalBeginReveal = line.BeginReveal
                            line.BeginReveal = function(self, delay, distance)
                                lineBeginCalls = lineBeginCalls + 1
                                return originalBeginReveal(self, delay, distance)
                            end
                        end

                        AzeriteEssenceUI:OnEssenceActivated(200, firstSlot)
                        requireCondition("activation marks reveal in progress", AzeriteEssenceUI:IsRevealInProgress() == true)

                        AzeriteEssenceUI:PlayReveal()

                        requireCondition("reveal still in progress after PlayReveal", AzeriteEssenceUI:IsRevealInProgress() == true)
                        requireCondition("one milestone reveal", milestoneBeginCalls == 1)
                        requireCondition("one line reveal", lineBeginCalls == 1)
                        requireCondition("one swirl reveal", swirlCalls == 1)
                        requireCondition("one reveal tracked", AzeriteEssenceUI.numRevealsPlaying == 1)

                        AzeriteEssenceUI:PlayReveal()

                        requireCondition("second PlayReveal did not repeat milestones", milestoneBeginCalls == 1)
                        requireCondition("second PlayReveal did not repeat lines", lineBeginCalls == 1)
                        requireCondition("second PlayReveal did not repeat swirls", swirlCalls == 1)
                        requireCondition("second PlayReveal preserved count", AzeriteEssenceUI.numRevealsPlaying == 1)

                        AzeriteEssenceUI:CancelReveal()

                        requireCondition("cancel clears reveal progress", AzeriteEssenceUI:IsRevealInProgress() == false)
                        requireCondition("cancel clears reveal count", AzeriteEssenceUI.numRevealsPlaying == nil)

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("Reveal animation lifecycle check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` reveal animation lifecycle did not match Blizzard behavior:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking reveal animation:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_reveal_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![major_milestone(), minor_milestone()],
        essences: [AzeriteEssenceInfo {
            id: ESSENCE_ID,
            name: "First Reveal Essence".to_string(),
            rank: 3,
            icon: 200_000,
            unlocked: true,
            valid: true,
            access_rank: 3,
            has_never_activated: true,
        }]
        .into_iter()
        .map(|essence| (essence.id, essence))
        .collect(),
        essence_order: vec![ESSENCE_ID],
        has_neck_equipped: true,
        has_never_activated: true,
        neck_power_level: 50,
        ..AzeriteEssenceState::default()
    };
}

fn major_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 100,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 10_000,
        rank: None,
        active_essence_id: None,
    }
}

fn minor_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 101,
        required_level: 51,
        slot: Some(PASSIVE_ONE_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: false,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 10_001,
        rank: None,
        active_essence_id: None,
    }
}
