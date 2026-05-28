# Client Profiles

The simulator targets five WoW client expansions concurrently — retail, wrath (3.3.5a), mists (5.4 / MoP Classic), era (1.x / Vanilla), and anniversary — selected at compile time via mutually-exclusive cargo features. Each profile pins its own vendor `wow-ui-source` checkout and routes the loader through profile-aware suffix and gametype filters.

## Active profile selection

`src/client_profile.rs` defines `enum ClientProfile { Retail, Wrath, Mists, Era, Anniversary }` and a single `pub const ACTIVE: ClientProfile` resolved by cfg-blocks against the enabled `client-*` cargo feature. A `compile_error!` block enforces *exactly one* feature must be enabled (10 mutual-exclusion pairs covered).

Feature ↔ profile ↔ vendor source ↔ TOC suffix:

| Feature              | Profile     | Vendor (`scripts/setup-blizzard-ui.sh`) | Primary TOC suffix |
|----------------------|-------------|-----------------------------------------|--------------------|
| `client-retail`      | Retail      | `Gethe/wow-ui-source@b062d332` (12.0.5) | `_Mainline`        |
| `client-wrath`       | Wrath       | `Gethe/wow-ui-source@c4e0255f` (3.3.5) | `_Wrath`     |
| `client-mists`       | Mists       | `Gethe/wow-ui-source@33d87412` (classic) | `_Mists`          |
| `client-era`         | Era         | `Gethe/wow-ui-source@e0099491` (1.15.8 build 67156) | `_Vanilla` |
| `client-anniversary` | Anniversary | `Gethe/wow-ui-source@b29b0d0a` (2.5.5 build 67157)  | `_Vanilla` |

Helper functions in `src/client_profile.rs`:

- `blizzard_ui_root()` → `./Interface/BlizzardUI/<Profile>/`
- `blizzard_ui_addons_dir()` → `<root>/AddOns`
- `blizzard_ui_addons_dir_under(root)` — same path anchored at `root` (used by tests under `CARGO_MANIFEST_DIR`)
- `blizzard_ui_framexml_toc()` → wrath-only `<root>/FrameXML/FrameXML.toc`; retail/mists/era/anniversary collapsed FrameXML into `Blizzard_*` addons

## Vendor layout

Each profile points `Interface/BlizzardUI/<Profile>` at a sparse-checkout symlink:

```
Interface/BlizzardUI/Retail      -> vendor/wow-ui-source-retail/Interface
Interface/BlizzardUI/Wrath       -> vendor/wow-ui-source-wrath
Interface/BlizzardUI/Mists       -> vendor/wow-ui-source-mists/Interface
Interface/BlizzardUI/Era         -> vendor/wow-ui-source-era/Interface
Interface/BlizzardUI/Anniversary -> vendor/wow-ui-source-anniversary/Interface
```

Both `Interface/BlizzardUI/` and `vendor/` are gitignored. `scripts/init-worktree.sh` creates all five symlinks in a fresh worktree; `scripts/setup-blizzard-ui.sh <profile> [ref]` clones / refreshes one profile.

## Profile-aware loader paths

### TOC discovery (`src/loader/mod.rs`)

`find_toc_file()` picks the variant matching `ClientProfile::ACTIVE`:

1. `<addon><primary_suffix>.toc` (e.g. `Bartender4_Wrath.toc` under wrath)
2. Plain `<addon>.toc`
3. Any `.toc` whose name doesn't contain another profile's suffix — driven by helpers `active_profile_toc_suffix()` and `other_profile_toc_suffixes()` (`scan_for_compatible_flavor_toc()` is the fallback walker)

### TOC content (`src/toc/mod.rs`)

`is_allowed_game_type()` reads inline `[AllowLoadGameType <type>]` annotations and matches against the active profile's allow-list:

| Profile     | Accepted gametypes                                |
|-------------|---------------------------------------------------|
| Retail      | `mainline`, `standard`                            |
| Wrath       | `wrath`, `wrath_classic`, `classic`               |
| Mists       | `mists`, `mists_classic`, `classic`               |
| Era         | `vanilla`, `classic_era`, `classic`               |
| Anniversary | `vanilla`, `classic_anniversary`, `classic`       |

`family_subdir()` substitutes the `[Family]` TOC token: retail → `Mainline`, wrath/mists/era/anniversary → `Classic`. The `[Game]` token is hardcoded to `Standard` (no plunderstorm/wowhack support).

`TocFile::is_game_type_restricted()` evaluates the `## AllowLoadGameType` *header* line (separate from the inline annotation parser) using the same allow-list.

## Per-profile compatibility shims

Each non-retail profile loads its own Lua bootstrap after `runtime_surface_bootstrap.lua` and before secure-environment cloning. Wired in `src/lua_api/env_init/mod.rs`:

