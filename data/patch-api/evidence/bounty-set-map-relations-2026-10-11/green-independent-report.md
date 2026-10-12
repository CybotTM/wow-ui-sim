# Independent bounded Bounty GREEN verification

**PASS for explicit-input implementation at `262987b1a65f049d5c3e158b18a8eb73cc22e4ba`, within the supplied two-profile scope. Historical expanded-RED authentication GAP remains. Owning prefork publication sweep proof remains pending. No whole BountyUI/native/all-profile/full-handoff acceptance.**

Machine-readable ledger: `proof.json`. Source inspection: `green-source-inspection.json`. No commands that build, test, check, poll, delegate or operate production were run by this verifier; saved evidence was inspected. Repository untouched.

## Authentication and attribution

Each of the four FIRST NEW sealed files was streamed exactly once in 1 MiB chunks. Each computed SHA-256/size/comparison was written immediately to its epoch's `<binary>-independent-auth.json` before any long output. No RED binary, library payload or old Calendar artifact reopened. Reads were only the four new binary payloads in this table:

| Profile | File | Bytes | Independently computed SHA-256 | Comparison |
|---|---|---:|---|---|
| client-retail | integration-sealed | 467726840 | `f0cadeee39cabaeac145e9cc8e3a8fe946055b44676c17ab82e78dfd27105862` | MATCH |
| client-retail | wow-sim-sealed | 407415544 | `afbfe1719eee8180707cf6ffe5b79421150dfec4d69138fbd524147420d332c2` | MATCH |
| client-ptr | integration-sealed | 467628688 | `996e17c54a9a919b2aeb3ed748aaea5fa871ff43174cec20db1f91511d1f540a` | MATCH |
| client-ptr | wow-sim-sealed | 414739280 | `783a5492600979c7dcc137d5e42ad6bea53a455016a0e386dd735150503f89fb` | MATCH |

Epochs: `default-green-compile-20261012T025403623595Z` and `ptr-green-compile-20261012T025546330152Z`. Both revision receipts name exact262. Both integration and wow-sim Cargo records equal their entries in the fully parsed compilation JSON stream, `fresh:false`; terminal `build-finished success:true`, compile exit0 and recorded unchanged source. Actual recorded toolchain: cargo1.99.0, rustc1.99.0, x86_64-unknown-linux-gnu, LLVM23.1.1. Both profile feature records include `retail-12-1-0`, so both secret tests actually execute, not merely compile.

Committed test SHA-256 `1eec1a5c32e2b610c6e485972b55af6f2e906af41f903104d2a1e457c4c27f56` matches each revision receipt and expanded9602. `git diff 9602fa8fbb8fdba3176ac44d7cfa4a370ba720df 262987b1a65f049d5c3e158b18a8eb73cc22e4ba -- src/lua_api/state.rs src/lua_api/state/sim_state.rs tests/c_map_bounty_set_maps.rs` is empty. All six inspected Rust files match exact262 git objects. Main concurrently updates spec/wiki/publication data; report evaluates the committed262 spec, not these mutable updates.

## Saved runtime proof and complete reachability

Recorded command each: `/usr/bin/timeout 90 <epoch>/integration-sealed c_map_bounty_set_maps:: --nocapture --test-threads=1`, cwd `/home/osso/Projects/wow/wow-ui-sim`. Full runtime stdout/stderr inspected.

- Default verbatim result: `test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 10747 filtered out; finished in 0.88s`, runtime exit0.
- PTR verbatim result: `test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 9925 filtered out; finished in 1.23s`, runtime exit0.

**11 logical features executed under two profiles =22 scope executions, not22 different cases.** Every listed test is named `ok` in both logs. Because the bodies use synchronous `env.exec(...).expect(...)` and ordinary assertions, success reaches all statements listed below, unlike both preserved RED stages. References are `tests/c_map_bounty_set_maps.rs`; shared shape checks L10-20 and L154-176 establish one return, dense numeric keys, exact order and complete `ipairs` consumption.

