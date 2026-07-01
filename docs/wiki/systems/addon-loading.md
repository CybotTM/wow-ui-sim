# Addon Loading

The addon loading pipeline discovers addon directories, parses TOC files, loads Lua and XML files in declared order, applies templates, fires startup events, and initializes SavedVariables.

## Blizzard UI cache & per-profile setup

The Blizzard UI source isn't checked into this repo. Runtime loading reads the active profile from a user-cache tree:

```
~/.cache/wow-ui-sim/blizzard-ui/retail/AddOns
~/.cache/wow-ui-sim/blizzard-ui/wrath/AddOns
~/.cache/wow-ui-sim/blizzard-ui/mists/AddOns
~/.cache/wow-ui-sim/blizzard-ui/era/AddOns
~/.cache/wow-ui-sim/blizzard-ui/anniversary/AddOns
```

The active profile (selected by the `client-*` cargo feature; see [[client-profiles]]) is the only one the loader reads from at runtime. Multiple profile caches can coexist on disk so a developer can flip features without replacing another profile's files.

`wow-cli casc sync-blizzard-ui` populates the active profile cache from the committed manifest in `data/blizzard-ui-files.txt`. It writes `.wow-ui-sim-blizzard-ui-complete` plus `.wow-ui-sim-blizzard-ui-provenance` after the manifest is synced. CASC is primary; the Gethe `wow-ui-source` archive remains a fallback source for files that are not available through the local listfile mapping.

`scripts/setup-blizzard-ui.sh` and `scripts/setup-blizzard-ui.ps1` are compatibility wrappers around the same sync command. Their optional profile argument is accepted for old workflows, but the Cargo feature still selects the active profile.

```bash
wow-cli casc sync-blizzard-ui
./scripts/setup-blizzard-ui.sh
./scripts/init-worktree.sh
```

`scripts/init-worktree.sh` is the bootstrap for a fresh worktree: it syncs the active profile cache so startup, benchmarks, and tests can find Blizzard addon sources. Without the completed cache, or when required profile files are missing from an otherwise completed cache, startup re-runs the sync/reports the stale `~/.cache/wow-ui-sim/blizzard-ui/<profile>/AddOns` tree instead of falling back to repo-local symlinks.

The fallback source list is canonical:

| Profile     | Vendor repo                                                | Pinned ref                                               |
|-------------|------------------------------------------------------------|----------------------------------------------------------|
| Retail      | `Gethe/wow-ui-source`                                      | `37181615` (12.0.7)                                      |
| Wrath       | `Gethe/wow-ui-source` tag `3.3.5`                    | `c4e0255f`                                               |
| Mists       | `Gethe/wow-ui-source` branch `classic`                     | `33d87412`                                               |
| Era         | `Gethe/wow-ui-source` branch `classic_era`                 | `e0099491` (1.15.8 build 67156)                          |
| Anniversary | `Gethe/wow-ui-source` branch `classic_anniversary`         | `b29b0d0a` (2.5.5 build 67157)                           |

## Addon Discovery and TOC Parsing

`find_toc_file()` (`src/loader/mod.rs`) prefers `{AddonName}<suffix>.toc` (suffix is profile-dependent: `_Mainline` retail, `_Wrath` wrath, `_Mists` mists, `_Vanilla` era + anniversary) over `{AddonName}.toc` over the first `.toc` whose name doesn't carry another profile's suffix. The suffix table and exclude list live in `active_profile_toc_suffix()` / `other_profile_toc_suffixes()`. See [[client-profiles]].

`TocFile` fields: `addon_dir`, `name`, `metadata: HashMap<String, String>` (Interface version, Title, Dependencies, RequiredDeps, OptionalDeps, LoadOnDemand, SavedVariables, SavedVariablesPerCharacter), `files: Vec<PathBuf>` for TOC-order addon loading, and `file_is_bootstrap: Vec<bool>` marking inline `[Bootstrap]` entries without moving them.

TOC parsing strips `#` comments, skips `[AllowLoadTextLocale]` lines for non-enUS, skips `[AllowLoadGameType]` lines unless the inline gametype matches the active profile's allow-list (retail: mainline/standard; wrath: wrath/wrath_classic/classic; mists: mists/mists_classic/classic; era: vanilla/classic_era/classic; anniversary: vanilla/classic_anniversary/classic), substitutes `[Family]` for the profile's family-subdir (Mainline retail, Classic everywhere else), substitutes `[Game]` for the active profile's game subdir, and normalizes backslashes. Path resolution is case-insensitive for Windows/macOS compatibility. See [[client-profiles]] for the full profile-aware tables.

