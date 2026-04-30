# Addon Loading

The addon loading pipeline discovers addon directories, parses TOC files, loads Lua and XML files in declared order, applies templates, fires startup events, and initializes SavedVariables.

## Addon Discovery and TOC Parsing

`find_toc_file()` (`src/loader/mod.rs`) prefers `{AddonName}<suffix>.toc` (suffix is profile-dependent: `_Mainline` retail, `_Wrath` wrath, `_Mists` mists, `_Vanilla` era + anniversary) over `{AddonName}.toc` over the first `.toc` whose name doesn't carry another profile's suffix. The suffix table and exclude list live in `active_profile_toc_suffix()` / `other_profile_toc_suffixes()`. See [[client-profiles]].

`TocFile` fields: `addon_dir`, `name`, `metadata: HashMap<String, String>` (Interface version, Title, Dependencies, RequiredDeps, OptionalDeps, LoadOnDemand, SavedVariables, SavedVariablesPerCharacter), `files: Vec<PathBuf>` in load order.

TOC parsing strips `#` comments, skips `[AllowLoadTextLocale]` lines for non-enUS, skips `[AllowLoadGameType]` lines unless the inline gametype matches the active profile's allow-list (retail: mainline/standard; wrath: wrath/wrath_classic/classic; mists: mists/mists_classic/classic; era: vanilla/classic_era/classic; anniversary: vanilla/classic_anniversary/classic), substitutes `[Family]` for the profile's family-subdir (Mainline retail, Classic everywhere else), `[Game]` → "Standard", normalizes backslashes. Path resolution is case-insensitive for Windows/macOS compatibility. See [[client-profiles]] for the full profile-aware tables.

## Load Flow (`src/loader/addon.rs`)

`AddonContext` holds `name`, private Lua `table`, and `addon_root`. Per-file process:
1. Check local overlay at `./Interface/AddOns/{addon}/{file}` first, fall back to addon root
2. `.lua` → `load_lua_file()`: strip BOM, transform path to `@Interface/AddOns/...` for debugstack, execute with `(addonName, addonTable)` varargs
3. `.xml` → `load_xml_file()`: parse with quick_xml, dispatch elements (Script/Include → load file; Font/FontFamily → create font object; ScopedModifier → recurse; frames → `create_frame_from_xml()`)
4. After each `.lua` file: inject C++ mixin stubs (empty `ModelSceneControlButtonMixin.OnLoad`, etc.)

`LoadResult` includes per-addon timing breakdown: `io_time`, `xml_parse_time`, `lua_exec_time`, `saved_vars_time`.

## Blizzard Addon Load Order

27 addons in hardcoded dependency order (`src/main.rs`):

- **Foundation**: SharedXMLBase → Colors → SharedXML → SharedXMLGame → UIPanelTemplates → FrameXMLBase
- **Core**: LoadLocale → Fonts_Shared → HelpPlate → AccessibilityTemplates → ObjectAPI → UIParent → TextStatusBar → ... → FrameXML
- **UI Panels**: UIPanels_Game → MapCanvas → WorldMap → ActionBar → GameMenu → UIWidgets → Minimap → AddOnList → Communities

Third-party addons loaded alphabetically after Blizzard addons.

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

1. Create `WowLuaEnv`, set paths, configure SavedVariables
2. Load 27 Blizzard addons in dependency order
3. Load third-party addons alphabetically
4. Apply post-load workarounds (UpdateMicroButtons stub, WorldMapFrame scroll init, etc.)
5. Fire startup events: `ADDON_LOADED("WoWUISim")`, `VARIABLES_LOADED`, `PLAYER_LOGIN`, `PLAYER_ENTERING_WORLD(true, false)`, `UPDATE_BINDINGS`, `DISPLAY_SIZE_CHANGED`, `UI_SCALE_CHANGED`
6. Launch GUI, dump frame tree, or render screenshot

## Error Handling

`LoadError` wraps Io, Toc, Xml, Lua variants. Non-fatal issues accumulate in `LoadResult.warnings`. Path resolution tries four strategies (case-sensitive/insensitive × relative-to-xml/addon-root) before failing.

## Sources

- [addon-loading-pipeline.md](../../addon-loading-pipeline.md) — TOC parsing, load flow, XML handlers, SavedVariables, load order

## See Also

- [[xml-template-system]] — XML parsing and template registry populated during loading
- [[event-system]] — startup events fired after all addons load
- [[lua-api]] — WowLuaEnv that executes all Lua files
