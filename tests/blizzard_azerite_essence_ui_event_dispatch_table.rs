use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use rilua::Val;
use wow_ui_sim::lua_api::state::{
    AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState,
};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;
const ESSENCE_ID: i32 = 200;
const MILESTONE_ID: i32 = 100;

#[test]
fn blizzard_azerite_essence_ui_dispatches_registered_events_to_expected_refresh_paths() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_dispatch_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                install_dispatch_spies(env);

                let cases = [
                    EventCase::new(
                        "AZERITE_ESSENCE_CHANGED",
                        vec![Val::Num(ESSENCE_ID.into()), Val::Num(3.0)],
                    ),
                    EventCase::new(
                        "AZERITE_ESSENCE_ACTIVATED",
                        vec![Val::Num(ESSENCE_ID.into()), Val::Num(MILESTONE_ID.into())],
                    ),
                    EventCase::new("AZERITE_ESSENCE_ACTIVATION_FAILED", Vec::new()),
                    EventCase::new("AZERITE_ESSENCE_UPDATE", Vec::new()),
                    EventCase::new("AZERITE_ESSENCE_FORGE_OPEN", Vec::new()),
                    EventCase::new("AZERITE_ESSENCE_FORGE_CLOSE", Vec::new()),
                    EventCase::new(
                        "AZERITE_ESSENCE_MILESTONE_UNLOCKED",
                        vec![Val::Num(MILESTONE_ID.into())],
                    ),
                    EventCase::new(
                        "AZERITE_ITEM_POWER_LEVEL_CHANGED",
                        vec![Val::Nil, Val::Num(50.0), Val::Num(50.0)],
                    ),
                    EventCase::new("AZERITE_ITEM_ENABLED_STATE_CHANGED", Vec::new()),
                ];

                for case in cases {
                    assert_registered(env, case.name);
                    reset_dispatch_counts(env);
                    env.fire_event_with_args(case.name, &case.args)
                        .unwrap_or_else(|err| {
                            panic!("{} should dispatch without error: {err}", case.name)
                        });
                    assert_dispatch_counts(env, case.name);
                }

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking event dispatch:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

struct EventCase<'a> {
    name: &'a str,
    args: Vec<Val>,
}

impl<'a> EventCase<'a> {
    fn new(name: &'a str, args: Vec<Val>) -> Self {
        Self { name, args }
    }
}

