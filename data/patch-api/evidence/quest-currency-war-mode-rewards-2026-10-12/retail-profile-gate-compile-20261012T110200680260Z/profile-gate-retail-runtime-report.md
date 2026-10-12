# Retail profile-gate runtime follow-up — PASS

Epoch: `retail-profile-gate-compile-20261012T110200680260Z`. Source `91324b10380f9be3e8d196491e9a2d20bc597fa1`; build/docs `298caf47a53eac18960c1cef6716489e54a351cc`. Original partial report/proof preserved; this report supersedes only their pending runtime/normal sections.

## Authentication

Integration receipt reused without hashing: expected/actual **467770528 bytes**, SHA256 `9bce78a7255ac0576c42cf56d71b53bf55fbd6982c075f11d2a96bc2728d683b`, match=true. Normal authenticated once and receipt persisted immediately before further inspection: expected/actual **407292576 bytes**, SHA256 `2b4b69226101d5bbbd036d8788a197da3a5dc8e82d038a3fdbd5929208712958`, size_match=true, sha256_match=true. Receipt: `profile-gate-independent-auth-normal.json`.

## Coverage matrix

| Scope | Actual evidence | Result |
|---|---|---|
| Quest integration | 13 named cases below; runtime exit 0; full stdout/stderr read | PASS 13/13 |
| Loaded normal raw queries | Four raw functions; four one-result checks; false/10/false/nil unseeded results; Default=10; two strict string-input rejection checks (15 assertions total) | PASS, finished exit 0 |
| Observed startup Lua errors | Actual normal stdout `[]`; final collector quotes below, not controller status | 0 unique / 0 occurrences |
| Source/readability/compile | Earlier epoch-local evidence reused unchanged; six manifest deprecation warnings retained | No new lint/build/source verification |
| Old flag9 / other cases / other profiles | No execution or new credit | EXCLUDED |

## Full quoted case counts and names

```text
running 13 tests
test quest_currency_war_mode::quest_currency_war_mode_actual_vendor_bonus_and_each_independent_gate ... ok
test quest_currency_war_mode::quest_currency_war_mode_actual_vendor_duplicate_first_instances_are_per_currency ... ok
test quest_currency_war_mode::quest_currency_war_mode_actual_vendor_inputs_are_environment_local ... ok
test quest_currency_war_mode::quest_currency_war_mode_actual_vendor_ordinary_reward_baseline ... ok
test quest_currency_war_mode::quest_currency_war_mode_desired_has_one_boolean_result ... ok
test quest_currency_war_mode::quest_currency_war_mode_eligibility_requires_numeric_identity ... ok
test quest_currency_war_mode::quest_currency_war_mode_faction_query_requires_numeric_identity ... ok
test quest_currency_war_mode::quest_currency_war_mode_relation_queries_authenticate_tainted_secrets_first ... ok
test quest_currency_war_mode::quest_currency_war_mode_supplied_desired_is_live_independent_and_environment_owned ... ok
test quest_currency_war_mode::quest_currency_war_mode_supplied_eligibility_is_not_world_quest_classification ... ok
test quest_currency_war_mode::quest_currency_war_mode_supplied_faction_projection_updates_removes_and_preserves_identity ... ok
test quest_currency_war_mode::quest_currency_war_mode_supplied_percentage_is_live_without_changing_default_query ... ok
test quest_currency_war_mode::quest_currency_war_mode_supplied_untainted_secrets_preserve_identity_and_caller ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 10767 filtered out; finished in 3.32s
```

All 13 include real public QuestUtils consumers, independent bonus gates, per-currency duplicate first-instance handling, environment isolation, boolean/result arity, live desired/percentage state with Default unchanged, numeric identity/type boundaries, eligibility separate from world-quest classification, faction update/removal/projection and tainted/untainted secret handling. Runtime stderr records the public consumer six times:

```text
Loaded public currency reward consumer: @Interface/AddOns/Blizzard_FrameXMLUtil/Mainline/QuestUtils.lua
```

The passed vendor gate case reaches desired=false; eligibility removed; world-quest=false; currency flags `(None,None)`, `(false,true)`, `(None,false)`; allowed `(true,false)`; faction grant present/removed; supplied reward percentage=40. Duplicate case reaches bonus counts **1,1,2,3,2,1** across repeated renders, first-instance flags and faction grant, while preserving three ordinary reward rows. This is consumer-test coverage, not nine separately rerun flag cases.

## Normal conditions actually reached

Finished normal invocation uses `--no-addons --no-saved-vars --exec-lua ... lua-errors`; authenticated sealed binary. Exact submitted assertion script:

```lua
local queries = {
    {C_PvP, 'IsWarModeDesired', false},
    {C_PvP, 'GetWarModeRewardBonus', 10},
    {C_QuestLog, 'QuestCanHaveWarModeBonus', false, 901001},
    {C_CurrencyInfo, 'GetFactionGrantedByCurrency', nil, 90001},
}
for _, row in ipairs(queries) do
    local f = rawget(row[1], row[2])
    assert(type(f) == 'function', row[2] .. ' raw publication')
    assert(select('#', f(row[4])) == 1, row[2] .. ' one result')
    assert(f(row[4]) == row[3], row[2] .. ' unseeded state')
end
assert(C_PvP.GetWarModeRewardBonusDefault() == 10)
for _, f in ipairs({C_QuestLog.QuestCanHaveWarModeBonus, C_CurrencyInfo.GetFactionGrantedByCurrency}) do
    local ok, message = pcall(f, '90001')
    assert(not ok and string.find(message, 'number', 1, true), 'strict numeric relation')
end
```

Evidence level: assertions are silent. Submission plus finished exit0 and actual empty error collection establishes the checks passed through the inspected `src/lua_errors.rs:47-80` execution/error-collection path; no fabricated per-assertion output. Full 3-byte stdout/14707-byte stderr read. Actual final stderr:

```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

Reached loading **291 Blizzard addons**, disabled third-party addons/SavedVariables/sound, font system `casc=false`, existing EditMode cache, bootstrap GC restart and final Lua collector. Clean applies only to this loaded headless probe/startup/update scope, not all addons, GUI, other profiles or native WoW parity.

## Reuse and limits

Unchanged source913/docs298 applicability and partial manual source/readability evidence reused. Six compile manifest warnings remain: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. No warning-free claim; no exhaustive repository cleanliness proof. No old root proof/auth changes or old binary reads. No flag9/filtered10767 execution credit. No service polling, waits, builds, runtime commands, delegation, repo edits, questions or operations; main owns ops.

Structured proof: `profile-gate-retail-runtime-proof.json` includes complete runtime and normal streams, outcomes, submission conditions and reused compile evidence.
