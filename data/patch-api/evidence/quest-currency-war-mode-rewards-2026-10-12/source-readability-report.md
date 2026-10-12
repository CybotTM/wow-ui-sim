# Bounded source/spec/readability verification

Date: 2026-10-12. Repository: `/home/osso/Projects/wow/wow-ui-sim`.

Scope: ONLY producer commits `bea55a2ae3c21482b752884dbaafb3243683c300`, `a295a7d726429d25041540f2f4dc68cf28e32827`, `067fdbb90743e0c16159888a3754fc71fcdfcee5`; supporting registration, state, dependency and existing integration source inspected read-only. Both requested skills read completely. No delegation, questions, source edits, cargo/build/test commands, or GREEN artifact authentication.

Reviewed HEAD: `4e19a8bad30ffe039a7604f3dc0cff45334e46ee`. The three requested commits are contiguous from parent `8a10302064ed91b8f0944eef475871902b311bf0`. `git diff 067fdbb90 -- <eight scoped Rust paths>` exits 0 with empty output: inspected working-tree source matches producer endpoint. Existing unrelated docs/untracked changes were not modified. Working-tree quest spec inspected as supporting requirements, not treated as committed producer evidence.

## Four artifact checks

- **[EXIST] PASS (source)** — all eight requested Rust files exist; counts/digests retained in adjacent JSON. Exact commit inventories comprise only these files: 107 added lines, zero removed lines, no vendor paths.
- **[SUBSTANTIVE] PASS (source)** — `war_mode.rs:26-37` reads owned `war_mode.desired`/`reward_bonus`; `c_quest_log.rs:19-33` reads `quest_war_mode_eligibility`; `item_spell/c_currency.rs:103-117` reads `currency_faction_grants`. Each returns exactly one pushed scalar by `Ok(1)`. No mocked getter, canned current-profile result, classification join, generic PvP inference, or seeded membership introduced.
- **[WIRED] PASS (source)** — actual call order and namespace retention established below; not inferred from an earlier agent report.
- **[ANTI-PATTERN] PASS (added lines)** — TODO/FIXME/HACK/XXX, clippy allows, dead-code allows: zero each across 107 added lines. Manual inspection found no empty catch/stub/commented-out implementation or vendor Lua override in the additions.

## Contract coverage matrix — source evidence only

| Contract | Evidence | Result / limit |
|---|---|---|
| Desired/reward input independence | `c_pvp/war_mode.rs:26-37`; `state/sim_state.rs:459-460`; `methods.rs:86-91` resolves app-data SimState | PASS source: live reads, no generic PvP dependency; runtime/environment-isolation execution not performed |
| Default policy / separate getter | unchanged `war_mode.rs:39-48` false/10; `state.rs:383`; `small_namespaces.rs:323-326` still constant 10 | PASS source; simulator policy, not native parity |
| Quest eligibility | `c_quest_log.rs:26-32`; `sim_state.rs:523`; `state.rs:442` empty HashSet | PASS source: supplied membership; absent/unrepresented numeric identity returns false |
| Immediate faction relation | `c_currency.rs:111-116`; `sim_state.rs:578`; `state.rs:495` empty map | PASS source: supplied map entry, exactly one number or nil; no inferred currency/faction join |
| Wrong types / numeric identity | `helpers.rs:7-17`, called by both relation queries; `from_stack.rs:42-49` missing argument gives nil | PASS source: only Val::Num accepted; strings/bools/tables/functions/nil error. Fractional, NaN/infinite, outside inclusive i32 range give None before cast, hence false/nil rather than alias. Represented i32 endpoints retained |
| Secret-first authentication | `c_quest_log.rs:20-26`, `c_currency.rs:104-110`; `Cargo.toml:123-124,153-154`; pinned rilua `table_security.rs:400-427` | PASS source: both current bundles activate unwrap_secret before validation/lookup. Dependency guard checks secure context and revoked access before returning payload. Takes immutable state; no wrapper mutation/declassification or taint setter in this path. Dynamic wrapper/caller-taint preservation not executed |
| Earlier paths / current publication | `c_pvp.rs:34-35`; `mod.rs:153-154,348-349`; `c_currency.rs:50-63`; `small_namespaces.rs:129-130,328-332`; `quest_surface/register.rs:51-52` | PASS source: new providers only client-retail/client-ptr; old reward/quest providers remain for other profiles, including historical profile-retail without client-retail; separate Default getter unaffected. No earlier-profile runtime checks performed |
| Existing currency flags | `c_currency.rs:74-101` unchanged by the producers | PASS source preservation only; earlier runtime proof neither rerun nor promoted |
| Vendor preservation | exact per-commit name inventories and all added lines; existing `tests/quest_currency_war_mode.rs:22-36,70-80` loads actual cached Blizzard closure and invokes public caller | PASS producer source: no vendor modifications/overrides. Existing consumer tests inspected, NOT executed; no startup, tooltip-output, catalog or native claim |

