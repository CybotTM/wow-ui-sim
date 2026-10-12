# PTR current913 profile-gate runtime — PASS

Epoch `ptr-profile-gate-compile-20261012T111318504844Z`. Source `91324b10380f9be3e8d196491e9a2d20bc597fa1`; compile/docs HEAD `298caf47a53eac18960c1cef6716489e54a351cc`; final-check/docs HEAD `a60bf28c0476dd8b7d0bb59c009411c53b6c328e`. Original partial report/proof preserved; only pending runtime/normal sections superseded.

| Scope | Actual proof | Result |
|---|---|---|
| Integration authentication | Receipt reused; 467628400 bytes; SHA256 dc342f072d3c65ed7b31ca215a55ec9bc90c63520ee735ff984284906f7a3921; expected/actual match | PASS; no rehash |
| Fresh normal authentication | 414749712 bytes; SHA256 5f6c43c9b93eecdeb309d8f9d4e7c1a6e6c6002a571eaf742903b23bd87a25bc; expected/actual size and hash match; receipt immediately persisted | PASS; exactly one hash |
| Quest | 13 named passes; zero failures/ignored; 9945 filtered, not executed; 5.32s; exit0 | PASS 13/13 |
| Normal publication | Four raw getters, each function/one-result/unseeded false/10/false/nil; separate Default10; both relation string inputs rejected; 15 silent assertions | PASS |
| Startup | 283 Blizzard addons; stdout [] and CLEAN zero unique/occurrence errors; no exec-lua failure marker | PASS bounded loaded probe |

## Exact cases

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
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 9945 filtered out; finished in 5.32s
```

Real public QuestUtils loaded six times. Passed cases reach ordinary baseline, each independent desired/eligibility/world-quest/currency/faction gate, percentages 25/40, duplicate bonus counts 1,1,2,3,2,1, live state/default independence, faction update/removal/identity, numeric validation, secret-first denial and untainted identity/caller preservation. Conditions use reused unchanged test-source evidence plus current named successes, not per-branch runtime logging. Nine flag tests are not newly rerun.

## Normal conditions

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

All runtime/normal stdout and stderr read fully and embedded verbatim in `profile-gate-ptr-runtime-proof.json`. Assertions are silent: submission, finished exit0, actual clean collector and absence of hidden exec-lua failures establish success through the previously inspected unchanged error-collection path; no invented per-assertion output. Third-party addons/SavedVariables/sound disabled; font system casc=false; existing EditMode cache loaded; GC restarted. Not pristine host, GUI, all-addon or native parity proof. Default getter raw publication/arity is not asserted.

```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

Compile and source/readability evidence reused unchanged. Six inherited manifest deprecations remain. No build/test/runtime commands, service polling/waits/delegation/questions/repository edits; no older seal read. Parent full handoff/native and frozen full-suite3FAIL remain OPEN.
