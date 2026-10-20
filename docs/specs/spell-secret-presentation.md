# Numeric-secret spell presentation selectors

`C_Spell` presentation queries select existing generated spell records using authentic numeric VM secrets. This is bounded simulator behavior, not a native secrecy claim. Source: `src/c_api/c_spell.rs`; [Lua API architecture](../lua-api.md).

## What it must do

- [x] `GetSpellName`, `GetSpellTexture`, and `GetSpellInfo` select the same known database name/icon for public and authentic numeric-secret inputs in secure and ordinarily tainted callers. Presentation fields and containers remain public.
- [x] Explicit user policy (2026-10-20): `GetSpellInfo.spellID` stays secret for a secret numeric selector; public selection retains ordinary numeric identity. No caller-taint clearing or VM-wide unwrap changes.
- [ ] `GetSpellDescription` returns public presentation text with existing public-selector behavior.
- [ ] Approved separate link contract: `GetSpellLink` returns a whole secret link string for secret numeric input because the link embeds identity. Public-selector results retain existing behavior. Nil misses remain nil, not secret-wrapped nil.
- [ ] Preserve exact arity and misses: Info one nil; Texture zero values; Name one `Unknown`; Description one empty string; Link one nil. Known Texture uses current epoch's three values (two public icon IDs and nil override).
- [ ] Only authentic numeric secrets are inspected. Retain each endpoint's existing numeric conversion: Info/Texture/Link accept finite nonnegative numbers with existing truncation/saturation; Name/Description require exact u32. In particular19750.5 selects19750 through Info/Texture but errors through typed Name. Wrap the resolved Info identity19750, not the original fractional selector. Negative/nonfinite/overflow and other secret kinds retain endpoint-appropriate error/miss behavior without payload disclosure. These are simulator conversion facts; native numeric-domain/error parity is **INFERRED**.
- [ ] Wrapper opacity, original payload, caller taint, source spell records and original consumer Lua remain unchanged. Glyph argument2 remains independently NeverSecret.

## How it works

- [Lua API architecture](../lua-api.md)
- [Frame data flow](../frame-data-flow.md)

## Implementation inventory

- `src/c_api/c_spell.rs` — opt-in presentation-selector inspection and identity-bearing output policy.
- `tests/secret_spell_presentation.rs` — authentic VM wrappers and original meter/reward consumers with real widget sinks.

## Tests asserting this spec

`tests/secret_spell_presentation.rs`. Runtime proof and exact revisions belong in the private handoff proof ledger until executed.

## Bounded Retail proof

Tests `6e06a4c6c` precede producer `b5a48499c`: four behavioral failures at the original meter icon/name, reward Info and typed secret-miss boundaries; existing spell API59, cooldown restriction17 and glyph security8 controls all pass. Baseline stock startup returns `[]` with original CVar stat/hash unchanged.

At description tests `4a2751af7`, the four core cases pass, including secret resolved identity and endpoint-specific fractional conversion. Original Contribution tooltip public control succeeds, then secret selection reaches `GetSpellDescription`'s typed-selector error: one distinct description RED, not a prior Info/dependency failure. Both compilations finish without warnings and their integration ELFs contain only this worktree's test paths. Description/final preservation acceptance remains pending.

## Known gaps (current cycle)

- [ ] Behavioral Retail RED→GREEN and touched-module preservation pending slot admission.
- [ ] Description producer now uses the same exact-u32 host selector path after original-tooltip RED; final GREEN remains pending.
- [ ] Link sub-batch held: original GuildPerks tooltip and flyout/chat paths reach `opt_string` sinks that do not consume authentic secret strings. Keep this downstream boundary separate from selector RED; no sink widening is authorized.

## Out of scope

- P3b power cost: actual `UnitCastingInfo` producer emits ordinary numeric spellID only; required real opaque cast source is unavailable. No fabricated casting provider or new casting secrecy model.
- Secret-string spell selectors, public identifier grammar expansion, new aliases, spell acquisition/override graphs, VM changes and render-sink widening.
- Native field secrecy, exact errors and natural secret provenance beyond literal generated documentation remain **INFERRED**. Original meter sources forward stored spell IDs, but injected wrappers do not establish a naturally secret startup call.
