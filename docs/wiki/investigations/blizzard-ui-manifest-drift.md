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

The error rows count the `attempt to …` message class, which is what the first pass tracked. A broader count that also takes `table index is nil` and `assertsafe` entries gives 52 occurrences across 11 distinct messages after; those entries were present before as well and are not part of the 659.

Two leads worth recording as refuted, because both cost time:

The `ptr` profile's Blizzard UI cache was suspected of holding stale 12.0.7 source, since the profile is named for a PTR while 12.1.0 has shipped live. It does not. A forced re-extraction against a live `12.1.0.69497` install produced 3388 files byte-identical to the existing cache. Note that `sync-blizzard-ui` treats an existing file as a cache hit and does not re-extract it, so a plain re-run cannot show this either way — the directory has to be moved aside first.

Pointing the simulator at the real `_retail_/WTF` via `WOW_SIM_WTF_PATH` looks like an improvement and is not. It does import the character's Edit Mode layout, but that layout arrives with **0 systems** and replaces the simulator's preset layout, which has 52. Measured: with the override `PlayerFrame` and `BuffFrame` have 0 anchor points and do not render at all; without it they have 1 each and appear where the client puts them. The simulator's `Ptr` profile also looks for WTF only under `_ptr_/WTF` (`src/paths.rs`, guarded by `ptr_wtf_candidates_use_ptr_install_flavor_only`), so on a live-only install it finds nothing by default — which is the better outcome here.

## Defects the restore uncovered

Clearing the aborts let execution reach code that had never run, which surfaced seventeen further defects. Each is its own commit and each was measured, not inferred.

**Mask coverage was decided by file name.** WoW ships two mask encodings — coverage in RGB with uniformly opaque alpha, or coverage in alpha with black RGB — and the path does not say which. `mask_coverage_for_path` matched only `alphamask` and `uiactionbariconframemask`, so every other alpha-coverage mask took the RGB branch, where `color.a *= mask.a * max(mask.rgb)` is zero everywhere. Unit-frame portraits rendered as empty rings for that reason and no other: `PlayerPortrait` carried fileDataID 237669 with the Paladin cell's texcoords the whole time, and removing the mask in an otherwise identical screenshot run makes it appear. A survey of the 77 mask BLPs in the extract cache found four that genuinely need the RGB rule and roughly eighteen on the broken branch, including every player and party health and mana bar fill, the class resource bar and the casting bar. `TextureEntry` now records whether every pixel's alpha is 255 and `resolve_mask_requests` sets or clears the flag from that.

**`Blizzard_FrameXML` loaded from the legacy flavor TOC.** `find_toc_file` prefers `<addon>_Mainline.toc`, which for FrameXML lists only the `.xml` half of five pairs whose XML carries no `<Script file=...>` include. Six startup errors traced to that one choice; correcting it took errors from 237 to 34. Not generalised: of the sixteen addons shipping both variants most have a *fuller* flavor TOC, so a blanket flip loses more than it fixes.

**Permanent auras rendered a backwards countdown.** `seed_buff_durations` computed `expirationTime - GetTime()` for auras reporting `duration == 0` and `expirationTime == 0`, displaying "-2 s". Blizzard gates the same computation on both fields being positive (BuffFrame.lua:812).

**`C_StringUtil` was rebuilt after the workaround layer populated it.** `register_c_string_util` assigned a fresh table over the global, and it runs a second time from `environment_cleanup_restore` — after `C_StringUtil.CreateSecondsFormatter` is installed. `ensure_global_table` keeps the existing namespace. The same replace-don't-merge shape may exist at other `src/c_api/` registration sites; not surveyed.

**The screenshot command encoded at WebP quality 15** and forced the extension, smearing exactly the small text and one-pixel borders a UI capture exists to show. `.png` is honoured losslessly now and WebP takes `--quality`.

