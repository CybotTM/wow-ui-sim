//! `AnimaDiversionPinMixin:HaveEnoughAnimaToActivate` currency probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const HAVE_ENOUGH_ANIMA_PROBE: &str = r#"
local originalGetTalentInfo = C_Garrison.GetTalentInfo
local originalGetAnimaInfo = C_CovenantSanctumUI.GetAnimaInfo
local originalGetCurrencyInfo = C_CurrencyInfo.GetCurrencyInfo
local quantity = 400
local costs = {
    { currencyType = 1820, currencyQuantity = 500 },
}
local talentIDs = {}
local requestedCurrencies = {}

C_Garrison.GetTalentInfo = function(talentID)
    table.insert(talentIDs, talentID)
    return { researchCurrencyCosts = costs }
end
C_CovenantSanctumUI.GetAnimaInfo = function()
    return 1820, 1000
end
C_CurrencyInfo.GetCurrencyInfo = function(currencyID)
    table.insert(requestedCurrencies, currencyID)
    return { quantity = quantity }
end

local pin = {
    nodeData = {
        talentID = 777,
    },
}
setmetatable(pin, { __index = AnimaDiversionPinMixin })

local underfunded = pin:HaveEnoughAnimaToActivate()

quantity = 600
local funded = pin:HaveEnoughAnimaToActivate()

costs = {
    { currencyType = 9999, currencyQuantity = 500 },
}
quantity = 400
local noMatchingCost = pin:HaveEnoughAnimaToActivate()

C_Garrison.GetTalentInfo = originalGetTalentInfo
C_CovenantSanctumUI.GetAnimaInfo = originalGetAnimaInfo
C_CurrencyInfo.GetCurrencyInfo = originalGetCurrencyInfo

return underfunded,
       funded,
       noMatchingCost,
       #talentIDs,
       talentIDs[1],
       talentIDs[2],
       talentIDs[3],
       #requestedCurrencies,
       requestedCurrencies[1],
       requestedCurrencies[2],
       requestedCurrencies[3]
"#;

#[test]
fn have_enough_anima_compares_matching_currency_cost() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: HaveEnoughAnimaState = env
            .eval(HAVE_ENOUGH_ANIMA_PROBE)
            .expect("have enough anima probe must run cleanly");

        assert_have_enough_anima_state(state);
    });
}

type HaveEnoughAnimaState = (bool, bool, bool, i64, i64, i64, i64, i64, i64, i64, i64);

fn assert_have_enough_anima_state(state: HaveEnoughAnimaState) {
    assert_currency_comparison_results((state.0, state.1, state.2));
    assert_talent_queries((state.3, state.4, state.5, state.6));
    assert_currency_queries((state.7, state.8, state.9, state.10));
}

fn assert_currency_comparison_results(state: (bool, bool, bool)) {
    let (underfunded, funded, no_matching_cost) = state;

    assert!(
        !underfunded,
        "Quantity below the matching anima cost must return false"
    );
    assert!(
        funded,
        "Quantity above the matching anima cost must return true"
    );
    assert!(
        no_matching_cost,
        "No cost for the active anima currency must return true"
    );
}

fn assert_talent_queries(state: (i64, i64, i64, i64)) {
    let (query_count, first_talent_id, second_talent_id, third_talent_id) = state;

    assert_eq!(query_count, 3, "`GetTalentInfo` must be read per check");
    assert_eq!(first_talent_id, 777, "First check must query pin talent ID");
    assert_eq!(
        second_talent_id, 777,
        "Second check must query pin talent ID"
    );
    assert_eq!(third_talent_id, 777, "Third check must query pin talent ID");
}

fn assert_currency_queries(state: (i64, i64, i64, i64)) {
    let (query_count, first_currency_id, second_currency_id, third_currency_id) = state;

    assert_eq!(
        query_count, 3,
        "`GetCurrencyInfo` must be read for the active anima currency per check"
    );
    assert_eq!(
        first_currency_id, 1820,
        "First check must query the anima currency"
    );
    assert_eq!(
        second_currency_id, 1820,
        "Second check must query the anima currency"
    );
    assert_eq!(
        third_currency_id, 1820,
        "Third check must query the anima currency"
    );
}
