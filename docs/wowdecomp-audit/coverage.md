# Coverage ledger

Snapshot: 2026-10-15. External reconstruction `0e4e9fe0a4a9fa08027c5eed4b2c5e873739cfd3`; simulator source baseline `2da20225ba147dda579dd01164917cc16f0618ed`. Later audit commits change documentation only. [Interpretation and security limits](README.md) apply.

## Actual coverage, not a total gap count

| Registration-row status | Count | Meaning |
|---|---:|---|
| CHECKED_MATCH | 20 | The **named aspect** was compared and agrees; not a whole-method/profile/native parity claim. |
| MISMATCH | 67 | An accepted static source difference, linked to an owning finding; supported-profile adoption remains conditional. |
| UNRESOLVED | 61 | Partial comparison, historical/current-contract conflict, unclosed producer/dispatch trace or an unaccepted candidate. |
| UNCHECKED | 2,752 | Literal registration inventoried; simulator behavior not compared. |
| **Total** | **2,900** | **105 source files; 148 rows have bounded or partial comparison notes.** |

There are **41 finding entries**, not 67 separate bugs: one finding can span several methods. Thirty-nine findings map to method rows; X1/X2 are XML-loader/template behavior outside this literal-method denominator. Include stack depth is an unmeasured shared boundary, not a counted discrepancy.

**The large unchecked remainder is deliberate and visible.** This audit does not establish that these 41 entries are all simulator gaps, that unchecked APIs work, or that any historical difference automatically warrants a modern-profile change.

## Subsystem matrix

Counts are contextual registration rows, not unique API names. `Match` always means only the row's stated aspect.

| Subsystem | Rows | Match | Difference | Unresolved | Unchecked |
|---|---:|---:|---:|---:|---:|
| Frame/object/layout | 114 | 3 | 4 | 5 | 102 |
| Button/CheckButton/Slider/StatusBar/ScrollFrame/ColorSelect | 86 | 5 | 8 | 12 | 61 |
| Texture/Font/FontString | 94 | 1 | 10 | 3 | 80 |
| EditBox/MessageFrame/ScrollingMessageFrame/SimpleHTML | 157 | 1 | 6 | 7 | 143 |
| Animation/group/path/control points | 77 | 2 | 10 | 3 | 62 |
| Other Engine registrations | 64 | 0 | 0 | 0 | 64 |
| Minimap/Cooldown/Tooltip | 93 | 6 | 15 | 4 | 68 |
| Action bar/spellbook | 73 | 0 | 4 | 11 | 58 |
| Containers/currency | 28 | 2 | 10 | 16 | 0 |
| Other WoW APIs/glue | 2,114 | 0 | 0 | 0 | 2,114 |
| **Total** | **2,900** | **20** | **67** | **61** | **2,752** |

Specific unchecked surfaces remain in the ledger, including six CheckButton-specific exports and 48 ScrollingMessageFrame-specific exports; examining their base classes did not silently mark them covered. The largest untouched files include GameUI (310 rows), unit/system exports in ScriptEvents (176), glue exports (113), Calendar (95) and ChatFrame (89). They remain future comparison work, not exclusions or passes.

## Inspectable artifacts

- [Per-export ledger](coverage.tsv): source file/line, table, export, implementation symbol, status, exact compared aspect, finding link and evidence note. Keep source/table context: repeated method names on different widgets are not interchangeable.
- [Per-file totals](coverage-by-file.tsv): all 105 files, including entirely unchecked ones.
- [Finding catalogue](findings.md): source differences, concrete proposed fixtures and required behavior changes are owned by the linked detail documents.

TSV files are plain UTF-8 tab-separated data. Source line and symbol columns identify **registration entries**, not necessarily function definitions or final runtime bindings. Match notes that never closed a concrete counterpart trace were conservatively retained as UNRESOLVED rather than counted as matches. No test was executed to assign these statuses.

## Inventory construction and its limits

The inventory scans `Engine/Source/**/*.cpp` and `WoW/Source/**/*.cpp` at the pinned external revision: 1,348 files (583 Engine, 765 WoW). It extracts unsized literal arrays declared with `FrameScript_Method`, `FrameScript_Function` or `luaL_Reg`, then literal string/function pairs. Many global-function tables use the `FrameScript_Method` C++ type; its name does not make those entries widget methods.

Independent read-only inventory inspection checked every recorded line for its literal name/function symbol: 2,900 rows, 105 files, zero bad row references. There are 592 Engine and 2,308 WoW rows; 2,687 distinct names, with repeated names intentionally retained by file/table context. No duplicate contextual inventory keys were found. This proves the bounded inventory's references, **not** 2,900 callable or implemented APIs.

The extraction is not a C++ compiler/preprocessor or a complete WoW API catalogue. It can miss fixed-size/qualified/alternate declarations, macros, dynamic/direct registration, generated entries and other source extensions. It does not resolve comments/conditional compilation exhaustively, registration reachability, inherited bindings, later overwrite/unregister order or supported client exposure. Headers, external scripts and build-generated registration are not a complete denominator here. Merely finding a table row or a simulator callable is insufficient to assign behavioral support.

## Corrections and remaining evidence boundaries

| Lead | Reconciled disposition |
|---|---|
| Alpha `SetChange` is a no-op | Rejected: actual registration selects the implemented setter. Transform A1 excludes working alpha and Path support. |
| Tooltip colour helper reads one component | Rejected: helper consumes RGB triplets. U3 is wrapping, not colour parsing. |
| StatusBar orientation missing | Rejected: shared methods and the actual render field are connected. |
| Draw-layer changes do not invalidate rendering | Rejected: `get_mut_visual` records visual dirtiness. No stale-cache fixture was established. |
| Splitting immediately subtracts the source stack | Rejected as an unproved ordering claim: traced source establishes pending cursor/split/lock, not completed placement. |

Historical no-op `LowerFrame`, absent/retired names, FontString's swapped indented-wrap registrations, old item-backed currencies, current supplied cooldown-enable assumptions and zero history policy remain specifically qualified in the detail documents. Intentional 3D **rendering** exclusion does not automatically exclude model-object API identity/return behavior from future investigation.

No runtime/native/GPU/client-profile test results were produced by this source-only audit. Existing test references describe inspected coverage, not runs performed here. The [Include-depth note](xml-and-dispatch.md#include-depth--unmeasured-boundary-not-a-demonstrated-mismatch) records the unknown practical recursion boundary separately from element nesting and Lua execution.
