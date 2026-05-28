# Paladin Aura Stance Bar

Paladin aura buttons vanished because the simulator seeded the player as a Paladin but left `SimState.shapeshift_forms` empty. Blizzard's `StanceBarMixin` only shows when `GetNumShapeshiftForms() > 0`, so the fix belongs in simulator state, not in StanceBar rendering.

## Root Cause

`GetNumShapeshiftForms()` is backed directly by `state.shapeshift_forms.len()`. Before the fix, default state used an empty vector, so `StanceBarMixin:Update()` set `numForms = 0`, skipped `UpdateState()`, and hid the bar even though Paladin aura spells existed in the spellbook and buff model.

The regression probe:

```lua
print(GetNumShapeshiftForms(), StanceBar:IsShown())
for i = 1, 3 do
    print(i, GetShapeshiftFormInfo(i))
end
```

Broken output reported zero forms. Fixed output reports three forms and populates StanceButton spell IDs `465`, `32223`, and `183435`.

## Fix

Default `SimState` now seeds three Paladin aura forms:

- Devotion Aura (`465`)
- Crusader Aura (`32223`)
- Retribution Aura (`183435`)

Each form resolves its display name and texture through the generated spell DB and texture manifest, then flows through the existing `GetShapeshiftFormInfo()` API. No Blizzard Lua or renderer workaround is involved.

## Guard Rails

Regression coverage now exists at two layers:

- `tests/c_shapeshift_globals.rs` verifies default Paladin state exposes exactly three aura forms through the raw shapeshift globals.
- `tests/blizzard_ui/blizzard_actionbar/behavior_stance_select.rs` verifies Blizzard `StanceBar:Update()` shows the bar, assigns the three spell IDs to the first three stance buttons, and keeps button 4 hidden.

## Sources

- [state.rs](../../src/lua_api/state.rs) — default `SimState` shapeshift-form seeding
- [shapeshift.rs](../../src/lua_api/globals/real/shapeshift.rs) — raw shapeshift globals backed by `SimState`
- [behavior_stance_select.rs](../../tests/blizzard_ui/blizzard_actionbar/behavior_stance_select.rs) — Blizzard StanceBar regression coverage

## See Also

- [[action-button-icon-mask]] — another action-bar-adjacent disappearance bug with a different render-layer root cause
- [[on-update-dirty]] — AuraButton performance investigation for BuffFrame, separate from the Paladin StanceBar state path