`[Bootstrap]` does not reorder TOC files. A line such as `Bootstrap.lua [Bootstrap]` stays in `files` at that exact position. For non-LoadOnDemand addons, the bootstrap file executes inline with the surrounding TOC files. For LoadOnDemand addons discovered during startup, only annotated bootstrap files execute during the startup scan; normal files wait for `C_AddOns.LoadAddOn`. A self `C_AddOns.LoadAddOn(thisAddon)` call from an executing bootstrap file is a benign reentrancy no-op and does not load normal files early. PTR-only bootstrap files must still be represented in the cache manifest/profile required-entry set so stale completed caches are rejected when the file is missing.

## Load Flow (`src/loader/addon.rs`)

During startup, addon discovery includes non-LoadOnDemand addons and LoadOnDemand addons that contain `[Bootstrap]` entries. The sorted startup loop preserves addon order: non-LoadOnDemand addons load their full TOC, while LoadOnDemand addons execute only annotated bootstrap files without marking the addon loaded and without firing `ADDON_LOADED`. This matches the pattern where LoD addons expose tiny public loader globals before `VARIABLES_LOADED`, while the full addon body still loads later on demand.

`AddonContext` holds `name`, private Lua `table`, and `addon_root`. Per-file process:
1. Check local overlay at `./Interface/AddOns/{addon}/{file}` first, fall back to addon root
2. `.lua` → `load_lua_file()`: strip BOM, transform path to `@Interface/AddOns/...` for debugstack, execute with `(addonName, addonTable)` varargs
3. `.xml` → `load_xml_file()`: parse with quick_xml, dispatch elements (Script/Include → load file; Font/FontFamily → create font object; ScopedModifier → recurse; frames → `create_frame_from_xml()`)
4. After each `.lua` file: inject C++ mixin stubs (empty `ModelSceneControlButtonMixin.OnLoad`, etc.)

`LoadResult` includes per-addon timing breakdown: `io_time`, `xml_parse_time`, `lua_exec_time`, `saved_vars_time`.

## Blizzard Addon Load Order

Retail and PTR Blizzard addons are discovered from the active profile cache, filtered by screen/profile metadata, and topologically sorted from TOC dependencies plus simulator implicit startup dependencies. Foundational SharedXML addons are promoted to `LoadFirst` so templates exist before other Blizzard addons instantiate frames. Older docs referred to a 27-addon hardcoded retail list; current runtime loading is discovery-based.

Third-party addons load after the Blizzard startup pass. Third-party LoadOnDemand addons with `[Bootstrap]` entries are registered during discovery and execute those bootstrap files in the same sorted startup order; non-LoadOnDemand third-party addons execute `[Bootstrap]` inline with normal files.

The non-retail profiles discover different addon sets:

- **Wrath** ships its UI as a flat `Interface/FrameXML/` tree alongside `Interface/AddOns/`, with no `Blizzard_FrameXML` addon. The loader detects this via `client_profile::blizzard_ui_framexml_toc()` (Some → wrath layout, None → addon layout) and synthesizes a virtual `FrameXML` addon that loads before the regular `Blizzard_*` discovery pass. Wrath ships 24 `Blizzard_*` addons + the synthetic FrameXML.
- **Mists** ships ~112 `Blizzard_*` addons (most retail addons exist with `_Mists.toc` variants).
- **Era / Anniversary** ship the `Gethe/wow-ui-source` multi-flavor addon set (Era uses ~35 `Blizzard_*_Vanilla.toc` variants; Anniversary uses the same vanilla TOCs against a 2.5.5 build).

Counts above are the discovered set after filtering by `is_allowed_game_type` and `default_enabled`; the on-disk addon directory may carry many more `_Cata.toc` / `_TBC.toc` / etc. variants the active profile skips.

## Third-Party Addon Enable State

Third-party addon state starts from the real character `WTF/Account/{account}/{realm}/{character}/AddOns.txt` when WTF import is enabled. When the `ServerSnapshot` addon has written `ServerSnapshotDB`, the captured `characters[...].addons.entries[*].enabled` values overlay the file state before third-party addon loading. This is the preferred live-client source because the AddOn List UI can expose per-character/dependency state that is not reliably represented by `AddOns.txt` rows alone.

