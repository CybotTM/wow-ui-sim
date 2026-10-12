# GUID modeled source GREEN verification

**PASS — bounded requested simulator scope only.** Revision `d6d8de29a134e791c6cc47ec31988ad0d4ba5957`; spec `docs/specs/guid-is-player.md`.

Named producer `wow-guid-player-green-20261012t191109506305z` reached terminal success (`SubState=exited`, `Result=success`, `ExecMainStatus=0`), first observed 2026-10-12 19:14:33 UTC, within the 25-minute bound. Controller completed both profiles. No verifier build/test/check reruns, source edits, operational mutations or delegation.

## Profile matrix

| Profile | Compile | Nine integration cases | Normal probe | fmt | Source hashes |
|---|---|---|---|---|---|
| ptr | exit 0 | 9 PASS / 0 FAIL; exit 0; 9974 filtered out | one marker + literal `[]`; exit 0 | exit 0 | all 9 unchanged |
| retail | exit 0 | 9 PASS / 0 FAIL; exit 0; 10796 filtered out | one marker + literal `[]`; exit 0 | exit 0 | all 9 unchanged |

Total: **18/18 integration cases**, two normal probes; no ignored cases. Filtered tests are not credited. All compile/runtime/normal admissions succeeded, including PTR. A future late PTR admission block would be RESOURCE_BLOCKED, not missing tests; none occurred here.

## Exact nine-case coverage

| Case (`guid_is_player::` prefix) | Retail | PTR |
|---|---|---|
| guid_is_player_active_populated_party_lifecycle | PASS | PASS |
| guid_is_player_authenticates_real_secrets_before_type_and_lookup | PASS | PASS |
| guid_is_player_does_not_leak_records_between_environments | PASS | PASS |
| guid_is_player_exact_local_identity_and_closed_world_misses | PASS | PASS |
| guid_is_player_original_consumer_preserves_actual_account_predicate_for_misses | PASS | PASS |
| guid_is_player_original_object_api_consumer_accepts_represented_player | PASS | PASS |
| guid_is_player_requires_string_without_numeric_coercion | PASS | PASS |
| guid_is_player_same_env_kind_updates_and_unit_removal_are_live | PASS | PASS |
| guid_is_player_target_and_focus_follow_explicit_kind_not_prefix | PASS | PASS |

## Spec and actual source

[EXIST] PASS — 40-line `src/c_api/c_player_info_guid.rs` present.
[SUBSTANTIVE] PASS — real state projection, not prefix/catalog/stub behavior.
[WIRED] PASS — profile-gated module `src/c_api/mod.rs:147-148`; registration `src/c_api/registration.rs:47-48` calls provider without removing sibling registration. Normal probe checks raw sibling function.
[ANTI-PATTERN] PASS — no new TODO/FIXME/HACK/XXX, suppressed warnings, commented-out implementation or placeholder body.

| Requirement | Source / proof level |
|---|---|
| Original secret authentication before UTF-8 string validation and lookup; one bool, no coercion | src/c_api/c_player_info_guid.rs:17-27; val_to_string src/lua_api/methods.rs:395-398. Both profiles strict-type and authentic-secret cases PASS; arity/value assertions in direct/secret helpers; invalid UTF-8 bytes code-inspected, not a distinct executed regression |
| Exact local/active populated party identities; explicit target/focus kind, never prefix | src/c_api/c_player_info_guid.rs:30-39; existing_guid_for_unit; resolve_unit_is_player. Both profiles local/misses, contradictory-prefix target/focus and roster cases PASS |
| Live replacement/kind/removal/roster/environment isolation | Fresh borrow on every call, no stored classifier cache. Both profiles replacement, same-env update/removal, party lifecycle, independent-environment cases PASS |
| Closed-world unknown and empty strings false | Exact equality over represented identities only. Both profiles local/misses case PASS, including empty/player-looking strings |
| Authentic wrapped strings allowed untainted, all wrapped values denied tainted, plain strings remain usable; wrappers/taint preserved | unwrap_secret before val_to_string and borrow_state. Both profiles authentic-secret case PASS, hits/misses/malformed values and post-call taint/wrapper assertions |
| Ordinary ObjectAPI closure, whole original constructor/IsValid, source provenance, unchanged account operand | tests/guid_is_player.rs:294-411. Both profiles two whole-consumer cases PASS; original method source asserted Blizzard_ObjectAPI/Mainline/PlayerLocation.lua; represented player exact true, misses preserve observed nil account operand; provider/method identity/provenance checks. No mocks, slicing or native account classification claim |

**Readability:** manually audited all 40 new source lines, wiring additions and test formatting delta against rust-readability skill. No violations found. Three short functions, explicit authentication → strict conversion → live query → one boolean; no cache, state accumulation, excessive nesting/parameters or warning suppressions. Metric tooling not run. Existing test helpers inspected for behavioral fidelity, not reclassified as new source.

