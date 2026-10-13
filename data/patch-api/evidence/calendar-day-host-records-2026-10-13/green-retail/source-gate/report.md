# Calendar day producer source/spec/readability gate

Target: 396f0c9a6ba705f9fd610d0a2c198ef910f67c4f.
Result: **no concrete in-scope defect hypotheses identified by source inspection**. Formatting PASS, exit 0. Runtime acceptance **UNPROVED**.

Canonical HEAD initially matched target; advanced concurrently to eb453a078ffc1ba3b6c76de35ba4d27300566ef4. Final comparison confirms all five scoped files still match target. No tracked edits performed.

## Artifact checks

[EXIST] PASS: spec 91 lines; c_calendar.rs 117; projection 81; queries 86; calendar_defaults.rs 172.
[SUBSTANTIVE] PASS: implemented strict argument parsing, host lookup, cloned DTO projection, and registrations; no new placeholder bodies.
[WIRED] PASS: c_api/mod.rs:375 calls c_calendar::register; c_calendar.rs:34-46 registers queries; queries.rs:40 calls projection. Existing external bootstrap callers at lua_api/globals/utility_system_spell/mod.rs:646 and workarounds/temporary/environment_cleanup_restore.rs:20. Workaround application remains at workarounds/mod.rs:316.
[ANTI-PATTERN] PASS for changed code: no new TODO/FIXME/HACK/XXX, warning suppression, empty implementation or commented-out code. Existing inert workaround functions were not newly added and remain outside producer scope.

## Contract coverage matrix

| Contract | Evidence | Proof level |
| --- | --- | --- |
| Empty host-owned day map, exact key, dense order | state/sim_state.rs:621; state.rs:522; queries.rs:13-19,27-38,69-86 | Source inspected; no runtime isolation/mutation proof |
| Required 2/3 numeric slots even on absent keys, no string/nil coercion | queries.rs:10-12,24-26,44-67; lua_bridge/from_stack.rs:42-49 returns nil on absent slot | Source inspected |
| All required secrets authenticated before type validation or lookup | queries.rs:51-57 authentication loop precedes :59-66 type loop; retail-12-1-0 gating | Source inspected; authentic secret runtime cases not executed |
| Finite integral representable selectors; no aliases | helpers.rs:7-17 integral i32 offset; queries.rs:69-86 integral u32 day and positive integral position below usize rounded upper boundary | Source inspected |
| Count integer numeric/zero miss; row exactly one table or nil | queries.rs:13-21,40-41; projection.rs:9-15 | Source inspected, no native arity/bounds claim |
| 21/20 outer fields; two independent six-field times | projection.rs:24-81 has 8 strings + 6 mandatory numbers + optional icon + 4 booleans + 2 times; :69-78 has six time fields | Source inventory checked, not Lua enumeration |
| Opaque IDs and primitive fidelity | projection.rs:27-28 directly interns owned IDs as strings; :40-63,69-78 preserves bool/f64 and exactly converts i32/u32 | Source inspected |
| Fresh snapshots, replacement/erase reflected in later lookups | queries.rs:31-38 clones owned DTO; projection.rs:14,67 allocates fresh tables per result/time | Source inspected; retained snapshots/GC scenarios not executed |
| Numeric-index/default fixture/filter unchanged | c_calendar.rs commit delta adds module declarations and two registrations only; old query/seed functions unchanged | Commit delta inspected |
| Remove only replaced count workaround | calendar_defaults.rs delta deletes only GetNumDayEvents constant-zero block | Commit delta inspected |
| No Lua editor, Club producer, normal day seed, native parity or output secrecy claims | Five-file delta adds query/projection only; spec explicitly excludes producer/parity and labels output secrecy UNPROVED | Source/spec inspected |
| Real cached ItemRef and CalendarUtil consumers | Unchanged by scoped commit; spec leaves GREEN acceptance pending | Not executed |

## Rooting/borrow/arity

Outer table is pushed immediately at projection.rs:14-15 and retained as the single result root. Each time table is pushed :67-68, populated, linked to rooted outer :79, then popped once :80. Two time publications leave exactly one result. Miss pushes one nil. Count pushes one Num; both queries return Ok(1).

methods.rs:576-590 temporarily roots each field value before key interning, raw-sets the field, applies barrier_back, and restores prior top. Nested times remain stack-rooted until linked. Installed rilua dependency at ~/.cargo/git/checkouts/rilua-fd5a0715e46b5888/842e4d3/src/vm/state.rs:252-345,943-960 shows table allocation/string interning track allocations without running collection; push/pop change stack top without callbacks. No collection/callback gap exists between allocation and initial push in this inspected path.

Host Ref is confined to queries.rs:31-38 and destroyed before projection at :40; count borrow temporary ends at count assignment before push. Required values remain original stack arguments; unwrap_secret takes &LuaState, preserves wrappers/taint, errors for insecure/revoked secret access (rilua table_security.rs:401-429), and leaves ordinary userdata to strict numeric rejection. No authentication shim added.

## Rust readability

All changed lines in four Rust files manually audited. No concrete violations identified: bounded helpers/nesting, clear effect-revealing publish names, explicit host borrow boundary, no suppressions or primitive parameter overload. Fixed-size validation loops implement mandatory ordered checks, not unnecessary Vec accumulation. rust-code-analysis-cli unavailable on PATH; no computed complexity score claimed. No adjacent cleanup suggested/performed.

## Command ledger / limitations

cargo fmt --check: run exactly once in canonical checkout; exit 0; stdout empty; stderr empty. Full captured output in cargo-fmt-check.log. Scoped files matched target at command time and final comparison. No check/build/tests/runtime executed, no hashes/binary/source archives, authentication, delegation or tracked edits.

Initial git call failed outside repository; corrected with explicit cwd. One evaluation referenced an unavailable prior variable and failed before invoking fmt. Final revision equality assertion failed because main advanced HEAD; scoped file comparison remained clean. These administrative failures produce no behavior evidence and did not trigger a fmt rerun.

Prior Retail 122e272bb / PTR 48902323e RED (each 0/9 PASS, nine seeded failures) is caller/spec-reported, not independently replayed or GREEN. Main owns compilation/runtime acceptance. Runtime rooting, authentic secrets, snapshots, consumer dependencies and numeric-index regressions remain unproved by this gate.