| Test suffix | Source | Executed assertion boundary | Default / PTR |
|---|---|---|---|
| `default_associations_return_one_empty_sequence` | L38-45 | Absent sets 90001/90002: exactly one table, no keys, length zero, zero ipairs elements. | PASS / PASS |
| `empty_results_are_fresh_and_mutation_isolated` | L48-70 | Three distinct empty snapshots; caller mutations leave other and subsequent queries empty. | PASS / PASS |
| `required_number_rejects_missing_nil_and_wrong_types` | L73-85 | Missing/nil/string/empty string/table/booleans/function error with nonempty context; valid recovery. | PASS / PASS |
| `numeric_edges_return_empty_without_associations` | L88-101 | Zero, negative, fractions, NaN, infinities/extremes empty under absent input; alone not anti-alias proof. | PASS / PASS |
| `untainted_secret_inputs_preserve_wrappers` | L105-123 | Two secret numeric absent-set queries; wrappers/values retained, nil caller taint retained. | PASS / PASS |
| `tainted_secret_inputs_are_denied_before_lookup` | L127-149 | Both secret inputs denied with untainted-caller message, taint/wrappers retained, no declassification; ordinary queries and outer restoration succeed. | PASS / PASS |
| `supplied_relations_preserve_dense_order_and_duplicates` | L199-211 | 90001->[71,19,71], 90002->[42,19], explicit empty and absent: dense numeric keys, exact single return and complete ipairs order. | PASS / PASS |
| `owned_updates_and_removal_preserve_snapshots_and_other_sets` | L214-250 | Insert [19,88] then remove: subsequent outputs track changes, old snapshots and unrelated sets preserved. | PASS / PASS |
| `positive_results_are_fresh_and_mutation_isolated` | L253-277 | Distinct positive tables; hole/extra/value mutations isolated; final Rust input unchanged. | PASS / PASS |
| `fractional_and_extreme_inputs_do_not_alias_supplied_ids` | L280-301 | Positive 90001/90002/i32::MAX controls before and after invalid/fraction/overflow/nonfinite loop; no truncate/wrap/saturation alias. | PASS / PASS |
| `actual_map_util_consumes_relations_and_preserves_cache` | L304-341 | Entire profile cached MapUtil read and executed, actual ordered positive/empty results memoized by identity; owned update reflected direct but not cached; caller cached mutation persists only in cache, other set preserved. | PASS / PASS |

The real MapUtil test reads and executes **the entire profile-scoped cached source**, not extracted function text or a test replacement. Source execution has its own `.expect` before result/cache assertions. The passed test proves ordered result, repeated-table identity, cached empty identity, stale cached snapshot after owned mutation and caller cached mutation semantics. Cache identity is deliberately different from the direct API's fresh-table semantics. Cached-source provenance/native catalog fidelity is not independently re-certified here; the supplied `source-contract.json` identifies that boundary.

Security tests prove authentication ordering before absent lookup via the denial message and source ordering; they preserve both wrappers, values, inner caller taint, failed declassification and outer restoration. They do not supply positive memberships for secret-ID queries. Numeric anti-alias proof comes from the separate supplied-positive fixture including `i32::MAX -> [42]` before and after the invalid loop, not the absence-only numeric test.

## Mode B: exact-source artifact verification

[EXIST] PASS — git objects and local files present: `src/c_api/c_map_bounties.rs`46 lines; `src/c_api/c_map.rs`585; `src/c_api/mod.rs`390; `src/lua_api/state.rs`1053; `src/lua_api/state/sim_state.rs`706; `tests/c_map_bounty_set_maps.rs`341; committed `docs/specs/bounty-set-map-relations.md`36.

[SUBSTANTIVE] PASS — `register`, `validate_bounty_set_id`, `query_bounty_set_maps` implement a real supplied-state projection. L9 uses `!cfg!(any(feature = "client-retail", feature = "client-ptr"))` guard; L17-24 rejects nonnumeric inputs and recognizes only finite, integral, i32-range identifiers. L31 unwraps/authenticates secrets before L32 validation and L35-37 lookup. Cloned owned vector is projected to a newly allocated dense table L40-45 with return count1. `SimState.bounty_set_maps: HashMap<i32, Vec<i32>>` at sim_state.rs:579 is a literal field, initialized empty at state.rs:496. There are no seeded/default memberships, hierarchy/faction inference or revival of the retired getter in this change. `unwrap_or_default` is absent-association policy explicitly required by the spec, not an alternate implementation fallback.

