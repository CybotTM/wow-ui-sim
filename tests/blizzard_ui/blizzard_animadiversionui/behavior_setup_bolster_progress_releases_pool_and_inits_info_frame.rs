//! `SetupBolsterProgressBar` pool/effect reset and info-frame probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const EXPECTED_EARLY_CALLS: &[&str] = &[
    "ReleaseAll",
    "ModelScene.ClearEffects",
    "OverlayModelScene.ClearEffects",
];
const BOLSTER_RESET_PROBE: &str = r#"
local frame = AnimaDiversionFrame
local info = frame.ReinforceInfoFrame
local pool = frame.bolsterProgressGemPool
frame.uiTextureKit = "Kyrian"
frame.covenantData = {
    animaGemsFullSoundKit = 0,
    animaNewGemSoundKit = 0,
}
frame.bolsterProgress = nil
frame.gemsFullSoundHandle = nil
info.AnimaNodeReinforceButton:Enable()
info.Title:SetText("stale")

local calls = {}
local setShownArg = nil
local initSelfMatches = false

local originalGetProgress = C_AnimaDiversion.GetReinforceProgress
local originalGetNodes = C_AnimaDiversion.GetAnimaDiversionNodes
local originalReleaseAll = pool.ReleaseAll
local originalModelClear = frame.ReinforceProgressFrame.ModelScene.ClearEffects
local originalOverlayClear = frame.ReinforceProgressFrame.OverlayModelScene.ClearEffects
local originalAddEffect = frame.ReinforceProgressFrame.ModelScene.AddEffect
local originalOverlayAddEffect = frame.ReinforceProgressFrame.OverlayModelScene.AddEffect
local originalInfoInit = info.Init
local originalInfoSetShown = info.SetShown
local originalPlaySound = PlaySound
local originalUpdateTutorialTips = frame.UpdateTutorialTips

C_AnimaDiversion.GetReinforceProgress = function()
    return 10
end
C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return { { state = Enum.AnimaDiversionNodeState.Available } }
end
pool.ReleaseAll = function(self)
    table.insert(calls, "ReleaseAll")
    return originalReleaseAll(self)
end
frame.ReinforceProgressFrame.ModelScene.ClearEffects = function(self)
    table.insert(calls, "ModelScene.ClearEffects")
    return originalModelClear(self)
end
frame.ReinforceProgressFrame.OverlayModelScene.ClearEffects = function(self)
    table.insert(calls, "OverlayModelScene.ClearEffects")
    return originalOverlayClear(self)
end
frame.ReinforceProgressFrame.ModelScene.AddEffect = function()
    return {}
end
frame.ReinforceProgressFrame.OverlayModelScene.AddEffect = function()
    return {}
end
info.Init = function(self)
    table.insert(calls, "ReinforceInfoFrame.Init")
    initSelfMatches = self == info
    return originalInfoInit(self)
end
info.SetShown = function(self, shown)
    table.insert(calls, "ReinforceInfoFrame.SetShown")
    setShownArg = shown
    return originalInfoSetShown(self, shown)
end
PlaySound = function()
    return nil, 2024
end
frame.UpdateTutorialTips = function() end

frame:SetupBolsterProgressBar()
local titleText = info.Title:GetText()
local reinforceButtonEnabled = info.AnimaNodeReinforceButton:IsEnabled()
local infoShown = info:IsShown()

C_AnimaDiversion.GetReinforceProgress = originalGetProgress
C_AnimaDiversion.GetAnimaDiversionNodes = originalGetNodes
pool.ReleaseAll = originalReleaseAll
frame.ReinforceProgressFrame.ModelScene.ClearEffects = originalModelClear
frame.ReinforceProgressFrame.OverlayModelScene.ClearEffects = originalOverlayClear
frame.ReinforceProgressFrame.ModelScene.AddEffect = originalAddEffect
frame.ReinforceProgressFrame.OverlayModelScene.AddEffect = originalOverlayAddEffect
info.Init = originalInfoInit
info.SetShown = originalInfoSetShown
PlaySound = originalPlaySound
frame.UpdateTutorialTips = originalUpdateTutorialTips

return calls,
       initSelfMatches,
       setShownArg,
       infoShown,
       titleText,
       reinforceButtonEnabled
"#;

#[test]
fn setup_bolster_progress_resets_pool_effects_and_initializes_info_frame() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: BolsterResetState = env
            .eval(BOLSTER_RESET_PROBE)
            .expect("bolster progress reset probe must run cleanly");

        assert_bolster_reset_state(state);
    });
}

type BolsterResetState = (Vec<String>, bool, bool, bool, String, bool);

fn assert_bolster_reset_state(state: BolsterResetState) {
    let (calls, init_self_matches, set_shown_arg, info_shown, title_text, reinforce_button_enabled) =
        state;

    assert_reset_call_order(&calls);
    assert_info_frame_init(init_self_matches, title_text, reinforce_button_enabled);
    assert_set_shown_receives_reinforce_ready(set_shown_arg, info_shown);
}

fn assert_reset_call_order(calls: &[String]) {
    for (index, expected) in EXPECTED_EARLY_CALLS.iter().enumerate() {
        assert_eq!(
            calls[index], *expected,
            "`SetupBolsterProgressBar` must call `{expected}` before rebuilding gems"
        );
    }
    assert_eq!(
        calls[calls.len() - 2],
        "ReinforceInfoFrame.Init",
        "`SetupBolsterProgressBar` must init the info frame after rebuilding gems"
    );
    assert_eq!(
        calls[calls.len() - 1],
        "ReinforceInfoFrame.SetShown",
        "`SetupBolsterProgressBar` must apply the reinforce-ready visibility after Init"
    );
}

fn assert_info_frame_init(
    init_self_matches: bool,
    title_text: String,
    reinforce_button_enabled: bool,
) {
    assert!(
        init_self_matches,
        "`SetupBolsterProgressBar` must call `Init` on `ReinforceInfoFrame`"
    );
    assert_eq!(
        title_text, "Select a location to Reinforce",
        "`ReinforceInfoFrame:Init` must reset Title to `ANIMA_DIVERSION_REINFORCE_READY`"
    );
    assert!(
        !reinforce_button_enabled,
        "`ReinforceInfoFrame:Init` must disable `AnimaNodeReinforceButton`"
    );
}

fn assert_set_shown_receives_reinforce_ready(set_shown_arg: bool, info_shown: bool) {
    assert!(
        set_shown_arg,
        "`SetupBolsterProgressBar` must pass `isReinforceReady` to `SetShown`"
    );
    assert!(
        info_shown,
        "`ReinforceInfoFrame` must be shown when progress reaches reinforce-ready"
    );
}
