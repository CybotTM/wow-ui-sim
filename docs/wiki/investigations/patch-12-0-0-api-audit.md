# Patch 12.0.0 API Audit

Neutral 12.0.0 occurrence audit generated reproducibly from versioned wowless retail snapshots. It records schema-surface deltas only; it does not claim historical 12.0.0 FrameXML or live runtime behavior.

## Content

### Source boundary

The generator compares the last explicit retail 11.2.7 snapshot at build `65299` (`d9efaadf92f558e2b4fbef622c7b8af0e843849a`) with the final explicit retail 12.0.0 snapshot at build `65727` (`a6d2717d06f9255e507ab07f811c1bafaea64939`). It also retains six 12.0.0 snapshots:

- build `65512` — `78e503fb24e467ec0354148f7ba41b77a3158ff6`
- build `65535` — `16ae143430a9e3704f639c5452a5315408d5dc18`
- build `65560` — `f39e3453ebb67f1be70e127e146092a8129954bb`
- build `65655` — `33cf699b1d91d4743acb5c003339b1f5ed2c28c2`
- build `65699` — `03bb5214f7a951ca5b5a6d38dc7ca56af164b281`
- build `65727` — `a6d2717d06f9255e507ab07f811c1bafaea64939`

The source covers wowless `apis`, `cvars`, `docs`, `events`, `globals`, `luaobjects`, `structures`, and `uiobjects` snapshots. The generator performs a semantic endpoint diff, preserves transient symbols found only in intermediate snapshots, and now retains normalized value+metadata payloads for exact enum, constant, signature, and structure triage; this produced 8 transient lifecycle rows.

### Register state

- **Occurrences:** 3410
- **Added:** 2554
- **Changed:** 313
- **Removed:** 543
- **Status:** all 3410 rows are `untriaged` with null final status
- **Source SHA-256:** `6f26d194d0c3f721b3a071217cf69714f1278950512369272298735bdf44c863`

Each source occurrence may carry optional typed `before`/`after` JSON payloads. Added rows carry `after`, removed rows `before`, changed rows both, and transient add/remove rows the corresponding side; row identity remains `direction+symbol`, and unknown occurrence fields remain rejected. This payload metadata improves exact triage without changing the neutral status or counts.

This is neutral inventory state, not a compatibility or completion claim. No rows have implementation, evidence-required, or exception status.

### Provenance and limits

The wowless snapshot history is the historical source for this audit. The active retail cache manifest recorded in `data/patch-api/12.0.0.json` (`data/blizzard-ui-files/retail.txt`, hash `42abf0ff8118e6be4d41ed321f6a0e7daeb83234928e451f33851d14a488b5ef`) is only validation-environment metadata; it is not historical 12.0.0 source provenance.

The register does not claim:

- a historical 12.0.0 FrameXML tree;
- Blizzard UI file load order or startup/LoD timing for that patch;
- live 12.0.0 addon observations or SavedVariables captures;
- exact runtime semantics inferred from schema names alone.

## Sources

- [12.0.0 register generator](../../../tools/gen_patch_12_0_0_register.py) — reproducible wowless-history snapshot diff.
- [12.0.0 source register](../../../data/patch-api/sources/12.0.0-register.json) — normalized source/provenance register.
- [12.0.0 manifest](../../../data/patch-api/12.0.0.json) — neutral 3410-row audit manifest and validation metadata.
- [12.0.0 checklist](../../generated/patch-12-0-0-checklist.md) — generated one-line-per-occurrence checklist.
- [12.0.0 occurrence inventory](patch-12-0-0-occurrence-inventory.md) — generated human-readable inventory.

## See Also

- [[patch-api-audit-manifest]] — register schema and completion contract.
- [[patch-12-0-5-api-audit]] — later probe-driven retail audit with separate evidence.
