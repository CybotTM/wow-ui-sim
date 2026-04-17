# Wiki Index

LLM-maintained knowledge base for the wow-ui-sim project.

## design/

| Page | Summary |
|------|---------|
| [[architecture-overview]] | Project goals, Lua+Rust dual system, module layout, design decisions, phase summary |
| [[scaling-coordinates]] | WoW bottom-left Y-up coordinate system, canvas sizing, projection matrix, known issues |
| [[debug-tools]] | Inspector panel (middle-click), dump-tree CLI (standalone + connected), debug overlay flags |

## reference/

| Page | Summary |
|------|---------|
| [[api-coverage]] | ~97% C_* coverage, missing APIs by category, three-layer stub methodology |
| [[cli-commands]] | wow-sim and wow-cli subcommands: lua-errors, run-tests, screenshot, dump-tree, audit-api, convert-texture |
| [[addon-compatibility]] | 127+ tested addons, Wowless integration, SavedVariables loading, Docker CI |
| [[development-phases]] | Active phases 31–33: widget stubs, audit tool, performance regression tests |

## systems/

| Page | Summary |
|------|---------|
| [[layout-system]] | AnchorPoint enum (9 positions), single vs multi-anchor resolution, coordinate system (top-left screen / bottom-left Lua), SetPoint API, cycle detection |
| [[rendering-pipeline]] | QuadBatch (36-byte QuadVertex), four-tier GPU texture atlas, WGSL shaders, strata/level sorting, alpha propagation, hit testing |
| [[widget-system]] | Frame struct (~140 fields), WidgetType enum (18 types), WidgetRegistry, default children, button text rendering, three-slice pattern |
| [[lua-api]] | WowLuaEnv, FrameHandle userdata, 300+ frame methods, 200+ globals, C_* namespaces, timer system, animation system |
| [[event-system]] | EventQueue, 36+ script handler types, dispatch flow, OnUpdate tick, startup event sequence, XML script setup |
| [[xml-template-system]] | XML parsing (30+ element types), template registry, inheritance chain resolution, XML-to-widget Lua code generation, inline scripts |
| [[addon-loading]] | TOC parsing, Blizzard load order (27 addons), per-file Lua/XML loading, SavedVariables, startup sequence |
| [[texture-atlas]] | TextureManager (BLP/PNG/WebP), ~50K-entry compiled atlas database, nine-slice kit detection, UV remapping |
| [[frame-data-flow]] | Parallel Lua/Rust systems, global tables (__frame_fields/__scripts), method lookup order, Mixin() application, event dispatch flow |
| [[taint-system]] | Combat lockdown on protected frames, dual Lua environment (genv/secureenv), issecure/securecall from Elune, SecureHandler stubs |

## investigations/

