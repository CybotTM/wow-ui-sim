# Development Phases

Phases 1–4 are complete. Active work is in phases 31–33.

## Completed Phases (1–4)

All foundational work is done:

- **Phase 1**: Lua 5.1 environment, CreateFrame, events, global aliases, Mixin system
- **Phase 2**: Widget API — alpha, strata, anchors, Texture/FontString creation
- **Phase 3**: Addon loading — TOC parser, XML templates, full Blizzard UI chain (`Blizzard_SharedXMLBase`/`SharedXML`/`SharedXMLGame`/`FrameXMLBase`/`FrameXMLUtil`/`FrameXML` + per-feature addons) plus user-addon TOC loading
- **Phase 4**: Rendering — iced canvas, strata z-ordering, text rendering, BLP/PNG/WebP textures, anchor cycle detection

## Phase 5: Real Addon Testing (In Progress)

The Blizzard UI tree and bundled third-party addons load. Remaining work is filling API stubs so addons load with fewer Lua errors. Tracked via `wow-cli audit-api --gaps`.

## Active: Phase 31 — Frame/Widget Method Stub Sweep

Replacing no-op stubs with stateful behavior:

- **31.9** (complete): security-related partials in `methods_attribute.rs` — `CanChangeProtectedState`, `SetFlattensRenderLayers`, `SetMotionScriptsWhileDisabled`
- **31.10** (pending): editbox stub family (15 methods), cooldown stub getters/setters (~12 methods), scrollbox callback registry/dispatch, scrollframe `UpdateScrollChildRect`

## Active: Phase 32 — Blizzard UI API Audit Tool

Static analysis of `vendor/wow-ui-source/` to track coverage gaps. The `wow-cli audit-api` command is operational.

**Mostly complete**: Lua scanner, XML scanner, `--filter-startup` flag, file location info in gap reports, false-positive reduction for C_* calls.

**Remaining**:
- 2 low-priority Enum namespaces: `Enum.CharacterCreateRaceMode` (1 ref), `Enum.TransmogOutfitSlotOptionSheatheCategory` (1 ref)
- 107 missing C_* methods — see [[api-coverage]] for breakdown by category
- Refactoring `audit_api.rs` (1073 lines, many functions exceeding 30-line limit)

## Active: Phase 33 — Performance Regression Tests

The repo already has ad hoc timing tools such as `src/bin/bench_talents.rs`,
but the regression path should live under normal `cargo test` so it can fail in
the same workflow as behavioral tests. The decision for this phase is:

1. **Infrastructure**: use simple `#[test]` timing assertions with
   `Instant::now()` for regression gates instead of adding `criterion`;
   create `tests/perf/` directory; shared Blizzard UI loader to amortize 5s+
   startup cost
2. **Startup**: measure `WowLuaEnv::new` through startup events; per-phase breakdown (XML, Lua, SavedVars)
3. **Layout**: full layout pass, incremental anchor change, `build_strata_buckets`
4. **Rendering**: quad batch build, dirty-tree rebuild, tooltip emission, glyph shaping
5. **Frame ops**: `CreateFrame` throughput (1000 frames), `SetPoint` throughput, event dispatch to 100+ handlers
6. **Memory**: widget registry size and Lua memory after full startup

All items in Phase 33 are currently unchecked.

## Sources

- [PLAN.md](../../../PLAN.md) — full task list with checkbox status

## See Also

- [[architecture-overview]] — what was built in phases 1–4
- [[api-coverage]] — Phase 32 gap analysis results
- [[addon-compatibility]] — Phase 5 addon testing scope
