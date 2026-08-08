# Patch 12.0.0 API Audit

12.0.0 occurrence audit generated reproducibly from versioned wowless retail snapshots. It records schema-surface deltas plus bounded evidence-backed behavioral slices; it does not claim historical 12.0.0 FrameXML or live runtime behavior.

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
- **Status:** 77 best-effort/behavioral rows, 147 evidence-required/unsafe rows, and 3186 untriaged rows with null final status
- **Source SHA-256:** `6f26d194d0c3f721b3a071217cf69714f1278950512369272298735bdf44c863`

Each source occurrence may carry optional typed `before`/`after` JSON payloads. Added rows carry `after`, removed rows `before`, changed rows both, and transient add/remove rows the corresponding side; row identity remains `direction+symbol`, and unknown occurrence fields remain rejected. This payload metadata improves exact triage without changing the occurrence counts.

This remains a bounded evidence-backed slice, not a compatibility or completion claim. The four `AbbreviateConfig` rows remain best-effort behavioral classifications; existing behavior proves factory/table proxy behavior, method dispatch, round-trip storage, per-instance isolation, read-only keys, and tostring, but not exact `arrayof NumberAbbrevData` structure fidelity. The twelve heal-prediction rows are best-effort behavioral classifications covering only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. The five `LuaFunctionContainer`/`C_FunctionContainers.CreateCallback` rows are best-effort behavioral classifications: the named `userdata_proxy` tests cover method exposure, cancellation/invoke suppression, per-instance fields, read-only keys, and tostring; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. The five duration/factory/StatusBar rows are best-effort behavioral classifications limited to factory/object shape, default-zero behavior, per-instance fields, tostring, and exact StatusBar duration-object identity round-trip. The 21 remaining duration-time, lifecycle, and secret rows are evidence-required unsafe classifications: current behavior is constant/no-op/incomplete, and authoritative semantics or a correct modeled implementation are required; no approval can close them. The nine curve-family best-effort rows cover only tested factory/table shape, scalar point insertion/linear interpolation/copy behavior, per-instance fields/tostring, and color-object/copy shape; exact retail userdata identity, curve types, ordering/duplicates, color evaluation, secret propagation, and defaults remain unproven. The 23 unresolved curve contracts remain evidence-required unsafe because the current generic proxy omits or does not faithfully establish them; authoritative semantics or a correct modeled implementation are required, and they cannot be approved closed. 17 curve-family metadata rows (curve enums, point fields, script-object APIs, and typedefs) remain untriaged outside those classifications. The `C_StringUtil.EscapeQuotedCodes` row is best-effort behavioral, limited to quoted-code pipe escaping for tested plain/color-code cases; exact edge/secret/localization semantics remain unproven. The eight remaining `C_StringUtil` rows are evidence-required unsafe because the current C_StringUtil model does not publish them; authoritative semantics or correct implementations are required, and no approval can close them. The `C_Timer.NewTimer` and `C_Timer.NewTicker` rows are best-effort behavioral, limited to function/container acceptance, returned container identity/proxy equality, cancellation, and independent ticker counts from the named timer-container tests; exact scheduling, lifecycle, GC, and edge semantics remain unproven. `C_Timer.After` is evidence-required unsafe because its only focused-looking test is ignored; callback/lifecycle semantics require a correct modeled implementation and executable behavioral proof, and no approval can close it. The two `C_ColorUtil` best-effort behavioral rows are limited to the tested RGB-to-`ffRRGGBB` code and explicit color-code text wrapping; the five conversion/`WrapTextInColor` rows are evidence-required unsafe because current behavior is absent, placeholder/identity/max-channel, or lacks focused executable proof. Edge/secret/localization/clamping semantics remain unproven. The twelve-row C_Spell slice covers duration-object lifecycle, spell metadata/display, and boolean contracts without focused proof; authoritative evidence or correct models are required, and no approval can close these rows. The bounded C_TradeSkillUI slice classifies `GetDependentReagents` as best-effort behavioral, limited to table return/iteration safety and nil/malformed/unknown-reagent behavior; exact retail dependency semantics remain unproven. Its eleven quality/recraft/reagent-link rows are evidence-required unsafe with empty tests and no approval path; current evidence distinguishes absent methods, placeholder empty-table/true behavior, and unproven removal behavior, so authoritative profession semantics or a correct model/test are required and no approval can close them. The bounded `C_DamageMeter` slice classifies exactly 19 best-effort behavioral rows, limited to exact seeded/empty/shape/type/lookup assertions named in the manifest; session `maxAmount` is asserted only as zero for empty meter types and source `maxAmount` only as numeric shape. 10 seeded-but-unasserted/reset rows remain `evidence-required`/`unsafe` with no approval path; authoritative semantics or a correct model/test are required. Six metadata-only structure rows remain untriaged. No complete retail aggregation/lifecycle/secret fidelity is claimed. The bounded `C_ActionBar` slice now covers exactly 17 best-effort behavioral rows: the 13 page/state-query rows plus four action-slot/profession/outfit rows. The four new rows use only the named profession-quality, main-bar button, and outfit-lock direct/end-to-end tests with ancestor commits `f831998ca`, `e1cbad52e`, and `7cbd694b7`; claims are limited to seeded/empty/malformed profession quality, modeled action-slot presence/texture, and modeled outfit-lock slot behavior. Broader source types, file-ID fidelity, gear state, and secure/UI lifecycle remain unproven. The slice has 25 evidence-required/unsafe rows: the three prior constant/derived queries plus 22 action queries/registration rows. Those 22 rows use source-register and current implementation/registration evidence with empty tests and null commit/approval/scope exception; notes distinguish absent, constant/default, partial-table, default-duration, and partial/no-op registration or slot-model behavior. Authoritative semantics or a correct model/test are required, and no approval can close them. The 11 `ActionBarChargeInfo`/`ActionBarCooldownInfo` structure/field rows remain untriaged. The bounded C_CombatAudioAlert slice classifies exactly 12 added API rows as evidence-required/unsafe using checked-in source-register evidence and the examined current `src/lua_api/globals/register.rs` surface; tests remain empty with null commit, approval, and scope exception. Notes avoid exhaustive absence claims and require authoritative evidence or a correct modeled subsystem for exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics; no approval can close these rows. The bounded `C_EncounterWarnings` slice classifies exactly 19 added structure/API rows as evidence-required/unsafe using checked-in source-register evidence and the clean current `src/lua_api/globals/missing_surface/encounter_warnings.rs`; tests remain empty with null commit, approval, and scope exception. `GetEditModeWarningInfo`/current structure fields are limited to fabricated preview/static payload behavior, `PlaySound` is a no-op, and `GetSoundKitForSeverity`, `IsFeatureAvailable`, and `IsFeatureEnabled` have no examined registration; exact encounter state, payload field meanings, feature availability/enabling, severity sound mapping, and audio playback require authoritative evidence or a correct modeled subsystem/test, and no approval can close these rows. The remaining 3186 rows are untriaged. No rows have `implemented` or `exception-requested` status.

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
- [12.0.0 manifest](../../../data/patch-api/12.0.0.json) — 3410-row audit manifest with the bounded classification slices and validation metadata.
- [12.0.0 checklist](../../generated/patch-12-0-0-checklist.md) — generated one-line-per-occurrence checklist.
- [12.0.0 occurrence inventory](patch-12-0-0-occurrence-inventory.md) — generated human-readable inventory.

## See Also

- [[patch-api-audit-manifest]] — register schema and completion contract.
- [[patch-12-0-5-api-audit]] — later probe-driven retail audit with separate evidence.
