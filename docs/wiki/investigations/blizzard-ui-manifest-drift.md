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

Separately, thirteen enum gaps that only became reachable once the aborts cleared are filled from `Blizzard_APIDocumentationGenerated`. Those are simulator-owned enum tables, not addon content, so the manifest fix does not cover them.

Three are edit-mode enums: `EditModeLossOfControlSetting` was absent; `EditModeSystem` was missing `RaidWarning` and `LossOfControl`, which also numbered `TotemActionBar` 24 instead of 25; `DamageMeterVisibility` was missing `InGroup`.

Ten more surfaced as execution reached further: `SecondsFormatterRounding`, `CooldownViewerSound` (94 values), `RecentAlliesFriendTag`, `BattleNetFriendLevel` (starts at 1), `BattleNetFriendTag`, `RaidDispelOverlayType`, `SocialSystemType`, `SocialUIPresenceType`, `SocialUIBlockType` and `VisualAlertType` (starts at 1). Three of those cascade: `SocialUIUtil.lua` aborted first on `SocialUIPresenceType` and then on `SocialUIBlockType`, which is why `SocialUIScrollableElementExtentPreviewerMixin` read as nil at three call sites in Blizzard_FriendsFrame and Blizzard_SocialUI even though Blizzard ships it in that same file.

Measured on `client-ptr` with `--no-addons --no-saved-vars`:

| | before | after |
| --- | --- | --- |
| addon directories in the cache | 310 | 348 |
| `sync-blizzard-ui` | 20 files missing, exit 1 | 403 extracted, 0 missing, exit 0 |
| startup Lua errors | 659 | 237 |
| distinct error messages | 56 | 12 |
| `EditModeManagerFrameMixin` methods | 4 | 138 |
| `ChatFrame1Background` alpha / vertex | 1.0 / 1,1,1 | 0.25 / 0,0,0 |
| `BottomManagedFrameContainer` | absent | anchored, `513,45 573x0` |
| `RightManagedFrameContainer` | absent | anchored, `1595,92 0x847` |
| edit-mode `registeredSystemFrames` | 0 | 47 |
| `PlayerFrame` anchor points | 0 | 1 |

Two leads worth recording as refuted, because both cost time:

The `ptr` profile's Blizzard UI cache was suspected of holding stale 12.0.7 source, since the profile is named for a PTR while 12.1.0 has shipped live. It does not. A forced re-extraction against a live `12.1.0.69497` install produced 3388 files byte-identical to the existing cache. Note that `sync-blizzard-ui` treats an existing file as a cache hit and does not re-extract it, so a plain re-run cannot show this either way — the directory has to be moved aside first.

Pointing the simulator at the real `_retail_/WTF` via `WOW_SIM_WTF_PATH` looks like an improvement and is not. It does import the character's Edit Mode layout, but that layout arrives with **0 systems** and replaces the simulator's preset layout, which has 52. Measured: with the override `PlayerFrame` and `BuffFrame` have 0 anchor points and do not render at all; without it they have 1 each and appear where the client puts them. The simulator's `Ptr` profile also looks for WTF only under `_ptr_/WTF` (`src/paths.rs`, guarded by `ptr_wtf_candidates_use_ptr_install_flavor_only`), so on a live-only install it finds nothing by default — which is the better outcome here.

## Sources

- [gen_blizzard_ui_manifest.py](../../tools/gen_blizzard_ui_manifest.py) — states the manifest contract this drifted from
- [gen_limited_listfile.py](../../tools/gen_limited_listfile.py) — the path-to-fileDataID subset
- [blizzard_ui_sync.rs](../../src/blizzard_ui_sync.rs) — `include_str!`s the manifests, so a regen needs a rebuild
- [edit_mode.rs](../../src/lua_api/globals/enum_data/edit_mode.rs) — edit-mode enum definitions

## See Also

- [[addon-loading]] — file-scope errors abort the rest of a Lua file
- [[client-profiles]] — per-profile manifests and vendor pinning
- [[casc-asset-cache]] — the extraction tiers this depends on
