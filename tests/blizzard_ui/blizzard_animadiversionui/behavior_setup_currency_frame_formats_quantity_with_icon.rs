//! `AnimaDiversionFrameMixin:SetupCurrencyFrame` formatting probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CURRENCY_FORMAT_PROBE: &str = r#"
local frame = AnimaDiversionFrame
local originalGetAnimaInfo = C_CovenantSanctumUI.GetAnimaInfo
local originalGetCurrencyInfo = C_CurrencyInfo.GetCurrencyInfo
local animaInfoReadCount = 0
local currencyInfoReadCount = 0
local requestedCurrencyID = nil

C_CovenantSanctumUI.GetAnimaInfo = function()
    animaInfoReadCount = animaInfoReadCount + 1
    return 1820, 9999
end
C_CurrencyInfo.GetCurrencyInfo = function(currencyID)
    currencyInfoReadCount = currencyInfoReadCount + 1
    requestedCurrencyID = currencyID
    return { quantity = 327, iconFileID = 4549946 }
end

frame:SetupCurrencyFrame()

local quantityText = frame.AnimaDiversionCurrencyFrame.CurrencyFrame.Quantity:GetText()
local formattedText = ANIMA_DIVERSION_CURRENCY_DISPLAY:format(327, 4549946)

C_CovenantSanctumUI.GetAnimaInfo = originalGetAnimaInfo
C_CurrencyInfo.GetCurrencyInfo = originalGetCurrencyInfo

return animaInfoReadCount,
       requestedCurrencyID,
       currencyInfoReadCount,
       quantityText,
       formattedText
"#;

#[test]
fn setup_currency_frame_formats_quantity_with_icon() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: CurrencyFormatState = env
            .eval(CURRENCY_FORMAT_PROBE)
            .expect("currency frame format probe must run cleanly");

        assert_currency_format_state(state);
    });
}

type CurrencyFormatState = (i64, i64, i64, String, String);

fn assert_currency_format_state(state: CurrencyFormatState) {
    let (
        anima_info_read_count,
        requested_currency_id,
        currency_info_read_count,
        quantity_text,
        formatted_text,
    ) = state;

    assert_eq!(
        anima_info_read_count, 1,
        "`SetupCurrencyFrame` must read `C_CovenantSanctumUI.GetAnimaInfo`"
    );
    assert_eq!(
        requested_currency_id, 1820,
        "`SetupCurrencyFrame` must query the anima currency returned by `GetAnimaInfo`"
    );
    assert_eq!(
        currency_info_read_count, 1,
        "`SetupCurrencyFrame` must read `C_CurrencyInfo.GetCurrencyInfo`"
    );
    assert_eq!(
        formatted_text, "327|T4549946:18:18|t",
        "`ANIMA_DIVERSION_CURRENCY_DISPLAY` must match the expected en-US format"
    );
    assert_eq!(
        quantity_text, formatted_text,
        "`SetupCurrencyFrame` must write the formatted quantity and icon text"
    );
}
