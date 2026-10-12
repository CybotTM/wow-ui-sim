# Independent retained default GREEN gate

**BOUNDED PASS**, producer `ce2c37ad5623642bec418733ac363600ea3e7b63`. No new build/run/test, operation, network, delegation or polling. Only requested authentication and report files written. No old/RED rehash or PTR-artifact inspection.

## Authentication first

Both NEW seals streamed exactly once. Each computed digest/size comparison was persisted immediately, before source/runtime analysis:

| Seal | Computed SHA-256 | Bytes | Result |
|---|---|---:|---|
| integration-sealed | 86774e8187f356f75b4f9b87b1afc6ad2d0bcf987c3bbbab6f82f3832410fbce | 467645008 | manifest digest/size/path MATCH |
| wow-sim-sealed | 20bd4d6832306badbf4c2dd5465fa7584b6fdc37f99bf6b4d0c20136666fc0b2 | 407293168 | manifest digest/size/path MATCH |

Receipts: `default-green-independent-auth-integration.json`, `default-green-independent-auth-normal.json`. Both manifest Cargo records exactly match retained compile.stdout records. Retained compile exit0, build-finished success true; 741 JSON records, no compiler-message records. This authenticates retained bytes/manifest consistency, not an independent reconstruction of binary-source provenance. revision.json supplies producer identity and compile-result.json supplies source_unchanged; scoped git diff against producer is empty for implementation/state/tests/Cargo. Current docs/owning-sweep data changes are not producer runtime evidence.

## Concrete coverage matrix

| Contract | Evidence | Proof level / remaining boundary |
|---|---|---|
| Absent/strict input/numeric identity | absent two nils; required nonnumeric rejection; fractional, nonfinite, out-of-i32 positive non-aliasing controls | retained default runtime PASS; absence is modeled policy, not native oracle |
| Supplied two independent nullable flags | nine nullable pairs, repeated live update/removal, unaffected currency, previously returned scalars unchanged | retained default runtime PASS; explicit supplied map only |
| Secret arguments | untainted wrappers/caller preserved, positive wrapped identity, wrapped invalid type; tainted numeric/string secrets denied before type/lookup | retained default runtime PASS; pinned guard denies tainted/revoked access, but no dedicated revocation case in these nine |
| Raw loaded publication | rawget function, two returns, unseeded nil/nil, string rejection after 291 Blizzard addons | retained normal PASS, [] and exit0; owning publication sweep remains separate/pending |
| Caller/native acceptance | QuestUtils policy and native catalogs not exercised positively | OPEN; getter does not establish caller model/full handoff |

Read FULL streams: runtime.stdout 14 lines/1042 bytes, runtime.stderr 99/5746; normal.stdout 1/3, normal.stderr 219/14698. Runtime: **9 PASS / 0 FAIL / 0 ignored / 0 measured / 10758 filtered**, exit0 (0.75s). Exact tests:

- `currency_war_mode_bonus::currency_war_mode_bonus_absent_records_return_two_nil_flags`
- `currency_war_mode_bonus::currency_war_mode_bonus_owned_updates_removal_and_currency_isolation`
- `currency_war_mode_bonus::currency_war_mode_bonus_positive_identity_does_not_alias_numeric_edges`
- `currency_war_mode_bonus::currency_war_mode_bonus_requires_number_without_string_coercion`
- `currency_war_mode_bonus::currency_war_mode_bonus_supplied_independent_nullable_combinations`
- `currency_war_mode_bonus::currency_war_mode_bonus_supplied_secret_identity_and_invalid_type`
- `currency_war_mode_bonus::currency_war_mode_bonus_tainted_secrets_denied_before_validation`
- `currency_war_mode_bonus::currency_war_mode_bonus_unrepresented_numbers_have_no_flags`
- `currency_war_mode_bonus::currency_war_mode_bonus_untainted_secrets_preserve_wrapper_and_caller`

