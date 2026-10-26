# Forever Camelot spellbook observations

The [contract](../../specs/forever-camelot-spellbook.md) supplies only the two outfit reads used by the selected Forever Camelot spellbook. Availability reuses [the shared observation](transmog-availability.md); outfit-list observations are independent. Neither input infers native defaults.

## Model

`SimState.forever_outfits` is `Option<Option<Vec<OutfitEntry>>>`: outer None means no supplied observation; Some(None) is an explicit zero-result observation; Some(Some(entries)) yields one fresh array. The common OutfitEntry keeps the seven non-nil fields from pinned Forever70532 `TransmogOutfitInfoDocumentation.lua:896–907`. It also serves the existing Retail/PTR catalog. Snapshot conversion copies rows and nested situationCategories; the modern catalog retains its existing behavior.

Only Forever publishes the supplied GetOutfitsInfo handler. The modern epoch-gated module, other outfit queries, mutations and catalog defaults remain unchanged.

## Selected consumer and ordering

Forever `Blizzard_PlayerSpells.toc` selects `[Game]` as Camelot and the shared SpellBook category. The selected Camelot CreateCategoryMixins:45 instantiates the transmog category at67; its Init:422 calls original UpdateSpellGroups:482, which reads GetOutfitsInfo before later IsAvailable:506 reads availability at507 and outfits at508. Therefore constructor failure alone cannot prove availability publication. The independent original query in each availability test runs before explicit construction, then GREEN tests use the constructor-created category and original UpdateAllSpellData:487.

The original PopulateSpellGroupsIndiciesByRange:463 reads GetOutfitsInfo without consulting availability. It rebuilds membership and ordered indices as100000+outfitID, including the clear-outfit index100000. Sparse replacement fixtures distinguish a live supplied list from empty/fixed/no-op producers.

## Proof status

Three intended initial RED selectors fail at availability507 or constructor catalog484. GREEN and final checks are pending. Tests retain unfiltered closure diagnostics; no full clean bootstrap, UI browsing, physical input or native entitlement proof is implied by bounded method execution.

## Sources

- [Contract](../../specs/forever-camelot-spellbook.md)
- [Original-consumer tests](../../../tests/forever_camelot_spellbook.rs)
- Pinned Forever1.60.1.70532 generated documentation, selected TOC, shared category/frame and Camelot frame override.

## See Also

- [Shared availability](transmog-availability.md)
- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md)
