# Independent configuration-baseline verification — 67598f430

**Source/spec/readability PASS; default Retail 2/2 PASS; explicitly selected Wrath 2/2 PASS. One-row negative expected FAIL101; selected-missing negative expected FAIL101; refresh helper PASS0. Retention 33/33 PASS; documentation/link PASS at 998b2dd22. No outstanding in-scope blockers.**

Revision: `67598f4303d0dc7c4a8ad1abd202531a9c705d20`. Saved proof epoch: `20261011T042623Z`. Auditor read source and saved outputs only; no builds, tests, replay, process polling, binary rehash, operational mutation, or delegation. Only `independent-report.md` and `independent-report.json` are auditor-owned outputs.

## Capability/proof matrix

| Contract | Evidence | Result |
|---|---|---|
| One active profile, actual compiled Retail epoch, actual forbidden/OnUpdate flags | `tests/method_diff_coverage.rs:65-95`: descending true `cfg!` selection; `ACTIVE.cache_subdir()`; independent flag cfgs | Source PASS |
| No enabled Retail epoch is `no-retail-epoch`, not metadata/1207 | Same helper never reads `ACTIVE_RETAIL_API_EPOCH` (whose metadata fallback exists separately) | Source PASS |
| Root only for explicit default key | Constant `retail/retail-12-1-0/forbidden-1-on-update-1`, exact key equality | Source PASS |
| Default execution only one Retail baseline; no all-profile loop or baseline directory scan | Two existing assertions each read one selected file; unchanged discovery | Source PASS + default 2/2 PASS |
| Exact comparator/discovery unchanged | Committed diff changes only fixture paths, adds selection helpers, changes ignored refresh destinations; 17 factories, 16 parsed native types; discovery Lua block unchanged | PASS |
| Frozen discovery and root defaults unchanged | Parent/commit identity and current Git blob/hash agreement | PASS |
| 12 new fixtures equal accepted 031711 sources | Each committed/current file equals corresponding `complete-diff_methods_{extra,missing}.txt` and accepted manifest SHA256 | 12/12 PASS |
| Default Retail current compiled source | Fresh integration compiler artifact, matching helper hash and features, saved sealed copy, two exact named executions | 2/2 PASS, each exit 0 |
| Explicit Wrath current compiled source | Fresh integration compiler artifact, only `client-wrath`; no Retail epoch/forbidden/OnUpdate features; same helper hash and sealed copy | 2/2 PASS, each exit 0 |
| Default fmt/check | `retail/check-0.json`, `check-1.json`, full stdout/stderr | Both exit 0; existing warnings |
| Selected Wrath one-row negative control | Corrected control detects exactly `Button:EnableSubtitles`; one failed exact case, exit101, assertion at `tests/method_diff_coverage.rs:302:5` | Expected FAIL101 verified |
| Active-only ignored refresh and restoration | Rust source writes only current configuration's two paths; saved helper selects one named case PASS0; all 16 recorded input hashes preserved/restored | Source wiring PASS + byte-preservation PASS (not write-trace proof) |
| Missing selected fixture fails without fallback | Selected Wrath extra file absent: one failed exact case, exit101, full selected-path read panic at `tests/method_diff_coverage.rs:122:53`; finally restores all 16 recorded input hashes | Expected FAIL101 verified; no historical epoch execution |

## Exact saved proof

Builds: `/usr/bin/cargo test --offline --locked --test integration --no-run --message-format=json`, default Retail; Wrath adds `--no-default-features --features sound,gui,casc,client-wrath`.

Each profile ran exactly two named selectors with `/usr/bin/timeout 90 <sealed integration-test> --exact <selector> --nocapture --test-threads=1`, `WOW_SIM_CASC=0`:

- `method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface`
- `method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface`

Each full stdout says `running 1 test`, the matching named test `... ok`, and `1 passed; 0 failed; 0 ignored; 0 measured`. Retail filters 10716 cases; Wrath filters 8034. Saved execution receipts agree; no zero-selected pass. Full streams and independently parsed counts retained in JSON.

Default `/usr/bin/cargo fmt --all -- --check` has empty streams, exit 0. `/usr/bin/cargo check --offline --locked --tests` exits 0; existing iced manifest deprecation and publication-sweep dead-code warnings are not suppressed or called warning-free.

Receipt paths under each profile: `compile-submission.json`, `compile-result.json`, `compile.stdout/stderr`, `artifact.json`, `execution-results.json`, `extra.stdout/stderr`, `missing.stdout/stderr`. Retail additionally has `check-{0,1}.json/stdout/stderr` and `outcome.json`. No build or completed positive was replayed by auditor.

Saved artifact identity:

