# Independent legacy-aura retry receipt admission

**PASS — 16/16 exact profile-case executions, 0 failures, all exits 0. Eight distinct tests × two profiles, not sixteen unique tests.**

Epoch `20261011T004047Z`: October 11, 2026, 00:40:47 UTC. Followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as independent verifier. Saved-receipt admission only: no tests, replay, builds, live operations/resource inspection, delegation, Bash, rg, fd or repository edits. Outputs: this report and compact `independent-evidence.json` in this epoch. Original receipts, including outcome admission `PENDING`, remain unchanged.

## Actual selection, stdout, stderr and exits

Read all 16 complete stdout and 16 complete stderr files. Independently checked each execution against profile `executions.json`, `results.json`, and `outcome.json.profiles`; all agree. Each argv is `/usr/bin/timeout 90 <accepted unit2 profile integration-test> aura_api::<selector> --exact --nocapture --test-threads=1`. Each stdout contains exactly `running 1 test`, exactly the requested named `... ok` case, and exactly one successful summary: 1 passed, 0 failed, 0 ignored, 0 measured. Filtered-out count: Wrath 8034, Mists 8198. All 16 recorded command exits are 0; no timeout or zero-selection pass admitted.

| Profile | Exact commands/cases | Passed | Failed | Recorded execution window UTC |
|---|---:|---:|---:|---|
| Wrath | 8 | 8 | 0 | 00:41:24.927770–00:41:27.300355 |
| Mists | 8 | 8 | 0 | 00:41:28.215940–00:41:30.541802 |

Each stderr has 11 startup timing lines, ending `WowLuaEnv::new complete`; 176 timing lines total. No Lua-error, panic or failure diagnostics in these saved stderr streams. Stderr is **not empty**. This is not a full Blizzard/startup or audio gate. Compact evidence records all selectors, parsed summaries, exits, timestamps, relative log paths and saved-log SHA256s.

## Actual assertion coverage — intentionally weak

Reuse the source assertion inspection in the preserved initial independent report, bound there to `tests/aura_api.rs` SHA256 `21b3e7bfd5cb3fbe6798cf3280ce72f00a73382b5dece457d7eb6b3cd96bf48a`. That digest also matches this epoch's two source maps and accepted baseline. No live source reinspection or rehash needed.

| Selector suffix (prefix `aura_api::`) | Actual assertion in `tests/aura_api.rs` | Wrath / Mists |
|---|---|---|
| test_unit_buff_returns_data_for_player | lines 14–18: first player buff name nonempty | 1 PASS / 1 PASS |
| test_unit_buff_returns_all_fields | lines 21–35: only name, tenth-slot spellId and fifth-slot duration non-nil | 1 PASS / 1 PASS |
| test_unit_buff_past_end_returns_nil | lines 38–42: player index 100 first return nil | 1 PASS / 1 PASS |
| test_unit_buff_non_player_returns_nil | lines 45–49: target index 1 first return nil in default fixture | 1 PASS / 1 PASS |
| test_unit_buff_count_between_4_and_8 | lines 52–67: contiguous first-return enumeration count 4–8 | 1 PASS / 1 PASS |
| test_unit_aura_helpful_returns_data | lines 74–78: HELPFUL first player name nonempty | 1 PASS / 1 PASS |
| test_unit_aura_harmful_returns_nil | lines 81–87: HARMFUL first player return nil | 1 PASS / 1 PASS |
| test_unit_debuff_returns_nil | lines 94–98: first player debuff return nil | 1 PASS / 1 PASS |

These construct default `WowLuaEnv`; they do not seed a concrete custom aura/caster fixture. The misleading `all_fields` name does not strengthen its assertions: icon, count, dispelName, expirationTime, source/caster, isStealable and nspp are destructured but unchecked. No exact tuple length/value/order, caster mapping/order, populated harmful-aura fixture, native parity, whole-profile or whole-handoff completion follows.

## Submission, controller and bounded resource admission

Parent `submission.json` records submission exit 0, empty stdout, stderr `Running as unit: wow-legacy-aura-retry-20261011t004047z.service`. Requested agents.slice, CPUQuota=1200%, MemoryMax=16G, repository working directory and file-backed stdin/stdout/stderr paths match worker/controller context. Submission success is not test success or a terminal service exit receipt.

Parent `controller.stdout` contains exactly one `completed` JSON record, null value, console exactly this epoch path. Its **executed** code equals `request.jsonl.code` and `worker.py` byte-for-byte. Parent `controller.stderr` is empty. No saved terminal service exit code exists; controller completion and 16 test exits are distinct evidence.

