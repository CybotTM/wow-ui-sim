# Mount-item final bounded verification

**BOUNDED RUNTIME PASS; RAW CONSUMER OBSERVATION GATE FAIL; PARENT OPEN.**

Source `7487c8becb5f9f8102e42492b524211011a71f07`. Main-owned unit `wow-mount-item-final-20261012t153719017422z.service` observed terminal `active/exited`, `Result=success`, `ExecMainStatus=0` at 2026-10-12T15:42:01Z, within requested 20-minute bound. No build/test/check/runtime replay, unit mutation, delegation, vendor repair, or repository edits. Report and `final-current-proof.json` saved only after terminal.

## Exact execution and reuse ledger

| Profile/operation | Actual result | Credit |
|---|---|---|
| Retail query | prior 8 passed, 0 failed, exit 0, 1.14s | REUSED from actual `ab8dd8739` proof |
| Retail spell preservation | prior 16 passed, 0 failed, exit 0, 1.63s | REUSED |
| Retail whole-template consumer | NEW 2 passed, 0 failed, exit 0, 3.54s | Actual baseline + live supplied relation contrast |
| Retail normal CLI | prior exit 0; one actual marker followed by `[]` | REUSED; 291 Blizzard addons loading recorded |
| PTR query | NEW 8 passed, 0 failed, exit 0, 0.61s | Actual supplied-state and secret ordering coverage |
| PTR spell preservation | NEW 16 passed, 0 failed, exit 0, 1.11s | Existing simulator policy preserved |
| PTR whole-template consumer | NEW 2 passed, 0 failed, exit 0, 3.08s | Actual baseline + live supplied relation contrast |
| PTR normal CLI | NEW exit 0; literal `MOUNT_ITEM_NORMAL_PROBE_PASS` followed by `[]` | One executed marker; parsed zero errors; 283 Blizzard addons loading recorded |
| Fresh default cargo check | NEW exit 0; 22.12s | Current default library scope, after both profiles |
| Fresh cargo fmt --check | NEW exit 0; empty stdout/stderr | Current captured formatting proof |

**28 new test executions = Retail 2 + PTR 26. 24 reused query/spell cases. 52 total test scope; zero test failures.** Normal CLI operations and check/fmt are separate, not added to test counts. Earlier Retail baseline 1 is not double credited. Independent `git diff ab8dd8739 -- src tests/mount_from_item.rs tests/mount_spell_identifier.rs Cargo.toml Cargo.lock` is empty, exit 0; producer recorded the same reuse scope. No old Retail normal seal or any other old seal was read, hashed, or executed.

## Consumer boundary: behavior passes, requested raw evidence missing

Both profiles passed the **unsliced original** `PlayerChoiceBaseOptionItemRewardTemplate`/intrinsic ItemButton tests. Source assertions require whole addon closure, vendor method identities and Lua source suffixes, generated Hearthstone item 6948 and nonempty item link, original `SetItem(6948)`, stored link, item ID, and boolean from actual vendor `IsDressupReward`. No mocked getters or replacement predicate.

The passing live-contrast assertions establish baseline **false**, supplied `6948 -> 401` **true**, removal **false**. These values follow from bool return, mapped assertion, inequality and removal equality—not raw emitted observations. First contrast executions GREEN in both profiles; no prior consumer-contrast RED claim.

**FAIL: all requested raw console provenance, input/link, stage, API type, result and predicate registration/type/value lines are absent from both captured stdout/stderr.** Retail full consumer stderr: 127,215 characters / 863 lines; PTR: 126,323 characters. No `[mount-item consumer console]` lines. Source `tests/mount_item_player_choice.rs:181-196` evaluates the script then drains only `SimState.console_output`. Cached `Blizzard_PrintHandler.lua:67-73` routes its default print handler to `DEFAULT_CHAT_FRAME:AddMessage`; `:96-98` replaces `print`. This explains a plausible capture boundary; no extra routing probe was executed. Captured raw supplier/result details remain unavailable, not invented from passing assertions. This observation gate prevents declaring consumer handoff complete.

Source-inferred supplier is an **existing unmodeled nil-returning namespace fallback**, not modeled/native dressability: no `IsDressableItemByID` implementation found in Rust source or cached vendor implementations; `runtime_surface_bootstrap.lua:66-77` fabricates and raw-registers a missing namespace member as a Lua function returning nil, and `:209-216/:1584` attaches C_* namespaces. Runtime raw predicate type/value/provenance is NOT retained. Cached original `Blizzard_PlayerChoiceOptionBase.lua:662-677` checks dressability first, mount second, pet third, then false. Passing controlled relation contrast proves consumer dependence under this simulator setup; does not establish native Hearthstone classification or real catalog semantics. No new predicate production or vendor repair.

