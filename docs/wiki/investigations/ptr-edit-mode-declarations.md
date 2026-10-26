# PTR Edit Mode declaration boundaries

PTR 12.1.7.70493 needs declaration-backed Edit Mode enums before initialization can proceed. The [contract](../../specs/ptr-edit-mode-declarations.md) now covers complete original display-info initialization, but not native default-layout policy or the separate Mainline preset's unread declarations.

## Declaration evidence

Pinned `Blizzard_APIDocumentationGenerated/EditModeManagerConstantsDocumentation.lua:111–122` declares BossFrameGrowDirection (Left=0, Center=1, Right=2); `:639–650` declares EditModePresetLayouts (Modern=0, Classic=1, Gamepad=2). Both families declare NumValues=3, MinValue=0, MaxValue=2. The generated simulator preset family previously contained only Modern/Classic. The existing Forever module supplies Gamepad only on Forever; it does not publish PTR declarations.

The dedicated C API module publishes the complete tables and their metadata after generated and compatibility enums, through an unconditional registration hook. The same module compiles on every profile; its only publication gate is the runtime `ACTIVE != ClientProfile::Ptr` early return. No broad initializer data, vendor files, GameRules or GraphicsOverrides presets are changed.

## Original consumers and proof limits

The selected Game closure loads `Blizzard_EditMode/Blizzard_EditMode.toc`, including `Shared/EditModeSettingDisplayInfo.lua` and `Shared/EditModePresetLayoutsManager.lua` under Global environment annotations. The tests authenticate this TOC selection and run the original closure before isolating the requested boundaries.

- Boss dropdown: the unchanged constructor at `EditModeSettingDisplayInfo.lua:250–261` reads the three direction values at257–259. A named chunk preserves its original source lines. Its numeric options and declaration metadata reject missing or constant direction producers.
- Preset manager: the complete unchanged shared manager reaches the Gamepad key at41 independently of the earlier Mainline preset. Supplied distinct maps avoid treating the earlier preset abort as a missing manager API. Assertions cover the three indices, Gamepad interface style, actual map selection and independent copies of supplied anchors.

`GetSystemSettingDisplayInfoMap` is defined by Blizzard at1628. Its absence after a file abort is an initialization consequence, not permission to install a replacement frame method. Successful bounded tests do not establish complete Edit Mode initialization.

## SwingTimer declaration boundary

Removing the earlier BossFrameGrowDirection read exposes `EditModeSettingDisplayInfo.lua:800`, whose Scale field is at physical line801. The pinned documentation declares EditModeSwingTimerSetting at702–717: Scale=0, Opacity=1, Visibility=2, Width=3, Height=4, ShowBarTitle=5, ShowTime=6, Meta7/0/6. The same original constructor also reads EditModeSwingTimerVisibility at873–875, declared at732–743: Always=0, InCombat=1, Hidden=2, Meta3/0/2. Both families now belong to the bounded contract.

The test authenticates the selected TOC and loads its dependency closure, then executes the unchanged complete SwingTimer constructor at799–877 with original line padding. It checks all seven setting identifiers and all three nested visibility options, plus both Meta tables. Test-only RED `523c4e729` fails at the original Scale read, reported at800. GREEN `4f5872e42` reaches the end and passes all assertions with byte-identical tests. No executable code outside the declaration producer differs between RED and GREEN.

Forever's pinned declaration at688–703 and718–729 has identical values, already registered by its separate profile module. Retail, Mists, Era and Anniversary documentation omits both families; Wrath has no corresponding declaration in its generated documentation cache. The source publication scan finds these names only in Forever's gated module before this change. Other profiles retain their existing publication paths.

### Historical next keyed-map boundary

Before the complete-file lane below, the selected display-info file reported `table index is nil`. A separate diagnostic executes its unchanged keyed SwingTimer entry at797–878, confirms the preceding 18 system keys are present and `rawget(Enum.EditModeSystem, "SwingTimer")` is nil, and reproduces the nil-index error. The documentation declares SwingTimer=29 in EditModeSystem at781 (family745–784). This was the next boundary after the SwingTimer constructor lane; the complete-file lane below resolves it.

Paired exact master `7576b25e1` and GREEN stock both contain 216 raw keys / 282 occurrences. Two Scale:800 keys disappear; two nil-index keys replace them, without changes to existing counts. The primary emitter is the keyed SwingTimer entry; the second raw key is its Blizzard_ScriptErrors.lua:59,72 echo. The earlier Scale read masked both. All seven guarded runtime invocations preserve the original persisted CVar stat and hash using byte-exact fresh child copies.

