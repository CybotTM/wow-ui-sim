# Bounded source/spec/readability gate

Date: 2026-10-11
Revision: `b5802042013b84a8311de53b374a502d450518ab`
Scope: independent artifact verification of the two new C API producers, bootstrap wiring, literal approved contracts, exact publication-gap removals and Rust readability. No builds, compiler checks, runtime invocations, source edits, operations or delegation performed. This report is the only written artifact.

**SOURCE: PASS. RUNTIME: PENDING.**

## Artifact checks

- [EXIST] PASS — `src/c_api/c_achievement_info_relations.rs` exists, 48 lines; `src/c_api/c_garrison_damage_classes.rs` exists, 46 lines. Both and `src/c_api/mod.rs` match the requested commit (`git diff --quiet b58020420 -- <file>`: exit 0 per file).
- [SUBSTANTIVE] PASS — achievement `read_achievement_id` (line 18) validates numeric signed-i32 representation; `query_superceding_achievements` (line 33) clones current supplied state and serializes a new table. Garrison `DAMAGE_CLASSES` (line 10) supplies seven explicit pure masks; `damage_class_values` (line 31) allocates outer/row tables. Neither producer is a placeholder.
- [WIRED] PASS — module declarations at `src/c_api/mod.rs:25,98`; registrar calls at `:342-343`. Utility bootstrap is called by `src/lua_api/globals/utility_system_spell/mod.rs:646` and cleanup restoration at `src/lua_api/workarounds/temporary/environment_cleanup_restore.rs:20`. `src/lua_api/globals/register.rs:118` invokes utility registration before the tail registrar at `:232`. Later achievement and Garrison registrars preserve existing namespace tables and write only different keys (`missing_surface/achievement_info.rs:43-72`, `missing_surface/garrison.rs:25-48`); Garrison tree registration also writes distinct keys (`src/c_api/c_garrison_trees.rs:29-50`). Retirement list at `src/c_api/patch_retired_members.rs:133` removes only GetMissionInfo/GetTalentTreeInfoForID. Scoped inspection of simulator Rust/Lua found no subsequent assignment to either new query or replacement of its namespace. This is source wiring evidence, not loaded-runtime proof.
- [ANTI-PATTERN] PASS — each new producer has zero TODO/FIXME/HACK/XXX markers, warning suppressions, commented-out function blocks or block comments. No empty implementation or alternate fallback producer introduced.

## Literal contract coverage

| Requirement | Source evidence | Gate |
|---|---|---|
| Supplied signed-i32 map; sequence retained including duplicates | `c_achievement_info_relations.rs:35-46`; existing `sim_state.rs:580` is HashMap<i32, Vec<i32>>, `state.rs:497` initializes it empty | PASS/source |
| Fresh result; absent/removed/unrepresentable numeric ID returns empty; successive reads observe current state | Clone-on-query plus new table per call, `:28-46`; no derived/default relations or result cache | PASS/source |
| Nonrepresentable numbers cannot alias a stored key | Finite, integral, inclusive i32 range checks precede lazy cast, `:28-30` | PASS/source |
| Nonnumeric input errors; AllowedWhenUntainted without modifying caller/wrapper | `:19-27`; pinned rilua `842e4d3/src/table_security.rs:401-428` authenticates secret payload through secure/nonrevoked caller check using immutable state. Default Retail and PTR include the gated feature (`Cargo.toml:123-124,153-154`) | PASS/source; runtime pending |
| Exactly one result table | Both producers push their result and return Ok(1), achievement `:43-48`, Garrison `:44-46` | PASS/source |
| Seven pure-mask enUS labels, not ordinals/composites/spell fallback | Garrison `:10-18`: 1 Physical, 2 Holy, 4 Fire, 8 Nature, 16 Frost, 32 Shadow, 64 Arcane. No spell lookup or composite derivation | PASS/source |
| Independent fresh outer/row tables | Garrison `:32-42`; allocation helpers `src/lua_api/methods.rs:532-554` allocate new tables. Immutable string sharing does not share row mutation | PASS/source |
| Legacy reward/tree implementation unchanged | Complete commit changes only seven paths: two new producers, mod wiring, two specs and two gap lists; no legacy reward/tree file changed. Current legacy achievement/tree files also match b58020420, exit 0 | PASS/source; preservation execution pending |
| Only exact current gap rows removed | 8.1 list: 58→57, sole removal `wt-global-api-C_AchievementInfo.GetSupercedingAchievements-13`; 9.0.1 list: 263→262, sole removal `wt-global-api-C_Garrison.GetAutoCombatDamageClassValues-286`; no additions | PASS/source |

## Rust readability

Manual line-by-line audit of both complete new files and four added module/bootstrap lines: no reportable violations. All new function bodies below 30 lines, nesting below the skill threshold, no cognitive/cyclomatic threshold concern, no suppressions, hidden external I/O, parameter overload or duplicated multi-step functions. Explicit constant data and named representation checks make masks and numeric validation inspectable. No compiler/linter/metric command run.

## Proof separation

Caller-accepted pre-producer RED: epoch173738 at `a0833b2dd`, eight achievement failures and three unique Garrison failures. This gate inspected the retained descriptions in both specs; it did not independently replay or certify the RED logs. Historical failures remain history.

Guarded GREEN unit `wow-addon-compatible-green-20261011t174814z` remains **PENDING for this report**; no runtime result is admitted here. No build, startup, preservation, authentic addon-consumer execution or native-parity PASS claimed. Native parity and adjacent hardening are outside this gate. Spec checkbox lists remain open pending runtime proof.
