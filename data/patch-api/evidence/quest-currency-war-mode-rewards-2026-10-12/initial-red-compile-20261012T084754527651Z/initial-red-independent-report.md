# Independent retained initial RED gate — 2026-10-12

**AUTHENTIC INITIAL RED CONFIRMED; feature acceptance remains OPEN.** Epoch `initial-red-compile-20261012T084754527651Z`, test revision `97d263565546023bb2d0f6c2cab0a07ef993ce77`. Compile exit **0**; runtime exit **101**; **5 selected/executed, 1 PASS, 4 FAIL**, 0 ignored/measured, 10,767 filtered out. No new tests, builds, runtime executions, operational actions, network, delegation or polling performed.

## Authentication first

Only this epoch's NEW `integration-sealed` was read/hash-authenticated, exactly once, against `artifact.json`. SHA-256 `e0c2a1154f94718df6ebca0b0f7028478d0ef5c71ea08cf4a3a09e08e8f9979b`; computed/expected size **467,686,160 bytes**. Both comparisons PASS. `initial-red-independent-auth.json` was immediately created at **2026-10-12T09:00:10.534212+00:00**, before source/log reporting, and preserved unchanged. No old currency-query seal was read or hashed; no seal was executed or reauthenticated.

## Streams and receipts

Read entire raw compile/runtime stdout/stderr; complete compile JSON decoded programmatically, rather than relying on truncated display: **574,432 bytes / 741 records** = 665 compiler artifacts, 75 build-script records, one successful build-finished record, zero compiler-message records. Two artifacts are non-fresh: test harness and integration test. Compile stderr has six iced manifest deprecation warnings, explicitly not warning-free; finish 49.93 seconds. Retained toolchain: Cargo/rustc **1.99.0**, host `x86_64-unknown-linux-gnu`. Stream byte counts/digests are in `initial-red-independent-proof.json`.

Read complete runtime stdout (1,037 bytes) and stderr (4,746 bytes, 69 lines), matching all five source test names and four exact panic messages. Runtime result took 1.00 second. `runtime-terminal.json` records `ExecMainStatus=101` / failed unit; its inspection command exit0 and runtime submission exit0 are NOT test success. Original `outcome.json` still says `COMPILED_NOT_EXECUTED` / zero cases: stale compile-only summary contradicted by subsequent full runtime evidence. Original receipts were not overwritten.

## Exact coverage and first failure boundary