[WIRED] PASS — `src/c_api/mod.rs:126` declares the module. `src/c_api/c_map.rs:95` invokes its registration from existing surface setup. `src/c_api/registration.rs:116` calls that surface; `src/lua_api/globals/missing_surface.rs:229` calls the registration seam. Integration entry `tests/integration.rs:1` expands harness macro; `crates/test-harness/build.rs:441-449` discovers top-level Rust test modules. Both actual runtime logs enumerate all11 module tests; normal raw-publication assertions independently show reachable public registration. Implementation is classified as modeled **C API** under `src/c_api/`, not a temporary shim or misc Lua-global default.

[ANTI-PATTERN] PASS — each inspected exact source/spec has TODO0/FIXME0/HACK0/XXX0/`except: pass`0. New46-line producer plus registration/state additions contain no empty catch, commented-out implementation, warning suppression, mock getter or stub. Representative substantive snippet: `sim.bounty_set_maps.get(&id)` copied and passed through `set_table_array(..., Val::Num(f64::from(id)))`; stored sequence order/duplicates are preserved.

OVERALL: PASS — bounded feature/wiring scope, with explicit gaps below.

## Normal publication, Calendar preservation and error-chain proof

Both normal sessions use their independently authenticated `wow-sim-sealed --no-addons --no-saved-vars --exec-lua <assertions> lua-errors`.

The actual recorded assertions require `rawget(C_Map,'GetBountySetMaps')` to be a function, direct absent-ID90001 result to be an empty table, second result to have distinct identity, and `C_Map.GetBountySetIDForMap==nil`. They then require all three Calendar mappings:910001->(0,15,1),910002->(0,15,2),910003->(1,4,1). Both saved normal stdout files are exactly `[]\n`; both normal exits0. Both complete stderr logs end `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`; no `[exec-lua] error`.

This is not an empty JSON output mistaken for skipped assertions. `src/bin/wow_sim/main.rs:549-559` forwards the provided exec string into `run_lua_errors` and exits1 on nonclean. `src/lua_errors.rs:47-71` executes the complete wrapped string synchronously via `exec_maybe_secure`, records any error through `call_error_handler`, then update ticks. `src/lua_api/script_helpers.rs:526-550` sinks the error **before** optional handler execution; L575-605 records raw/count/owner data. `src/lua_errors.rs:214-224,265-278` collects those entries; L79-85 serializes and returns whether errors are empty. Thus successful full assertion code plus empty recorded errors/exit0 proves these actual normal assertions ran. This preserves Calendar3 in the new joint normal sessions without rerunning any old full Calendar scope. ALSA no-device diagnostics in both logs are host audio observations, not Lua errors or new repair tasks.

## Recorded final gates and preserved limitations

`final-fmt.json`: exact262, `/usr/bin/cargo fmt --check`, exit0, stdout/stderr empty. `input-scaffold-readability.json` and `producer-readability.json`: manual scoped changed-Rust audits, violations[]. Initial `test-source-readability.json` is explicitly initial-source/NOT_EXECUTED; it is not presented as expanded-runtime proof.

`check-20261012T025820254115Z`: **finished**, saved outcome `EXECUTED`, exit0, `sources_unchanged:true`; submitted `/usr/bin/cargo check --offline --locked -j 12` at exact262. Sources_before equal sources_after and match inspected current code hashes. Saved stdout empty; stderr ends `Finished dev profile [optimized + debuginfo] target(s) in 32.06s`. Both compiles and this final check retain six iced manifest deprecation warnings; this is not warning-free evidence. No gate rerun.

Initial authentic RED6 report/proof retained:0PASS6FAIL/exit101. Expanded RED11 behavior/reachability report retained:0PASS11FAIL/exit101 before producer. **Expanded RED independently computed hash comparison was printed then lost, with no retained auth receipt; GAP MUST REMAIN.** No second RED payload read, library authentication, history recovery or Calendar artifact reread attempted. `positive-red-auth-recovery-gap.json` remains authoritative on that limitation.

Publication current row77 was present at spawn per caller. Both normal sessions now prove runtime publication; main may remove current row after that proof and was observed changing the gap file concurrently. This verifier neither edits/removes the row nor treats removal as the separate owning-prefork sweep proof. **Owning sweep proof pending**. Frozen historical records stay separate.

Not proved: whole Bounty board lifecycle, all profiles, native catalog/order/unknown-ID/build fidelity, secret supplied-positive lookup, or full handoff completion. No adjacent warning/audio/source drift observation converted into an implementation task. Verifier writes only private-root auth/source/report/proof records.
