# Narrow secret-fixture delta verification

Date: 2026-10-12. Base `be9c734a45c50c422ae95b1815477fa9fc0a6839`; target/observed HEAD `6ec0c1cb84489963a24cd2017a4099a5c571301a`.

**PASS — source-level preservation of the exact getter's secret contract; no changed-test readability findings. Not a current runtime PASS, producer fix, helper fix, or native acceptance.**

## Scope and reusable evidence

Read verify and Rust readability skills; independently inspected the two-file delta, affected security fixture, literal spec, unchanged getter, actual value-access fallback and saved first-GREEN diagnostic. Delta: `tests/friend_info_exact.rs` and `docs/specs/friend-info-name-query.md`, 7 insertions/7 deletions. No changes under `src/`, `Cargo.toml`, or `Cargo.lock`. Observed working source/test/spec match target. Existing untracked `.code-index.db` and `verification/` preserved.

`source-proof.json` and `source-readability-report.md` remain reusable **for the unchanged producer only**, at their original source-only proof level. Their test-file/HEAD identity statements describe the earlier revision; they are not current security-test execution evidence. This report covers the new test delta. Spec requirement bullets are unchanged; only the Known gaps/progress paragraph changed and remains unchecked.

## Falsification: did removal weaken getter assertions?

Literal requirement, `docs/specs/friend-info-name-query.md:9`: “Authenticate wrapped values before validation: untainted secret names work, tainted secret arguments fail without clearing caller taint, ordinary names remain usable by tainted callers.” No requirement makes `canaccessvalue` an oracle or prerequisite for this getter.

| Exact contract/control | Target fixture evidence (`tests/friend_info_exact.rs`) | Delta and proof level |
|---|---|---|
| Real secret identity | `secretwrap(NAME)` at100; `issecretvalue(name)` at101 and113 | Both identity assertions retained; source only. No fabricated replacement global/value. |
| Untainted lookup | nil stack taint99; `check(query(name))`108 and121; one result103, table105, exact name/note106 | Outcomes and arity retained; source only. |
| Wrapped type rejection | `pcall(query, secretwrap(901001))`109; failure and string-validation message110 | Unchanged; source only. |
| Tainted secret rejection and ordinary acceptance | actual closure taint118; caller context112; failing `pcall(query, name)`114; exact ordinary-name result115 | Unchanged behavioral assertions; source only. Accept-all or reject-all getter substitutions still fail this fixture. |
| Taint preservation/restoration | caller remains `ExactFriendProbe`116; nil caller taint120; secret query works again121 | Unchanged. Preservation is checked after the rejected-secret and accepted-ordinary sequence, not separately immediately after rejection; no stronger claim. |

Only two **helper** predicates removed: positive `canaccessvalue(name)` in untainted setup, negative `not canaccessvalue(name)` in tainted probe. All getter calls, expected fields/type/arity, rejection outcomes and taint checks remain. Remaining changed lines add diagnostic assertion messages. A getter returning an incorrect name/note, wrong type/arity, accepting wrapped nonstrings, accepting tainted secrets, rejecting ordinary tainted strings, or leaving taint cleared/nonrestored still violates retained assertions. Source preservation does not prove those assertions now pass.

## Why the prerequisites are unrelated

`src/lua_api/globals/security/value_access.rs:136-148`: `issecretvalue_fallback` returns `value_is_secret_shallow`; `canaccessvalue_fallback` returns its negation. It does **not** consult caller taint or `can_access_secrets`. `secret_values.rs:115-125` recognizes actual VM secret wrappers first. Consequently an authentic secret yields identity=true/access=false even in an untainted context. The former untainted conjunction requires a predicate and its negation simultaneously; the former tainted negative access predicate adds no caller-sensitive guard for authentic secrets.

Registration is conditional on a missing global (`security/mod.rs:84-90`), so this report does not assert every possible runtime uses the fallback. The observed saved diagnostic exhibits its behavior. Getter `src/c_api/c_friend_list.rs:24-28` instead invokes VM `unwrap_secret` before strict string validation. Inspected pinned rilua `842e4d3/src/table_security.rs:401-428` checks authentic payload access with secure caller AND nonrevoked context; ordinary values pass through. No getter dependency on the access helper and no getter taint mutation.

## Saved first-GREEN diagnostic, not new execution

Epoch: `retail-green-20261012T122455266641Z` (no retry epoch evidence used).

- `exact.stdout` records **4PASS/1FAIL**, exit101 in `exact-result.json`; failure is the security fixture. `exact.stderr` reports generic `assertion failed!` at the Rust exec expectation, not a Lua assertion line. Do not attribute an exact Lua line from that traceback alone.
- `secret-helper-diagnostic-submission.json` records ordered assertions: nil caller taint → authentic secret identity → `C_FriendList.GetFriendInfo(name)` returning `info.name == 'Arthax'` → `canaccessvalue(name)`.
- `secret-helper-diagnostic.stdout` and stderr identify only the last assertion: `fixture helper canaccessvalue rejects authentic accessible secret`; result exit1. The sequential diagnostic therefore reached and passed the prior getter/name assertion before failing the helper. It does **not** prove supplied NOTE, result arity, wrapped-type rejection, tainted rejection, ordinary tainted acceptance or restoration. These remain target fixture obligations, not established by that diagnostic.
- Saved `source-applicability.json` identifies producer revision `be9c734a4` with empty diff/exit0. No new artifact/binary authentication or hashing performed. This is reuse of saved evidence, not fresh runtime authentication.

## Separate helper/native parity gap

The helper's secret-identity negation disagrees with caller-sensitive VM accessibility for the saved untainted authentic wrapper. Removing its predicates stops coupling getter acceptance to that independent helper gap; it does not repair or establish native parity for `canaccessvalue`. No native-client execution was inspected. Helper parity remains open separately. No producer/helper code changed, and no producer fix is claimed by `6ec0c1cb8`.

## Changed-test readability

Manual changed-line and affected-function audit: **no findings**. Named messages clarify failure boundaries; local `name`, `query`, `info`, `ok`, `message` expose their role. Two local helpers share the same concrete fixture and express result checking/tainted probe without unrelated orchestration. The Rust test has31 body lines (below200 test threshold); Lua nesting at most2; predicates have at most2 operators. Numeric901001 is visibly an intentional nonstring input, not an unexplained domain constant. No added suppression, TODO/FIXME/HACK/XXX, hidden I/O, conditional state accumulation, or excessive parameters. Existing neutral helper names were not introduced by this delta. `rust-code-analysis-cli` and `readability-audit` absent on PATH; no automated metric or formatter PASS claimed.

## Proof ledger and limits

`secret-fixture-delta-git-proof.json` stores exact read-only Git argv/output/exits: delta/stat/paths, empty producer/dependency delta, empty target/worktree diff, revisions and status. `secret-fixture-delta-proof.json` stores this narrow verdict and contract matrix. Initial Pyrun commands lacked repository cwd (exit1/128), and a later evaluation referenced an unavailable `repo` variable; neither produced evidence or changed repository state. Corrected commands explicitly set cwd; saved proof commands all exit0.

Cargo invocations0; runtime/build executions0; delegations0; repository edits0; binary hashes0. Only verification artifacts written outside repository. No target security GREEN, startup/check/fmt, cross-profile or native acceptance claimed. Original first-GREEN failure and diagnostic retained untouched.
