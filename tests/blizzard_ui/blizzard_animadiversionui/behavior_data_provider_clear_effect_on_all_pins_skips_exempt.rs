//! `AnimaDiversionDataProviderMixin:ClearEffectOnAllPins` probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CLEAR_EFFECTS_PROBE: &str = r#"
local cancelCount = 0
local addCount = 0
local function buildProvider()
    local provider = {
        pinEffects = {},
        modelScenePin = {
            ModelScene = {
                AddEffect = function()
                    addCount = addCount + 1
                    return {
                        CancelEffect = function()
                            cancelCount = cancelCount + 1
                        end,
                    }
                end,
            },
        },
    }
    setmetatable(provider, { __index = AnimaDiversionDataProviderMixin })
    return provider
end

local first = {}
local second = {}
local exempt = {}
local provider = buildProvider()
provider:AddEffectOnPin(22, first)
provider:AddEffectOnPin(22, second)
provider:AddEffectOnPin(22, exempt)
provider:ClearEffectOnAllPins(22, false, exempt)

local exemptPreserved = provider.pinEffects[exempt] and provider.pinEffects[exempt][22] ~= nil
local nonExemptCleared = provider.pinEffects[first][22] == nil
    and provider.pinEffects[second][22] == nil
local twoNonExemptCancelled = cancelCount == 2

local temporaryPin = {}
local permanentPin = {}
local permanentProvider = buildProvider()
cancelCount = 0
permanentProvider:AddEffectOnPin(22, temporaryPin)
permanentProvider:AddEffectOnPin(22, permanentPin, true)
permanentProvider:ClearEffectOnAllPins(22, true)

local temporaryCleared = permanentProvider.pinEffects[temporaryPin][22] == nil
local permanentPreserved = permanentProvider.pinEffects[permanentPin][22] ~= nil
local onlyTemporaryCancelled = cancelCount == 1

return addCount,
       exemptPreserved,
       nonExemptCleared,
       twoNonExemptCancelled,
       temporaryCleared,
       permanentPreserved,
       onlyTemporaryCancelled
"#;

#[test]
fn clear_effect_on_all_pins_skips_exempt_and_preserves_permanent_effects() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: ClearEffectsState = env
            .eval(CLEAR_EFFECTS_PROBE)
            .expect("clear effects probe must run cleanly");

        assert_clear_effects_state(state);
    });
}

type ClearEffectsState = (i64, bool, bool, bool, bool, bool, bool);

fn assert_clear_effects_state(state: ClearEffectsState) {
    assert_eq!(state.0, 5, "Probe should create five effects total");
    assert_exempt_clear((state.1, state.2, state.3));
    assert_temporary_only_clear((state.4, state.5, state.6));
}

fn assert_exempt_clear(state: (bool, bool, bool)) {
    let (exempt_preserved, non_exempt_cleared, two_non_exempt_cancelled) = state;

    assert!(exempt_preserved, "Exempt pin effect must be preserved");
    assert!(non_exempt_cleared, "Non-exempt pin effects must be cleared");
    assert!(
        two_non_exempt_cancelled,
        "Only non-exempt effects must be cancelled"
    );
}

fn assert_temporary_only_clear(state: (bool, bool, bool)) {
    let (temporary_cleared, permanent_preserved, only_temporary_cancelled) = state;

    assert!(
        temporary_cleared,
        "`onlyTemporaryEffects` must clear temporary effects"
    );
    assert!(
        permanent_preserved,
        "`onlyTemporaryEffects` must preserve permanent effects"
    );
    assert!(
        only_temporary_cancelled,
        "`onlyTemporaryEffects` must cancel only the temporary effect"
    );
}