The simulator-local `~/.local/share/wow-sim/AddOns.txt` remains a compatibility overlay after the real WTF file. Avoid treating it as the primary source for real-client state; it is a simulator UI save artifact, not a live-client snapshot.

The effective state pass is dependency-aware. Required dependencies can be enabled for explicitly enabled addons, but addons whose required dependency is explicitly disabled are disabled even when the dependent addon's own TOC default is enabled. This prevents data-shard addons such as RaiderIO DB modules from loading without their required base addon.

Third-party addon metadata is registered in `C_AddOns` for the full discovered, dependency-sorted addon list before any eager third-party Lua executes. The loader then runs only enabled, non-`LoadOnDemand` addons in order. This matches addons that inspect TOC metadata during file load: for example, `!BugGrabber` scans `C_AddOns.GetNumAddOns()` and `GetAddOnMetadata(i, "X-BugGrabber-Display")` before `BugSack` Lua runs, so `BugSack` must already be visible as an enabled addon with its display metadata even though its files load later.

## XML Element Handlers

| Category | Elements |
|----------|----------|
| File refs | Script, Include (case-insensitive variants) |
| Frames | Frame, Button, CheckButton, EditBox, ScrollFrame, Slider, StatusBar, GameTooltip, Model, ModelScene, MessageFrame, Minimap |
| Regions | Texture, FontString |
| Containers | ScopedModifier |
| Fonts | Font, FontFamily |

Frame creation generates and executes a Lua string: `CreateFrame(...)` + configuration calls + `SetScript(...)`. Template application runs automatically when `inherits` is specified.

## SavedVariables (`src/saved_variables.rs`)

Priority: WTF loading (`WTF/Account/{account}/SavedVariables/{addon}.lua` and per-character variant) then JSON fallback stored in `~/.local/share/wow-sim/SavedVariables/`. `SavedVariablesManager` tracks registered variables per addon, loads before Lua execution, and persists after shutdown.

## Startup Sequence

1. Create `WowLuaEnv`, set paths, configure SavedVariables, import account `bindings-cache.wtf`, and import `ServerSnapshotDB` action bars/keybindings when present
2. Load Blizzard startup addons in dependency order (full load for non-LoD, bootstrap-only for LoD addons with `[Bootstrap]` files)
3. Load third-party startup addons alphabetically/dependency-sorted, overlaying `ServerSnapshotDB` addon states when present (same full-load vs bootstrap-only split)
4. Apply post-load workarounds (UpdateMicroButtons stub, WorldMapFrame scroll init, etc.)
5. Fire startup events: `ADDON_LOADED("WoWUISim")`, `VARIABLES_LOADED`, `PLAYER_LOGIN`, `PLAYER_ENTERING_WORLD(true, false)`, `UPDATE_BINDINGS`, `DISPLAY_SIZE_CHANGED`, `UI_SCALE_CHANGED`
6. Launch GUI, dump frame tree, or render screenshot

## Error Handling

`LoadError` wraps Io, Toc, Xml, Lua variants. Non-fatal issues accumulate in `LoadResult.warnings`. Path resolution tries four strategies (case-sensitive/insensitive × relative-to-xml/addon-root) before failing.

## Sources

- [addon-loading-pipeline.md](../../addon-loading-pipeline.md) — TOC parsing, load flow, XML handlers, SavedVariables, load order
- [addon_loading.rs](../../../src/bin/wow_sim/addon_loading.rs) — Blizzard eager load, Blizzard bootstrap pass hook, third-party addon discovery, metadata pre-registration, enable-state application, and load loop
- [toc/mod.rs](../../../src/toc/mod.rs) — TOC metadata, normal file list, `[Bootstrap]` parsing, path resolution
- [loader/addon.rs](../../../src/loader/addon.rs) — normal addon file execution and bootstrap-only file execution
- [blizzard-ui-files.txt](../../../data/blizzard-ui-files.txt) — committed Blizzard UI cache manifest, including bootstrap files needed by the active profile cache
- [blizzard_ui_sync.rs](../../../src/blizzard_ui_sync.rs) and [profile_cache.rs](../../../src/blizzard_ui_sync/profile_cache.rs) — profile-filtered cache sync and repo-fallback handling for manifest entries

## See Also

- [[client-profiles]] — cargo-feature selection, vendor pinning, profile-aware loader paths and gametypes
- [[xml-template-system]] — XML parsing and template registry populated during loading
- [[event-system]] — startup events fired after all addons load
- [[lua-api]] — WowLuaEnv that executes all Lua files
