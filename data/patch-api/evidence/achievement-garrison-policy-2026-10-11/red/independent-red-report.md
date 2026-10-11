# Independent saved-RED authentication

Verified: 2026-10-11. Scope: saved artifacts ONLY in `20261011T173738Z`, against Git revision `a0833b2dd7c5dd8a3e7f69d1abd10263ff83ac42` in `/home/osso/Projects/wow/wow-ui-sim`.

Followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as the independent verifier. No build, executable execution, test rerun, source edit, operations, or delegation performed. Executable hash not duplicated. Report is the only intentional write.

## Verdict

**Authenticated bounded RED evidence:** saved compilation exit 0; achievement **0 PASS / 8 FAIL**; Garrison **0 PASS / 3 FAIL**, both outer executions exit 101. Nested Garrison consumer **0 PASS / 1 FAIL** is the child of one outer case, not a fourth independent Garrison failure.

This is not GREEN verification or proof that every named behavior was reached. Generic assertions bound several failures only to their enclosing Lua chunks. In particular, neither secret case proves its exact security preconditions passed.

## Cargo/artifact/source authentication

- `compile-submission.json` records `/usr/bin/cargo test --offline --locked --jobs 12 --test integration --no-run --message-format=json`, revision above. `compile-result.json`: exit 0. `compile.stderr`: `Finished test profile [optimized + debuginfo] ... in 56.33s`, plus six patched iced manifest deprecation warnings; not warning-free.
- Parsed all 741 JSON records in `compile.stdout`: 665 compiler-artifact records, 75 build-script-executed records, one build-finished record (`success: true`). No compiler-message records. Integration compiler-artifact identifies package `wow-ui-sim` 0.1.0, manifest in this repository, test/bin target `integration`, `tests/integration.rs`, edition 2024, `fresh: false`, executable `target/debug/deps/integration-9f07e4a55f2bbbf7`.
- Artifact profile matches `artifact.json`: opt-level 1, line-tables-only debuginfo, debug assertions and overflow checks enabled, test true. All 21 feature entries match: aura-containers, aura-instance-enumeration, aura-xml-widgets, base-spell-relationships, casc, client-retail, default, forbidden-aspects, gui, native-duration-formatting, numeric-rule-formatters, on-update-modes, player-cast-durations, prefork-full-ui, profile-retail, retail-12-0-0, retail-12-0-5, retail-12-0-7, retail-12-1-0, rodio, sound.
- Sealed `integration-test` exists and its observed size is 467,504,192 bytes, matching `artifact.json`. Recorded SHA-256: `a53a7d99ad7fe423278f384095c87835669ca306efe3583a40a853684c0a50c8`. No independent executable hash was recomputed under the no-duplicate instruction; byte identity relies on the supplied prior artifact authentication and recorded seal, not a new digest check.
- Independently hashed `git show <revision>:<path>` text for all **11** recorded input paths. Every revision digest equals its saved before digest, and every before digest equals its saved after digest: Cargo.lock, Cargo.toml, build.rs, src/c_api/mod.rs, src/c_api/registration.rs, src/lua_api/globals/missing_surface/achievement_info.rs, src/lua_api/state.rs, src/lua_api/state/sim_state.rs, tests/c_achievement_info_probes.rs, tests/c_achievement_info_probes/superceding.rs, tests/garrison_damage_classes.rs. These are selected-input assurances, not whole-tree/dependency/cache immutability proof. Current checkout cleanliness was neither assumed nor required.
- Revision `tests/integration.rs` invokes `wow_ui_sim_test_harness::integration_tests!()`. Test module `tests/c_achievement_info_probes.rs:11-12` includes `superceding`. Revision-scoped `git grep` finds neither queried endpoint name in `src`; no explicit literal producer registration there. This does not prove an absent Lua function: dynamic/generated namespace lookup may expose a function. Achievement modeled registration at `achievement_info.rs:43-70` registers other endpoints, not this relation query. New concurrent unregistered implementation modules are outside this saved revision proof.

## Counts / execution provenance

`execution-results.json` uses the sealed executable, filters `c_achievement_info_probes::superceding::` and `garrison_damage_classes::`, `--nocapture --test-threads=1`, each under `/usr/bin/timeout 90`.

Exact saved terminal results:

- Achievement: `test result: FAILED. 0 passed; 8 failed; 0 ignored; 0 measured; 10725 filtered out; finished in 0.65s`.
- Garrison outer: `test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 10730 filtered out; finished in 1.26s`.
- Garrison child: `test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10732 filtered out; finished in 0.99s`.