| Profile module        | Files                                                                | What it stubs |
|-----------------------|----------------------------------------------------------------------|---------------|
| `src/wrath/`          | `compat_bootstrap.{lua,rs}`, `compat_frame_proxies.lua`, `frame_methods.rs`, `post_load.{lua,rs}` | ~30 wrath-specific stubs + Lua-5.0 string/math aliases + `MiniMapTrackingIcon`/`PlayerArrowEffectFrame` proxies (wrath-only). Also registers `IgnoreDepth`, `SetBackdropColor`, `SetBackdropBorderColor`, `SetPlayerTextureWidth/Height`, `SetMaxBytes`, `GetTextHeight` directly on the frame metatable for code paths that call them as methods. |
| `src/mists/`          | `compat_bootstrap.{lua,rs}`                                          | ~46 mists-only globals (post-Cataclysm leftovers MoP kept that retail removed: `GetActionBarPage`, `GetComboPoints`, `GetQuestLog*` family, `GetRuneType`, etc.) |
| `src/era/`            | `compat_bootstrap.{lua,rs}` (shared by era + anniversary)            | ~30 vanilla-only globals: `IsInGlobalEnvironment`, `GetActionBarPage/Toggles`, `GetComboPoints`, `GetPVPYesterdayStats`, `MoneyFrame_OnLoad`, `MoneyInputFrame_*`, `SecureMixin`, `UIParent_OnLoad`, `IsKeyRingEnabled`, `HasKey`, `HasPetUI`, `SetSelectedSkill`, etc. |

Promotion rule: a stub starts in the per-addon shim (`tools/classic-addon-compat/<addon>/<shim>/<shim>.lua` with `## LoadFirst: 1`); if the same gap surfaces across multiple addons under one profile, it gets promoted to the matching profile-level bootstrap so per-addon shims stay narrow.

The wrath module is shared with mists/era/anniversary at the cfg level (`src/lib.rs`: `#[cfg(any(client-wrath, client-mists, client-era, client-anniversary))] pub mod wrath;`) because all four profiles need its `frame_methods::register_all` (no-op stubs for backdrop / depth / player-texture methods that vendor frames call directly). Only wrath actually loads `compat_bootstrap.lua` itself; mists has its own; era + anniversary share `src/era/compat_bootstrap.lua`. The wrath-only `compat_frame_proxies.lua` (real `Blizzard_SharedXML` would shadow it on mists) is gated tighter at `#[cfg(feature = "client-wrath")]`.

`src/event/valid_events.rs` follows the same shape: retail uses the strict generated `EVENTS_A/B/C` tables; wrath/mists/era/anniversary route through `crate::wrath::is_registerable_event(name)` which accepts any non-empty event name (the mainline `events.yaml` doesn't cover pre-Cataclysm or vanilla).

## Synthetic FrameXML addon (wrath only)

Wrath ships its UI as a flat `Interface/FrameXML/` tree alongside `Interface/AddOns/`; retail/mists/era/anniversary collapsed FrameXML into a `Blizzard_FrameXML` addon. The loader detects this via `client_profile::blizzard_ui_framexml_toc()` and synthesizes a virtual addon called `FrameXML` that loads before the regular `Blizzard_*` discovery pass.

## CI matrix

`.github/workflows/test.yml` `client-profile-smoke` currently runs the Mists profile only. The job runs `setup-blizzard-ui.sh mists` → `cargo build --features client-mists` → `cargo check --tests` → `lua-errors > lua-errors.json`, then diffs against `docs/baselines/mists-lua-errors.json`. Retail stays on the dedicated `cargo-test` job because tests are written against the retail UI surface.

The addon harness is Mists-only and is driven locally by `scripts/test-classic-addons.sh` / `scripts/ci-mists-guard.sh` from `tools/classic-addon-manifest.tsv`. See `docs/baselines/classic-addon-test-targets.md` for the retained addon picks.

## Mists baselines

Captured in `docs/baselines/`:

- `mists-lua-errors.json` — clean boot-time error snapshot
- `mists-panels.md`, `mists-panel-interactions.md`, and `mists-panel-visuals.tsv` — panel parity artifacts
- `mists-test-coverage.md`, `mists-release-proof.md`, and `mists-lod-audit.md` — retained Mists proof notes
- `classic-addon-test-targets.md` — Mists addon harness target set

## Sources

- `src/client_profile.rs` — enum, `ACTIVE` const, profile path helpers
- `src/loader/mod.rs` — `find_toc_file`, `active_profile_toc_suffix`, `other_profile_toc_suffixes`
- `src/toc/mod.rs` — `is_allowed_game_type`, `family_subdir`, `TocFile::is_game_type_restricted`
- `src/lib.rs` — `pub mod wrath`/`mists`/`era` cfg gates
- `src/lua_api/env_init/mod.rs` — bootstrap call ordering
- `src/wrath/`, `src/mists/`, `src/era/` — per-profile modules
- `src/event/valid_events.rs` — strict vs permissive event validator dispatch
- `scripts/setup-blizzard-ui.sh`, `scripts/init-worktree.sh` — vendor pinning
- `.github/workflows/{test,addon-harness}.yml` — CI matrix

## See Also

- [[addon-loading]] — TOC discovery, addon load order, SavedVariables (now profile-aware)
- [[taint-system]] — `runtime_surface_bootstrap.lua` runs before each profile's compat bootstrap
- [[lua-api]] — frame methods registered globally vs profile-conditional
- [[event-system]] — strict-vs-permissive event validator gating
