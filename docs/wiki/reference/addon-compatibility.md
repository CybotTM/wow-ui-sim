# Addon Compatibility

The simulator loads the full Blizzard UI tree from `~/.cache/wow-ui-sim/blizzard-ui`, populated by `wow-cli casc sync-blizzard-ui` from the committed manifest, plus a small bundled set of third-party addons under `Interface/AddOns/`. CI and Wowless lanes can also mount external addons at test time.

## Tested Addons

Smoke targets include heavy-weight frameworks (**Ace3**, **WeakAuras**, **Details**, **BigWigs**) plus the bundled test/diagnostic addons (`TestFramework`, `DebugTests`, `BugSack`/`!BugGrabber`, `BetterBags`, `AutoRoll`, `SimCommands`, `Admin`, `FcTest`, `DebugCheck`). These exercise the full API surface — any stub returning nil instead of a typed value will surface as a Lua error or silent behavioral failure.

## Integration with Wowless

**Wowless** (`~/Repos/wowless`) is an independent headless WoW Lua/XML interpreter used as a cross-reference for API semantics. The simulator includes `Interface/AddOns/Wowless/` and `Interface/AddOns/WowlessData/` as test suites.

**Important**: never modify files in these directories. `WowlessData/` is regenerated from `~/Repos/wowless/data/` via `python3 tools/gen_wowless_data.py`.

The Wowless test suite (`run-tests Wowless`) validates API compatibility but takes 60s+ — use `cargo test` for normal development.

## SavedVariables Loading

The simulator loads real `SavedVariables` from `~/Projects/wow/WTF` (the actual WoW installation). This gives addons realistic persisted state. Loading can be skipped with `--no-saved-vars` (saves ~18% startup time).

SavedVariables loading is performed per-addon after Lua execution, matching WoW's load order.

## Docker CI

A CI image is published to `ghcr.io/osso/wow-ui-sim` for running tests in addon pipelines:

```yaml
# GitHub Action
- uses: osso/wow-ui-sim@v1
  with:
    addon: MyAddon
```

```bash
# Docker directly
docker run --rm -v ./MyAddon:/app/Interface/AddOns/MyAddon ghcr.io/osso/wow-ui-sim run-tests MyAddon

# With full Blizzard addons loaded
docker run --rm -v ./MyAddon:/app/Interface/AddOns/MyAddon ghcr.io/osso/wow-ui-sim --no-saved-vars run-tests MyAddon
```

The Docker image is headless-only (~220MB): no audio, no textures, no GPU drivers. `screenshot` is not supported in Docker. The image is optimized for `run-tests`, `self-test`, and `lua-errors`.

## Load Order

The simulator follows WoW's addon load order. The cached Blizzard tree provides the full chain — `Blizzard_SharedXMLBase`, `Blizzard_SharedXML`, `Blizzard_SharedXMLGame`, `Blizzard_FrameXMLBase`, `Blizzard_FrameXMLUtil`, `Blizzard_FrameXML`, then per-feature `Blizzard_*` addons. User and third-party addons under `Interface/AddOns/` load on top via TOC files. Per-addon SavedVariables are applied after each addon's Lua executes.

## Test Lanes

Blizzard UI coverage is intentionally split into two lanes:

- `tests/blizzard_ui_unit.rs` covers isolated helpers and component behavior that can run without a full Blizzard startup sequence.
- `tests/addon_coverage.rs` covers addon-bootstrap behavior that only exists after the relevant Blizzard addons load.

The split keeps unit-style regressions fast and focused while reserving the heavier bootstrap suite for startup-order, dependency, and load-on-demand behavior.
The loader now exposes `discover_blizzard_addon_closure_for_screen()` so the bootstrap lane can load an explicit TOC-derived closure for a target addon set, including load-on-demand roots, instead of a fake monolithic Blizzard bundle.

## Known Issues

- `BetterWardrobe/ColorFilter.lua` uses large constant tables that work in WoW's patched LuaJIT but are slow in standard Lua 5.1
- Many addons load with non-fatal Lua errors due to still-missing API stubs (tracked in PLAN.md Phase 32)

## Sources

- [AGENTS.md](../../../AGENTS.md) — Docker CI, Wowless integration, SavedVariables paths

## See Also

- [[api-coverage]] — which APIs are missing (causes addon errors)
- [[cli-commands]] — `run-tests`, `lua-errors`, Docker usage
- [[development-phases]] — Phase 5 progress and missing API work
- [[blizzard-ui-test-lanes]] — explicit unit vs addon-bootstrap test split
