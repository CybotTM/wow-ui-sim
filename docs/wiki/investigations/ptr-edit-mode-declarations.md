# PTR Edit Mode declaration boundaries

PTR 12.1.7.70493 needs declaration-backed Edit Mode enums before initialization can proceed. The bounded [contract](../../specs/ptr-edit-mode-declarations.md) does not cover the later missing settings, systems or native default-layout policy.

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

The test authenticates the selected TOC and loads its dependency closure, then executes the unchanged complete SwingTimer constructor at799–877 with original line padding. It checks all seven setting identifiers and all three nested visibility options, plus both Meta tables. Test-only RED `523c4e729` fails at the original Scale read, reported at800. Producer-only GREEN execution remains pending.

Forever's pinned declaration at688–703 and718–729 has identical values, already registered by its separate profile module. Retail, Mists, Era and Anniversary documentation omits both families; Wrath has no corresponding declaration in its generated documentation cache. The source publication scan finds these names only in Forever's gated module before this change. Other profiles retain their existing publication paths.

The Mainline preset's earlier generic nil-index failure remains. This work does not install missing methods on frames or claim full display-info initialization.

## Sources

- [Contract](../../specs/ptr-edit-mode-declarations.md).
- [Behavioral selectors](../../../tests/ptr_edit_mode_enums.rs).
- [Producer](../../../src/c_api/ptr_edit_mode_enums.rs).
- [Consumer applicability policy](consumer-selection-and-red-proof.md).

## See Also

- [[patch-12-1-7-api-audit]] — neighboring PTR publication work; separate ownership.

## Runtime receipts

The two exact selectors each list one test. Both fail independently on test-only `ce2509f0a` and pass on `2d42b56e6`, with byte-identical tests. Exact master `1f914b26e` stock has 270 keys / 376 occurrences; GREEN has 268 / 374. Four predecessor keys are removed, two display-info:800 successor keys are added, and existing-key counts are unchanged. All invocations preserve the original CVar stat/hash using a fresh byte-exact child copy. PTR, Retail and Era full-feature all-target checks pass with zero Cargo JSON or plain Rust warnings. Each has a separate accepted source-binding receipt. This reduced matrix is permitted because the producer and hook compile without feature gates on every profile; only runtime ACTIVE selects publication. Formatting and manual readability pass. These results do not establish clean startup.