Each profile retains 118 empty addon-warning records, 40 nonempty nil-observation records, 18 nonempty missing-requirement records, and 122 empty records each for Lua errors/error records/error counts (366 empty snapshots). Missing requirements remain diagnostics, not test assertion failures. Full raw streams, including nonempty diagnostics, embedded in proof.

## Query/source coverage

- Eight exact query tests cover exact-one-nil miss, required-number error, authentic secret permission before type/lookup, explicit no-catalog/no-ownership relation, replacement/removal, fresh-env isolation, populated-neighbor/range nonaliasing, and mapped-secret secure hit/tainted denial/ordinary recovery.
- All 11 source inputs present; before/after compile/runtime for both profiles plus before/after fresh check and independent current hashes match. Independent current source applicability diff against `7487c8bec` is empty. Exact paths/hashes retained in proof.
- `c_api/mod.rs:130-131` and `registration.rs:30-31`: Retail/PTR module/register gates. `c_mount_item.rs:16`: authentic secret unwrap first; `helpers.rs:7-16`: finite integral signed32-bit supplied-relation simulator policy, not native range. `sim_state.rs:580` map; `state.rs:496` empty default; `c_mount_item.rs:23` live lookup. No production relation seeding.
- Existing `c_mount_spell_lookup.rs` diff against `3cc6d36af` empty. Separate SpellIdentifier reader and ordinary journal-record lookup unchanged. 16 preservation cases per profile pass; native AllowedWhenTainted remains unmodeled. Earlier-profile cfg exclusion is source-inspected, not runtime-tested.
- Manual scoped Rust readability: no violations observed; no metrics/check rerun. New query/query tests/consumer test have zero TODO/FIXME/HACK/XXX/allow markers. Spec sections follow required order; unchecked requirements and GREEN/implementation-pending prose remain stale relative to query proof. Observation gap is real. No documentation mutation authorized here.

## Artifacts, compiler, capacity

Only three **new** seals authenticated, once each, after receipt absence check; exclusive immediate `independent-auth-{integration,normal}.json` receipts saved in corresponding new epochs. Existing receipts reused without seal reads. Each metadata Cargo record occurs exactly once in its captured compiler JSON; 741 records per compile, build-finished success=true, zero compiler-message records. Both compile exits 0.

| New seal | Bytes | SHA256 |
|---|---:|---|
| Retail final integration | 468253640 | `ac7e30792e8158adcee7a70bab72f4ffc324eb1055ca2ab8d4a31999229af623` |
| PTR final integration | 467563248 | `66c9b58bcfbe8d4986470b9abe1344ce4aba295403773d7b22d0f0dee41274ee` |
| PTR final normal | 414448552 | `b903e672115ae2023844d4489135fbf85303d53acd4b847af381ab0f05ebec6b` |

Retail metadata carries default/client-retail/retail-12-1-0; PTR client-ptr/retail-12-1-5; GUI/CASC/sound included. Cargo/rustc 1.99.0 Arch; rustc `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`, x86_64-unknown-linux-gnu, LLVM 23.1.1.

**Not warning-free:** six distinct iced-wgpu-patched manifest deprecated hyphenated Clippy keys plus six-warning summary recur in each compile and fresh check. Full stderr retained. PTR runtime admission initially false at load1 29.14 despite 25.50 GiB available; admitted after bounded wait at load1 23.44 / 34.93 GiB. Not test failure, timeout, or missing-case inference. All capacity histories retained.

Runtime inherited allowlist recorded unset. Query/consumer no overrides; normal `WOW_SIM_NO_SOUND=1`, `--no-addons --no-saved-vars`. No historical/child/cache-content equivalence attestation.

## Immutable history and open gates

Original 0/3 and expanded 0/7 genuine missing-API RED preserved; original retained report/proof hashes match prior accepted evidence. Unreached downstream assertions receive no retrospective RED credit. Eighth mapped-secret query first actual GREEN, no prior RED. New consumer contrast first GREEN, no prior RED.

Old default-check proof invalid after state/library change; fresh captured default check now PASS. Bounded query PASS; bounded original-consumer contrast assertions PASS; **raw consumer observation gate FAIL; parent OPEN**. No full-suite/native/catalog/dressability/credit-completion proof.

Observer diagnostics retained: one pre-operation NameError on nonpersistent `root`, resolved by self-contained observer; one optional post-save old-normal summary KeyError (`stderr` vs retained `full_stderr`), resolved by inspecting saved schema/output. Neither caused replay or seal reauthentication. No failures suppressed.