## Registration-route falsification

1. `env_init/mod.rs:67` calls `globals::register_globals`. `globals/register.rs:41-43` orders bootstrap → frame globals → tail globals.
2. Frame foundation calls `utility_system_spell::register_all` (`globals/register.rs:118`). It invokes utility C API bootstrap (`utility_system_spell/mod.rs:646`), which installs the quest provider (`c_api/mod.rs:348-349`) and follows auxiliary/token registration to PvP (`mod.rs:365,380-387`; `c_pvp.rs:34-35`). The War Mode registration installs both getters (`war_mode.rs:15-23`).
3. Tail core surfaces call `missing_surface::register_all` BEFORE `quest_surface::register_all` (`globals/register.rs:232-233`). Missing surface registers item/trait then world/social/group namespaces (`missing_surface.rs:80-83`). Item registration follows `:175-183` → `missing_surface/item_spell/mod.rs:10-11` → `c_api/item_spell/mod.rs:29-33` → `c_currency.rs:45-63`, installing faction relation. Later group registration calls small namespaces (`missing_surface.rs:302`; `small_namespaces.rs:24-29`), but its reward provider entry is excluded for current Retail/PTR (`:129-130`); Default remains registered.
4. Quest surface retains the existing table (`quest_surface/mod.rs:26-35`) and writes only listed members (`quest_surface/register.rs:176-181`). Its false eligibility entry is excluded on current Retail/PTR (`:51-52`). Subsequent `register_quest_log_overrides` is indeed AFTER quest surface (`globals/register.rs:236`), but its `missing_surface/quest_log.rs:48-76` member list does not contain the new eligibility getter. It cannot replace that member through its ordinary registration loop.
5. Namespace helpers reuse existing tables (`c_api/helpers.rs:49-65`, `missing_surface.rs:551-570`); `table_set_rust_fn_static` installs individual keys via raw_set (`lua_bridge/table_builder.rs:47-79`), not a namespace replacement. Post-cleanup restoration re-invokes utility bootstrap (`environment_cleanup_restore.rs:20`), reaching quest/PvP again; that is a source route, not executed restoration/startup proof. No matching new getter names found in env_init bootstrap sources.

Thus the hypothesis that a later canned quest/PvP provider necessarily overwrites the new getters is falsified by actual order, key-level writes and current-profile exclusions. This does not prove every dynamic loaded session's final namespace state.

## Readability finding

**[DUPLICATE] `src/c_api/helpers.rs:7` — `validate_supplied_relation_id()`**

Problem: commit `bea55a2ae` introduces a second copy of the existing `item_spell/c_currency.rs:74-83::validate_war_mode_currency_id` number/type/range/integrality algorithm. The only material variation is the error-message argument versus embedded literal. This is newly introduced cross-file duplication, not an unrelated preexisting finding.

Suggested fix (not applied): retain one shared validator; have the existing currency-bonus query pass its existing error message to it. Preserve existing authentication order and output behavior. This is a readability/maintenance finding, not evidence of wrong current outputs.

No other new readability violations found in manually inspected changed lines. Automated scoped metrics unavailable: neither rust-code-analysis-cli nor readability-audit is on PATH; documented `/home/osso/.cargo/bin/rust-code-analysis-cli` absent. Manual scope/brace counts corroborate short functions; maximum audited body length 27 lines, maximum brace depth 2 including function body. Approximation only, not compiler-derived cognitive/cyclomatic metrics. No broad repo readability issues reported.

**OVERALL: source/spec/wiring PASS; readability FAIL (one duplication).** Runtime/compile/format/startup/loaded consumer/native/final handoff: NOT VERIFIED here. Async GREEN work remains outside this packet. Saved evidence is restricted to source inspection and bounded source-derived counts.
