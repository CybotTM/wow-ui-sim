# Blizzard UI Manifest Drift

`data/blizzard-ui-files/ptr.txt` had drifted to 310 addon directories against the 348 the Gethe `ptr` branch ships, so 39 directories were never synced into the runtime cache. Every symbol they define read as nil, and because 25 of those call sites sit at file scope, each of them silently truncated its own Blizzard file.

## Content

Symptoms:

- `ChatFrame1Background` rendered as an opaque white box where the client draws it black at 25%, even though `GetChatWindowInfo(1)` correctly reported `r=0 g=0 b=0 alpha=0.25`.
- `ChatFrame1:IsEventRegistered("UPDATE_CHAT_WINDOWS")` was false and `CHAT_FRAMES` did not contain `ChatFrame1`, so `FloatingChatFrame_Update` never applied the window colour or alpha.
- `EditModeManagerFrameMixin` held 4 methods where `EditModeManager.lua` defines 138.
- `ManagedFrameMixin` was nil at nine call sites; `StatusTrayManager`, `UIModeUtil`, `ItemButtonUtil`, `CommunitiesUtil`, `PartyUtil` and `ArenaUtil` were nil too.
- 659 startup Lua errors across 56 distinct messages.

Root cause:

`tools/gen_blizzard_ui_manifest.py` documents each per-profile manifest as "the *full* file list of the matching Gethe branch's `Interface/AddOns` tree — no filtering". `ptr.txt` no longer was: 3999 lines covering 310 addon directories, against 4041 lines and 348 directories in the branch. The 39 missing directories include `Blizzard_ManagedFrameSystem` (named by twelve `## Dependencies:` lines in TOCs that *were* synced), `Blizzard_StatusTrayManager`, `Blizzard_GameMenuEsc`, `Blizzard_UIModes`, `Blizzard_UIErrorsFrame`, `Blizzard_ColorPickerFrame`, `Blizzard_ChatBubble` and `Blizzard_TalentUI`.

`Blizzard_GameMenuEsc/Blizzard_GameMenuEsc.lua` is the costly one. It defines `GameMenuEscPriority` and `RegisterGameMenuEscHandler`, and it also redefines `ToggleGameMenu` to walk the registered handlers. Twenty-five Blizzard files call `RegisterGameMenuEscHandler` at FILE SCOPE, so without the addon each of them raised there and lost every definition below that line. `EditModeManager.lua` calls it on line 21, two lines before the first `EditModeManagerFrameMixin` method, so `RegisterSystemFrame` did not exist and `EditModeSystemMixin:OnSystemLoad` raised for every frame inheriting `EditModeSystemTemplate` — `ChatFrame1` among them, which is why the chat frame never reached its `RegisterEvent("UPDATE_CHAT_WINDOWS")`.

A second gate sits behind the manifest. `wow-cli casc sync-blizzard-ui` resolves each manifest path to a fileDataID through the bundled `data/wow-ui-sim-listfile.csv`; the new addons had no entry there, so extraction fell back to a CASC path lookup, which fails for files whose root record carries no name hash (`Path not found in root file`). Both artifacts have to be regenerated, manifest first.

Fix:

- `python3 tools/gen_blizzard_ui_manifest.py ptr` — 3999 to 4041 lines, 310 to 348 addon directories.
- `python3 tools/gen_limited_listfile.py --source <community-listfile.csv>` — 145251 to 145721 rows, from the wowdev community listfile release `202608301946`.
- `wow-cli casc sync-blizzard-ui` then completes for the first time: 403 files extracted, 0 missing, exit 0.

Separately, fourteen enum gaps that only became reachable once the aborts cleared are filled from `Blizzard_APIDocumentationGenerated`. Those are simulator-owned enum tables, not addon content, so the manifest fix does not cover them.

Three are edit-mode enums: `EditModeLossOfControlSetting` was absent; `EditModeSystem` was missing `RaidWarning` and `LossOfControl`, which also numbered `TotemActionBar` 24 instead of 25; `DamageMeterVisibility` was missing `InGroup`.

