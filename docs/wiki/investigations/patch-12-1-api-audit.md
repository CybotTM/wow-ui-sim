# Patch 12.1 API Audit

Patch 12.1 API surface work in wow-ui-sim is split between compatible bridges that can be safely modeled as inert/additive simulator behavior and paused items that require exact Blizzard PTR observations before implementation.

## Content

The per-item machine SSOT is `data/patch-api/12.1-framexml.json`; [[patch-api-audit-manifest]] documents validation and checklist generation. Draft `untriaged` resolutions remain completion blockers and are not approved exceptions.

### Completed compatible bridge work

The 12.1 compatibility work is currently captured by these commits:

- `856bd7c6d` — bridged the first 12.1 PTR API surface pass.
- `088be550c` — fixed 12.1 bridge edge cases.
- `390086d31` — hid removed 12.1 PTR symbols.
- `bda32ebd4` — gated 12.1 API surface behind cumulative retail epoch features.
- `ed34635c5` — moved strict 12.1 removals after startup so Blizzard UI can still load current EditMode code.
- `16b7d85d6` — modeled 12.1 forbidden aspect inheritance for compatible frame/object behavior.
- `4b5fc502d` — bridged remaining inert social, Discord, Battle.net title-friend, encounter-journal, and housing/blueprint probes.
- `85c2b11d3` — added 12.1 `DurationTextBinding` color-curve compatibility methods on the table returned by `C_DurationUtil.CreateDurationTextBinding`.
- `15f4ecc18` — modeled 12.1 Battle.net title-friend custom names/tags as best-effort per-friend metadata on `SimState.bnet_friends`.
- `1211024ce` — modeled 12.1 Encounter Journal difficulty helpers from generated instance data (`is_raid` → base/valid difficulty guesses).
- `aa889bd7f` — modeled `C_Discord.IsEnabled` from the existing `discordClientEnabled` CVar while leaving the rest of Discord as inert service placeholders.
- `78f8053ec` — modeled pending Battle.net friend invites with `SendVerifiedBattleNetFriendInvite` and `GetFriendInviteInfo` backed by `SimState.bnet_friend_invites`.
- `106a6617e` — made Battle.net feature probes return true for the modeled friend-list/title-friend/tag surfaces.
- `4728e37ab` — modeled `C_Housing` owned-house/plot probes and `ResetHouse` against local `SimState.housing` flags and favor state.
- `ec807facb` — modeled safe `C_HousingBlueprint` share-code/import/export calls against local `SimState.housing` blueprint state.
- `96299bb1b` — modeled safe housing editor/customize/decor/layout probes against local `SimState.housing` state and removed their Lua inert defaults.
- `fdcdd4c62` — modeled Battle.net appear-offline intent on `SimState.bnet_appear_offline` and moved the title-friend unit invite probe out of Lua inert defaults.
- `1701c1c4e` — modeled Discord OAuth/link/settings/server/channel probes against local `SimState.discord` state and removed the final 12.1 Lua inert defaults.
- `d91e8a342` — modeled housing blueprint availability probes as local `SimState.housing` result codes.

Key implementation locations:

- `Cargo.toml`, `src/client_profile.rs` — `retail-12-1-0` epoch feature and interface-version mapping.
- `src/ptr/compat_bootstrap.rs`, `src/ptr/strict_removals.lua` — 12.1 compatibility bootstrap and post-startup strict removals.
- `src/lua_api/workarounds/temporary/patch_12_1_inert_defaults.rs` — temporary inert defaults for additive 12.1 APIs without a backing simulator model.
- `src/loader/tests/wow_api_globals/startup_globals.rs` — safe bridge and strict removal regression coverage.
- `src/loader/tests/wow_api_globals/frames_and_attributes.rs`, `src/loader/tests/xml_basics_extra.rs`, `src/loader/tests/runtime_templates.rs`, `src/loader/tests/runtime_template_misc.rs` — 12.1 widget/XML/private-partition behavior coverage.

Verified proof logs for the final compatible bridge pass:

- `/tmp/wow_12_1_inert_full_fmt-check.out`
- `/tmp/wow_12_1_inert_full_check-default.out`
- `/tmp/wow_12_1_inert_full_check-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_check-ptr.out`
- `/tmp/wow_12_1_inert_full_test-safe-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_test-safe-ptr.out`
- `/tmp/wow_12_1_inert_full_test-strict-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_build-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_lua-retail-12-1.out`

Rust readability metrics for the final bridge are under `/tmp/rust_readability_12_1_inert` with no high-complexity findings. The `LoadAddOnWithErrorHandling` wrapper regression proof is `/tmp/pi-pyrun-12-1-load-addon-wrapper-test.log`; readability output is `/tmp/rust_readability_12_1_load_addon_wrapper`.

### Implementation matrix