All three imply the same 10,733-case executable inventory. Child failure propagates through `tests/common/timeout_reexec.rs:184` with `exit code 101`; not a timeout (no exit 124).

## Achievement failure boundaries

References below are to `tests/c_achievement_info_probes/superceding.rs` at the authenticated revision. Saved stderr gives Rust unwrap locations, not precise Lua statement traceback.

| Case | Saved Rust failure line | Authenticated boundary and limit |
|---|---:|---|
| absent relation | 27 | Explicit `expected a relation table` at Lua source line 23. Prior count assertion at 22 passed: one returned value, not a table. Does **not** identify its exact type or prove nil. Empty-table contents never tested. |
| distinct sequences | 47 | Generic assertion in lines 36-44. Includes one-result checks and sequence/table checks; exact failing assertion unknown. Fixture map assignments at 8-9 precede this chunk; no observed projection success. |
| updates/removal | 68 | Generic assertion in lines 61-65, after initial query chunk at 53 succeeded and Rust map update at 58. Initial result contents unproven. Removal at 69 and its following assertions were not reached. |
| isolated tables | 98 | Generic assertion in lines 87-95. Exact failing assertion unknown; cannot claim mutation/isolation checks ran or passed, nor subsequent Rust state assertions. |
| requires number | 119 | Generic assertion in lines 112-116. At least one invalid-argument `pcall` returned truthy; exact input not identified by log. Does not prove all invalid inputs accepted. |
| unrepresented numbers | 137 | Generic assertion at 133 or 134. Both query calls at 131-132 completed before assertions; table/emptiness conjunction failure does not identify nil or aliasing. |
| untainted secret | 176 | Generic assertion anywhere in lines 166-173. Rust secret-helper registration/rooting completed before Lua execution, but `issecure()` at 166 and `issecretvalue(secret)` at 169 are themselves generic assertions. Cannot prove either passed, query reached, secret accepted, result nil, or taint preservation. |
| tainted secret | 201 | Generic assertion anywhere in lines 188-198. Cannot prove addon stack taint precondition at 188 passed, ordinary query fixture assertion at 190 passed, secret `pcall` at 191 reached, or any preservation assertion passed. No invented nil claim. |

## Garrison query versus fixture/consumer boundaries

References: authenticated `tests/garrison_damage_classes.rs`; saved `garrison.stderr` read through its final line.

- **Catalog case, Rust line 38:** exact message `damage-class query must return a table`. Its earlier single-return assertion at lines 13-14 passed. Thus a callable query completed and returned a non-table on the checked call; exact value is not established by this assertion. Rows, seven school masks, and enUS labels were not reached.
- **Isolation case, Rust line 103:** generic `assertion failed!`. Calls at 46-47 precede assertions, but the saved error cannot locate which assertion in 48-100 failed. Query-shape, fixture expectations, freshness and mutation isolation are not separately authenticated by this log.
- **Authentic cached consumer, Rust line 180:** exact error `bad argument #1 to 'ipairs' (table expected, got nil) at ...Blizzard_AdventuresCombatLog.lua:90`. Independently read actual `/home/osso/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_GarrisonUI/Mainline/Blizzard_AdventuresCombatLog.lua:80-95`: OnLoad sets message-frame properties, initializes `self.damageTypeKeys`, assigns `local damageClassStrings = C_Garrison.GetAutoCombatDamageClassValues()` at 88, then passes that local directly to `ipairs` at 90. **This consumer observation supports nil at the ipairs boundary**, unlike the generic achievement assertions.
- Test fixture `log:OnLoad()` is at line 150. Its mixin-table assertion at 115-116 and collaborator construction necessarily precede this terminal error. The error is the genuine consumer/query result boundary, not the later Fire/Holy fixture assertions at 151-152. Spell collaborator replacement at 155, AddCombatEvent calls at 169-170, and formatted-message assertions at 171-177 were not reached. Fire-versus-ordinal rendering remains unproven by RED.
- Saved loading also logs unrelated missing APIs/fields and a frame-creation error at this same cached OnLoad line 90. Those background load errors do not replace the test's terminal OnLoad/ipairs failure. Cached source currently matches the logged line boundary; the saved bundle has no cached-source digest establishing historical byte identity.

## Evidence limits

This report authenticates saved metadata, revision-selected inputs, execution counts, and the failure boundaries supported by saved messages plus revision/cached source. It does not prove native WoW catalog order/labels, every security precondition, whole-tree compile provenance, current implementation status, or future GREEN. No permission to execute or repair was inferred.
