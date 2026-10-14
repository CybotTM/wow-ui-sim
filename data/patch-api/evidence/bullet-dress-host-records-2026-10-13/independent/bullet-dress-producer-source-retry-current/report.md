# Bounded SOURCE/spec/manual-readability retry

Date: 2026-10-13
Requested revision labels: 6af710814 + 4b9cb30dc + 402ab5a88.
Disposition: bounded source observations PASS; original harness failure failed4126 preserved; reportaccepted=false. This is not operational acceptance.

## Evidence boundary

Only absolute-path functions.read and functions.write used. No cwd switch, delegation, commands, build, runtime, default checks, formatter, hashes, executable/archive/library/cache reads, or previous accepted-payload reads. Observed current files, not authenticated commit contents. Revision membership and source-byte provenance remain UNPROVED. Exact original harness error text was not supplied or recovered; failed4126 is retained as supplied, not reclassified or overwritten. This retry cannot establish that its harness failure is repaired.

## Coverage matrix

| Claim | Observation | Proof level |
|---|---|---|
| Original VM-secret numeric widget argument | queries.rs:29-35 passes stack_val(state, 1) directly to unwrap_secret, propagates error, then requires Val::Num. bullet.rs:26-30 calls this shared boundary. | Source PASS; secret/taint execution UNPROVED |
| Clone before allocation | bullet.rs:31-43 clones the selected host record before create_table. Missing record pushes one Nil and returns 1. | Source PASS; runtime arity/lifetime UNPROVED |
| Rooted fresh fifteen-field row | bullet.rs:42-48 allocates/pushes row before populating it. Numeric writer supplies ten fields, string writer three, plus lines and hasTimer. | Source PASS; GC/native parity UNPROVED |
| Rooted fresh ordered lines | bullet.rs:82-92 creates/pushes a new lines table, writes strings at enumerate index + 1, attaches it to row, then removes the temporary stack root. Empty input leaves an empty new table. | Source PASS; runtime freshness/GC UNPROVED |
| Original item unwrap before type/parse | c_item.rs:200-216 unwraps stack_val(state, 1) before Num/Str validation and parse_item_id_from_val. Existing parser is reused, not a new range/grammar policy. | Source PASS; malformed-secret/taint/recovery execution UNPROVED |
| Explicit bool map, false miss | c_item.rs:207-215 reads only sim.item_dressability.get(...).copied(), with false for absent parsed identity or absent map record; returns Val::Bool. No metadata, equipment, consumable or mount inference in this query. | Source PASS; runtime projection/isolation UNPROVED |
| Publication only Retail/PTR runtime const | bullet.rs:12-23 and c_item.rs:86-99 return before method registration unless ACTIVE matches Retail or Ptr. queries.rs:24 reaches bullet registration; c_item.rs:77-83 reaches dressability registration. | Local wiring source PASS; exhaustive older-profile public-surface absence UNPROVED |
| Uniform empty carrier does not itself publish API | sim_state.rs:732-733 declares a plain HashMap<u32, bool> field without profile cfg; state.rs:625 initializes HashMap::new(). Publication is a separate guarded registration, not a field method. | Source PASS for inspected paths |
| Corrected seeded-set rejection baseline | tests/widget_bullet/feed.rs:84 onward snapshots complete existing sets before missing-set rejection and compares the full map afterward. After authorized empty-set supply it snapshots the full map again, then compares after type mismatch, checks empty bullet backing and membership, and zero observed notification counters. | Authored oracle source PASS; corrected GREEN UNPROVED |
| Spec alignment | Both owning specs describe explicit supplied inputs, original secret authentication, fresh snapshots, Retail/PTR publication, pending GREEN and native limits. They do not claim native classification or passing consumer behavior. | Bounded source/spec PASS |

## Literal representation

Bullet query uses a cloned BulletTextListRecord value read through ui_widget_inputs.bullet_text_lists. Its Lua result row/lines are Val values returned by create_table and temporarily pushed on the Lua stack. Item storage is a SimState HashMap<u32, bool> field, not a separate dressability class/type. Registered C API functions are Rust function pointers installed by table_set_rust_fn_static. Tests create a Frame via public CreateFrame and attach an OnEvent script via SetScript; frame internals and public frame type/raw-operation behavior were not inspected or established in this retry.

## Manual bounded readability

Inspected producer/helper/query/registration lines, uniform field/constructor lines and feed tests. No in-scope manual readability violation identified. Short named projection helpers separate numeric, string and lines writes; clone precedes allocation; temporary stack-root lifetime is visible; item unwrap/validate/parse/read/push sequence is explicit. Both feed test bodies stay below the skill's 200-line test threshold. No TODO/FIXME/HACK/XXX, warning suppression or commented-out implementation observed in the bounded new producer/query portions. No automated complexity metrics or repository-wide scan performed. Adjacent existing item helpers were context only, not a broad readability gate.

## Limits and retained failures

- failed4126 remains the original unresolved harness failure; no report acceptance is granted.
- No native, GREEN, build, runtime, earlier-profile execution, consumer, GC stress or source-byte provenance proof established.
- Rejection test source proves its full seeded-set comparison oracle exists, not that rejection production behavior passes. Host feed implementation and fixture definitions were outside the requested file reads.
- Caller-supplied formatting ledger: 3 Bullet + 3 item + 1 test Rust files, exit 0. Not rerun or independently evidenced here.
- Main owns operational proof. Broader registration/bootstrap paths were not exhaustively inspected.
- functions.write confirms file writes only. No fsync command/tool used; durable persistence across crash/power loss is UNPROVED.

No repository files modified. Private retry report/proof only.