| Profile | Saved SHA256 | Bytes |
|---|---|---:|
| Retail | `742b940b02f089198833c50b6856111e83c29164c7bf69cb7c5568de7287e80e` | 467322704 |
| Wrath | `eea776373d6aa353e5530c8ee9c35ac85769270f281fdca4a65a627261956ea5` | 435617376 |

Compiler metadata and sealed file size agree. Auditor composes saved seals, not a fresh binary cryptographic claim. Input SHA256 covers helper, frozen discovery, root defaults, and 12 copied fixtures, not every repository dependency.

## Saved controls

Corrected controls: `/home/osso/.local/state/wow-ui-sim/verification/method-baseline-controls-current/20261011T043308Z/`. Full `negative.stdout/stderr`, `negative.json`, `refresh.stdout/stderr`, `submission.json`, and `outcome.json` independently audited. Same fresh sealed Wrath artifact and 16 source/fixture input hashes as positive proof. Whole-line negative removes exactly one `Button:EnableSubtitles` row (not the `CheckButton` suffix match); stderr reports that row only under `Add new entries`. `finally` restores original bytes. Refresh stdout contains exactly one named helper PASS, zero ignored cases, exit0. Before/after/current input hashes agree.

Missing control: `/home/osso/.local/state/wow-ui-sim/verification/method-baseline-missing-current/20261011T043648Z/`. Full `missing.stdout/stderr`, `submission.json`, `outcome.json` independently audited. Missing selected Wrath extra fixture produces full selected-path read error, not comparator-delta error; one failed exact case, exit101. Same recorded Wrath artifact; finally restoration and all 16 current hashes verified.

Both corrected workers equal saved requests and completed controllers' executed code; queue exits0, controller stderr empty. Initial selection worker's substring-uniqueness error occurred before mutation; controls supersede only that incomplete control step, without positive/build/check replay.

Selected-only refresh destinations are established by Rust source wiring. Runtime unchanged-byte evidence does **not** prove unselected files were never rewritten with identical bytes; no filesystem write tracing performed. Ordinary comparisons fail on absent selected data; explicitly invoking the ignored refresh helper creates its selected directory/files intentionally.

## Readability / integration

Changed Rust helpers at lines 65,79,87,98 have 11/5/8/1 body lines. Ignored test helper at line 343 has 22 body lines. Manual changed-line audit found no suppressions, opaque boolean branches, long functions, bumpy-road violations, excessive nesting, or new TODO/FIXME/HACK/XXX markers. No readability tooling or checks were executed.

Reachability: `tests/integration.rs:1` includes generated harness; `build.rs:25,54-90,463-498` discovers and includes top-level Rust test modules. That existing Rust harness scan is not an all-baseline test loop or baseline-directory scan.

## Retention/documentation follow-up — afee9a1c0

Packet: `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-baseline-selection/`.

**33/33** declared receipt/stream files match untouched private sources byte-for-byte, declared SHA256, declared byte size, and committed `afee9a1c0` contents. Committed packet inventory is exactly **35 files**: those 33 copies plus README and manifest. No missing/unexpected files. Compiler streams and binaries intentionally remain private; copied artifact metadata matches previously audited private evidence.

Current spec/README correctly distinguish ordinary selected-file read failure from ignored refresh creation, selected-only source wiring from unchanged-byte runtime proof, default one-Retail execution from explicitly chosen Wrath, and copied other-five data from fresh GREEN. Epoch key typo corrected to `retail-12-1-0`. Runtime/source changes were not introduced; existing proof remains valid without replay.

**Documentation finding resolved:** `998b2dd22ff5c40a6f8b8599c71cebb474cdda0e` changes only packet `README.md:3` to `../../../../../../docs/specs/method-surface-baselines.md`. Committed/current text matches; link resolves to the exact existing spec. Current packet retains correctly bounded claims. Accepted 33/33 retention proof is composed unchanged; no receipt hashes, builds, tests, or binaries were rerun/rehashed for the link fix. Five unrelated historical README links are pre-existing and outside changed scope. Packet's final-audit-pending label is valid until main accepts this final report; main should update it on acceptance. No outstanding in-scope blockers.

## Limits / selected missing scopes

Historical Retail configurations `retail-12-0-0`, `retail-12-0-5`, `retail-12-0-7`, and Retail with no enabled epoch currently lack both selected fixture files (default flags 0/0). JSON records exact paths. Other uncaptured profile/epoch/flag combinations also receive no fallback. Missing-data failure has executed selected-Wrath-extra proof; historical Retail missing scopes remain source/existence evidence only. One-row negative is a separate comparator sensitivity test.

PTR, Mists, Era, Anniversary, and Forever copied baselines are not claimed GREEN under newly compiled selection code. Previous `031711` 12-case capture remains 1 PASS / 11 FAIL, retained and not rerun. No native oracle, native parity, historical-epoch capture, full suite, or parent closure is established. Native UNKNOWN; parent OPEN.
