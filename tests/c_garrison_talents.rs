//! Integration tests for the `C_Garrison.GetTalentInfo` and
//! `C_Garrison.GetTalentUnlockWorldQuest` probes plus the
//! `GetGarrisonTalentCostString` global helper.
//!
//! Drives `Blizzard_AnimaDiversionUI/AnimaDiversionDataProvider.lua`
//! (`AnimaDiversionPinMixin:HaveEnoughAnimaToActivate` reads
//! `talentInfo.researchCurrencyCosts`,
//! `RefreshTooltip` reads the unlock-quest id) and
//! `vendor/wow-ui-source/Interface/AddOns/Blizzard_FrameXMLUtil/Mainline/GarrisonBaseUtils.lua`
//! (`GetGarrisonTalentCostString`).

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{GarrisonTalentCurrencyCostInfo, GarrisonTalentInfo};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn seed_two_talents(env: &WowLuaEnv) {
    let mut sim = env.state().borrow_mut();
    sim.garrison_talents
        .talents
        .insert(201, bolster_bastion_talent());
    sim.garrison_talents
        .talents
        .insert(202, ascendant_echo_talent());
    sim.garrison_talents.unlock_world_quests.insert(202, 67890);
}

fn bolster_bastion_talent() -> GarrisonTalentInfo {
    GarrisonTalentInfo {
        id: 201,
        name: "Bolster Bastion".to_string(),
        description: "Reinforce Bastion's anima reservoir.".to_string(),
        icon: 3528287,
        tier: 1,
        ui_order: 0,
        talent_rank: 0,
        talent_max_rank: 1,
        is_being_researched: false,
        researched: false,
        selected: false,
        perk_spell_id: 350001,
        talent_availability: 1,
        research_duration: 0,
        start_time: 0,
        time_remaining: 0,
        research_gold_cost: 0,
        research_currency_costs: vec![currency_cost(1820, 500)],
    }
}

fn ascendant_echo_talent() -> GarrisonTalentInfo {
    GarrisonTalentInfo {
        id: 202,
        name: "Ascendant Echo".to_string(),
        description: "Permanent power for the Ascended.".to_string(),
        icon: 3528288,
        tier: 2,
        ui_order: 1,
        talent_rank: 0,
        talent_max_rank: 1,
        is_being_researched: true,
        researched: false,
        selected: false,
        perk_spell_id: 350002,
        talent_availability: 2,
        research_duration: 600,
        start_time: 1_700_000_000,
        time_remaining: 300,
        research_gold_cost: 50,
        research_currency_costs: vec![currency_cost(1820, 1500), currency_cost(1822, 25)],
    }
}

fn currency_cost(currency_type: i64, currency_quantity: i64) -> GarrisonTalentCurrencyCostInfo {
    GarrisonTalentCurrencyCostInfo {
        currency_type,
        currency_quantity,
    }
}

#[test]
fn get_talent_info_returns_seeded_shape() {
    let env = env();
    seed_two_talents(&env);
    let name: String = env
        .eval("return C_Garrison.GetTalentInfo(201).name")
        .unwrap();
    assert_eq!(name, "Bolster Bastion");

    let perk_spell_id: f64 = env
        .eval("return C_Garrison.GetTalentInfo(201).perkSpellID")
        .unwrap();
    assert_eq!(
        perk_spell_id as i64, 350001,
        "perkSpellID must round-trip via Lua marshalling"
    );

    let is_researching: bool = env
        .eval("return C_Garrison.GetTalentInfo(202).isBeingResearched")
        .unwrap();
    assert!(
        is_researching,
        "isBeingResearched must mirror the seeded boolean"
    );

    let cost_count: f64 = env
        .eval("return #C_Garrison.GetTalentInfo(202).researchCurrencyCosts")
        .unwrap();
    assert_eq!(
        cost_count as i64, 2,
        "researchCurrencyCosts must be a 1-based array of GarrisonTalentCurrencyCostInfo rows"
    );

    let second_currency: f64 = env
        .eval("return C_Garrison.GetTalentInfo(202).researchCurrencyCosts[2].currencyType")
        .unwrap();
    assert_eq!(
        second_currency as i64, 1822,
        "second cost row must surface the seeded currencyType"
    );

    let second_quantity: f64 = env
        .eval("return C_Garrison.GetTalentInfo(202).researchCurrencyCosts[2].currencyQuantity")
        .unwrap();
    assert_eq!(
        second_quantity as i64, 25,
        "second cost row must surface the seeded currencyQuantity"
    );
}

