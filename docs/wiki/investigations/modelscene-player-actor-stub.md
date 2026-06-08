# ModelScene Player Actor Stub

## Symptom

Collectionator's `RecoverTMogSource` utility crashed from an `OnUpdate` handler while probing the player model:

```lua
modelScene:GetPlayerActor():SetModelByUnit("player")
```

The simulator intentionally does not render 3D model previews, but addon and Blizzard Lua still expect the `ModelScene` actor object contract to exist.

## Root Cause

`ModelScene` exposed the generic actor-management methods (`CreateActor`, indexed lookup, tagged lookup), but not the retail convenience path `GetPlayerActor`. Addons that use the convenience method got nil before they could reach the existing stub actor surface.

## Fix

`GetPlayerActor` now returns a reusable stub `ModelSceneActor` tagged as `player`, creating it on first use. `SetModelByUnit` records the requested unit in model state and returns true so model-probing addon code can complete without requiring real 3D rendering.

The boundary remains the same: 3D preview rendering is out of scope, but the Lua object/method contract should be present when addons use model scenes for capability probes.
