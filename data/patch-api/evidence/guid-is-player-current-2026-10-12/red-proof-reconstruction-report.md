# RED saved-evidence reconstruction

**Original RED JSON bytes NOT_RECOVERED.** This is a structured reconstruction, **not byte-exact history**. Original RED report, raw epoch records and authentication receipt remain unchanged. Generic `proof.json`, GREEN proof, binaries, libraries, old seals and repository source were neither read nor written.

## Historical evidence — no replay

- Revision `76f405e1d38668b3eed4eeab2c3957294d8cee3a`; Retail only. Saved runtime: 0 passed, 9 failed, 10796 filtered out, 3.14s; exit 101, not timeout 124.
- Eight missing raw callable-provider failures; one exact-true consumer mismatch. No new executions or authentication: all reconstruction execution counts are zero.
- Full compile stream: 741 JSON records; 665 artifacts (663 fresh, 2 rebuilt), 75 build-script records, one successful build-finished, zero compiler-message records. Compile exit 0; six manifest deprecations plus their summary retained verbatim in JSON.
- Saved fmt exit 0 with empty stdout/stderr. Exact profile/features/toolchain, seven-source equivalence across four records, admission history and original submission/receipt records retained in JSON.
- Receipt metadata agrees with artifact metadata: 468339000 bytes, SHA-256 `77f3d02d33eb4219f17adcf4dbd41463fa82146c11b05f901a04be8c479f5e0c`, historical auth time `2026-10-12T18:57:09.319684+00:00`. No binary read, hashing or reauthentication.

## Exact historical failure boundaries

| Case | Assertion / panic lines | Deepest reached | Unreached — no credit |
|---|---|---|---|
| `guid_is_player::guid_is_player_exact_local_identity_and_closed_world_misses` | 48 / 58 | First local identity raw-provider guard; no query called | All local/miss values, arity and boolean results |
| `guid_is_player::guid_is_player_target_and_focus_follow_explicit_kind_not_prefix` | 48 / 58 | Target/focus fixture construction completes; first target raw-provider guard | Focus query, replacements, kind/prefix semantics |
| `guid_is_player::guid_is_player_same_env_kind_updates_and_unit_removal_are_live` | 48 / 58 | Target/focus construction completes; first target raw-provider guard | Live kind flips, unit removal and local query |
| `guid_is_player::guid_is_player_active_populated_party_lifecycle` | 48 / 58 | Party construction and both String UnitGUID extractions complete; distinct GUID assertion passes; first party raw-provider guard | Classification, activation, shrink/clear/repopulate/remove lifecycle |
| `guid_is_player::guid_is_player_does_not_leak_records_between_environments` | 48 / 58 | First environment target/focus/party setup and String UnitGUID extraction complete; first target raw-provider guard | Party classification, second environment creation and isolation mutations |
| `guid_is_player::guid_is_player_requires_string_without_numeric_coercion` | 214 / 228 | Raw-provider guard before stringError calls | Missing/nil/nonstring validation, numeric coercion and error text |
| `guid_is_player::guid_is_player_authenticates_real_secrets_before_type_and_lookup` | 240 / 290 | Fixtures complete; taint mode enabled; untainted-setup assertion passes; raw-provider guard fails | Secret wrapper construction/authenticity, query authorization, type ordering, tainted closure, wrapper retention/taint restoration |
| `guid_is_player::guid_is_player_original_object_api_consumer_accepts_represented_player` | 364 / 367 | Whole ordinary closure readiness and provenance pass; constructor/dispatch/GUID assertions pass; pcall IsValid succeeds; exact result == true fails | Successful consumer acceptance; exact actual value/type and provider provenance not logged |
| `guid_is_player::guid_is_player_original_consumer_preserves_actual_account_predicate_for_misses` | 48 / 58 (call 377) | Whole ordinary closure readiness and provenance pass; first NPC-focus raw-provider guard fails | Unknown direct query; account predicate provenance/return diagnostics; miss consumer calls and account equality |

Raw runtime confirms each panic and diagnostic. Assertion lines and source-order boundaries come from surviving `red-current-report.md`, not new source inspection. Consumer actual return value/type and provider provenance remain unknown; successful pcall proves neither query success nor player classification.

## Saved reads and preservation

JSON includes full surviving text records and an explicit saved-read ledger. Historical argv/cwd/env/result records are labeled `saved_read_only`, never executed. The captured controller request is inert text, not evaluated. No behavior counts are added to the historical nine failures.

Only these exclusive new files were created with `open(..., "x")`:

- `red-current-proof-reconstruction.json`
- `red-proof-reconstruction-report.md`

Post-write verification is limited to reading these two new text outputs and comparing original saved text records with their pre-write contents. No build/test/check/fmt, operational command, commit, delegation, repository edit or hash operation occurred.

Reconstruction PASS (saved-evidence consistency only). Historical RED feature acceptance remains FAIL. Original RED structured JSON recovery remains NOT_RECOVERED.
