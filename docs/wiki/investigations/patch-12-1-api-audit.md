# Patch 12.1 API Audit

Patch 12.1 API surface work in wow-ui-sim is split between compatible bridges that can be safely modeled as inert/additive simulator behavior and paused items that require exact Blizzard PTR observations before implementation.

## Content

The per-item machine SSOTs are `data/patch-api/12.1-framexml.json` for the 432 FrameXML symbol occurrences and `data/patch-api/12.1-behaviors.json` for 53 independently testable non-FrameXML behavior boundaries. [[patch-api-audit-manifest]] documents validation and checklist generation. Draft `untriaged` resolutions remain completion blockers and are not approved exceptions.

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
| FrameXML symbol snapshot | Full 320-added/112-removed inventory is itemized in [[patch-12-1-framexml-symbol-inventory]]. Current classification: 1 implemented, 431 best-effort, 0 exception-requested, and 0 neutral untriaged rows. Both `MacroFrame_SaveMacro` occurrences are covered by focused PTR placeholder-to-LoD-replacement lifecycle proof. Both `PlayerChoiceToggle_TryShow` occurrences are covered by focused PTR before/after-LoD publication and button-state behavior proof. The proposed legacy `ShakeFrame` and `ShakeFrameRandom` additions are stale snapshot entries: active PTR leaves those globals nil and publishes distinct `ScriptAnimationUtil` methods. `GetTimeSinceLastQuestProgress` is vendor-present in PTRFeedback; focused proof documents its current upstream nil-arithmetic invocation defect without adding a guessed correction. The proposed `InterfaceUtil` screen-scale additions are stale while the two global screen-scale removals remain vendor-present; focused 1024×768 proof returns `1.0` for both globals. The five proposed `InputUtil` cursor methods are likewise stale namespace moves, while four proposed global removals remain vendor-present; focused proof exercises scale division, frame forwarding, mouse offsets, and inspect-cursor selection. The proposed removals of `UIDoFramesIntersect`, `GetNotchHeight`, and `GetUIParentOffset` are also reversed; focused proof covers strict edge intersection, notch normalization, and maximum debug/notch offset selection. Ten proposed interrupt/start/end/death `CombatAudioAlertUtil` additions are stale: recursive PTR source proof finds none, while the active runtime namespace publishes other methods and leaves all ten nil. All 29 proposed `FriendsListUtil` additions are stale qualified names: PTR retains similarly named `FriendsFrame_*` globals, but exact-qualified source proof and startup runtime enumeration leave `FriendsListUtil` nil. All 13 proposed `SocialUIUtil` additions are also stale qualified names; exact-qualified source proof and startup runtime enumeration leave that namespace nil. All 14 proposed `NarrationUtil` additions follow the same stale qualified-name pattern and remain absent from source and startup runtime. Ten proposed `GuildControlUI_*` globals are likewise absent from the full PTR source corpus and startup runtime. Colon-aware proof classifies 29 proposed utility-namespace additions as stale and disproves removal of `PingUtil.GetContextualPingTypeForUnit`, which still forwards to `C_Ping`. A conservative generated batch classifies 175 additional proposed additions as stale only when their bare token is absent from every PTR Lua/XML/TOC file and the complete game-compatible addon closure, including LoD roots, leaves the exact path nil. Some all-LoD addons emit recorded Lua errors, so these rows remain explicitly best-effort rather than exact fidelity claims. The final publication matrix classifies seven source-present but unpublished additions as stale and 99 proposed removals as vendor-present functions after the same all-LoD closure. The snapshot's proposed `InterpolatorUtil.GetSmoothProgressChange` addition is reversed while the proposed global `GetSmoothProgressChange` removal remains vendor-present: focused PTR proof verifies the namespace member is nil while the retained global computes the expected value. The five `DifficultyUtil` color helpers are dynamic delegates; `GetTimeStringFromSeconds` is cross-flavor contamination; AuctionHouse/GuildBank/Garrison/CustomerOrders hide wrappers are source/runtime mismatches; ItemUpgrade and BlackMarket hide entries are reversed-name mismatches. Addon-owned mismatch claims use explicit PTR source/runtime proof. |

### Broader fidelity classification and exception candidates

A broad approval recorded on 2026-07-14 is superseded: the itemized checklist was not presented in chat and the 431 FrameXML rows had not yet been independently classified. The FrameXML register is now complete. The broader fidelity boundaries are now itemized separately in [[patch-12-1-behavior-inventory]] as 54 rows: 21 direct-test-backed best-effort rows and 33 untriaged. Candidate disposition remains 30 safe best-effort, 21 unsafe, and 3 impossible. Family names below are summaries only; approval remains per row. No exception approval is requested or recorded by this section.