**The minimap terrain sat inside a ring of empty space.** The terrain quad is the full 198×198 frame; the mask shrank it. `build_minimap_quads` stretched the built-in `Interface\HUD\UIMinimapMask` over the frame bounds, and decoding that asset (256×256, BLP2 uncompressed BGRA, alpha uniformly 255) shows its opaque disc at x 27..229 / y 20..229 — 203×210, off-centre on the canvas. Stretched over the frame, the disc covers ~79% of it. Nothing in Blizzard Lua or XML declares this mask; the client applies it C-side, and a 203px disc against a 198px frame is authored to cover the frame edge to edge. The mask rectangle is now the frame expanded so the disc maps onto it (`default_minimap_mask_rect`); addon masks set through `SetMaskTexture` keep the stretched behaviour. Measured in the annulus between radius 80 and 118 where the gap sat: dark backdrop pixels 58.9% → 2.9%.

**`Blizzard_SharedXML`'s legacy TOC drops its `Blizzard_Narration` dependency.** Load order placed SharedXML at index 10 and Narration at 182, and template mixins resolve by global name at frame-creation time, so the one slider Blizzard_EditMode builds at index 20 lost `NarrationSliderMixin`. Restored as an implicit startup dependency; flipping SharedXML to its bare TOC instead was measured to abort the Tutorial subsystem (30 → 70 errors) and reverted.

**The `<Shadow>` element of font definitions was dropped.** `FontXml` had no field for it, so every font object built from `Fonts.xml` — `GameFontNormal` included, which inherits `SystemFont_Shadow_Med1` — had no drop shadow, and neither did any FontString using it. The loader now emits `SetShadowOffset` / `SetShadowColor` on all three font-object paths; `GameFontNormal` reports offset (1, −1), colour (0, 0, 0, 1) afterwards. An A/B of startup Lua errors with and without the emission is identical (52 occurrences, 11 distinct messages).

**Headless startup forced the objective tracker's height on every tick.** `normalize_headless_frame_positions` ran `ObjectiveTrackerFrame:SetHeight(836.5)` after each OnUpdate tick, including the three the screenshot command runs after `--exec-lua`. Blizzard sizes the tracker from its managed container (`ObjectiveTrackerContainerMixin:UpdateHeight` → parent height), and `FramePositionDelegate:ManageRightFrameContainer` sizes the container to `UIParent height − MinimapCluster height − 100`. At UI scale 1.6875 the forced 836.5 units are 1411 px on a 1440 px canvas; the frame is clamped to screen, so the renderer pushed it to y=28 over the minimap while Lua's `GetRect` still reported it under the minimap at height 500.8. A `SetHeight` hook showed the write arriving from that chunk three times after the scaled layout. The line is gone; the tracker measures 847.5 at 1600×1200 and 500.8 at 3440×1440 / 1.6875, both from Blizzard's formula.

**The 2x atlas art was never drawn.** The atlas database carries a `-2x` sibling for most HUD art, and the client draws it once a UI unit spans more than one pixel. `get_render_atlas_info` preferred the 2x entry only for a one-name allow-list, so at 1.6875 px per unit `ui-hud-minimap-frame` (215×226 texels) was stretched over 363×381 px and every tracker POI icon and action-bar slot frame went the same way — the soft, frayed look that survived every scale and encoder fix. The render lookup now prefers the paired 2x entry whenever `prefer_hires_atlases` is set, which `wow-sim screenshot --ui-scale` above 1.0 does (`WOW_SIM_HIRES_ATLASES` forces either choice), with the logical size taken from the 1x sibling because the 2x art is not exactly twice the size (438×460 for that frame).

**Vertex colours skipped the sRGB decode.** Atlas samples arrive in the fragment shader linear (sRGB texture formats) and the target re-encodes on write, so texels round-trip unchanged; `in.color` arrived as the sRGB value Lua set and was multiplied in as linear, so it was encoded twice. Measured with the lift off: `SetColorTexture(0.5)` rendered 187 and `0.25` rendered 137 (128 and 64 expected); every solid backdrop, bar fill and tint was on that curve while untinted art was right. The shader now decodes the vertex colour's RGB; the probe renders 127 / 64 afterwards. Text was never affected: the glyph rasteriser bakes the font colour into the linear glyph atlas, and the tracker title measures (244, 196, 51) before and after.

