//! `AnimaDiversionFrameMixin:TryShow` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const TRY_SHOW_PROBE: &str = r#"
local frame = AnimaDiversionFrame
frame.uiTextureKit = "Original"
frame.mapID = 7
frame.covenantData = { ID = 99 }
frame.setupBolsterCount = 0
frame.setupCurrencyCount = 0
frame.shownViaPanel = false

local originalSetupBolster = frame.SetupBolsterProgressBar
local originalSetupCurrency = frame.SetupCurrencyFrame
local originalShowUIPanel = ShowUIPanel
local originalGetCovenantData = C_Covenants.GetCovenantData
local covenantCallID = nil
local covenantData = { ID = 1, name = "Kyrian Sentinel" }

frame.SetupBolsterProgressBar = function(self)
    self.setupBolsterCount = self.setupBolsterCount + 1
end
frame.SetupCurrencyFrame = function(self)
    self.setupCurrencyCount = self.setupCurrencyCount + 1
end
ShowUIPanel = function(panel)
    frame.shownViaPanel = panel == frame
end
C_Covenants.GetCovenantData = function(covenantID)
    covenantCallID = covenantID
    return covenantData
end

frame:TryShow(nil)
local nilTextureKit = frame.uiTextureKit
local nilMapID = frame.mapID
local nilCovenantID = frame.covenantData.ID
local nilBolsterCount = frame.setupBolsterCount
local nilCurrencyCount = frame.setupCurrencyCount
local nilShownViaPanel = frame.shownViaPanel

frame:TryShow({ textureKit = "Kyrian", title = "Bastion", mapID = 1543 })
local successTextureKit = frame.uiTextureKit
local successMapID = frame.mapID
local successCovenantMatches = frame.covenantData == covenantData
local successCovenantID = frame.covenantData and frame.covenantData.ID
local successCovenantCallID = covenantCallID
local successBolsterCount = frame.setupBolsterCount
local successCurrencyCount = frame.setupCurrencyCount
local successShownViaPanel = frame.shownViaPanel

frame.SetupBolsterProgressBar = originalSetupBolster
frame.SetupCurrencyFrame = originalSetupCurrency
ShowUIPanel = originalShowUIPanel
C_Covenants.GetCovenantData = originalGetCovenantData

return nilTextureKit,
       nilMapID,
       nilCovenantID,
       nilBolsterCount,
       nilCurrencyCount,
       nilShownViaPanel,
       successTextureKit,
       successMapID,
       successCovenantMatches,
       successCovenantID,
       successCovenantCallID,
       successBolsterCount,
       successCurrencyCount,
       successShownViaPanel
"#;

#[test]
fn try_show_seeds_state_runs_setup_and_opens_panel() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: TryShowState = env
            .eval(TRY_SHOW_PROBE)
            .expect("AnimaDiversionFrame TryShow probe must run cleanly");

        assert_try_show_state(state);
    });
}

type TryShowState = (
    String,
    i64,
    i64,
    i64,
    i64,
    bool,
    String,
    i64,
    bool,
    Option<i64>,
    Option<i64>,
    i64,
    i64,
    bool,
);
type NilTryShowState = (String, i64, i64, i64, i64, bool);
type TryShowSuccess = (String, i64, bool, Option<i64>, Option<i64>, i64, i64, bool);

fn assert_try_show_state(state: TryShowState) {
    assert_nil_try_show_returned_early(nil_try_show_state(&state));
    assert_successful_try_show(successful_try_show_state(&state));
}

fn nil_try_show_state(state: &TryShowState) -> NilTryShowState {
    (state.0.clone(), state.1, state.2, state.3, state.4, state.5)
}

fn successful_try_show_state(state: &TryShowState) -> TryShowSuccess {
    (
        state.6.clone(),
        state.7,
        state.8,
        state.9,
        state.10,
        state.11,
        state.12,
        state.13,
    )
}

fn assert_nil_try_show_returned_early(state: NilTryShowState) {
    let (texture_kit, map_id, covenant_id, bolster_count, currency_count, shown_via_panel) = state;

    assert_eq!(
        texture_kit, "Original",
        "`TryShow(nil)` must not mutate `uiTextureKit`"
    );
    assert_eq!(map_id, 7, "`TryShow(nil)` must not mutate `mapID`");
    assert_eq!(
        covenant_id, 99,
        "`TryShow(nil)` must not mutate `covenantData`"
    );
    assert_eq!(
        bolster_count, 0,
        "`TryShow(nil)` must not call `SetupBolsterProgressBar`"
    );
    assert_eq!(
        currency_count, 0,
        "`TryShow(nil)` must not call `SetupCurrencyFrame`"
    );
    assert!(
        !shown_via_panel,
        "`TryShow(nil)` must not call `ShowUIPanel`"
    );
}

fn assert_successful_try_show(success: TryShowSuccess) {
    let (
        texture_kit,
        map_id,
        covenant_data_matches,
        covenant_id,
        covenant_call_id,
        bolster_count,
        currency_count,
        shown_via_panel,
    ) = success;

    assert_eq!(texture_kit, "Kyrian", "`TryShow` must store `textureKit`");
    assert_eq!(map_id, 1543, "`TryShow` must store `mapID`");
    assert!(
        covenant_data_matches,
        "`TryShow` must store `C_Covenants.GetCovenantData(1)` for Kyrian"
    );
    assert_eq!(
        covenant_id,
        Some(1),
        "`TryShow` must resolve Kyrian to covenant ID 1"
    );
    assert_eq!(
        covenant_call_id,
        Some(1),
        "`TryShow` must call `C_Covenants.GetCovenantData(1)` for Kyrian"
    );
    assert_eq!(
        bolster_count, 1,
        "`TryShow` must call `SetupBolsterProgressBar` exactly once"
    );
    assert_eq!(
        currency_count, 1,
        "`TryShow` must call `SetupCurrencyFrame` exactly once"
    );
    assert!(shown_via_panel, "`TryShow` must call `ShowUIPanel(self)`");
}