Separate canonical slot-build receipts pass PTR, Retail and Era full-feature all-target checks with zero JSON or plain Rust warnings/errors. The unconditional module and hook and runtime-only publication gate are unchanged. Formatting and manual Rust readability pass. Checks cover the final Rust/test source at4f5872e42; later documentation does not change it.

That constructor-only proof did not install missing methods on frames or establish full display-info initialization.

## Complete original display-info initialization

The complete-file lane extends existing PTR enum tables instead of replacing them. A runtime scan of every declared enum member read by the original file found 16 missing or wrong fields. All come from the same pinned `EditModeManagerConstantsDocumentation.lua`:

| Fields | Declaration lines |
|---|---|
| MainActionBarEndCap=26, GroupFinder=27, LossOfControl=28, SwingTimer=29, GamepadMainActionBar=30 | 778–782 |
| Gamepad settings Compact=0, ShowEmptyBars=1, ShowButtonPrompts=2, ShowHighlight=3, ShouldExpand=4 | 505–509 |
| GroupFinder Size=0; EndCap Hidden=0; PersonalResourceDisplay AllowMouseoverThrough=15 | 520; 556; 635 |
| UnitFrame DebuffIconSize=19, UseHorizontalLayout=23, GrowDirection=24 | 823; 827–828 |

LossOfControl previously had value26; DebuffIconSize had value23. These are corrected rather than duplicated. The original file does not read ShowSwingTimer or the affected Meta tables, so this lane does not publish them. Existing unrelated enum members and the runtime-only PTR publication gate are preserved.

The complete unchanged selected file fails on test-only `b777460c0`, then passes with producer `8a23c562e`. Final strict producer-only RED `2d87067c4` and GREEN `86ceb6611` contain byte-identical tests. Each exact selector lists one test and is run independently. The full-file selector checks concrete system maps26–30, all seven SwingTimer records, visibility options and original scale conversion. The second selector authenticates the original FrameXML LossOfControl Lua/XML selection and proves its first OnLoad reaches the map assignment at EditModeSystemTemplates.lua:27, Selection:SetSystem, event registration, text-width initialization and final Anim OnFinished installation. Both fail on RED and pass on GREEN without replacement Lua functions.

Paired stock uses master `eaf484fbc` production source (test-only build `b777460c0`) and GREEN `8a23c562e`, with byte-exact fresh CVar copies and unchanged original stat/hash. Raw maps change214 keys/280 occurrences →134/154:84 keys removed,4 added, no existing-key count changes. Two additions are the original Mainline preset's newly reached missing EndCapSystemIndices at reported630 (physical631), masked by UseHorizontalLayout's nil keyed setting at348. The other two additions are unchanged CompactRaidFrameManager dividerVerticalPool errors whose anonymous routing-frame ID changes34171→34181; exact raw keys are retained, not normalized away. Per-key traces are retained in the lane report artifacts.

The complete display-info file has no remaining boundary. The separate Mainline preset still cannot initialize; its EndCapSystemIndices family is not read by display-info and is not added. Stock startup is not clean. PTR/Retail/Era each pass a separate accepted canonical full-feature all-target receipt at `41ec8cd64`, with zero Cargo JSON or plain Rust warnings/errors and no dep-info errors. Formatting and manual changed-Rust readability pass. The seven build/check units exited successfully, and the slot owner marker was preserved by rename on release. Later changes are documentation-only.

## Sources

- [Contract](../../specs/ptr-edit-mode-declarations.md).
- [Behavioral selectors](../../../tests/ptr_edit_mode_enums.rs).
- [Producer](../../../src/c_api/ptr_edit_mode_enums.rs).
- [Consumer applicability policy](consumer-selection-and-red-proof.md).

## See Also

- [[patch-12-1-7-api-audit]] — neighboring PTR publication work; separate ownership.

## Runtime receipts

The two exact selectors each list one test. Both fail independently on test-only `ce2509f0a` and pass on `2d42b56e6`, with byte-identical tests. Exact master `1f914b26e` stock has 270 keys / 376 occurrences; GREEN has 268 / 374. Four predecessor keys are removed, two display-info:800 successor keys are added, and existing-key counts are unchanged. All invocations preserve the original CVar stat/hash using a fresh byte-exact child copy. PTR, Retail and Era full-feature all-target checks pass with zero Cargo JSON or plain Rust warnings. Each has a separate accepted source-binding receipt. This reduced matrix is permitted because the producer and hook compile without feature gates on every profile; only runtime ACTIVE selects publication. Formatting and manual readability pass. These results do not establish clean startup.
