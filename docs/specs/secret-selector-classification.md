# Numeric-secret spell classifications

Retail/PTR 12.0.5+ classification-selector compatibility in `src/c_api/c_spell_classification.rs`; [C API overview](../wiki/systems/lua-api.md#c-api-numeric-secret-selector-boundaries).

## What it must do

- [ ] `C_Spell.IsSpellImportant`, `IsSpellCrowdControl`, and `IsConsumableSpell` accept authentic VM numeric-secret spell selectors from secure and tainted callers, with existing numeric/name conversion, no alias-first expansion or shared-reader widening.
- [ ] Results remain exactly one ordinary boolean from existing supplied important/classification facts; unknown identities remain false. INFERRED: retaining ordinary booleans is approved simulator policy, not native selector-driven output-secrecy proof.
- [ ] `IsExternalDefensive` remains AllowedWhenUntainted: secure numeric-secret selection works; tainted-secret selection fails. Non-numeric secret payloads are not widened.
- [ ] Original NamePlateAurasMixin:IsAuraCrowdControl returns configured true; CastingBarMixin:UpdateHighlightImportantCast highlights the configured important spell, with public/secret × secure/tainted controls.
- [ ] Model rows, input opacity and caller taint remain unchanged. Invalid/non-numeric controls preserve existing endpoint conversion and miss behavior, not the complete native SpellIdentifier grammar.

## How it works

- [C API numeric-secret boundaries](../wiki/systems/lua-api.md#c-api-numeric-secret-selector-boundaries)

## Implementation inventory

- `src/c_api/c_spell_classification.rs`: endpoint-local host-only numeric-secret selection; original fact tables and external-defensive permission.

## Tests asserting this spec

- `tests/secret_selector_classification.rs`
- `tests/p1200_rest_spell_plate.rs`

## Known gaps (current cycle)

- [ ] Compiled original-consumer GREEN and preservation receipts; retained RED at `8b1c6d953` precedes producer, with original nameplate directly reaching VM selector rejection.

## Out of scope

Generated `SpellDocumentation.lua` marks the three widened endpoints AllowedWhenTainted and ExternalDefensive AllowedWhenUntainted. Original-consumer fixture injection proves selector compatibility, not naturally secret spell IDs at startup. New catalogs, secret-string widening, native output-secrecy parity, vendor edits, shims and VM changes are excluded.