Ten more surfaced as execution reached further: `SecondsFormatterRounding`, `CooldownViewerSound` (94 values), `RecentAlliesFriendTag`, `BattleNetFriendLevel` (starts at 1), `BattleNetFriendTag`, `RaidDispelOverlayType`, `SocialSystemType`, `SocialUIPresenceType`, `SocialUIBlockType` and `VisualAlertType` (starts at 1). Three of those cascade: `SocialUIUtil.lua` aborted first on `SocialUIPresenceType` and then on `SocialUIBlockType`, which is why `SocialUIScrollableElementExtentPreviewerMixin` read as nil at three call sites in Blizzard_FriendsFrame and Blizzard_SocialUI even though Blizzard ships it in that same file.

Measured on `client-ptr` with `--no-addons --no-saved-vars`:

| | before | after |
| --- | --- | --- |
| addon directories in the cache | 310 | 348 |
| `sync-blizzard-ui` | 20 files missing, exit 1 | 403 extracted, 0 missing, exit 0 |
| startup Lua errors | 659 | 26 |
| distinct error messages | 56 | 4 |
| `EditModeManagerFrameMixin` methods | 4 | 138 |
| `ChatFrame1Background` alpha / vertex | 1.0 / 1,1,1 | 0.25 / 0,0,0 |
| `BottomManagedFrameContainer` | absent | anchored, `513,45 573x0` |
| `RightManagedFrameContainer` | absent | anchored, `1595,92 0x847` |
| edit-mode `registeredSystemFrames` | 0 | 47 |
| `PlayerFrame` anchor points | 0 | 1 |

Two leads worth recording as refuted, because both cost time:

The `ptr` profile's Blizzard UI cache was suspected of holding stale 12.0.7 source, since the profile is named for a PTR while 12.1.0 has shipped live. It does not. A forced re-extraction against a live `12.1.0.69497` install produced 3388 files byte-identical to the existing cache. Note that `sync-blizzard-ui` treats an existing file as a cache hit and does not re-extract it, so a plain re-run cannot show this either way — the directory has to be moved aside first.

Pointing the simulator at the real `_retail_/WTF` via `WOW_SIM_WTF_PATH` looks like an improvement and is not. It does import the character's Edit Mode layout, but that layout arrives with **0 systems** and replaces the simulator's preset layout, which has 52. Measured: with the override `PlayerFrame` and `BuffFrame` have 0 anchor points and do not render at all; without it they have 1 each and appear where the client puts them. The simulator's `Ptr` profile also looks for WTF only under `_ptr_/WTF` (`src/paths.rs`, guarded by `ptr_wtf_candidates_use_ptr_install_flavor_only`), so on a live-only install it finds nothing by default — which is the better outcome here.

## Defects the restore uncovered

Clearing the aborts let execution reach code that had never run, which surfaced five further defects. Each is its own commit and each was measured, not inferred.

**Mask coverage was decided by file name.** WoW ships two mask encodings — coverage in RGB with uniformly opaque alpha, or coverage in alpha with black RGB — and the path does not say which. `mask_coverage_for_path` matched only `alphamask` and `uiactionbariconframemask`, so every other alpha-coverage mask took the RGB branch, where `color.a *= mask.a * max(mask.rgb)` is zero everywhere. Unit-frame portraits rendered as empty rings for that reason and no other: `PlayerPortrait` carried fileDataID 237669 with the Paladin cell's texcoords the whole time, and removing the mask in an otherwise identical screenshot run makes it appear. A survey of the 77 mask BLPs in the extract cache found four that genuinely need the RGB rule and roughly eighteen on the broken branch, including every player and party health and mana bar fill, the class resource bar and the casting bar. `TextureEntry` now records whether every pixel's alpha is 255 and `resolve_mask_requests` sets or clears the flag from that.

**`Blizzard_FrameXML` loaded from the legacy flavor TOC.** `find_toc_file` prefers `<addon>_Mainline.toc`, which for FrameXML lists only the `.xml` half of five pairs whose XML carries no `<Script file=...>` include. Six startup errors traced to that one choice; correcting it took errors from 237 to 34. Not generalised: of the sixteen addons shipping both variants most have a *fuller* flavor TOC, so a blanket flip loses more than it fixes.