| Area | Current state |
|------|---------------|
| Global added APIs | Present under `retail-12-1-0` as Rust-backed APIs or compatibility bridges. `src/lua_api/workarounds/temporary/patch_12_1_inert_defaults.rs` now contains no active Lua inert defaults. Rough source scan has no missing global-added names. `LoadAddOnWithErrorHandling` is explicitly modeled as the tested canonical wrapper over `UIParentLoadAddOn`. |
| Removed APIs | Hidden for addon-facing checks after startup by `src/ptr/strict_removals.lua`; not moved earlier because current Blizzard UI still needs some load-time compatibility. |
| Events/CVars/Enums | Added event/CVar/enum names are gated by the 12.1 retail epoch where implemented. |
| Widget methods | Compatible 12.1 widget methods are implemented/tested: forbidden-aspect queries, texture radial-progress-bar methods, roleset/on-update mode methods, statusbar render mode, minimap icon scale, VectorGraphics/SVG stubs. |
| Private/forbidden XML partition mechanics | Compatible XML/private table behavior is implemented/tested for `useForbiddenObjectTable`, private KeyValues, partition-aware mixins, and secure delegates. |
| `DurationTextBinding` color methods | Implemented as compatibility methods on `C_DurationUtil.CreateDurationTextBinding(...)`: `ClearTextColorCurve`, `GetFormattedTextColor`, `GetTextColorCurve`, `SetTextColorCurve`. |
| Housing owned-house probes | Best-effort modeled on local `SimState.housing` flags: `IsInsideOwnedHouse`, `IsInsideOwnedPlot`, and `IsInsideOwnedHouseOrPlot` reflect test/service-seeded state; `ResetHouse` clears the local owned-location and favor-bar state. Exact service payloads and zone transitions still need PTR probes. |
| Housing blueprint APIs | Best-effort modeled for safe local flows: export calls return deterministic `wow-ui-sim:*` share codes, validity/type/hyperlink helpers recognize those codes, import/delete/rename/request calls record intent in `SimState.housing`, `CanImportTypeFromCurrentLocation` reflects the owned-house/plot state, and export/feature/import availability helpers return local `SimState.housing` result codes (`0` by default). Exact code format, availability enums, server collection payloads, and import validation still need PTR/service probes before fidelity claims. |
| Housing editor/customize/decor/layout APIs | Best-effort modeled for safe local state: `C_HouseEditor.GetHouseEditorPlayerType`, selected decor-pet APIs, decor room/pet/budget probes, and layout room/floorplan probes read or record `SimState.housing`. Exact service payloads, enum values, budget semantics, floorplan structures, and decorator validation still need PTR/service probes. |
| Discord APIs | Best-effort modeled on local simulator state: `IsEnabled` reflects `discordClientEnabled`; `Authorize`, `RefreshAuth`, `GuildLink`, `GuildUnlink`, `SetGuildSetting`, `UpdateDiscordServers`, and `UpdateGuildLobby` record local intent in `SimState.discord`; server/channel/name/count/linkable-channel getters read seeded `SimState.discord` metadata. Exact OAuth flow, Discord IDs, guild-link status codes, server/channel payload shapes, and service events still need PTR/service probes. |
| Battle.net title-friend custom names/tags | Best-effort modeled on `SimState.bnet_friends`: `SetCustomTitleFriendName` stores/clears a per-friend custom title name returned by `GetCustomTitleFriendName`; `SetFriendTags` stores array tags reflected in `GetFriendAccountInfo(...).friendTags`; the related `AreFriendTagsEnabled`, `AreTitleFriendsEnabled`, `AreTitleFriendCustomNamesEnabled`, `IsBattleNetFriendsListEnabled`, and `IsBattleNetFriendsListSupported` feature probes return true because the simulator exposes those modeled surfaces. `SetAppearOffline` records local presence intent in `SimState.bnet_appear_offline`, while `BNCheckTitleFriendInviteToUnit` is a deterministic false best-effort probe because title-friend unit-invite service state is not modeled. Exact PTR behavior still needs probes. |
| Pending Battle.net friend invites | Best-effort modeled on `SimState.bnet_friend_invites`: `SendVerifiedBattleNetFriendInvite(name)` creates one deduplicated pending invite; `GetFriendInviteInfo(index)` returns invite/account/friend-level/timestamp fields. Exact verified-invite flow, title-friend unit invite checks, and deprecated wrapper parity need probes. |
| Encounter Journal difficulty helpers | Best-effort modeled from generated Encounter Journal instance data: `GetBaseDifficultyID` returns `1` for dungeons and `14` for raids; `InstanceHasDifficultyID` accepts common dungeon IDs `1/2/8/23` and raid IDs `14/15/16/17`. Exact per-instance difficulty masks need generated data or PTR probes. |
| FrameXML symbol snapshot | Full 320-added/112-removed inventory is itemized in [[patch-12-1-framexml-symbol-inventory]]. Current classification: 1 implemented, 21 best-effort, 0 exception-requested, and 410 neutral untriaged rows. Both `MacroFrame_SaveMacro` occurrences are covered by focused PTR placeholder-to-LoD-replacement lifecycle proof. The five `DifficultyUtil` color helpers are dynamic delegates; `GetTimeStringFromSeconds` is cross-flavor contamination; AuctionHouse/GuildBank/Garrison/CustomerOrders hide wrappers are source/runtime mismatches; ItemUpgrade and BlackMarket hide entries are reversed-name mismatches. Addon-owned mismatch claims use explicit PTR source/runtime proof. |