**The texture atlases were sampled with a nearest filter.** `create_texture_sampler` has used `FilterMode::Nearest` since commit `048333a4b` ("Fix tab texture sampling seams"). Above 1 px per unit every magnified texel becomes a hard block: quest icons in the objective tracker and the minimap's compass marks rendered as stair-stepped pixel doubling at 1.6875, where the client is smooth. The sampler is bilinear again (the glyph sampler always was). The character-frame tabs the nearest filter was chosen for keep faint brightness steps at their three-slice joins with either filter, and enabling the half-texel UV inset for those crops changes nothing about them, so that exclusion stays. A headless test draws a 2×2 checker at 16×16 and requires a ramp of intermediate values; with Nearest it reads `[0 ×8, 255 ×8]`.

**Two more file-scope aborts on a nil table key.** `Blizzard_FrameXMLBase/Constants.lua:497` indexes `LFG_CATEGORY_NAMES` by `LE_LFG_CATEGORY_LAIR`, the category 12.1.0 adds; the simulator's table stopped at `BATTLEFIELD = 7`, so the file died there and `QUEST_TAG_ATLAS` (line 529) never existed — opening the world map then raised at `QuestUtils.lua:639`. The LE_ family is a 1-based ordinal with no documented value; LAIR is 8, the next slot, marked as such. `Blizzard_RecentAllies/Blizzard_RecentAlliesUtil.lua:112` indexes by `Enum.RolodexType.LegacyFriend`, which `RolodexConstantsDocumentation.lua` gives as 23 with 21 and 22 absent; the enum moved to the explicit-value table. The `LAIR` global string itself was missing too; a nil value does not abort a file, only a nil key does, and the string arrived with the table regeneration below.

**`Blizzard_SharedXML`'s legacy TOC omits `ListTemplates`.** `find_toc_file` prefers `Blizzard_SharedXML_Mainline.toc`, which lacks `ListTemplates.lua` / `.xml` (bare TOC lines 233–234); nothing else defines `ListHeaderMixin`, so ten files inheriting `ListHeaderThreeSliceTemplate` rendered bare headers and `QuestMapFrame.lua:417` raised when the world map opened. Since flipping the addon to its bare TOC aborts the Tutorial subsystem (30 → 70 errors, measured earlier), the loader appends just that pair after the picked TOC's files. Together with the two constants: `lua-errors` on client-ptr goes from 33 distinct / 39 occurrences to 25 / 29 with nothing new, and six of the seven `render_order_world_map` tests pass where the five that open the map all failed before.

**The global string table was a client build behind, and its generator cut multi-line strings.** `data/global_strings.rs` held 25229 strings from an older build. The 12.1.0 UI reads 16 that were not in it: `LAIR`, and the `SLASH_CAA_*` set that `TextToSpeechCommands.lua` formats inside file-scope do-blocks — line 652 raised on `SLASH_CAA_HELP_SAY_COMBAT_START_SOUND:format(...)` and the twenty command blocks below it never registered. Separately, `gen_global_strings` iterated the CSV with `lines()`, but 36 strings span lines inside a quoted field (`COMMUNITY_MEMBER_LIST_CROSS_FACTION`, `PROFESSIONS_HELP_2`, the crafting-order tutorials); each kept only its first line, so the table carried `"%s|r"` for a two-line string. `csv_util::read_csv_records` already joins such records and three other generators use it; the strings generator does now. Regenerated from the wago.tools GlobalStrings export of build 12.1.0.69497 (26066 rows; 868 added, 165 changed, 33 dropped — all WOWHACK/OUTFITTER, which the 12.1.0 UI names once as a table value in `GameRulesUtil.lua:32`, nil in the client as well). The table is shared by every profile, so the retail profile sees the 12.1.0 wording too; its UI references none of the dropped keys beyond that same line. `lua-errors` on client-ptr: 25 / 29 → 22 / 26; gone are the TextToSpeech abort and the "Not all BattleNetFriendTags have a label defined" assertion, whose labels were among the added strings. The generator's input path `~/Projects/wow/data/GlobalStrings.csv` did not exist on this machine and holds that export now.