Normal stdout is literally `[]\n`, not empty output. Final stderr: `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`, zero attributed/unattributed occurrences. Normal capture disables third-party addons and SavedVariables, logs sound disabled and CASC=false/font fallback; not full-addon/assets acceptance. Timings are observations, not performance proof.

### Actual normal query script

```lua
local f=rawget(C_CurrencyInfo,'DoesWarModeBonusApply'); assert(type(f)=='function'); assert(select('#',f(90001))==2); local a,b=f(90001); assert(a==nil and b==nil); local ok,e=pcall(f,'90001'); assert(not ok and string.find(e,'number',1,true))
```

`src/bin/wow_sim/main.rs:549-559` forwards exec_lua and converts unclean summary to exit1. `src/lua_errors.rs:26-85` executes post-startup code, routes probe errors into error collection, ticks updates, emits JSON and returns errors.is_empty(). Thus submitted assertions plus []/exit0 support raw publication, not just addon loading. No positive membership seed or QuestUtils invocation is in this script.

## Source/model/security

`src/c_api/item_spell/c_currency.rs:45-101`: Retail/PTR registration at 50-58; pure numeric validator at 67-77 rejects nonnumbers without string coercion, finite/integral/in-range validation before i32 conversion; query at 79-96 authenticates before validation, briefly borrows owned state, copies independent flags, pushes two nullable scalar values and returns2. Unknown/unrepresented numeric identity maps to two nils by explicit selected policy. No inferred eligibility from existing currency catalog, quantity, faction, or PvP state.

`src/lua_api/state/sim_state.rs:572-573` owns `HashMap<i32, (Option<bool>, Option<bool>)>`; `state.rs:492` initializes it empty. Tests mutate this real map, not a mocked getter. External registration: `c_api/item_spell/mod.rs:29-33`, forwarding shim `lua_api/globals/missing_surface/item_spell/mod.rs:10-11`, caller `missing_surface.rs:182-183`. Test wiring: `tests/integration.rs:1` invokes dev harness; `crates/test-harness/build.rs:30-66,441-448` discovers top-level Rust modules; retained named executions establish these nine are linked/executed.

Pinned Cargo rilua `842e4d3ff8592af45a5a93058eddf93419d29012`, local checkout `~/.cargo/git/checkouts/rilua-fd5a0715e46b5888/842e4d3/src/table_security.rs:400-427`: unwrap_secret reads payload without mutating wrapper, checks can_access_secrets before returning it; guard requires secure stack and nonrevoked context. Query's secret handling cfg is enabled in these default features; no other-profile/PTR runtime claim.

Retail cache `CurrencyInfoDocumentation.lua:35-49` declares required number, AllowedWhenUntainted and two nullable booleans; declarations are not native eligibility catalogs. `Blizzard_FrameXMLUtil/Mainline/QuestUtils.lua:683-694` separately checks WarModeDesired, applies/once/firstInstance, world-quest identity, QuestCanHaveWarModeBonus and faction-granted relationship. Getter supplied-state proof establishes none of those caller models or positive tooltip rendering. No vendor rewrite claimed.

## Changed Rust readability only

Full rust-readability skill read. Independent manual audit: producer-added registration/validator/query, empty state initializer/map field and focused test helpers/supplied controls. **No violations found**. Validator/query each under30 body lines, shallow control flow, named numeric predicates, at most two boolean operators, short iterator chain, short-lived immutable state borrow and explicit scalar pushes; tests use concrete combinations and owned updates, no new warning suppression or placeholder markers. No metrics/audit executable run and no broad re-audit. Existing source outside these changed areas not certified.

## Warnings retained / unpromoted gates

- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
- warning: `iced_wgpu` (manifest) generated 6 warnings

Six inherited iced manifest lint-name deprecation warnings plus aggregate remain; no warning-free claim. Retained compile success is not cargo fmt/check acceptance. Normal raw publication is not owning-sweep completion. Native catalog, normal positive session membership, QuestUtils caller models, PTR, final Rust gate, full suite and parent handoff remain OPEN. Historical RED preserved, not reopened/rehashed. No claim that bounded GREEN resolves other failures.
