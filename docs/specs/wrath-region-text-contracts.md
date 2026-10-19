# Wrath region and text contracts

Wrath 3.3.5a build 12340 region/text behavior, supplied by laptop findings batches 1–3 (2026-10-18). Cataclysm 15595 corroborates only. Other profiles retain their existing behavior. Entry points live under `src/lua_api/frame/methods/` and `src/lua_api/globals/font_strings_collection/fonts.rs`.

## What it must do

- [ ] T1: SetTexCoord accepts exactly four or eight arguments; converts each position numerically without compacting, with failed conversion zero. Coordinates outside inclusive ±10000 and infinities error atomically; NaN is stored. Success returns no values.
- [ ] F1: SetAllPoints omitted/scalar uses parent; explicit nil uses root; string/number names resolve; table requires valid slot-zero identity. Unknown targets and recursive dependencies error before mutation. Protected denial precedes argument validation and returns without mutation.
- [ ] T2: SetBlendMode recognizes DISABLE, BLEND, ALPHAKEY, ADD, MOD case-insensitively; invalid arguments error before mutation.
- [ ] T5: FontString invalid filename/height and nonpositive height error before mutation; valid-height empty filename returns no values and retains font. Font/EditBox nonpositive height rejects without error or mutation.
- [ ] T7: SetJustifyH/V shares six case-insensitive tokens; wrong-axis tokens are accepted and read UNKNOWN without changing the other axis. Unknown tokens error before mutation.
- [ ] T8: SetTextColor numerically converts RGB (failed conversion zero) and alpha (failed conversion one), clamps to 0..1, and returns no values.

## How it works

- [Lua API](../lua-api.md)
- [Anchor resolution](../anchor-resolution.md)

## Implementation inventory

- `src/lua_api/wrath_arguments.rs` — VM conversions and shared token parser.
- `src/lua_api/methods.rs` — slot-zero-only identity extraction.
- `src/lua_api/frame/methods/widgets/texture/coords.rs` — texture arguments.
- `src/lua_api/frame/methods/button_anchor_hierarchy/anchors.rs` — target/protection validation.
- `src/lua_api/frame/methods/text_attribute_event/text/formatting.rs` — frame-backed SetFont.
- `src/lua_api/frame/methods/text_attribute_event/text/style.rs` — color and justification.
- `src/lua_api/globals/font_strings_collection/fonts.rs` — Font-table methods.

## Tests asserting this spec

- `tests/classic_texture_texcoord_validation.rs`
- `tests/classic_region_set_all_points.rs`
- `tests/wrath_text_fonts.rs`

## Known gaps (current cycle)

- [ ] T5 exact generation acceptance, success/failure return captures, epsilon cutoff, numeric-string height, flags grammar and Font/EditBox success arity remain unknown. Renderer family fallback is not a generation oracle.
- [ ] F1 CanBeAnchorFor veto classes and native nil-vs-root GetPoint representation remain unknown.
- [ ] T1 atlas-backed and sub-f32 boundary behavior remain unproved; NaN rendering is unknown.

## Out of scope

T3 indentation, T6 propagation, blend pixel equations/batching and non-Wrath native parity. T2/T7 setter arities preserve existing simulator behavior rather than asserting native return counts.
