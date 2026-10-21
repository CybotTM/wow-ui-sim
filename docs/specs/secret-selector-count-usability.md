# Numeric-secret spell counts and usability

Retail/PTR 12.0.5+ numeric-selector compatibility in `src/c_api/c_spell_counts.rs` and `src/c_api/c_spell.rs`. [C API overview](../wiki/systems/lua-api.md).

## What it must do

- [ ] Cast count and maximum cumulative aura applications accept authentic numeric-secret selectors in secure/tainted callers, with the existing alias-first identity lookup and strict integral u32 domain. Public numbers/strings remain unchanged; non-numeric secrets remain rejected.
- [ ] Usability accepts numeric-secret selectors in secure/tainted callers with its existing strict u32 domain, known-spell membership, and exactly two ordinary booleans. No action-usability model substitution.
- [ ] Counts return exactly one number, including zero on miss; usability returns `(false,false)` on miss. Configured spell19750 returns count3, maximum7, and `(true,false)`.
- [ ] Cast-count secrecy still depends only on the existing cooldown restriction, maximum-count secrecy on the existing unit-aura restriction. Selector secrecy does not replace either policy. INFERRED: retaining the existing output policy is approved simulator behavior; generated AllowedWhenTainted alone does not establish native selector-driven output secrecy.
- [ ] Original viewer CacheChargeValues caches count3/shown=true with genuinely absent charge rows; original RefreshIconColor sets real icon texture white for the configured known spell. Public/secret × secure/tainted run unrestricted; restricted-secure count also runs. Restricted-tainted comparison/setter failure is not a selector regression.
- [ ] Queries preserve wrappers, caller taint, model rows and original Blizzard source. Invalid numeric domains and non-numeric secrets remain explicit failures.

## How it works

- [Existing spell runtime](../wiki/systems/lua-api.md)

## Implementation inventory

- `src/c_api/c_spell_counts.rs`: endpoint-local numeric-secret lookup, independent count publication.
- `src/c_api/c_spell.rs`: endpoint-local usability selector.

## Tests asserting this spec

- `tests/secret_selector_counts.rs`
- `tests/spell_count_outputs.rs`
- `tests/spell_max_cumulative_aura_applications.rs`

## Known gaps (current cycle)

- [ ] Compiled original-consumer RED→GREEN and preservation receipts.

## Out of scope

Natural startup secret-ID production, secret string selectors, count-display selectors, native output-secrecy parity, restricted-tainted original viewer comparison/widget consumption, action-usability changes, catalogs and shared-reader widening.
