//! `AnimaDiversionFrameMixin:SetupBolsterProgressBar` progress/effect probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const BOLSTER_PROGRESS_PROBE: &str = r#"
local frame = AnimaDiversionFrame
frame.uiTextureKit = "Kyrian"
frame.covenantData = {
    animaGemsFullSoundKit = 7788,
    animaNewGemSoundKit = 0,
}
frame.bolsterProgress = nil
frame.gemsFullSoundHandle = nil

local progress = 8
local overlayEffects = {}
local baseEffects = {}
local clearCount = 0
local soundKitSeen = nil

local originalGetProgress = C_AnimaDiversion.GetReinforceProgress
local originalOverlayAddEffect = frame.ReinforceProgressFrame.OverlayModelScene.AddEffect
local originalBaseAddEffect = frame.ReinforceProgressFrame.ModelScene.AddEffect
local originalOverlayClearEffects = frame.ReinforceProgressFrame.OverlayModelScene.ClearEffects
local originalBaseClearEffects = frame.ReinforceProgressFrame.ModelScene.ClearEffects
local originalPlaySound = PlaySound
local originalUpdateTutorialTips = frame.UpdateTutorialTips

C_AnimaDiversion.GetReinforceProgress = function()
    return progress
end
frame.ReinforceProgressFrame.OverlayModelScene.AddEffect = function(self, effectID, source, target)
    table.insert(overlayEffects, { effectID = effectID, sameSourceTarget = source == target })
    return { effectID = effectID }
end
frame.ReinforceProgressFrame.ModelScene.AddEffect = function(self, effectID, source, target)
    table.insert(baseEffects, { effectID = effectID, sameSourceTarget = source == target })
    return { effectID = effectID }
end
frame.ReinforceProgressFrame.OverlayModelScene.ClearEffects = function()
    clearCount = clearCount + 1
end
frame.ReinforceProgressFrame.ModelScene.ClearEffects = function()
    clearCount = clearCount + 1
end
PlaySound = function(soundKit)
    soundKitSeen = soundKit
    return nil, 424242
end
frame.UpdateTutorialTips = function() end

frame:SetupBolsterProgressBar()
local initialProgress = frame.bolsterProgress
local initialOverlayEffectCount = #overlayEffects
local initialActiveGems = frame.bolsterProgressGemPool:GetNumActive()

progress = 12
frame:SetupBolsterProgressBar()
local cappedProgress = frame.bolsterProgress
local finalOverlayEffectCount = #overlayEffects
local finalBaseEffectCount = #baseEffects
local finalActiveGems = frame.bolsterProgressGemPool:GetNumActive()
local firstNewEffectID = overlayEffects[1] and overlayEffects[1].effectID
local secondNewEffectID = overlayEffects[2] and overlayEffects[2].effectID
local firstNewEffectUsesSameGem = overlayEffects[1] and overlayEffects[1].sameSourceTarget
local secondNewEffectUsesSameGem = overlayEffects[2] and overlayEffects[2].sameSourceTarget
local gemsFullSoundHandle = frame.gemsFullSoundHandle

C_AnimaDiversion.GetReinforceProgress = originalGetProgress
frame.ReinforceProgressFrame.OverlayModelScene.AddEffect = originalOverlayAddEffect
frame.ReinforceProgressFrame.ModelScene.AddEffect = originalBaseAddEffect
frame.ReinforceProgressFrame.OverlayModelScene.ClearEffects = originalOverlayClearEffects
frame.ReinforceProgressFrame.ModelScene.ClearEffects = originalBaseClearEffects
PlaySound = originalPlaySound
frame.UpdateTutorialTips = originalUpdateTutorialTips

return initialProgress,
       initialOverlayEffectCount,
       initialActiveGems,
       cappedProgress,
       finalOverlayEffectCount,
       finalBaseEffectCount,
       finalActiveGems,
       firstNewEffectID,
       secondNewEffectID,
       firstNewEffectUsesSameGem,
       secondNewEffectUsesSameGem,
       soundKitSeen,
       gemsFullSoundHandle,
       clearCount
"#;

#[test]
fn setup_bolster_progress_caps_progress_and_marks_new_gems() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: BolsterProgressState = env
            .eval(BOLSTER_PROGRESS_PROBE)
            .expect("bolster progress setup probe must run cleanly");

        assert_bolster_progress_state(state);
    });
}

type BolsterProgressState = (
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    bool,
    bool,
    i64,
    i64,
    i64,
);

fn assert_bolster_progress_state(state: BolsterProgressState) {
    let (
        initial_progress,
        initial_overlay_effect_count,
        initial_active_gems,
        capped_progress,
        final_overlay_effect_count,
        final_base_effect_count,
        final_active_gems,
        first_new_effect_id,
        second_new_effect_id,
        first_new_effect_uses_same_gem,
        second_new_effect_uses_same_gem,
        sound_kit_seen,
        gems_full_sound_handle,
        clear_count,
    ) = state;

    assert_initial_progress_state(
        initial_progress,
        initial_overlay_effect_count,
        initial_active_gems,
    );
    assert_capped_progress_state(capped_progress, final_active_gems, clear_count);
    assert_new_gem_effects(
        final_overlay_effect_count,
        final_base_effect_count,
        first_new_effect_id,
        second_new_effect_id,
        first_new_effect_uses_same_gem,
        second_new_effect_uses_same_gem,
    );
    assert_full_gems_sound(sound_kit_seen, gems_full_sound_handle);
}

fn assert_initial_progress_state(progress: i64, overlay_effect_count: i64, active_gems: i64) {
    assert_eq!(progress, 8, "initial setup must store progress 8");
    assert_eq!(
        overlay_effect_count, 0,
        "initial setup from nil progress must not mark any gems as new"
    );
    assert_eq!(
        active_gems, 8,
        "initial setup must acquire one gem per progress"
    );
}

fn assert_capped_progress_state(progress: i64, active_gems: i64, clear_count: i64) {
    assert_eq!(
        progress, 10,
        "`SetupBolsterProgressBar` must clamp progress to `MAX_ANIMA_GEM_COUNT`"
    );
    assert_eq!(
        active_gems, 10,
        "capped setup must acquire exactly ten active gem frames"
    );
    assert_eq!(
        clear_count, 4,
        "`SetupBolsterProgressBar` must clear both model scenes on each setup pass"
    );
}

fn assert_new_gem_effects(
    overlay_effect_count: i64,
    base_effect_count: i64,
    first_effect_id: i64,
    second_effect_id: i64,
    first_uses_same_gem: bool,
    second_uses_same_gem: bool,
) {
    assert_eq!(
        overlay_effect_count, 2,
        "progress 8 -> 10 must mark exactly two new gems"
    );
    assert_eq!(
        base_effect_count, 0,
        "new-gem effects must go through the overlay model scene"
    );
    assert_eq!(first_effect_id, 23, "Kyrian new-gem effect ID must be used");
    assert_eq!(
        second_effect_id, 23,
        "Kyrian new-gem effect ID must be used"
    );
    assert!(
        first_uses_same_gem,
        "new-gem source and target must be the gem"
    );
    assert!(
        second_uses_same_gem,
        "new-gem source and target must be the gem"
    );
}

fn assert_full_gems_sound(sound_kit_seen: i64, gems_full_sound_handle: i64) {
    assert_eq!(
        sound_kit_seen, 7788,
        "reaching full progress must play the covenant full-gems sound"
    );
    assert_eq!(
        gems_full_sound_handle, 424242,
        "full progress must retain the sound handle on `gemsFullSoundHandle`"
    );
}