| Page | Summary |
|------|---------|
| [[action-bar-spell-icons]] | 4 bugs: SetDrawLayer no-op, draw order, sublevel ignored, textureSubLevel not parsed |
| [[addon-load-order]] | Bag buttons partially initialized at load; workaround mirrors real WoW event recovery |
| [[bag-button]] | nil texture from GetInventorySlotInfo, stub returning 0 slots, ItemContextOverlay, frame_level_offset |
| [[talent-performance]] | Lazy `_G` lookup (431ms→263ms), rect-dirty stale cache causing infinite OnUpdate loop |
| [[character-select-performance]] | Lazy atlas crop stalls (fixed), first-resize relayout deduplication (partial) |
| [[class-talents-artifact]] | Gold blob ruled out as lossy WebP encoding artifact, not a live render bug |
| [[editmode-layout]] | 3 frame regressions from EditMode overrides after `__index` ordering fix; fenv workaround |
| [[generated-stubs-audit]] | 6 priority findings in generated_stubs.rs affecting startup/panel-load paths |
| [[chatframe-scrollbar-anchor-reapply]] | Inherited child anchor reapply used the child name for `$parent...` substitution, pushing `ChatFrame1` scrollbar descendants off-screen |
| [[hero-spec-icon-bug]] | Retired — 5 layers of evidence confirm icon renders correctly |
| [[hit-testing]] | Two-phase algorithm: HitGrid spatial index + depth-first child drill-down |
| [[keybinding-system]] | Two Lua tables, key press pipeline, default bindings, Lua API |
| [[mask-texture]] | UV computation, useAtlasSize default, SmallActionButtonMixin override |
| [[method-dispatch-refactor]] | Runtime pollution fixed; target: direct Rust dispatch |
| [[minimap]] | Basic circular placeholder; missing real content/mask/blips/POIs |
| [[on-update-dirty]] | Blanket dirty discard suppresses cast bar; now tracks the compact-raid cleanup, remaining leave-button/BuffFrame churn, and the same-day `GameTimeFrame_SetDate()` calendar-atlas no-op fix |
| [[startup-createframe-profile]] | Runtime `CreateFrame` profiling started with action-bar button template expansion, then widened into the XML loader fast path; current safe loader state lands around 4.8s-5.8s on debug no-addons/no-saved-vars runs, with remaining misses dominated by XML script bodies |
| [[table-rehashing]] | 97K rehashes on startup; 98% from non-frame Lua tables, 81% land at hash size ≤16; root cause is `OP_NEWTABLE(0,0)` for addon `local t = {}` patterns |
| [[layout-profile]] | Layout was 7.5% of release startup; `LayoutCache` siphash dominated. `FxHashMap` switch drops to 5.0%, −170M layout samples, −219M total siphash samples |
| [[intern-string-ranking]] | 1.25M intern_string calls per startup; rilua bug found (mid-cycle inserts swept) + fixed. Migration landed for registry/metatable helpers: 1.25M → 1.10M (−12%), 1.18s → 1.15s, and follow-up perf shows `lua_hash` itself is down to 0.08% of startup. `frame_ref_cache` still deferred |
| [[partyframe-tree]] | `rilua-migration` regresses `PartyFrame` from master's `(120x244)` 4-member layout down to `(4x2)` with zero member frames; regression test pinned against the master dump |
| [[world-map-onupdate-hover-polling]] | Chat-frame hover polling was forcing mutable `IsMouseOver()` work on every idle tick; clean-layout hover checks are now read-only, empty `UIParent` worklists short-circuit, but the fresh 90s world-map profile still sits at 31 steady-state handlers |
| [[world-map-voice-chat-alerts]] | Reduced world-map stacks can show voice prompt frames above the map when `Blizzard_Channels` is loaded without `Blizzard_SocialToast` / chat-alert prerequisites |
| [[protected-frames]] | 3-condition enforcement, covered methods, remaining gaps |
| [[transparent-wrapper-render-order]] | Renderless `Frame`/`ScrollFrame` wrappers were creating fake z-order boundaries; descendant regions now hoist through them |
| [[talent-sheen]] | 22s synchronized sweep; white rectangle bug when masking broken |
| [[tooltip-alignment]] | NineSlice inner box vs outer bounds; 15px effective inset |
| [[glow-effects]] | Additive blending end-to-end; one gap: SetBorderBlendMode missing |
| [[global-frame-index]] | Lazy `_G` lookup design; Phase 1 done, Phases 2-3 planned |
| [[world-map-frame-level-rebuilds]] | World map pins were forcing no-op `SetFrameLevel()` invalidations; steady-state bucket rebuilds are now gone |
| [[world-map-create-texture-sublevel]] | World-map textures were created at sublevel 0 because `CreateTexture(..., subLevel)` ignored its fourth argument; immediate `SetDrawLayer()` repair churn is now gone |
| [[world-map-fog-of-war-first-open-size]] | First-open world-map fog pins could keep a stale size because `FogOfWarPinMixin` only resized on canvas scale changes; simulator now patches size-change handling for existing and future pins |
| [[world-map-fog-of-war-overlay-model]] | Current world map has no `UiMapFogOfWar` entry; simulator now hides fake fog and seeds one real unexplored overlay chunk until character exploration state exists |
| [[world-map-texture-loading-budget]] | World map tile uploads were hidden in BC texture work; preload/draw now share BC cache, honor `RequestPreloadMap()`, keep the fast tick alive until GPU uploads finish, and count BC-preloaded tiles as cached during budgeted draw |
