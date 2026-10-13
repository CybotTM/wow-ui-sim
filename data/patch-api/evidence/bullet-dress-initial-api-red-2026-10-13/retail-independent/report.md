# Retail initial dressability API RED — independent historical classification

## Scope
Read-only classification of already executed evidence at `TOKEN_RETAIL_ROOT`; source revision **TOKEN_IDENTIFIER_07** as recorded by metadata. Read complete runtime admission.json, argv.json, result.json, launcher.txt, stdout and stderr; receipt-integration.json, seal-integration.json, revision.json, source-scope.json, submission.json; and source/tests/item_dressability.rs. No test, build or runtime rerun. No executable, source, archive, library or cache authentication. Metadata provenance is reported, not independently certified. Historic inputs remain unchanged. No parent/producer acceptance claim.

## Result
**Genuine reached API behavior RED, not an observed setup failure.** Historical summary: **0 passed, 2 failed, 0 ignored, 0 measured, 11005 filtered out; exit 101**. Stdout duration 0.63s; launcher reports exit-code/status 101, 710ms service runtime, 650ms CPU and 130.2M peak memory. Both fresh WowLuaEnv initializations complete before their assertion failures. Admission records allowed=true, Retail, worker max 4GiB/high 3GiB. No recorded timeout, OOM, missing API or VM initialization failure caused these two failures. This does not establish unobserved setup properties.

Recorded command selects `item_dressability:: --nocapture --test-threads=1`, through systemd-run, timeout 86s and bwrap; source snapshot is read-only mounted at the project path, canonical target is overlaid read-only, network unshared, /tmp tmpfs, WOW_SIM_CASC=0 and WOW_SIM_NO_SOUND=1. Unit name contains `speak-retail-green`, but argv and failure output identify this dressability run; the name is not a GREEN result.

## Exact first failures and coverage
| Case | First reached failure | Proven before failure | Unreached |
|---|---|---|---|
| item_dressability_empty_input_returns_exactly_one_boolean_false | `miss(query(6948))`, source line 22; predicate at lines 16–17: `empty supplied classification must return boolean false`; Rust panic at tests/item_dressability.rs:28:6 | Fresh environment; public query is a function (12); generated 6948 hyperlink exists and matches (19–21); numeric 6948 query returns exactly one value (14) | Numeric 901001 (23), short item string `item:6948` (24), generated hyperlink query (25) |
| item_dressability_requires_item_info_and_authenticates_vm_secrets | Zero-argument `argumentError()` at line 46; predicate at 42: `required ItemInfo must reject input`; Rust panic at tests/item_dressability.rs:100:6 | Fresh environment; settaintmode call completes (36); nil stack taint assertion passes (37); public query is a function (39); pcall executes (41) | Second validation predicate (43–44), explicit nil/false/table/function inputs (47–50), all remaining selectors and security checks (52–97) |

First test proves the single returned value is **not boolean false**. Output does not expose that value: do not claim true rather than a nonboolean. Second test proves zero-argument pcall did **not produce the required failure with a string error**. The conjunction does not distinguish a successful call from failure with a nonstring error; output alone cannot resolve that distinction.

Security coverage is **unreached**, not failed or passing: creation/authenticity of numeric, hyperlink, false and nil VM wrappers (55–65); untainted secret queries and malformed/missing validation (66–69); object-tainted closure execution (92–93); AllowedWhenUntainted denial before parsing/lookup for all four wrappers (71–86); wrapper retention, unwrap denial and caller-taint preservation (77–79); tainted ordinary numeric/link misses and false validation (87–90); outer taint restoration, wrapper round-trip and final untainted secret queries (94–97). Nil stack taint setup is the only reached taint assertion, not proof of these security behaviors. Hyperlink generation passes only in the first test; hyperlink dressability selection is never reached in either.

## Receipt limits
Existing receipt records PASS, hash_reads=1 and sha256_matches_seal=true for TOKEN_IDENTIFIER_09. Receipt stat, seal stat and runtime result reused stat match as recorded: device 66306, inode 142896100, size 471300104, mtime_ns 1791934848436960034. Seal records Retail/client-retail and SHA-256 TOKEN_IDENTIFIER_01. No new hash/stat/authentication of that executable was performed. Receipt explicitly excludes external libraries/cache/settings and source archive authentication. Source linkage is metadata attribution, not independent authentication.

These failures concern the test's approved empty host classification policy (source line 1), not native item classification proof. Supplied nonempty data, consumer acceptance, PTR or later revisions are not established. Preserve both historic failures regardless of later GREEN evidence.

## Output handling
Absolute report.md and proof.json written privately (files 0600; directory 0700 for owner traversal), each file fsynced, directory fsynced, then immediately fully read back. No historic evidence modified.
