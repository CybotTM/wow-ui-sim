//! `AnimaDiversionPinMixin:OnClick` routing probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const PIN_CLICK_PROBE: &str = r#"
local state = Enum.AnimaDiversionNodeState
local originalStaticPopupShow = StaticPopup_Show
local originalGetTalentInfo = C_Garrison.GetTalentInfo
local originalGetAnimaInfo = C_CovenantSanctumUI.GetAnimaInfo
local originalGetCurrencyInfo = C_CurrencyInfo.GetCurrencyInfo
local originalDisallowSelection = AnimaDiversionFrame.disallowSelection
local reinforceFrame = AnimaDiversionFrame.ReinforceInfoFrame
local originalSelectNodeToReinforce = reinforceFrame.SelectNodeToReinforce
local canReinforce = false
local popupCalls = {}
local selectedNodes = {}

StaticPopup_Show = function(which, textArg1, textArg2, data)
    table.insert(popupCalls, {
        which = which,
        textArg1 = textArg1,
        textArg2 = textArg2,
        data = data,
    })
end
C_Garrison.GetTalentInfo = function()
    return {
        researchCurrencyCosts = {
            { currencyType = 1820, currencyQuantity = 10 },
        },
    }
end
C_CovenantSanctumUI.GetAnimaInfo = function()
    return 1820, 100
end
C_CurrencyInfo.GetCurrencyInfo = function()
    return { quantity = 25 }
end
reinforceFrame.SelectNodeToReinforce = function(self, pin)
    table.insert(selectedNodes, pin)
end

local owner = {
    CanReinforceNode = function()
        return canReinforce
    end,
}
local function buildPin(nodeData)
    local pin = {
        owner = owner,
        nodeData = nodeData,
    }
    setmetatable(pin, { __index = AnimaDiversionPinMixin })
    return pin
end

local originPin = buildPin(nil)
originPin:OnClick("LeftButton")
local noOpAfterOrigin = #popupCalls == 0 and #selectedNodes == 0

local availablePin = buildPin({
    state = state.Available,
    talentID = 777,
    name = "Flowing Anima",
})
AnimaDiversionFrame.disallowSelection = true
availablePin:OnClick("LeftButton")
local noOpWhileDisallowed = #popupCalls == 0 and #selectedNodes == 0

AnimaDiversionFrame.disallowSelection = false
availablePin:OnClick("RightButton")
local noOpOnRightButton = #popupCalls == 0 and #selectedNodes == 0

canReinforce = false
availablePin:OnClick("LeftButton")
local popup = popupCalls[1]

canReinforce = true
availablePin:OnClick("LeftButton")

StaticPopup_Show = originalStaticPopupShow
C_Garrison.GetTalentInfo = originalGetTalentInfo
C_CovenantSanctumUI.GetAnimaInfo = originalGetAnimaInfo
C_CurrencyInfo.GetCurrencyInfo = originalGetCurrencyInfo
AnimaDiversionFrame.disallowSelection = originalDisallowSelection
reinforceFrame.SelectNodeToReinforce = originalSelectNodeToReinforce

return noOpAfterOrigin,
       noOpWhileDisallowed,
       noOpOnRightButton,
       #popupCalls,
       popup and popup.which == "ANIMA_DIVERSION_CONFIRM_CHANNEL",
       popup and popup.textArg1 == "Flowing Anima",
       popup and popup.textArg2 == nil,
       popup and popup.data == availablePin,
       #selectedNodes,
       selectedNodes[1] == availablePin
"#;

#[test]
fn pin_click_routes_by_reinforce_state_and_ignores_noops() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: PinClickState = env
            .eval(PIN_CLICK_PROBE)
            .expect("pin click probe must run cleanly");

        assert_pin_click_state(state);
    });
}

type PinClickState = (bool, bool, bool, i64, bool, bool, bool, bool, i64, bool);

fn assert_pin_click_state(state: PinClickState) {
    assert_noop_guards((state.0, state.1, state.2));
    assert_channel_popup((state.3, state.4, state.5, state.6, state.7));
    assert_reinforce_route((state.8, state.9));
}

fn assert_noop_guards(state: (bool, bool, bool)) {
    let (origin_noop, disallowed_noop, right_button_noop) = state;

    assert!(origin_noop, "Origin pin must return before routing clicks");
    assert!(
        disallowed_noop,
        "`disallowSelection` must return before routing clicks"
    );
    assert!(
        right_button_noop,
        "Non-left clicks must return before routing clicks"
    );
}

fn assert_channel_popup(state: (i64, bool, bool, bool, bool)) {
    let (popup_count, which_matches, name_matches, text_arg2_is_nil, data_matches) = state;

    assert_eq!(popup_count, 1, "Channel path must show exactly one popup");
    assert!(
        which_matches,
        "Channel path must use ANIMA_DIVERSION_CONFIRM_CHANNEL"
    );
    assert!(name_matches, "Channel path must pass the node name");
    assert!(text_arg2_is_nil, "Channel path must pass nil for textArg2");
    assert!(data_matches, "Channel path must pass the pin as popup data");
}

fn assert_reinforce_route(state: (i64, bool)) {
    let (selection_count, selected_pin_matches) = state;

    assert_eq!(
        selection_count, 1,
        "Reinforce-ready path must select exactly one pin"
    );
    assert!(
        selected_pin_matches,
        "Reinforce-ready path must pass the clicked pin"
    );
}
