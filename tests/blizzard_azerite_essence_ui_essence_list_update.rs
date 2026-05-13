use crate::common;

use std::collections::HashMap;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::lua_api::state::{
    AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo, AzeriteEssenceState, SimState,
};

const ROOT: &str = "Blizzard_AzeriteEssenceUI";
const MAIN_SLOT: i32 = 0;

#[test]
fn blizzard_azerite_essence_ui_essence_list_hides_invalid_essences_when_collapsed() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                seed_essence_list_state(&mut env.state().borrow_mut());
                clear_recorded_lua_errors(env);
                env.exec(r#"SetCVar("otherRolesAzeriteEssencesHidden", "1")"#)
                    .expect("hidden-invalid-essences CVar should be seeded before addon load");

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteEssenceUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteEssenceUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local capturedDataProvider
                        local originalSetDataProvider = AzeriteEssenceUI.EssenceList.ScrollBox.SetDataProvider

                        local function requireCondition(label, condition)
                            if not condition then
                                table.insert(failures, label)
                            end
                        end

                        AzeriteEssenceUI.EssenceList.ScrollBox.SetDataProvider = function(self, dataProvider, ...)
                            capturedDataProvider = dataProvider
                            return originalSetDataProvider(self, dataProvider, ...)
                        end

                        AzeriteEssenceUI:Show()
                        AzeriteEssenceUI.EssenceList:Show()
                        AzeriteEssenceUI.EssenceList:Update()

                        requireCondition("collapsed invalid section", not AzeriteEssenceUI.EssenceList:ShouldShowInvalidEssences())
                        requireCondition("cached essences include header", #AzeriteEssenceUI.EssenceList:GetCachedEssences() == 5)
                        requireCondition("header index", AzeriteEssenceUI.EssenceList:GetHeaderIndex() == 4)
                        requireCondition("data provider captured", type(capturedDataProvider) == "table")

                        local renderedIDs = {}
                        local renderedInvalidIDs = {}
                        local renderedHeaderCount = 0
                        if capturedDataProvider then
                            requireCondition("data provider size", capturedDataProvider:GetSize() == 4)
                            for index, essenceInfo in capturedDataProvider:Enumerate() do
                                if essenceInfo.isHeader then
                                    renderedHeaderCount = renderedHeaderCount + 1
                                elseif essenceInfo.valid then
                                    table.insert(renderedIDs, essenceInfo.ID)
                                else
                                    table.insert(renderedInvalidIDs, essenceInfo.ID)
                                end
                            end
                        end

                        table.sort(renderedIDs)
                        requireCondition("three valid essence rows", #renderedIDs == 3)
                        requireCondition("valid essence 101", renderedIDs[1] == 101)
                        requireCondition("valid essence 102", renderedIDs[2] == 102)
                        requireCondition("valid essence 103", renderedIDs[3] == 103)
                        requireCondition("invalid essence excluded", #renderedInvalidIDs == 0)
                        requireCondition("collapsed header remains", renderedHeaderCount == 1)
                        requireCondition("num unlocked seeded", C_AzeriteEssence.GetNumUnlockedEssences() == 4)

                        return table.concat(failures, "\n")
                    "#,
                    )
                    .expect("EssenceList filtering check should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` EssenceList did not filter invalid essences as expected:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking EssenceList filtering:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_essence_list_state(state: &mut SimState) {
    state.azerite_essence = AzeriteEssenceState {
        milestones: vec![main_slot_milestone()],
        essences: essence_map(),
        essence_order: vec![101, 102, 103, 199],
        num_unlocked: 4,
        has_neck_equipped: true,
        neck_power_level: 50,
        ..AzeriteEssenceState::default()
    };
}

fn essence_map() -> HashMap<i32, AzeriteEssenceInfo> {
    [
        valid_essence(101, "Valid Rank One", 1),
        valid_essence(102, "Valid Rank Two", 2),
        valid_essence(103, "Valid Rank Three", 3),
        AzeriteEssenceInfo {
            id: 199,
            name: "Invalid Essence".to_string(),
            rank: 4,
            icon: 199_000,
            unlocked: true,
            valid: false,
            access_rank: 4,
            has_never_activated: false,
        },
    ]
    .into_iter()
    .map(|essence| (essence.id, essence))
    .collect()
}

fn valid_essence(id: i32, name: &str, rank: i32) -> AzeriteEssenceInfo {
    AzeriteEssenceInfo {
        id,
        name: name.to_string(),
        rank,
        icon: id * 1_000,
        unlocked: true,
        valid: true,
        access_rank: rank,
        has_never_activated: false,
    }
}

fn main_slot_milestone() -> AzeriteEssenceMilestoneInfo {
    AzeriteEssenceMilestoneInfo {
        id: 5_001,
        required_level: 50,
        slot: Some(MAIN_SLOT),
        unlocked: true,
        can_unlock: false,
        is_major_slot: true,
        swirl_scale: 1.0,
        requires_only_aura: false,
        spell_id: 50_001,
        rank: None,
        active_essence_id: None,
    }
}