#[test]
fn get_talent_info_returns_nil_when_unknown() {
    let env = env();
    seed_two_talents(&env);
    let kind: String = env
        .eval("return type(C_Garrison.GetTalentInfo(999))")
        .unwrap();
    assert_eq!(
        kind, "nil",
        "unknown talent ids must return nil per the official Nilable annotation"
    );
}

#[test]
fn get_talent_unlock_world_quest_returns_seeded_id() {
    let env = env();
    seed_two_talents(&env);
    let quest_id: f64 = env
        .eval("return C_Garrison.GetTalentUnlockWorldQuest(202)")
        .unwrap();
    assert_eq!(
        quest_id as i64, 67890,
        "unlock world quest id must round-trip the seeded value"
    );
}

#[test]
fn get_talent_unlock_world_quest_returns_nil_when_absent() {
    let env = env();
    seed_two_talents(&env);
    let kind: String = env
        .eval("return type(C_Garrison.GetTalentUnlockWorldQuest(201))")
        .unwrap();
    assert_eq!(
        kind, "nil",
        "talents without an unlock quest must return nil; the API is documented as MayReturnNothing"
    );
}

#[test]
fn cost_string_joins_seeded_rows_with_color_code() {
    let env = env();
    seed_two_talents(&env);
    let formatted: String = env
        .eval(
            r#"
            local info = C_Garrison.GetTalentInfo(202)
            return GetGarrisonTalentCostString(info, false, "|cFFFFFFFF")
            "#,
        )
        .unwrap();
    assert_eq!(
        formatted, "|cFFFFFFFF1500|T1820:14|t|r |cFFFFFFFF25|T1822:14|t|r",
        "two-row cost string must concatenate with the supplied color prefix and reset suffix"
    );
}

#[test]
fn cost_string_omits_color_when_unset() {
    let env = env();
    seed_two_talents(&env);
    let formatted: String = env
        .eval(
            r#"
            local info = C_Garrison.GetTalentInfo(201)
            return GetGarrisonTalentCostString(info, false)
            "#,
        )
        .unwrap();
    assert_eq!(
        formatted, "500|T1820:14|t",
        "missing color code arg must produce the bare quantity/icon glyph"
    );
}

#[test]
fn cost_string_abbreviates_when_requested() {
    let env = env();
    seed_two_talents(&env);
    let formatted: String = env
        .eval(
            r#"
            local info = C_Garrison.GetTalentInfo(202)
            return GetGarrisonTalentCostString(info, true)
            "#,
        )
        .unwrap();
    assert!(
        formatted.contains("1.5k|T1820:14|t"),
        "abbreviated cost must collapse 1500 → 1.5k; got {formatted}"
    );
    assert!(
        formatted.contains("25|T1822:14|t"),
        "small quantities must stay un-abbreviated; got {formatted}"
    );
}

#[test]
fn cost_string_returns_nil_for_empty_cost_list() {
    let env = env();
    let kind: String = env
        .eval(
            r#"
            return type(GetGarrisonTalentCostString({researchCurrencyCosts = {}}, false))
            "#,
        )
        .unwrap();
    assert_eq!(
        kind, "nil",
        "empty cost list must return nil so callers can branch on missing data"
    );
}
