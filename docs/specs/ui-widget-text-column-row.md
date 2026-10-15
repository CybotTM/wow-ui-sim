# Supplied TextColumnRow visualization

`C_UIWidgetManager.GetTextColumnRowVisualizationInfo` is the cached Retail/PTR type-21 query. [Manager architecture](../wiki/systems/ui-widget-manager-state.md) owns state, feeds and consumer loading.

## Required behavior

- Accept complete explicitly supplied outer/nested records. No inferred fields, defaults, catalogs or demo seeds. Keep `text_column_rows` initially empty and environment-local; empty entries differ from an absent record.
- Preserve existing numeric-key policy and shared required-number/security boundary. Return one nil for an absent record, otherwise complete fresh outer/entries/entry tables. Caller mutation, replacement or deletion must not alter retained snapshots or another environment.
- Use normal type-21 feeds with existing type/set validation and state-before-synchronous-event semantics. Rejected updates leave records, membership and notifications unchanged. Membership removal retains independent visualization data.
- Exercise actual profile-cache registration, activation/update, nested entry styling/layout, Hidden release, Shown reacquisition, membership removal and live unregister through normal feeds/OnUpdate. No substitute provider, private handler invocation or vendor mutation.

## Cached declarations

Both Retail/PTR `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua` declare these required, non-nil fields. Numeric/String Rust representations follow simulator conventions, not native ABI proof. No global field-count or declaration-order contract is introduced.

| Outer field | Declared type |
| --- | --- |
| `shownState` | `WidgetShownState` |
| `entries` | table of `TextColumnRowEntryInfo` |
| `textSizeType` | `UIWidgetTextSizeType` |
| `fontType` | `UIWidgetFontType` |
| `tooltip` | string |
| `tooltipLoc` | `UIWidgetTooltipLocation` |
| `bottomPadding` | number |
| `widgetSizeSetting` | number |
| `textureKit` | textureKit |
| `frameTextureKit` | textureKit |
| `hasTimer` | bool |
| `orderIndex` | number |
| `widgetTag` | string |
| `inAnimType` | `WidgetAnimationType` |
| `outAnimType` | `WidgetAnimationType` |
| `widgetScale` | `UIWidgetScale` |
| `layoutDirection` | `UIWidgetLayoutDirection` |
| `modelSceneLayer` | `UIWidgetModelSceneLayer` |
| `scriptedAnimationEffectID` | number |

| Entry field | Declared type |
| --- | --- |
| `text` | string |
| `enabledState` | `WidgetEnabledState` |
| `hAlign` | `WidgetTextHorizontalAlignmentType` |
| `columnWidth` | number |

## Current source checkpoint — 2026-10-14

Passive record structs, empty environment-owned map and normal type-21 storage routing are implemented under `src/c_api/ui_widget_manager/`. Getter/projector/registration are not implemented. Generic callable/nil compatibility is not modeled-query evidence.

`tests/ui_widget_text_column_row.rs` discovers six base cases under Retail/PTR plus numeric/real-host-secret selector supplements; the secret case additionally requires `retail-12-1-0`. Base cases cover populated projection/deep freshness; replacement/empty entries/deletion/isolation; numeric keys; first-feed callback state and removal; atomic rejection; actual cached nested-pool lifecycle. Supplements use distinct integer/fraction records through normal feeds, complete recursive public snapshots, actual host-secret NUMBER wrappers, secure allowance/tainted denial/recovery and unchanged backing/membership. Shared helper assertions do not substitute a getter or API. Fixtures and observational Lua assertions live in `tests/widget_text_column/`. Each case owns its environment; populated preconditions are not a global test-order rule. First-feed setup removes its query seed before publication.

These tests are uncompiled/unexecuted: zero observed RED or GREEN. Consumer tag/style/reset assertions remain authored expectations, not runtime facts. Query-only texture/effect/timer values are synthetic; consumer records neutralize them. Source/readability inspection is separate from compilation and behavior.

## Saved nonbehavioral follow-up — 2026-10-14

Root `cargo fmt --check` at `c66bfa263` passes with empty complete stdout/stderr. The earlier 512M formatter attempt timed out without a formatter result; the corrective 2G attempt passes. This proves formatting only, not compilation or behavior. Later `ed9fdba0e` changes six manifest Clippy-key spellings, preserving values, dependencies and Rust sources; one offline/locked/no-deps metadata invocation passes without those warnings. Historical warning streams remain unchanged; metadata does not prove compiler/linter cleanliness.

Four subsequent Retail integration `--no-run` attempts at `c66bfa263` ended in guard-issued SIGTERM without Cargo `build-finished` or a completed integration artifact. Triggers were host full PSI avg10 (two attempts), host full PSI avg60, then worker full PSI avg10. These are incomplete compilations, not compiler-error failures or behavioral RED. Eight authored cases remain uncompiled/unexecuted; getter/projector/registration remain absent. The earlier owning compile aborts remain separate history in the [Spacer contract](ui-widget-spacer.md).

Saved guard checks establish bounded pure threshold decisions, not unconditional malformed-input fail-closed handling, successful live stopping or machine responsiveness. Resource observations do not establish workload causality.

The separate [frozen historical71 packet](../../data/patch-api/evidence/spacer-text-column-late-text-2026-10-14/README.md) retains the third Spacer abort, formatter attempts and source-only TextColumn/shared-doc reports. Independent private exactness/privacy review passes after preserving earlier privacy FAIL25/FAIL4; independent canonical verification at `055b57afd` passes 71/71 exact payload comparisons, catalog/owner link, commit scope and bounded 73-file privacy review. It excludes newer compile attempts and current-source acceptance.

## Remaining proof

- [ ] Compile tests and observe genuine populated result-boundary RED before getter implementation. Import/harness failures and absence-only controls do not qualify.
- [ ] Implement/prove projection, shared selector/security behavior and deep snapshot isolation, then exercise each actual Retail/PTR cached consumer and affected diagnostics/checks.
- [ ] Reconcile applicable publication evidence only after ordinary owning observation; preserve historical failures and broader acceptance gaps.

Native catalogs/publishers, native interpolation/security/return parity, rendered output, empty-entry consumer lifecycle and nonneutral texture/effect/timer activation are not established by this slice. Other profiles and full-parent acceptance remain separately scoped.
