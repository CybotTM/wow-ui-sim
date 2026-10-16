# CheckButton checked input

Bounded simulator contract required by G2's original Wrath consumer, not native parity. Implementation belongs only in `src/lua_api/frame/methods/widgets/slider/checkbutton.rs`; shared widget boolean extraction stays unchanged.

## What it must do

- [ ] Preserve literal booleans; nil or an omitted argument clears checked state.
- [ ] Numeric zero clears checked state; any nonzero number checks it.
- [ ] Preserve current string behavior: all strings remain unchecked. Do not introduce general Lua truthiness or string coercion.
- [ ] Original Wrath `ActionButton_UpdateState` checks and unchecks a real CheckButton through its numeric branches.

## Grounding and limits

Original cached `~/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/ActionButton.lua:302-310` passes `SetChecked(1)` for current/repeat and `SetChecked(0)` otherwise. Seven-profile cache inspection found numeric literals only zero/one, with zero in inactive/reset branches; none observed requiring zero-as-truthy. Dynamic expressions are not a native semantic oracle. Wrath `SpellBookFrame.lua:405-412` also supplies strings `"true"`/`"false"`; their currently unchecked behavior is deliberately unchanged by this numeric fix.

## How it works

- [Widget system](../widget-system.md)
- [Supplied repeat-active contract](action-repeat-active.md)

## Tests asserting this spec

- `tests/checkbutton_checked_input.rs`: direct boolean/nil/string/numeric controls and complete unchanged cached Wrath ActionButton source, driving its original UpdateState function.
- Existing CheckButton tests remain Retail preservation evidence; G2's original START/STOP consumer is the higher-level regression.

## Known gaps

- [ ] Tests-only behavioral RED, producer GREEN and Retail preservation pending.
- [ ] Native parity and string normalization are not established.
