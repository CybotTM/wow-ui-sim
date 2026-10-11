# Achievement superceding relations

`C_AchievementInfo.GetSupercedingAchievements` projects explicitly supplied achievement-ID relations. User selected explicit map input with one empty table for an absent ID; this is simulator policy, not native catalog or unknown-ID parity. Current cached Retail/PTR generated declarations establish required numeric input, one non-nil numeric table return and `AllowedWhenUntainted`. API spelling is `Superceding`.

## What it must do

- [ ] Return exactly one fresh table for supplied relations, preserving the supplied ID sequence without deriving, sorting or deduplicating it.
- [ ] Return one fresh empty table when relation input is absent, including after removal. Seed no default relations.
- [ ] Observe successive relation updates and keep other keys independent; Lua result mutation must not change supplied state or another returned table.
- [ ] Require numeric input; authenticate secret arguments under the VM's untainted-caller rule without changing wrapper or caller taint.
- [ ] Keep existing achievement/reward APIs unchanged.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/state/sim_state.rs` — explicit `HashMap<i32, Vec<i32>>` relation input alongside existing achievement state.
- `src/lua_api/state.rs` — empty relation-map initialization, no native defaults.
- `src/c_api/c_achievement_info_relations.rs` — real namespace query, registered by the canonical C API utility bootstrap; legacy achievement registrar stays unchanged. Producer is implemented after runtime RED; GREEN remains pending.
- `tests/c_achievement_info_probes/superceding.rs` — absent/populated/update/removal/isolation/type/security behavior.

## Tests asserting this spec

`tests/c_achievement_info_probes/superceding.rs`; existing `c_achievement_info_probes` and reward update tests provide separate preservation evidence.

[Retained initial RED](../../data/patch-api/evidence/achievement-superceding-red-2026-10-11/README.md) at `37d17371b`:64 existing PASS/1 new FAIL/65 executed, exit101; exactly one non-table result failed the relation-table assertion. Exact result value was not recorded. No query producer change preceded that capture. Expanded attempt at `c9f3043bf` was RESOURCE_BLOCKED/zero executions. Later joint attempt165520 was COMPILE_FAILED/zero cases from duplicate harness macros, repaired by7635fef91. Epoch173738 at `a0833b2dd` compiles successfully and executes all eight expanded cases:0PASS/8FAIL, exit101, eleven endpoint hashes unchanged, before either query producer. Generic assertion messages do not establish each secret fixture's exact internal assertion boundary. Current producer and preservation/GREEN proof remain pending.

## Known gaps (current cycle)

- [ ] Implement query after expanded behavioral RED; obtain targeted GREEN, relevant format/check/startup and independent proof.
- Native catalogs, native ordering and absent-ID semantics remain unverified, not blockers for coherent addon-compatible supplied-state behavior under the [current acceptance authority](../wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md#acceptance-authority--2026-10-11). Historical/all-profile runtime coverage is not established by this policy.
- [ ] Numeric IDs use the existing signed-i32 model. Other numeric values cannot alias a stored key; no native coercion/range policy is claimed.

## Out of scope

No guessed relations, catalog import, graph traversal, derived previous/next/category relations, new setters/events, vendor edits or changes to reward/criteria semantics. Historical8.1 publication is name-only evidence; current generated declarations are not projected backward as historical native contracts.
