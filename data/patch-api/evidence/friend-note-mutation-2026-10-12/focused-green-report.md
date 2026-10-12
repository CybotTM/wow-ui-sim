# Focused friend-note current verification

**Bounded GREEN PASS; requested default cargo check BLOCKED (zero credit).**

Revision: `a883157c49deec4685fb6a8afc2f72e041564676`. Final HEAD: `a883157c49deec4685fb6a8afc2f72e041564676`. Tracked source/config/data hashes stable across producers, runtimes, final audit (27273 files). No source/spec edits, operations, delegation, old executable rehash, or broad suites.

## Command evidence

| Stage | Actual result |
|---|---|
| Retail fresh cargo test producer | exit 0; integration and normal wow-sim artifacts `fresh=false` |
| Retail integration | `8 passed; 0 failed; 0 ignored; 0 measured; 10805 filtered out; finished in 2.02s`; exit 0 |
| PTR fresh cargo test producer | exit 0; integration and normal wow-sim artifacts `fresh=false` |
| PTR integration | `8 passed; 0 failed; 0 ignored; 0 measured; 9983 filtered out; finished in 1.97s`; exit 0 |
| Retail normal startup probe | exit 0; stdout `FRIEND_NOTE_NORMAL_PROBE_PASS\n[]\n`; stderr `Lua errors: 0 unique, 0 occurrence(s)` |
| PTR normal startup probe | exit 0; stdout `FRIEND_NOTE_NORMAL_PROBE_PASS\n[]\n`; stderr `Lua errors: 0 unique, 0 occurrence(s)` |
| `/usr/bin/cargo fmt --check` | exit 0, once, empty output |
| `/usr/bin/cargo check --offline --locked -j12` | **Never invoked**: two bounded admission cycles failed load<24, samples 32.94–43.01; zero check credit |

Cargo: `cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)`. Rustc: `rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)`. Actual versions/config/env and full stdout/stderr saved in epochs. Cargo producers preserve six iced-wgpu manifest deprecation warnings apiece; zero compiler-message diagnostics. Popup optional guarded nil-probe observations remain full in stderr; closure loader warnings and recorded Lua errors assertions passed.

Retail command: `/usr/bin/cargo test --test integration --no-run --message-format=json --offline --locked -j12`.
PTR command: same, with `--no-default-features --features sound,gui,casc,client-ptr`.
Each profile produced its new executables, copied to exclusive separate seals, bound to producer stdout SHA-256 and artifact JSON; each new seal authenticated once with immediate exclusive receipt. Integration argv: `timeout 90 <integration-sealed> friend_note_mutation:: --nocapture --test-threads=1`, environment `WOW_SIM_CASC=0 WOW_SIM_NO_SOUND=1`. Normal argv: `timeout 90 <wow-sim-sealed> --no-addons --no-saved-vars --exec-lua <existing normal-probe.lua contents> lua-errors`, `WOW_SIM_NO_SOUND=1`. Exact argv resides in command ledger. No command reruns.

## Capability matrix

| Spec bullet | Retail/PTR proof |
|---|---|
| 1: Raw setter registration; exact hit/miss; one bool; unchanged hit true; empty clears; preserves count/order and all other friend fields | PASS — Both profiles: updates_all_live_views_and_found_is_not_changed + exact_misses_do_not_mutate_or_create_records; normal probe also raw owned C API debug.getinfo what=C |
| 2: Exact getter, online-first indexed getter, unsorted C_Social index/GetFriends; unrelated state and two environment independence | PASS — Both profiles: shared checkNotes and assert_friends_match; plain_tainted_calls_stay_environment_local |
| 3: Strict strings/UTF-8; actual secretwrap values; untainted either/both; malformed wrapped numbers/bools/table/function/invalid UTF-8/nil positional errors; wrappers retained | PASS — Both profiles: requires_two_strict_utf8_strings_without_coercion; accepts_untainted_authentic_secret_strings_on_each_argument; untainted_malformed_secret_payloads_have_positioned_type_errors |
| 4: Actual object-tainted caller; secret access errors before validation/write on hits/misses/invalid payloads; invalid plain arg1+secret arg2; retained taint/wrappers/state; plain under taint | PASS — Both profiles: authenticates_both_original_arguments_before_type_or_write; plain_tainted_calls_stay_environment_local |
| 5: Whole cached closure; loaded vendor source checks/identity retention; actual button Click with real SetText; whole original Enter zero-return; hide/reopen after accepted/cleared notes; no recorded Lua errors or loader warnings | PASS — Both profiles: real_accept_click_and_original_enter_reopen_from_getter |

All eight tests completed; downstream assertions earned credit only through completed cases. Manually inspected real secrets/types/taint ordering and the entire popup block: original vendor source/identity checks, real edit-box writes, actual Accept Click, whole original Enter, zero returns/root-hide, reopen reads changed/cleared notes, retained handlers and friend fields. No shortened/copied handler fixtures or weakened assertions.

## Readability/spec

Changed production/tests manually audited against entire rust-readability skill: no violations. Short explicit setter, scoped mutation borrow, named values; bounded fixture/test functions and shared behavioral assertions. rust-code-analysis-cli unavailable at documented path. At the recorded verification epoch the spec had pending-GREEN/unchecked prose; later documentation reconciliation is separate. Five contract bullets now have bounded evidence above, not native semantics or publication closure.

## Evidence and limits

`focused-green-current-proof.json` and `command-proof-ledger.json` identify exact revision, scope, commands, output, source stability and admission blocks. Separate producer/runtime/audit epochs retain every artifact and full log. First conservative admission block gave zero credit, then corrected relevance admitted Retail; no failed producer retry. Retail normal marker stdout parsing corrected from saved output, not rerun. Default check remains a capacity blocker, not a code failure. No claims for publication/native/network/events/persistence/earlier-profile/broad/whole acceptance.
