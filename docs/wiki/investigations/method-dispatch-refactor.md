# Method Dispatch Refactor

Refactor of frame method dispatch to fix runtime pollution (wrong-type methods exposed) and align `__index` lookup order with real WoW behavior.

## Current State (Fixed)

Verified by source inspection on 2026-10-09: `env_init/frames.rs` registers method groups on a base frame metatable, then builds a shallow shared `__index` without `__*` keys. `methods/frame_metatable.rs` clones that index into cached per-widget metatables, excluding explicitly named scroll/message methods and `SetStatusBarAtlas` for StatusBar. WorldFrame uses the base metatable. No `is_method_allowed` registry or positive per-type allowlist exists.

`methods.rs::attach_frame_metatable` attaches the selected metatable to each Lua frame table. Its filtered `__index` therefore participates in ordinary lookup, not just introspection. Shared registration does not prove all-type dispatch. These inspected files do not support the former introspection-only filter claim; no invocation tests or native parity proof were run for this correction.

The reported historical **491-row delta across 31 names** and the **11,985-row baseline file total** measure different scopes, not contradictory totals. The historical delta was not independently reproduced here. Neither count alone justifies snapshot replacement, a native allowlist, or a new publication contract.

## Current Investigation — Saved469 and Alpha Receiver (2026-10-10)

Saved suite revision `469a92d27` (not 469 tests) remains **FAIL**; parent goal **OPEN**. Raw log lines 6874–6957 confirm 84 stale extra-snapshot rows: six retired minimap setters × 14 widget types (`SetBlipTexture`, `SetCorpsePOIArrowTexture`, `SetIconTexture`, `SetPOIArrowTexture`, `SetPlayerTexture`, `SetStaticPOIArrowTexture`). Lines 7494–7518 identify five stale missing-snapshot rows: FontString `GetAlphaGradient`, `GetScaleAnimationMode`, `SetScaleAnimationMode`, `SetVertexColorFromBoolean`, and Texture `SetVertexColorFromBoolean`; plus 16 newly missing `SetPreventSecretValues` rows. “Remove stale entries” means those five keys are now exposed, not that they remain absent; line 7495 specifically names **FontString**, not Frame.

Epoch gates intentionally retire the six minimap setters from `retail-12-0-7` and `SetPreventSecretValues` from `retail-12-0-5`. Main source inspection places the minimap gates in `map_frames.rs:62–83`, epoch lookup checks in `tests/global_frames.rs:605–615`, and absence checks in `tests/patch_12_0_7_removed_native_surface.rs:165–168`. Do not restore retired methods or treat snapshot synchronization as a semantic repair. Valid native Minimap invocation on the current epoch is not proven: the setters are absent there. Native discovery README date **2026-02-23** supplies no build/profile identity; snapshot consistency is not native parity.

**Independent saved-capture audit PASS, bounded scope:** diagnostic `20261011T023000Z` (October 10 local work date) reused a sealed normal Retail artifact. Six FontString states assert exactly two values; Frame lookup returned a function and explicit invocation returned `true, 0, 0` (Frame arity not asserted); plain-table `pcall` returned false. Exit 0, unique completion marker and trailing JSON `[]` were verified. Getter resolves frame backing and generic accessibility, without FontString-specific validation. Native receiver compatibility remains unknown; this is not a semantic repair. Shared source representation is `Val::Table`, not userdata. Unsupported 3D, saved suite FAIL and parent OPEN remain unchanged.

[Prior behavioral PASS lines](../../../data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-alpha-receiver/saved469-behavior-excerpts.txt) reuse eight scale-mode/vertex-boolean tests plus two minimap-absence tests already inside saved469 totals. They prove simulator stored-state/arity/isolation and lookup absence, not animation, rendering, native coercion or new executions. Setter retirement and FontString gradient controls also passed at saved469 lines8158/8101. No tests were replayed to classify these deltas.

Privacy custody incident: a read-only agent altered private stderr/submission. Main preserved its sanitized copies and restored originals: stderr matches the original pre-mutation SHA; submission fields and canonical serialization match the verifier’s pre-incident report, but no original submission byte hash existed. [Audit](../../../data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-alpha-receiver/independent-report.md) discloses this limit, non-hermetic EditMode/cache inputs, endpoint-only seals and requested—not independently proven effective—resource limits.

[Method/alpha/receiver packet](../../../data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-alpha-receiver/) retains stdout, privacy-redacted stderr, probe and small receipts/hashes; no binary or private identity is copied. [Saved469 evidence](../../../data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/fullsuite-469/evidence.json) records original raw-log location and suite failure boundaries.

## Complete Method Result Files — 2026-10-11 UTC

[Full before/after packet](../../../data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-full-snapshots/) retains complete comparison files: **11,901 extra + 33 missing = 11,934 rows**, versus previous **11,985 + 22 = 12,007**. The 105 figure is changed rows, not full output size. Sets are reconstructed from saved469's complete emitted delta; relevant source/discovery/original inputs are unchanged, not a fresh runtime capture. Fresh generation025508 was resource-blocked across five admissions/zero executions; both unchanged assertion checks still require fresh resource-admitted proof. Native epoch/parity unknown; original saved469 FAIL and parent OPEN remain.

## Current Source Representation

- Shared registration/index construction: `src/lua_api/env_init/frames.rs`
- Named per-widget exclusions/index cloning: `src/lua_api/methods/frame_metatable.rs`
- Frame-table metatable attachment: `src/lua_api/methods.rs`

## Historical Target Architecture

```
__index(ud, key):
  1. mixin_overrides[frame_id][key]    — Mixin() shadows
  2. children_keys[key]                — child frame lookup
  3. custom_fields[frame_id][key]      — script handlers, properties
  4. rust_dispatch(widget_type, key)   — direct Rust match, no Lua table
```

Step 4 resolves methods directly from widget type + name via a `HashMap<(WidgetType, &str), mlua::Function>`. No intermediate Lua table, no shared-method fallback.

## Historical Proposed Work (Not Current Requirements)

- Trim `diff_methods_extra.txt` (types still exposing too many methods in discovery)
- Add diff-driven coverage so unintentional method-surface changes fail tests
- Optionally collapse runtime dispatch and metatable exposure onto one direct Rust lookup

## Sources

- [frame_metatable.rs](../../../src/lua_api/methods/frame_metatable.rs) — named exclusions and cached metatable clones
- [frames.rs](../../../src/lua_api/env_init/frames.rs) — shared registration and method-index construction
- [methods.rs](../../../src/lua_api/methods.rs) — attaches the selected metatable to frame tables
- [method-dispatch-refactor.md](../../method-dispatch-refactor.md) — historical design

## See Also

- [[editmode-layout]] — regression caused by the Lua-fields-first `__index` change
- [[global-frame-index]] — related `__index` on `_G` for lazy frame lookup
