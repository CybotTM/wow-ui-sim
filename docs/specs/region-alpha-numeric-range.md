# Region alpha numeric range

Ordinary numeric `Region:SetAlpha` inputs follow the [10.0.0 change-note summary](../../data/patch-api/sources/10.0.0-api-changes.txt): “Region:SetAlpha() errors if the argument is not between 0.0 and 1.0.” This bounded contract targets Retail/PTR Frame, Texture and FontString values. [Widget architecture](../wiki/systems/widget-system.md) describes stored and effective alpha.

## What it must do

- [ ] Accept numeric zero, one and interior values, returning no values.
- [ ] Reject ordinary numeric inputs outside the inclusive range, including infinities and NaN, before changing stored or inherited alpha.
- [ ] Validate the original numeric precision before narrowing to stored `f32`; values just beyond either boundary must not become valid through rounding.
- [ ] Preserve valid-value dirty/no-op behavior. Earlier profiles retain the existing clamping behavior as simulator compatibility, not independently verified native history.

## How it works

- [Widget architecture](../wiki/systems/widget-system.md)
- [Rendering architecture](../rendering-pipeline.md)

## Implementation inventory

- `src/lua_api/frame/methods/core_state/alpha.rs`: public alpha setters and effective-alpha propagation; ordinary Retail/PTR numeric range validation precedes narrowing/mutation, while earlier profiles retain clamping.
- `tests/region_alpha_validation.rs` and `region_alpha_validation/ordinary_range.lua`: concrete ordinary-numeric boundaries and rejection/state-preservation contracts.
- `tests/noop_hot_setters.rs`: existing valid-value dirty/no-op behavior and historical clamping expectation.

## Tests asserting this spec

Two new Retail/PTR Rust cases are authored, not yet compiled/executed. Their complete shared Lua fixture executed on existing normal binaries before the producer: valid boundaries completed on both profiles; invalid mode failed at the first Frame `SetAlpha(-0.25)` rejection assertion on both profiles. Later rejection/state-preservation and Texture/FontString assertions were unreached. Independent RED audit and producer GREEN remain pending. The shared complete Lua fixture can also run through normal startup without replacing methods or getters. Historical clamping expectations must remain distinguished from modern range rejection.

## Known gaps (current cycle)

- [ ] Execute behavioral RED before changing the setter; verify modern GREEN, preservation, startup diagnostics and check/format afterward.

## Out of scope

`SetAlphaFromBoolean`, secret/coercion redesign, other numeric methods and native/render parity are outside this bounded ordinary-numeric contract. Cached current declarations use `SingleColorValue` and allow tainted arguments; the historical range note does not establish their complete secret-value semantics or native error wording.