| Family | Current classification | Remaining fidelity boundary | Exception state |
|---|---|---|---|
| UnitAura secrecy and errors | No enforcement is claimed for addon-vs-Blizzard secret aura errors, fully secret `AuraData`, or secret `UNIT_AURA` payloads. | Exact taint-sensitive call-site rules and error shapes require live evidence before enforcement. | **Candidate: unsafe to guess.** Pending informed approval; not approved. |
| Private Script Objects / Forbidden Partition | **Best-effort:** compatible XML/private-table mechanics are modeled and tested. | Public/forbidden identity, inaccessible key paths, child visibility, hooks, script storage, and delegate enforcement remain unproven. | **Candidate for the security-enforcement remainder: unsafe to guess.** Pending informed approval; not approved. |
| Forbidden Aspects enforcement | **Best-effort:** inheritance plus query/add APIs are modeled and tested. | Exact restrictions for `UntrustedScriptExecution`, `UntrustedLayoutScriptExecution`, `EventRegistrations`, `AlwaysPropagateInput`, `ScriptedInput`, and `QueryFocus` are unknown. | **Candidate: unsafe to guess.** Pending informed approval; not approved. |
| AuraContainer / AuraButton / ManagedAuraContainer | **Best-effort:** names, compatible creation, and XML paths are bridged. | Assignment, filtering, sorting, partition placement, automatic button management, tooltips, and secret visibility are not exact fidelity claims. | No exception requested; retain the documented best-effort contract. |
| Standalone RadialProgress script object | Texture/statusbar radial-progress widget methods are bridged. | The standalone `RadialProgress:*` object has no known constructor path in the audited API surface. | **Candidate: impossible to model faithfully without constructor evidence.** Pending informed approval; not approved. |
| DurationTextBinding object fidelity | **Best-effort:** compatibility-table methods and 12.1 color-curve methods are implemented. | Blizzard lifetime, metatable identity, formatter semantics, and interpolation remain unproven. | No exception requested; retain the documented best-effort contract. |
| Changed structure payloads with real service data | **Best-effort:** local state backs Battle.net, invites, Encounter Journal, Discord, and housing behavior. | Exact Discord/housing/cooldown/pet/LFG/player-choice/tiered/private-aura payloads require service models or captures. | No exception requested; retain the documented best-effort contract. |
| Deprecated wrappers vs strict-removal timing | **Best-effort:** strict removals are hidden from addons after startup while preserving current Blizzard load compatibility. | Moving removal earlier can break the pinned Blizzard UI until current-source/load-order evidence proves otherwise. | **Candidate: unsafe to change timing.** Pending informed approval; not approved. |
| FrameXML symbol snapshot | **Complete:** [[patch-12-1-framexml-symbol-inventory]] contains 1 implemented, 431 best-effort, 0 exception-requested, and 0 untriaged occurrences. | Some rows deliberately document vendor defects or conservative source/runtime absence rather than exact behavioral fidelity. | No blanket exception requested or approved. |

### Practical next step

Resolve the remaining 9 safe-best-effort rows with focused simulator-contract evidence first. Then present the remaining unsafe/impossible rows item by item in chat before requesting approval. For security, taint, error-shape, and private-data behavior, create probe evidence before enforcing restrictions.

## Sources

- `/tmp/warcraft_patch_12_1_api_changes.txt` — source patch-note/API-change list used for the audit.
- `data/patch-api/sources/12.1-behaviors.json` — normalized broader behavior boundaries and candidate disposition.
- [[patch-12-1-behavior-inventory]] — itemized broader behavior machine state and candidate classification.
- `src/loader/tests/wow_api_globals/startup_globals.rs` — regression coverage for safe bridges and strict removals.
- `src/lua_api/workarounds/temporary/patch_12_1_inert_defaults.rs` — now-empty version-gated hook; safe 12.1 social/housing bridges moved to Rust-backed simulator state.
- `src/ptr/compat_bootstrap.lua` — 12.1 compatibility globals, including `LoadAddOnWithErrorHandling`.
- `src/ptr/strict_removals.lua` — post-startup hiding of removed 12.1 symbols.
- [[patch-12-1-framexml-symbol-inventory]] — exhaustive local FrameXML added/removed status inventory.
- [[patch-12-1-behavior-inventory]] — separate non-FrameXML behavior inventory.
- `src/lua_api/frame/methods/forbidden_aspects.rs` — compatible forbidden-aspect query/inheritance implementation.
- `docs/wiki/systems/client-profiles.md` — retail epoch feature model.

## See Also

- [[client-profiles]] — retail epoch features used to gate 12.1 API surface.
- [[xml-template-system]] — private/forbidden XML partition and mixin behavior.
- [[lua-api]] — Lua runtime surface and C API bridge context.
- [[taint-system]] — secure/taint behavior that overlaps with forbidden aspects and aura secrecy.
- [[patch-12-1-framexml-symbol-inventory]] — exhaustive local FrameXML added/removed symbol status inventory.
