# Client Profiles

The simulator targets six WoW client profiles concurrently — retail, PTR, wrath (3.3.5a), mists (5.4 / MoP Classic), era (1.x / Vanilla), and anniversary — selected at compile time via mutually-exclusive cargo features. Each profile uses a profile-scoped Blizzard UI cache and routes the loader through profile-aware suffix and gametype filters.

## Active profile selection

`src/client_profile.rs` defines `enum ClientProfile { Retail, Ptr, Wrath, Mists, Era, Anniversary }` and a single `pub const ACTIVE: ClientProfile` resolved by cfg-blocks against the enabled `client-*` cargo feature. A `compile_error!` block enforces *exactly one* feature must be enabled.

Feature ↔ profile ↔ vendor source ↔ TOC suffix:

| Feature              | Profile     | Cache subdir | Interface | Primary TOC suffix |
|----------------------|-------------|--------------|-----------|--------------------|
| `client-retail`      | Retail      | `retail`     | `120007`  | `_Mainline`        |
| `client-ptr`         | Ptr         | `ptr`        | `120100`  | `_Mainline`        |
| `client-wrath`       | Wrath       | `wrath`      | `38001`   | `_Wrath`           |
| `client-mists`       | Mists       | `mists`      | `50504`   | `_Mists`           |
| `client-era`         | Era         | `era`        | `11507`   | `_Vanilla`         |
| `client-anniversary` | Anniversary | `anniversary`| `11507`   | `_Vanilla`         |

Helper functions in `src/client_profile.rs`:

- `cache_subdir()` → profile cache directory name under `~/.cache/wow-ui-sim/blizzard-ui/`
- `interface_version()` → active TOC interface version
- `blizzard_ui_addons_dir()` → completed cache path for the active profile, or the profile-scoped default cache path
- `blizzard_ui_addons_dir_under(root)` — test fallback path anchored at `root`
- `blizzard_ui_framexml_toc()` → wrath-only `<cache>/FrameXML/FrameXML.toc`; retail/PTR/mists/era/anniversary collapsed FrameXML into `Blizzard_*` addons

## Runtime Cache

Runtime Blizzard UI files live under the user cache:

```
~/.cache/wow-ui-sim/blizzard-ui/<profile>/AddOns
```

Populate it with `wow-cli casc sync-blizzard-ui` or the compatibility wrapper `scripts/setup-blizzard-ui.sh`. Do not use `Interface/BlizzardUI/` or repo-local `vendor/wow-ui-source-*` checkouts for runtime loading.

Each profile uses its own committed manifest in `data/blizzard-ui-files/<profile>.txt`. PTR uses the `wowt` CASC product and the `ptr.txt` manifest; retail uses the `wow` CASC product and `retail.txt`. This avoids mixing divergent PTR and retail addon file lists in one union manifest. The non-PTR manifests currently mirror the previous shared baseline and are expected to diverge as each profile's CASC-backed file list is curated.

Local install discovery uses the active profile's WoW flavor directory. PTR reads addons, WTF, and BlizzardInterfaceArt from `_ptr_`; retail continues to use `_retail_` with optional `_beta_` addon fallback.

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
| Ptr         | `mainline`, `standard`                            |
| Wrath       | `wrath`, `wrath_classic`, `classic`               |
| Mists       | `mists`, `mists_classic`, `classic`               |
| Era         | `vanilla`, `classic_era`, `classic`               |
| Anniversary | `vanilla`, `classic_anniversary`, `classic`       |

`family_subdir()` substitutes the `[Family]` TOC token: retail/PTR → `Mainline`, wrath/mists/era/anniversary → `Classic`. The `[Game]` token maps retail/PTR to `Standard`, wrath to `Wrath`, mists to `Mists`, and era/anniversary to `Vanilla`.

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

`src/event/valid_events.rs` follows the same shape: retail/PTR use the strict generated `EVENTS_A/B/C` tables; wrath/mists/era/anniversary route through `crate::wrath::is_registerable_event(name)` which accepts any non-empty event name (the mainline `events.yaml` doesn't cover pre-Cataclysm or vanilla).

## Synthetic FrameXML addon (wrath only)

Wrath ships its UI as a flat `Interface/FrameXML/` tree alongside `Interface/AddOns/`; retail/PTR/mists/era/anniversary collapsed FrameXML into a `Blizzard_FrameXML` addon. The loader detects this via `client_profile::blizzard_ui_framexml_toc()` and synthesizes a virtual addon called `FrameXML` that loads before the regular `Blizzard_*` discovery pass.

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
