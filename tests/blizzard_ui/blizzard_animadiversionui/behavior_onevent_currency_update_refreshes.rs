//! `AnimaDiversionFrameMixin:OnEvent("CURRENCY_DISPLAY_UPDATE")` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CURRENCY_UPDATE_PROBE: &str = r#"
local frame = AnimaDiversionFrame
frame.mapID = C_Map.GetCurrentMapID()
frame.uiTextureKit = "Kyrian"
frame.covenantData = {
    animaGemsFullSoundKit = 0,
    animaNewGemSoundKit = 0,
    animaChannelActiveSoundKit = 0,
}
frame.bolsterProgress = 1
frame:Show()

local originalGetReinforceProgress = C_AnimaDiversion.GetReinforceProgress
local originalGetAnimaInfo = C_CovenantSanctumUI.GetAnimaInfo
local originalGetCurrencyInfo = C_CurrencyInfo.GetCurrencyInfo

local reinforceProgressReadCount = 0
local currencyInfoReadCount = 0
local animaInfoReadCount = 0
local animaCurrencyIDSeen = nil
local requestedCurrencyID = nil

C_AnimaDiversion.GetReinforceProgress = function()
    reinforceProgressReadCount = reinforceProgressReadCount + 1
    return 4
end
C_CovenantSanctumUI.GetAnimaInfo = function()
    animaInfoReadCount = animaInfoReadCount + 1
    return 1813, 9999
end
C_CurrencyInfo.GetCurrencyInfo = function(currencyID)
    currencyInfoReadCount = currencyInfoReadCount + 1
    requestedCurrencyID = currencyID
    return { quantity = 321, iconFileID = 654321 }
end

frame:OnEvent("CURRENCY_DISPLAY_UPDATE")

local refreshedBolsterProgress = frame.bolsterProgress
local quantityText = frame.AnimaDiversionCurrencyFrame.CurrencyFrame.Quantity:GetText()
local expectedQuantityText = ANIMA_DIVERSION_CURRENCY_DISPLAY:format(321, 654321)

C_AnimaDiversion.GetReinforceProgress = originalGetReinforceProgress
C_CovenantSanctumUI.GetAnimaInfo = originalGetAnimaInfo
C_CurrencyInfo.GetCurrencyInfo = originalGetCurrencyInfo

return reinforceProgressReadCount,
       refreshedBolsterProgress,
       animaInfoReadCount,
       requestedCurrencyID,
       currencyInfoReadCount,
       quantityText,
       expectedQuantityText
"#;

#[test]
fn currency_display_update_refreshes_bolster_and_currency_text() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: CurrencyUpdateState = env
            .eval(CURRENCY_UPDATE_PROBE)
            .expect("CURRENCY_DISPLAY_UPDATE refresh probe must run cleanly");

        assert_currency_update_state(state);
    });
}

type CurrencyUpdateState = (i64, i64, i64, i64, i64, String, String);

fn assert_currency_update_state(state: CurrencyUpdateState) {
    let (
        reinforce_progress_read_count,
        refreshed_bolster_progress,
        anima_info_read_count,
        requested_currency_id,
        currency_info_read_count,
        quantity_text,
        expected_quantity_text,
    ) = state;

    assert_reinforce_progress_refreshed(reinforce_progress_read_count, refreshed_bolster_progress);
    assert_currency_text_refreshed(
        anima_info_read_count,
        requested_currency_id,
        currency_info_read_count,
        quantity_text,
        expected_quantity_text,
    );
}

fn assert_reinforce_progress_refreshed(read_count: i64, refreshed_progress: i64) {
    assert!(
        read_count >= 1,
        "`CURRENCY_DISPLAY_UPDATE` must re-read `C_AnimaDiversion.GetReinforceProgress`"
    );
    assert_eq!(
        refreshed_progress, 4,
        "`CURRENCY_DISPLAY_UPDATE` must refresh `AnimaDiversionFrame.bolsterProgress`"
    );
}

fn assert_currency_text_refreshed(
    anima_info_read_count: i64,
    requested_currency_id: i64,
    currency_info_read_count: i64,
    quantity_text: String,
    expected_quantity_text: String,
) {
    assert_eq!(
        anima_info_read_count, 1,
        "`CURRENCY_DISPLAY_UPDATE` must re-read `C_CovenantSanctumUI.GetAnimaInfo`"
    );
    assert_eq!(
        requested_currency_id, 1813,
        "`SetupCurrencyFrame` must pass the anima currency ID to `C_CurrencyInfo.GetCurrencyInfo`"
    );
    assert_eq!(
        currency_info_read_count, 1,
        "`CURRENCY_DISPLAY_UPDATE` must re-read `C_CurrencyInfo.GetCurrencyInfo`"
    );
    assert_eq!(
        quantity_text, expected_quantity_text,
        "`SetupCurrencyFrame` must format `Quantity` from the refreshed currency info"
    );
}