### Exception requests pending informed approval

A broad approval recorded on 2026-07-14 is superseded: the itemized checklist was not presented in chat and 431 FrameXML entries were mass-deferred rather than justified under the unsafe/impossible bar. The following items remain pending until re-triage and informed per-item approval:

- **UnitAura secrecy and errors** — PTR notes say `C_UnitAura`/`C_TooltipInfo` aura access by index/slot/instance ID Lua-errors for addons while auras are secret, spell-ID/name access remains callable, `UNIT_AURA` payloads become fully secret, and `AuraData` structs are fully secret. Retained as an approved exception because approximating this without exact taint/addon vs Blizzard call-site behavior and error shapes risks enforcing the wrong security boundary.
- **Private Script Objects / Forbidden Partition** — compatible XML/private-table mechanics are modeled, but the full object partition contract is not proven. Retained as an approved exception for public/forbidden table identity, inaccessible key paths, child object visibility, hooks, script storage, and delegate edge cases until live behavior is captured.
- **Forbidden Aspects enforcement** — inheritance and query/add APIs are modeled, but exact restrictions for `UntrustedScriptExecution`, `UntrustedLayoutScriptExecution`, `EventRegistrations`, `AlwaysPropagateInput`, `ScriptedInput`, and `QueryFocus` need probes before blocking frame methods, focus/input queries, event registration, hooks, or script execution. Retained as an approved exception because premature enforcement would break addons on guessed security semantics.
- **AuraContainer / AuraButton / ManagedAuraContainer full behavior** — object names and compatible creation/XML paths are bridged, but full aura assignment, filtering, sorting, forbidden partition placement, automatic button management, tooltip behavior, and secret `IsShown` behavior are not modeled. Retained as an approved exception until aura secrecy and container lifecycle probes exist.
- **RadialProgress standalone script object fidelity** — texture/statusbar radial-progress-bar widget methods are bridged, but the standalone `RadialProgress:*` script object has no known constructor path in the current API audit. Retained as an approved exception rather than inventing a global constructor.
- **Full DurationTextBinding object fidelity** — compatibility methods exist, including 12.1 color-curve methods, but exact Blizzard object lifetime, metatable identity, formatter semantics, and color-curve interpolation remain unproven. Retained as an approved exception for fidelity beyond the documented best-effort table contract.
- **Changed structure payloads with real service data** — safe local state now backs Battle.net title-friend metadata, pending Battle.net invites, Encounter Journal difficulty helpers, Discord local state, and housing local state. Retained as an approved exception for exact payloads in Discord service responses, housing service collections/availability, cooldown viewer, pet journal, LFG, player choice, tiered entrance, and private aura structures until backing models or live captures exist.
- **Deprecated wrappers vs strict removals timing** — strict removed symbols are hidden for addon-facing 12.1 checks after startup. Retained as an approved exception for earlier removal timing until current PTR Blizzard UI no longer needs those values during load.
- **FrameXML symbol snapshot** — [[patch-12-1-framexml-symbol-inventory]] itemizes all 432 local snapshot entries (430 distinct names; two occur in both the added and removed lists). Current classification is 1 implemented, 19 best-effort, 0 exception-requested, and 412 neutral untriaged rows; no blanket exception is requested or approved.

### Practical next step

Best-effort simulator behavior is acceptable before PTR probes when it is backed by plausible existing state and tests document the simulator contract. Mark those guesses explicitly here, then replace them with probe-backed semantics later. For security/taint/error-shape-sensitive behavior, create probe addons before enforcing restrictions.

## Sources

- `/tmp/warcraft_patch_12_1_api_changes.txt` — source patch-note/API-change list used for the audit.
- `src/loader/tests/wow_api_globals/startup_globals.rs` — regression coverage for safe bridges and strict removals.
- `src/lua_api/workarounds/temporary/patch_12_1_inert_defaults.rs` — now-empty version-gated hook; safe 12.1 social/housing bridges moved to Rust-backed simulator state.
- `src/ptr/compat_bootstrap.lua` — 12.1 compatibility globals, including `LoadAddOnWithErrorHandling`.
- `src/ptr/strict_removals.lua` — post-startup hiding of removed 12.1 symbols.
- [[patch-12-1-framexml-symbol-inventory]] — exhaustive local FrameXML added/removed status inventory.
- `src/lua_api/frame/methods/forbidden_aspects.rs` — compatible forbidden-aspect query/inheritance implementation.
- `docs/wiki/systems/client-profiles.md` — retail epoch feature model.

## See Also

- [[client-profiles]] — retail epoch features used to gate 12.1 API surface.
- [[xml-template-system]] — private/forbidden XML partition and mixin behavior.
- [[lua-api]] — Lua runtime surface and C API bridge context.
- [[taint-system]] — secure/taint behavior that overlaps with forbidden aspects and aura secrecy.
- [[patch-12-1-framexml-symbol-inventory]] — exhaustive local FrameXML added/removed symbol status inventory.