Every Rust failure ends at `tests/quest_currency_war_mode.rs:13:10` (`probe`'s expect). Lua assertion locations and unreachable controls:

| Case | Retained result / first failure | Later controls NOT reached |
|---|---|---|
| Actual vendor ordinary reward baseline | **PASS**, real loaded public `QuestUtils_AddQuestCurrencyRewardsToTooltip` with real `GameTooltip`; `(1,false,nil)`; exact sole line `\|TInterface\\Icons\\INV_Misc_QuestionMark:16:16\|t 7 Fixture Marks`; recorded Lua errors empty | No positive War Mode input/bonus test exists in this initial case |
| Desired mode | **FAIL**, line97: `War Mode desired is a real boolean`. One-result arity assertion already passed; callable getter returns nil, not a boolean | Initial false policy; reward10 assertion; separate default10 assertion |
| Quest eligibility numeric identity | **FAIL**, line117: `quest eligibility must reject nonnumeric identity`. First input is string `'901001'`; false placeholder accepts it without error | False/table/function arguments; omitted/nil argument rejection; absent eligibility false |
| Currency faction numeric identity | **FAIL**, line136: `faction query must reject nonnumeric identity`. First input is string `'90001'`; callable nil-returning namespace fallback accepts it | False/table/function arguments; omitted/nil arguments; absent relation one-result/nil checks |
| Tainted relation authentication | **FAIL**, line163: `tainted relation secret denied`. First getter is quest eligibility, first argument wraps numeric901001; false placeholder accepts the tainted secret | Error-text authentication ordering, wrapper preservation, no declassification, caller-taint preservation, invalid wrapped string, faction-query security and caller restoration |

Do not credit unreached assertions. The fourth failure proves the first quest-secret denial requirement fails; it does NOT independently demonstrate a faction-secret failure.

## Public callable behavior, not invented nil calls

At test revision, `QuestCanHaveWarModeBonus` is registered to `return_false` (`src/lua_api/globals/quest_surface/register.rs:51,175-179`; `handlers.rs:566-568`), accepting arguments without validation. Missing desired/faction members participate in the existing public namespace fallback: `runtime_surface_bootstrap.lua:64-78` returns/caches a callable function whose body returns one nil; namespace attachment/name seeding is at112-125,209-215. Failures are explicit test assertions, not attempts to call a nil function. Existing percentage providers both push10 (`small_namespaces.rs:322-330`); their source implementation is observed, but their later test assertions remain unreachable.

## Whole vendor closure and source-data fixture

`tests/quest_currency_war_mode.rs:22-36` selects the active profile cache and loads the **whole `Blizzard_FrameXMLUtil` dependency closure with empty overrides**, asserting the addon loaded. Shared harness `blizzard_addon_closure.rs:24-47,59-83` discovers TOCs and uses the ordinary `load_addon` path. TOC includes `[Family]\\QuestUtils.lua`, `CurrencyContainer.lua`, dependencies SharedXMLGame/Colors/StaticPopup and other normal addon files; not an extracted-function imitation. Runtime explicitly reports:

`Loaded public currency reward consumer: @Interface/AddOns/Blizzard_FrameXMLUtil/Mainline/QuestUtils.lua`

Fixture supply is separate Rust data mutation after loading (`tests/quest_currency_war_mode.rs:39-64`): cloned existing quest record with ID901001, world-quest flag true, reward currency90001/amount7/nameFixture Marks/icon question mark/base amountNone; CurrencyInfo quality2/default remaining fields. No replaced vendor getter, fabricated color helper or vendor override. Actual `CurrencyContainer.lua:21-34` computes overflow/color from currency quantity/cap. Tooltip output is asserted from real simulator tooltip state, not a fake Lua tooltip.

Current full FrameXMLUtil identity is retained in the proof: **43 files / 219,352 bytes**, individual SHA-256/size/path for EVERY file. Whole QuestUtils is **41,718 bytes**, SHA-256 `e2654815fad42e9560210d49ee873862427985e6a0a81ec8a28dba4f10f63a6e`. Cache provenance records retail `12.1.0.69933`, source `casc-local-or-cdn`, fallbacknone; committed retail manifest hash matches recorded `aa7dfec3fb3bc9440a8c737c2274363502cb3d22c7939b8a570a58e717202edd`. These are **current cache identities**, not an independently retained run-time byte snapshot or native-client oracle. Runtime source path plus unchanged test/harness establishes the loading boundary; it does not prove native catalog contents or positive bonus behavior.

## Source contracts versus simulator spec policies

- Cached `PvpInfoDocumentation.lua:1063-1069` declares one nonnil bool desired result;785-800 declare numeric reward/default percentage. **Initial false and compatibility10 are simulator spec policies**, not native state/value evidence.
- `QuestLogDocumentation.lua:1064-1078` declares required numeric quest identity, one nonnil bool and `SecretArguments = "AllowedWhenUntainted"`. Independent supplied eligibility/absentfalse is the spec policy; world-quest classification is not eligibility equivalence or a supplied native catalog.
- `CurrencyInfoDocumentation.lua:346-359` declares required numeric currency identity, one nullable numeric immediate-reputation faction relation and `AllowedWhenUntainted`. Explicit absent relationnil is simulator policy, not native unknown-ID truth.
- Actual QuestUtils `683-694,898-985` separately gates desired mode, nullable currency apply/once flags, locally computed firstInstance, world-quest classification, eligibility and faction exclusion; percentage comes from a separate getter. Baseline cannot prove all these gates or a positive caller.

## Revision applicability — subsequent source exists

Retained `revision.json` and `source-applicability.json` identify test/build revision97d263565 and then-clean diff; current test source digest matches `b15634da097da3e74c5ee1a69d4f4fe7eac5cedfc934457beab7e84e19f8359d`. Inspected committed HEAD `db12d3f9f8c0f8c28557205abea83b41ad4b642a` differs from test revision only in two wiki documents.

**However, working-tree code changed DURING this inspection.** Observed modified `src/c_api/c_pvp.rs`, `src/lua_api/state.rs`, `src/lua_api/state/sim_state.rs`, plus untracked `src/c_api/c_pvp/war_mode.rs`: a subsequent owned-input scaffold is now appearing (WarModeState, quest eligibility and faction maps). Do NOT claim current working-tree code is unchanged or that no scaffold currently exists. This report covers only the authenticated retained initial RED at97d263565; subsequent scaffold/producer/expanded controls have no execution credit here. Exact snapshot and per-file comparisons are retained in the proof.

**Remaining OPEN:** expanded positive controls and owned-input producer proof, GREEN, positive public War Mode caller, PTR execution, startup/all-profile/native catalog/parity/full handoff. Ordinary reward baselinePASS is bounded compatibility evidence only.
