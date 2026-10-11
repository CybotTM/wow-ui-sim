# Independent MovieFrame subtitle arity audit

**PASS — bounded saved Retail simulator evidence. Native/full-profile/parent acceptance remains open.**

Read and followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as assigned verifier. Saved-artifact audit only; no delegation, replay, build, test, live operational checks, Bash, rg, fd or large-binary rehash. Only this report and its JSON companion written. Original private evidence unchanged.

## Exact CLI proof

Epoch `20261011T034451Z`, October 11, 2026 UTC, 03:44:51.298786–03:44:55.687879. One actual command:

`timeout 90 <accepted Retail sealed wow-sim-normal> --no-addons --no-saved-vars --exec-lua @<epoch>/probe.lua lua-errors`, cwd `/home/osso/Projects/wow/wow-ui-sim`, `WOW_SIM_CASC=0`.

Two actual `CreateFrame('MovieFrame', ..., UIParent)` instances, two passing `GetObjectType() == 'MovieFrame'` assertions, then explicit Hide on both. Four cases: first_true, second_true, first_false, second_false. Each asserts `select('#', movie:EnableSubtitles(enabled)) == 0`. Thus **4/4 exact-zero-return calls**, true and false on each of two instances; not a nil-expression inference.

Extracted setter explicitly called with ordinary Frame and `{}`: **2/2 rejected receivers**, via `not pcall(setter, receiver, true)`. No captured error text or native receiver-policy claim.

After each boolean call and after both rejected controls, both public `IsShown()` results remain false: **5 pair checkpoints / 10 queries**. This is public hidden-state preservation only, not proof of underlying Rust subtitle preference or native playback.

Full raw stdout, eight lines:

```text
MOVIE_ARITY first_true 0
MOVIE_ARITY second_true 0
MOVIE_ARITY first_false 0
MOVIE_ARITY second_false 0
MOVIE_RECEIVER Frame rejected
MOVIE_RECEIVER plain_table rejected
MOVIE_ARITY_DONE
[]
```

Outcome: **exit 0, one completion marker, JSON [] / 0 entries**. Full stderr read; ends `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`, `Attributed owners: 0`, `Unattributed Lua errors: 0 occurrence(s)`. ALSA/no-audio diagnostics present; stderr is not empty. Private identity-bearing diagnostics not reproduced here or modified.

## Custody and requested guard

Root worker code equals request code equals completed controller executed code, byte-for-byte. Root/epoch probes identical; small probe SHA matches submission `45a4e4a44b88a95370f614a5507ff560240b8fbb846ac005def8e02fa7340e58`. Controller stderr 0 bytes; controller completion names this epoch. Queue submission exits 0 for requested service `wow-movie-arity-20261011t034450z`.

Submission artifact equals accepted unit2 Retail `normal-artifact.json`. Accepted independent unit2 report identifies normal non-test Retail artifact, 407315976 bytes, SHA **66402b0b9bd93cbad7dc082706962ff28a41ca79601e61b804e9542bd10c4451**. Worker pre-execution hash must match this seal to proceed; outcome post-execution `artifact_unchanged: true`, `probe_unchanged: true`. These are saved worker attestations plus accepted artifact/source custody, not independent binary rehash or new source seal.

Accepted unit2 source report: 3846 proof-scope inputs equal cc4f4c782; later 24c727075 docs-only difference does not change those inputs. This audit reuses that accepted report, not a new current-HEAD/hermetic provenance claim.

Requested `agents.slice`, CPUQuota=1200%, MemoryMax=16G. Worker receipt records membership in that named service under agents.slice. Admission attempt 1: 34.00035858154297 GiB available, load1 5.31, relevant Cargo list empty; worker threshold >=6 GiB, load<24, no relevant Cargo. Up to five bounded backoff attempts are coded, only one used. Timeout 90 seconds is in executed argv. **Requested properties and saved membership do not prove effective/inherited cgroup limits or enforcement.**

## Prior saved469 test — separate proof

Original raw log has 16513 lines. Line **7781**, exactly one movie test runner record:

```text
PASS [   0.284s] ( 7078/10698) wow-ui-sim::integration movie_subtitles::movie_subtitles_follow_boolean_setter_without_starting_playback
```

Read saved469 test body via read-only `git show 469a92d27bd2c45e590b1d22bf9137ce4c70f3d1:tests/movie_subtitles.rs` (exit 0), byte-equal to current inspected body. Valid XML `<Ui>` contains two hidden MovieFrame elements; load warnings empty; both stored object_type_name values MovieFrame. Rust storage checks observe false/false initially, true/false after first true, false/true after second true plus first false. Three successful setter calls; public hidden checks; ordinary Frame rejection plus five bad argument controls (string, number, table, explicit nil, missing). Final stored state preserved and collected Lua errors empty.

**Prior `EnableSubtitles(...) == nil` assertions do not distinguish zero results from one nil.** Their PASS proves only those expressions, not exact arity. The new four `select('#', ...) == 0` assertions close that narrow saved Retail arity gap. New CLI does not independently observe the Rust boolean store/isolation. Whole saved469 suite remains FAIL with three failures; do not relabel this old execution as fresh current-source execution.

## Source interpretation

[EXIST] PASS: implementation, registration, metatable/table representation and test files inspected.

[SUBSTANTIVE] PASS: `src/lua_api/frame/methods/widgets/movie.rs:10–30` extracts frame identity, requires Val::Bool, resolves widget, requires `object_type_name == Some("MovieFrame")`, writes instance field, returns `Ok(0)`. Playback remains unsupported.

[WIRED] PASS: movie register at widgets/mod.rs:98; widgets registration at env_init/frames.rs:61; shared __index clone at :70–78. Widget index filtering in methods/frame_metatable.rs does not hide EnableSubtitles. Consequently broad __index keys/enumeration do **not** imply universal successful dispatch: receiver validation is in the implementation. Ordinary Frame rejection is consistent with that guard; plain-table rejection exercises frame identity extraction instead.

Frame refs are **Val::Table**, not userdata (`methods.rs:126–144`). `table_builder.rs:98–110` allocates frame table with native backing and slot-0 userdata identity token; token representation must not be confused with the frame value. No native `type`/raw-operation compatibility claim.

[ANTI-PATTERN] PASS: inspected movie implementation contains 0 TODO/FIXME/HACK/XXX markers, no placeholder setter body. This is a bounded source inspection, not a new code-change review or full-project scan.

## Limits

No native coercion, exact native errors, native receiver policy, movie playback/rendering, all-profile acceptance, effective cgroup enforcement or whole-suite/handoff completion. Public hidden state, prior Rust preference assertions and exact return arity are distinct proof scopes. Saved receipts are evidence, not hermetic build provenance. Main owns later retained packet and stderr identity redaction; originals stay private and untouched.

Machine-readable details: `independent-report.json`.

Retention note: private verification root redacted; raw report JSON remains private.