## Whole consumer/account diagnostics

Both profiles load ordinary Blizzard_ObjectAPI addon closure and assert original full CreateFromGUID/IsValid source provenance. Represented player yields exact `true`. Direct NPC/unknown query yields `false`; original consumer misses preserve account operand **nil**, not false and not native account classification.

Transport: Lua returns full diagnostic string → Rust `String` from `env.eval` → `eprintln!` → retained `runtime.stderr` under `--nocapture`. Each profile emits exactly five diagnostic lines.

### ptr

```text
GUID account-provider	Lua	=(string)	raw-provider	function
GUID account-return	Player-990001-00000002	true	nil	nil
GUID consumer-return	Player-990001-00000002	true	nil	nil
GUID account-return	Player-990001-00000099	true	nil	nil
GUID consumer-return	Player-990001-00000099	true	nil	nil
```

### retail

```text
GUID account-provider	Lua	=(string)	raw-provider	function
GUID account-return	Player-990001-00000002	true	nil	nil
GUID consumer-return	Player-990001-00000002	true	nil	nil
GUID account-return	Player-990001-00000099	true	nil	nil
GUID consumer-return	Player-990001-00000099	true	nil	nil
```

## Normal session

Each sealed non-test wow-sim runs ordinary Blizzard startup with `--no-addons --no-saved-vars --exec-lua <probe> lua-errors`, `WOW_SIM_NO_SOUND=1`. Probe asserts raw GUIDIsPlayer function, exactly one boolean, local UnitGUID true, unknown false, raw GetAlternateFormInfo function retained. Startup/workarounds remain enabled; this is not a new seeded session or native assertion.

Both normal stdout streams exactly:
```text
GUID_PLAYER_NORMAL_PROBE_PASS
[]
```
Normal stderr ends CLEAN: zero unique/errors/occurrences. No whole-session environment/cache/native attestation claimed.

## Independent single authentication receipts

Each NEW integration/normal seal read and SHA-256 authenticated once; receipt immediately saved using exclusive `open("x")`. No original executable, library, RED or old artifact read/hash. Receipt timestamps and full Cargo target/profile/features retained in proof.json.

| Profile / seal | Bytes | Independent SHA-256 | Receipt |
|---|---|---|---|
| ptr/integration | 467738152 | `2319eee62f217135d0048ac7af2c1521262520eff450b022b34e7c1dc4f6398f` | [ptr-green-20261012T191247449812Z/independent-auth-integration.json](ptr-green-20261012T191247449812Z/independent-auth-integration.json) |
| ptr/normal | 414547880 | `912a80b1da9a31f7f41f04738ac48465780796ddc340547e3a138727ad2345e0` | [ptr-green-20261012T191247449812Z/independent-auth-normal.json](ptr-green-20261012T191247449812Z/independent-auth-normal.json) |
| retail/integration | 468100344 | `a8e7c91a975a3a138fd22bfedafdb4b05da2e61414caf2795ba234f0587ac809` | [retail-green-20261012T191109578339Z/independent-auth-integration.json](retail-green-20261012T191109578339Z/independent-auth-integration.json) |
| retail/normal | 407302792 | `0a13f78d4a59ea2b48b939158ac4fa7f9acc6e2170e146f87095a1e7170c1d2f` | [retail-green-20261012T191109578339Z/independent-auth-normal.json](retail-green-20261012T191109578339Z/independent-auth-normal.json) |

4/4 independent receipts match producer seal manifests in byte count and digest. Authentication does not imply native parity or independently measured executable behavior beyond retained producer runs.

## Toolchain, full outputs, warnings

### ptr

```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1
```

Compilation stdout fully parsed: 741 JSONL records, build-finished success true, zero compiler-message records. Compile stderr nevertheless contains **six manifest deprecations** plus one generated-6-warnings summary; not warning-free.

- compile, exit 0: [compile.stdout](ptr-green-20261012T191247449812Z/compile.stdout) (574600 bytes), [compile.stderr](ptr-green-20261012T191247449812Z/compile.stderr) (1275 bytes); full streams read, not inferred from exits.
- runtime, exit 0: [runtime.stdout](ptr-green-20261012T191247449812Z/runtime.stdout) (932 bytes), [runtime.stderr](ptr-green-20261012T191247449812Z/runtime.stderr) (6668 bytes); full streams read, not inferred from exits.
- normal, exit 0: [normal.stdout](ptr-green-20261012T191247449812Z/normal.stdout) (33 bytes), [normal.stderr](ptr-green-20261012T191247449812Z/normal.stderr) (14687 bytes); full streams read, not inferred from exits.
- fmt, exit 0: [fmt.stdout](ptr-green-20261012T191247449812Z/fmt.stdout) (0 bytes), [fmt.stderr](ptr-green-20261012T191247449812Z/fmt.stderr) (0 bytes); full streams read, not inferred from exits.
- cargo-version, exit 0: [cargo-version.stdout](ptr-green-20261012T191247449812Z/cargo-version.stdout) (65 bytes), [cargo-version.stderr](ptr-green-20261012T191247449812Z/cargo-version.stderr) (0 bytes); full streams read, not inferred from exits.
- rustc-version, exit 0: [rustc-version.stdout](ptr-green-20261012T191247449812Z/rustc-version.stdout) (225 bytes), [rustc-version.stderr](ptr-green-20261012T191247449812Z/rustc-version.stderr) (0 bytes); full streams read, not inferred from exits.
### retail

