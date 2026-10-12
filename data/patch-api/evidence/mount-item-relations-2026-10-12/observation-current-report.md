# Mount-item observation verification — bounded only

**BOUNDED OBSERVATION PASS; PARENT OPEN.** Revision `609033535`. Main unit `wow-mount-observations-20261012t154947271597z.service` terminal `active/exited`, `Result=success`, `ExecMainStatus=0`; exited **2026-10-12 15:51:41 UTC**. Independently observed terminal 15:51:59 UTC, under the 20-minute bound. No delegation, replay, build, test, check, service mutation, repository/vendor edit, or prior seal access.

## Exact runtime coverage

| Profile | Actual cases | Original invocations | Raw rows | Result order | Current fmt |
|---|---:|---:|---:|---|---|
| Retail | 2 passed / 0 failed; exit 0; 2.47s | 4 | 44 | false,true,false,false | captured exit 0, empty stdout/stderr |
| PTR | 2 passed / 0 failed; exit 0; 4.74s | 4 | 44 | false,true,false,false | captured exit 0, empty stdout/stderr |

Per profile: **16 provenance + 4 input + 8 stage + 4 API + 4 result + 4 predicate-value + 4 predicate-source = 44** raw rows. Eight invocations / four passing cases / 88 rows total. Counts independently recomputed from full stderr and equal retained outcome rows; not inferred from success summaries. Harness runs `original_consumer_tracks_live_supplied_relation` first (baseline false, supplied `6948 -> 401` true, removal false), then `real_item_consumer_returns_boolean` (false). Earlier proposed false,false,true,false ordering was incorrect; main explicitly clarified actual order. No rows reordered and no replay.

Source `tests/mount_item_player_choice.rs:107-132` retains baseline/live-map/removal assertions: mapped true, unequal baseline, removal returns baseline. Unsliced cached vendor method, original intrinsic ItemButton and SetItem/GetItemInfo identities/provenance asserted. Every input is Hearthstone, item 6948, literal generated link `|cnIQ1:|Hitem:6948::::::::80:70:::::::::|h[Hearthstone]|h|r`; original SetItem stage then original IsDressupReward stage. Vendor provenance: PlayerChoice `662–676`; ItemButton `146–149`, `151–175`, `177–191`. API types function/function/function.

## Actual predicate — not native dressability

All eight actual predicate rows are **raw registered type function; return type nil; value nil**. All source rows are **Lua, `=(string)`, lines 72–74**. `runtime_surface_bootstrap.lua:66–77` contains the existing namespace `__index` fallback; its nested function at 72–74 returns nil and is raw-registered at 75. No named IsDressableItemByID implementation found under src. Current Retail/PTR vendor method at 662–676 checks dressability, then mount, then pet, then false. This supports the existing **unmodeled nil fallback** classification, not a modeled/native boolean dressability rule or native Hearthstone classification. Controlled supplied relation establishes consumer dependence in this simulator context only. No inferred catalog or normal-session seeding.

## Scope reuse, source, readability

Independent `git diff 7487c8bec -- src tests/mount_from_item.rs tests/mount_spell_identifier.rs Cargo.toml Cargo.lock` empty, exit 0; both producer reuse snapshots agree. Production/query/spell/normal/default-library check scope unchanged: prior proof remains applicable; no rerun justified. Current change is test-owned observation transport (return bool + string and eprintln), plus spec inventory wording. Eleven before/after-compile/after-runtime source hashes per profile equal independently observed current hashes. Producer applicability against 609033535 empty, exit 0. Full source snapshots and full compiler/stdout/stderr/outcome/inputs/admission records retained in observation-current-proof.json.

Manual changed-Rust readability: no violations observed; invoke helper 23 body lines, nesting <=2, explicit diagnostic side effects, error handling retained, no new warning suppression or TODO/FIXME/HACK/XXX. Existing integration macro reaches auto-discovered tests via crates/test-harness/build.rs:29–66 and src/lib.rs:7; actual stdout names prove both cases executed. No metric/lint/build/test replay.

## Artifact and warning evidence

Only NEW Retail/PTR integration seals authenticated exactly once each. Receipt absence checked, exclusive immediate independent-auth-integration.json saved. Existing receipts reused without seal reads. Cargo metadata exact-match occurs once per full 741-record compile (665 artifacts, 75 build scripts, one successful build-finished; zero compiler-message records). Both compile exits 0. Retail includes client-retail/default/retail-12-1-0; PTR includes client-ptr/retail-12-1-5. Cargo/rustc 1.99.0 Arch, rustc b940084d7eb6a299eb4bfeb8e34901bc051e7ac4, LLVM 23.1.1.

| NEW seal | Bytes | SHA256 |
|---|---:|---|
| Retail | 468248576 | d94aae762553f2ee60ef281146c9276850e605646b477df62bd91a39ced57b68 |
| PTR | 467574288 | 89027ce11b3e751b11fde49099172a7a850db202640a7665e956c15b17378452 |

**Not warning-free.** Six deprecated iced-wgpu-patched Clippy manifest keys retained: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity; six-warning summary in each compile. Per profile: 118 empty addon-warning rows; **40 nonempty nil-observation rows and 18 nonempty missing-requirement rows** retained, not suppressed or called test failures. 122 empty rows each for Lua errors/error records/error counts (366 empty snapshots). Full consumer stderr: Retail 132380 characters/907 lines, PTR 131495/907. Admission histories admit first attempt, retained with actual memory/load/pressure. Inherited allowlist unset; no consumer overrides; no historical/child/cache-content equivalence claim.

## History and remaining gaps

Prior `final-current-report.md` **RAW CONSUMER OBSERVATION GATE FAIL** history and final-current-proof.json unchanged; no overwrite. New transported raw observation gate PASS does not rewrite earlier missing raw-console evidence. Initial 0/3 and expanded 0/7 missing-API RED preserved, not retrospective downstream RED. Eighth mapped-secret and consumer contrast have no separate prior RED. Parent, native/catalog/range/classification/ownership/full-suite acceptance remain OPEN/unproven. Earlier-profile runtime exclusion unexecuted. Spec unchecked requirements and implementation/GREEN-pending prose remain stale; no edit authorized. All warnings, nil/missing requirements and uncertainty retained.

Observer diagnostics: initial read-only git calls lacked cwd (usage/not-repository); corrected with explicit cwd. Cross-evaluation ctx unavailable; inspected real artifacts/receipts instead, no duplicate authentication. Asynchronous read-only observer plus bounded direct status observations only; terminal reached before deadline.