const DISPATCH_SPIES_LUA: &str = r#"
        AzeriteEssenceUI:Show()

        local counts = {}
        local expectedByEvent = {
            AZERITE_ESSENCE_CHANGED = {
                RefreshSlots = 1,
                ["EssenceList.Update"] = 1,
                ["EssenceList.OnEssenceChanged"] = 1,
                ["LearnAnim.PlayAnim"] = 1,
            },
            AZERITE_ESSENCE_ACTIVATED = {
                ClearNewlyActivatedEssence = 1,
                RefreshSlots = 1,
                ["EssenceList.Update"] = 1,
            },
            AZERITE_ESSENCE_ACTIVATION_FAILED = {
                ClearNewlyActivatedEssence = 1,
                RefreshSlots = 1,
                ["EssenceList.Update"] = 1,
            },
            AZERITE_ESSENCE_UPDATE = {
                ClearNewlyActivatedEssence = 1,
                RefreshSlots = 1,
                ["EssenceList.Update"] = 1,
            },
            AZERITE_ESSENCE_FORGE_OPEN = {
                RefreshMilestones = 1,
            },
            AZERITE_ESSENCE_FORGE_CLOSE = {
                RefreshMilestones = 1,
            },
            AZERITE_ESSENCE_MILESTONE_UNLOCKED = {
                RefreshMilestones = 1,
                ["Milestone.OnUnlocked"] = 1,
            },
            AZERITE_ITEM_POWER_LEVEL_CHANGED = {
                RefreshPowerLevel = 1,
                RefreshMilestones = 1,
            },
            AZERITE_ITEM_ENABLED_STATE_CHANGED = {
                RefreshPowerLevel = 1,
                RefreshMilestones = 1,
                RefreshSlots = 1,
                ["EssenceList.Update"] = 1,
                UpdateEnabledAppearance = 1,
            },
        }

        local function bump(key)
            counts[key] = (counts[key] or 0) + 1
        end

        local function wrap(object, methodName, key)
            local original = object[methodName]
            object[methodName] = function(self, ...)
                bump(key)
                return original(self, ...)
            end
        end

        wrap(AzeriteEssenceUI, "RefreshMilestones", "RefreshMilestones")
        wrap(AzeriteEssenceUI, "RefreshSlots", "RefreshSlots")
        wrap(AzeriteEssenceUI, "RefreshPowerLevel", "RefreshPowerLevel")
        wrap(AzeriteEssenceUI, "ClearNewlyActivatedEssence", "ClearNewlyActivatedEssence")
        wrap(AzeriteEssenceUI, "UpdateEnabledAppearance", "UpdateEnabledAppearance")
        wrap(AzeriteEssenceUI.EssenceList, "Update", "EssenceList.Update")
        wrap(AzeriteEssenceUI.EssenceList, "OnEssenceChanged", "EssenceList.OnEssenceChanged")
        wrap(AzeriteEssenceLearnAnimFrame, "PlayAnim", "LearnAnim.PlayAnim")

        local milestone = AzeriteEssenceUI:GetMilestoneFrame(100)
        wrap(milestone, "OnUnlocked", "Milestone.OnUnlocked")

        function __reset_azerite_dispatch_counts()
            for key in pairs(counts) do
                counts[key] = nil
            end
        end

        function __assert_azerite_dispatch_counts(eventName)
            local failures = {}
            local expected = expectedByEvent[eventName]
            if not expected then
                return "missing expected dispatch table for " .. eventName
            end

            for key, count in pairs(expected) do
                if counts[key] ~= count then
                    table.insert(failures, eventName .. " expected " .. key .. "=" .. count .. ", got " .. tostring(counts[key]))
                end
            end

            for key, count in pairs(counts) do
                if expected[key] == nil and count ~= 0 then
                    table.insert(failures, eventName .. " unexpectedly called " .. key .. "=" .. count)
                end
            end

            return table.concat(failures, "\n")
        end
    "#;

fn install_dispatch_spies(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(DISPATCH_SPIES_LUA)
        .expect("event dispatch spies should install");
}

fn assert_registered(env: &wow_ui_sim::lua_api::WowLuaEnv, event_name: &str) {
    let is_registered: bool = env
        .eval(&format!(
            r#"return AzeriteEssenceUI:IsEventRegistered("{event_name}")"#
        ))
        .unwrap_or_else(|err| panic!("{event_name} registration check should run: {err}"));
    assert!(
        is_registered,
        "`{ROOT}` should register `{event_name}` while shown"
    );
}

fn reset_dispatch_counts(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec("__reset_azerite_dispatch_counts()")
        .expect("dispatch spy counts should reset");
}

fn assert_dispatch_counts(env: &wow_ui_sim::lua_api::WowLuaEnv, event_name: &str) {
    let failures: String = env
        .eval(&format!(
            r#"return __assert_azerite_dispatch_counts("{event_name}")"#
        ))
        .unwrap_or_else(|err| panic!("{event_name} dispatch assertion should run: {err}"));
    assert!(
        failures.is_empty(),
        "`{ROOT}` did not dispatch `{event_name}` through the expected refresh path:\n{failures}"
    );
}

fn seed_dispatch_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![major_milestone()],
        essences: [AzeriteEssenceInfo {
            id: ESSENCE_ID,
            name: "Dispatch Essence".to_string(),
            rank: 3,
            icon: 200_000,
            unlocked: true,
            valid: true,
            access_rank: 3,
            has_never_activated: false,
        }]
        .into_iter()
        .map(|essence| (essence.id, essence))
        .collect(),
        essence_order: vec![ESSENCE_ID],
        has_neck_equipped: true,
        neck_power_level: 50,
        ..AzeriteEssenceState::default()
    };
}

fn major_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: MILESTONE_ID,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: true,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 100_200,
        rank: None,
        active_essence_id: None,
    }
}