```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1
```

Compilation stdout fully parsed: 741 JSONL records, build-finished success true, zero compiler-message records. Compile stderr nevertheless contains **six manifest deprecations** plus one generated-6-warnings summary; not warning-free.

- compile, exit 0: [compile.stdout](retail-green-20261012T191109578339Z/compile.stdout) (574439 bytes), [compile.stderr](retail-green-20261012T191109578339Z/compile.stderr) (1376 bytes); full streams read, not inferred from exits.
- runtime, exit 0: [runtime.stdout](retail-green-20261012T191109578339Z/runtime.stdout) (933 bytes), [runtime.stderr](retail-green-20261012T191109578339Z/runtime.stderr) (6666 bytes); full streams read, not inferred from exits.
- normal, exit 0: [normal.stdout](retail-green-20261012T191109578339Z/normal.stdout) (33 bytes), [normal.stderr](retail-green-20261012T191109578339Z/normal.stderr) (14704 bytes); full streams read, not inferred from exits.
- fmt, exit 0: [fmt.stdout](retail-green-20261012T191109578339Z/fmt.stdout) (0 bytes), [fmt.stderr](retail-green-20261012T191109578339Z/fmt.stderr) (0 bytes); full streams read, not inferred from exits.
- cargo-version, exit 0: [cargo-version.stdout](retail-green-20261012T191109578339Z/cargo-version.stdout) (65 bytes), [cargo-version.stderr](retail-green-20261012T191109578339Z/cargo-version.stderr) (0 bytes); full streams read, not inferred from exits.
- rustc-version, exit 0: [rustc-version.stdout](retail-green-20261012T191109578339Z/rustc-version.stdout) (225 bytes), [rustc-version.stderr](retail-green-20261012T191109578339Z/rustc-version.stderr) (0 bytes); full streams read, not inferred from exits.

Exact warning lines (identical in both profiles):
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
```

## Nine source hashes

Independent current-file digests match both profile before/after-compile/after-runtime snapshots. Current nine-path git diff against requested revision is empty, exit 0. Snapshot scope is these nine files; unrelated files, dependencies and libraries are not hash-attested.

| Source | SHA-256 |
|---|---|
| `Cargo.lock` | `5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1` |
| `Cargo.toml` | `91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9` |
| `docs/specs/guid-is-player.md` | `d73dd6300aa4390c47fd0fdf8207391d63cbd7473b0521d598be04582d31cf9e` |
| `src/c_api/c_player_info_guid.rs` | `f8b12bedcc9c17a5dbbd05145216f7f6a18e62723b21e839c11dade39572df2a` |
| `src/c_api/mod.rs` | `efecc942f984a872af29250b436ccd999ef127786bcb7dbd8427a68e894802d1` |
| `src/c_api/registration.rs` | `b7b3cac6c78aa226307af8529d2e41b0752bf0a66b98d652406c8146274f579a` |
| `src/lua_api/globals/unit_misc.rs` | `1477e79cdc665910c4656c5ad8c7e265309a2d56d1e645b35c535256d7d8134a` |
| `src/lua_api/globals/unit_probes.rs` | `651ce9d2a71f3eec54a78a37e77a6517972eff85d69a41abba2bd0808394523d` |
| `tests/guid_is_player.rs` | `8db6d89da188e08d76be60dd93e16c0f4b68166e469139d940514ab1b044c2f6` |

## Retained / excluded

Initial RED remains retained: already independently confirmed 9 FAIL before producer, per supplied context/spec. Not reread, rehashed or rerun here; no RED-derived GREEN credit. Spec checkboxes remain unchanged by this verifier.

**No native GUID/coercion/account/catalog parity claim. No default cargo-check PASS, publication reconciliation, full-suite PASS or full-handoff acceptance.** Default check/publication remain separately pending. Invalid UTF-8 rejection is source-inspected, not separately exercised by these nine cases.

Machine-readable evidence: [proof.json](proof.json). Original producer logs/manifests and RED evidence preserved.