**Permanent auras rendered a backwards countdown.** `seed_buff_durations` computed `expirationTime - GetTime()` for auras reporting `duration == 0` and `expirationTime == 0`, displaying "-2 s". Blizzard gates the same computation on both fields being positive (BuffFrame.lua:812).

**`C_StringUtil` was rebuilt after the workaround layer populated it.** `register_c_string_util` assigned a fresh table over the global, and it runs a second time from `environment_cleanup_restore` — after `C_StringUtil.CreateSecondsFormatter` is installed. `ensure_global_table` keeps the existing namespace. The same replace-don't-merge shape may exist at other `src/c_api/` registration sites; not surveyed.

**The screenshot command encoded at WebP quality 15** and forced the extension, smearing exactly the small text and one-pixel borders a UI capture exists to show. `.png` is honoured losslessly now and WebP takes `--quality`.

**The minimap terrain sat inside a ring of empty space.** The terrain quad is the full 198×198 frame; the mask shrank it. `build_minimap_quads` stretched the built-in `Interface\HUD\UIMinimapMask` over the frame bounds, and decoding that asset (256×256, BLP2 uncompressed BGRA, alpha uniformly 255) shows its opaque disc at x 27..229 / y 20..229 — 203×210, off-centre on the canvas. Stretched over the frame, the disc covers ~79% of it. Nothing in Blizzard Lua or XML declares this mask; the client applies it C-side, and a 203px disc against a 198px frame is authored to cover the frame edge to edge. The mask rectangle is now the frame expanded so the disc maps onto it (`default_minimap_mask_rect`); addon masks set through `SetMaskTexture` keep the stretched behaviour. Measured in the annulus between radius 80 and 118 where the gap sat: dark backdrop pixels 58.9% → 2.9%.

**Two things that looked like defects and are not.** The row of glyph-like shapes under the player frame is the Paladin Holy Power bar at 0/5 (`PaladinPowerBarFrame`, atlas `uf-holypower-runeholder`), rendered pixel-correctly. The yellow halo around objective-tracker quest icons is `POIButton.xml`'s deliberate `Glow` texture with `alphaMode="ADD"`; the residual softness of those icons is the 1.25× upscale of a 22px asset, inherent to any UI scale above 1.0.

## Rendering a screenshot that looks like the client

Two settings that are not defaults and are easy to get wrong:

`UIParent` stays at scale 1 and the `uiScale` / `useUiScale` cvars are ignored, so the UI keeps native pixel size and looks small on a large canvas. At 1440p, 1.25 is the largest workable value: `ObjectiveTrackerFrame` is 836 UI units tall and `RightManagedFrameContainer` hangs 260 below the top, so the UI space must be at least 1096 tall. Above that the tracker does not clamp its height, is pushed to the screen top and overlaps the minimap — measured, at scale 1.5 its top sits 260 units above its own anchor target.

`Blizzard_PTRFeedback` ("Issue Reporter") is marked `## OnlyBetaAndPTR: 1` and the client skips it on live builds. `is_ptr_only()` skips it only when the profile is *not* `client-ptr` — and `client-ptr` is what targets 12.1.0, which has since shipped live (`.build.info` reports `12.1.0.69497`, product `wow`). Whether the profile should still claim `IsTestBuild()` is a question for the profile's semantics, not something to change quietly; hide the frame when capturing.

## Sources

- [gen_blizzard_ui_manifest.py](../../tools/gen_blizzard_ui_manifest.py) — states the manifest contract this drifted from
- [gen_limited_listfile.py](../../tools/gen_limited_listfile.py) — the path-to-fileDataID subset
- [blizzard_ui_sync.rs](../../src/blizzard_ui_sync.rs) — `include_str!`s the manifests, so a regen needs a rebuild
- [edit_mode.rs](../../src/lua_api/globals/enum_data/edit_mode.rs) — edit-mode enum definitions

## See Also

- [[addon-loading]] — file-scope errors abort the rest of a Lua file
- [[client-profiles]] — per-profile manifests and vendor pinning
- [[casc-asset-cache]] — the extraction tiers this depends on