Worker/request/controller implement **up to five admission observations per profile**, not five observed attempts or five test retries. Predicate: MemAvailable ≥6 GiB AND load1 <24 AND no Cargo process whose cwd is the repository or sibling `full-suite-checkout`. Rejected observations sleep `min(10*2**attempt_zero_based,60)+uniform(0,5)` only before another observation: configured ranges 10–15, 20–25, 40–45, 60–65 seconds. No actual sleep timestamps/jitter receipts were saved; verify policy, not exact elapsed backoff.

| Profile / attempt | Available GiB | Load1 | Relevant Cargo PID | Admitted |
|---|---:|---:|---|---|
| Wrath 1 | 29.88833999633789 | 13.43 | 438094 | false |
| Wrath 2 | 29.318912506103516 | 14.13 | 438094 | false |
| Wrath 3 | 34.22566223144531 | 13.27 | none | true |
| Mists 1 | 32.61342239379883 | 13.89 | none | true |

All saved observations independently satisfy the predicate. Wrath's first two rejections are solely recorded Cargo presence, not load/memory; accepted third observation has no relevant Cargo. Mists admits immediately. No exhausted five-attempt branch occurred.

## Saved actual cgroup limits and membership

Before/after membership exactly `/user.slice/user-1000.slice/user@1000.service/agents.slice/wow-legacy-aura-retry-20261011t004047z.service`, matching submission. Both actual `cpu.max` = `1200000 100000`: 12 CPU equivalents quota. Both `memory.max` = `17179869184`: 16 GiB. Thus actual recorded limits match requested properties, not merely command intent.

CPU usage 170223→6940341 us (+6770118); user 130187→5081184 (+4950997); system 40035→1859157 (+1819122). Periods 2→75; throttled periods/usec stay 0. Nice, forced-idle, burst counters stay 0. Memory peak 35778560→1349713920 bytes (+1313935360). These are controller cgroup endpoint counters, not exclusive test/compile costs or continuous per-child membership proof. No saved OOM/event counters; no OOM-free claim.

## Accepted unit2 identity composition — no repeated live hashing

Compose accepted `../../unit2-green-current/independent-report.md` and `../../unit2-green-current/independent-evidence.json`, reference epoch `20261010T235452Z`, with `../../unit2-retention-current/report.md`. Prior acceptance established sealed binary hashes/sizes and 3846 proof-scope inputs matching cc4f4c782, despite documentation-only successor 24c727075a69a8a4bbf813156248e0547f3150f9. Retention independently preserves that bounded acceptance; its two pre-existing documentation-link defects are not new functional/identity failures.

Parsed both full saved source maps: before 3846, after 3846, each exactly equals accepted `baseline-source.json`; 0 missing, added or changed entries. Source/build/test scope remains src, tests, Cargo.toml, Cargo.lock, build.rs, .cargo/config.toml. No live binary or 3846-file rehash performed.

| Profile | Accepted SHA256 | Bytes | Recorded features |
|---|---|---:|---|
| Wrath | f458b5f445a816cbf61b4a76aade638c18c3660ae6c3a450968a597da0fb1a9f | 435601288 | casc, client-wrath, gui, rodio, sound |
| Mists | 0cd805af673f8bbafb9d8af38a3eb9454fd7cb036aa325277e2eeefd2b6ed33c | 438312496 | casc, client-mists, gui, prefork-full-ui, rodio, sound |

Retry metadata exactly equals reference profile artifact metadata and accepted independent hashes/sizes; exactly one matching client feature each. Both build profiles: test=true, debug_assertions=true, overflow_checks=true, debuginfo=line-tables-only, opt_level=1. Mists prefork-full-ui is retained metadata, not proof these default-fixture controls exercise full-UI prefork.

Completed executed worker code performs each pre-execution binary SHA256 guard against reference metadata, saves metadata only after passing, then records `artifact_unchanged: true` from post-execution SHA256 equality and `source_equal: true` against baseline for each profile. These recorded guards compose with prior accepted binary/source proof; they are not new independent binary recomputations, continuous immutability or hermetic provenance. Execution argv points exactly to those sealed reference binaries, not unspecified current live builds.

## Preserved initial blocked epoch

`../../legacy-aura-controls-current/20261011T003802Z/independent-report.md` remains untouched. Initial 00:38:02 UTC epoch has **0 cases executed**, both RESOURCE_BLOCKED because load1=26.71 fails <24; no test stdout/stderr/exits or epoch artifact checks. Its blocked-receipt consistency PASS is not functional PASS. This separate retry's 16 actual executions do not retroactively convert initial zero executions into passes, or erase that independent report.

**Conclusion: admit only 8 distinct weak legacy controls × Wrath/Mists = 16 passing profile cases, with consistent saved selection, exit, identity-composition, controller, resource-admission and cgroup receipts. Broader tuple/caster/native/full-profile/handoff claims remain unproved.**