**`SetFontObject` snapshotted the loader's stale font fields.** A font object carries two field sets: `__font` / `__height` / `__outline` written by the XML loader (with `__height = 12.0` as a placeholder for a `<Font>` that inherits its size) and `__fontPath` / `__fontHeight` / `__fontFlags` kept by `SetFont` and read by `GetFont`. The snapshot preferred the loader set, so `GameFontNormalSmall` answered 10 to `GetFont()` while every FontString on it rendered at 12 — `PlayerName` and `PlayerLevelText` among them, which is why the level number sat out of line in a 15-unit box. The snapshot now prefers the live set.

**Text shadows were drawn above the glyphs.** `SetShadowOffset(1, -1)` is one unit right and one DOWN (WoW's y points up); the quad builders passed the pair through in screen space (y down) and unscaled, so the shadow sat one pixel over the ascenders and nothing under the baseline — measured at 1.6875: glyph rows 12..30, shadow rows 11..15. The offset is flipped and scaled now: shadow rows 13..30. A render test with a (2, −2) shadow pins the direction.

**The world-map strip test was watching the first-run tutorial.** `WorldMapTutorialMixin:CheckAndShowTooltip` shows the HelpPlate tooltip above the map's "?" button the first time the map opens and then sets `closedInfoFrames` bit 14; startup in the full simulator has already flipped it, the isolated test environment had not, so the tooltip landed in the strip the test watches. The test closes the tutorial before opening the map.

**The brightness lift is a display-side offset in the reference, not a UI property — inferred, not proven.** With `WOW_SIM_BRIGHTNESS_BOOST=0` the simulator reproduces atlas texels 1:1 — the action bar's gryphon end cap, aligned pixel-exactly in a client capture and a simulator render by masked FFT correlation (both at (1085,1236) on a 3440×1440 canvas at 1.6875), renders source luminance 50 → 53, 90 → 90, 145 → 139. The client capture shows the same texels at 93, 136 and 194: a near-constant offset of about +45 across all bins. Two more observations point the same way: no pixel anywhere in the capture is darker than luminance 31 (text outlines and slot shadows included; 0 pixels below 20 in the action bar, tracker and minimap regions), and the nominal `NORMAL_FONT_COLOR` yellow (255, 209, 0) appears as (255, 255, 31). A gamma curve would leave black at black; an offset lifts it. The likely source is the client's Brightness / Contrast / Gamma display settings, which the capture's owner can confirm. The historical `pow(rgb, 1/1.5)` lift happens to land within 4 luminance points of that capture in the midtones and 25 short in the highlights; it stays the default as an on-screen aid, and a capture meant to be compared texel-for-texel wants it off.

**Three things that looked like defects and are not.** The row of glyph-like shapes under the player frame is the Paladin Holy Power bar at 0/5 (`PaladinPowerBarFrame`, atlas `uf-holypower-runeholder`), rendered pixel-correctly. The yellow halo around objective-tracker quest icons is `POIButton.xml`'s deliberate `Glow` texture with `alphaMode="ADD"`; the residual softness of those icons is the upscale of a 22px asset, inherent to any UI scale above 1.0. `PlayerFrame` at `BOTTOMRIGHT UIParent BOTTOM −300 250` (bottom centre-left) at every scale is the preset edit-mode layout the simulator ships; a capture from a client with a custom layout will place it elsewhere, and the WTF import currently brings such a layout in with 0 systems (see above).

## Rendering a screenshot that looks like the client

The client renders one UI unit as `height / 768 × uiScale` pixels; a 3440×1440 capture at uiScale 0.9 is 1.6875 px per unit. The simulator ignores the `uiScale` / `useUiScale` cvars and leaves `UIParent` at scale 1, so the UI keeps native pixel size and looks small on a large canvas. `wow-sim screenshot --ui-scale 1.6875` applies the scale where it has to go: between startup settle and `set_screen_size`. That call fires `DISPLAY_SIZE_CHANGED` / `UI_SCALE_CHANGED` and replays the edit-mode anchor-changed hooks, which is the pass that re-runs `ManageFramePositions` and sizes the managed containers for the scaled `UIParent`. Scaling from `--exec-lua` instead runs after that pass — measured, `RightManagedFrameContainer` keeps the 1087.5 computed for scale 1 on a UI space 853 tall, and the tracker inherits it. Text is rasterised at `font_size × effective_scale` (`quad_builders.rs`), so glyphs sharpen with the scale rather than being upscaled bitmaps.

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
