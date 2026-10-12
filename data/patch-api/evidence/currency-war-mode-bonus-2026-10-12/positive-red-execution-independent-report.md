# Expanded positive RED — retained runtime follow-up

**RED execution proven; feature acceptance FAIL. Exactly 9 tests executed, 0 PASS, 9 FAIL; runtime exit 101.** Retained stdout reports 0 ignored, 0 measured, 10758 filtered out, 1.34s. This is nine test functions, not nine successful nullable-pair checks.

## Scope and provenance

Read `/home/osso/AgentConfig/skills/verify/SKILL.md` and full raw `runtime.stdout` and `runtime.stderr` for epoch `positive-red-compile-20261012T075738427394Z`, plus submission/terminal records. Neither stream was truncated. Reused `positive-red-independent-auth.json`: authentication_passed true, one prior hash pass, digest/size matched. No binary read or hash performed. Authentication receipt and earlier pending report/proof remain untouched.

Retained `revision.json` identifies built revision `a1de57b4ea99f485bd77c430d9fea4301ed3764a`. Evidence applies to that retained executable, not subsequent source. Current test source was read only to locate assertions/control ordering; no fresh revision comparison was performed. No claim that current source or HEAD remains unchanged. Source producer may proceed after main observes this RED; later commits do not invalidate this historical RED or turn it into GREEN evidence.

## Submission versus execution

`runtime-submission.json` records successful asynchronous submission (exit 0) to `wow-currency-war-mode-positive-red-cases-20261012t080146591858z`, selecting `currency_war_mode_bonus::`, `--nocapture`, `--test-threads=1`, with `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`. Scope: bare-environment query cases, not Blizzard consumer/full suite/VNC.

`runtime-terminal.json` inspection command exit 0 is **not** test success. Its retained terminal properties are `ActiveState=failed`, `SubState=failed`, `Result=exit-code`, **`ExecMainStatus=101`**. Raw stdout independently reports all nine failures. Raw stderr contains nine test panics, each at `tests/currency_war_mode_bonus.rs:28:10`, after environment initialization; no compile failure or admission block is the final execution boundary.

## Concrete failure/control matrix

Names below have prefix `currency_war_mode_bonus::currency_war_mode_bonus_`. Assertion locations refer to the read test source; raw panic location is line 28 for every case.

| Case | Actual raw failure | First boundary / controls not reached |
|---|---|---|
| absent_records_return_two_nil_flags | `currency bonus query returns two values` | First absent query (90001) fails return-count assertion; nil-value assertion and subsequent absent queries not reached. |
| requires_number_without_string_coercion | `required nonnumeric currency input must be rejected` | First `rejected()` fails rejection assertion; type-context assertion, explicit nil/string/boolean/table/function inputs and final numeric absence control not reached. |
| unrepresented_numbers_have_no_flags | `currency bonus query returns two values` | First loop value 0 fails return-count assertion; remaining negative/fractional/NaN/infinite/extreme controls not reached. |
| untainted_secrets_preserve_wrapper_and_caller | `currency bonus query returns two values` | First wrapped ID absence check fails; second wrapped ID and wrapper/value/caller-preservation assertions not reached. |
| tainted_secrets_denied_before_validation | `secret denied` | First wrapped numeric ID fails combined `not ok and type(message) == 'string'` assertion. Output does not distinguish which conjunct failed. Authentication-before-validation message check, invalid wrapped type, no-declassification, preserved taint and restored-caller controls not reached. |
| supplied_independent_nullable_combinations | `supplied query returns exactly two values` | Nine records seeded; first `(nil,nil)` query fails return count. Neither flags for this query nor other eight pairs/absent ID 90010 proven. |
| owned_updates_removal_and_currency_isolation | `supplied applies flag preserved` | First query stored into two Lua variables passes helper argument-count assertion (two variables), then applies comparison fails. This does not prove getter return count. Once comparison, second currency, update/removal/snapshot/isolation controls not reached. |
| positive_identity_does_not_alias_numeric_edges | `supplied query returns exactly two values` | First positive ID 90001 fails count; i32 limits, numeric non-aliasing edge loop, string rejection and repeat positive control not reached. |
| supplied_secret_identity_and_invalid_type | `supplied query returns exactly two values` | First supplied wrapped ID fails count; wrapped fraction/string rejection, wrapper identity and caller preservation not reached. |

Execution proves public query assertions fail at these boundaries. It does not independently establish the getter implementation/root cause, actual return values/count beyond assertion failure, denial mechanism, all nullable combinations, or later controls. QuestUtils/tooltip behavior remains untested.

## Earlier resource block preserved

Earlier snapshot remains historically valid: admission at `2026-10-12T08:00:13.018676+00:00` denied with load1 34.33 against `<24`, available 26.84356689453125 GiB; `runtime-blocked-20261012T080013001761Z.json` records RESOURCE_BLOCKED, submitted false, executed_cases 0. Later successful submission and terminal runtime evidence resolve execution PENDING without rewriting that earlier boundary. Prior compile PASS_WITH_WARNINGS remains prior evidence, not reverified here.

## Follow-up actions

Only local retained-file/test-source reads, directory discovery, and these two requested report/proof writes. Zero commands, tests, builds, hashes, delegation, network, operations or polling. No sealed or old binary opened. No source edits. No first-auth or prior pending snapshot overwritten.
