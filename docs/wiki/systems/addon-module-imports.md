# Addon Module Imports

`require` is a WoW Forever-only loader import: it returns the first value of an addon Lua file that has already completed, without re-executing a file or consulting a filesystem/package loader. Loader integration landed in `78cf08372`; targeted loader evidence is 10/10 before formatting, extraction, and final verification.

## Content

`src/loader/lua_file.rs` registers each TOC or XML Lua file before execution, then publishes its first return only after successful completion. Missing, self, forward, and failed files therefore have no completed value. Registry roots retain returned values, including `nil`, `false`, tables, and functions, across GC; separate stored prototype metadata retains disk-file caller identity for delayed closures. Dynamically compiled chunks remain dynamic even when their source text resembles an addon path.

`src/loader/module_import.rs` resolves logical `Addon.Module` and leading-dot relative paths without file lookup. Disk-file cross-addon imports need a direct `## Dep:` or `## OptionalDeps:` target, compared ASCII-case-insensitively. Same-addon imports need no dependency. Dynamic callers may use absolute imports without that dependency check, but cannot make relative imports because they have no disk-file origin. The registry returns the documented errors for no completed module, cross-addon escape, and a missing direct dependency.

`src/lua_api/env_init/mod.rs` installs the completed-module bindings before secure-environment copying. `addon_modules::enabled()` uses ACTIVE to select PTR and Forever for both publication and TOC/XML completion tracking. Both receive `import`; only Forever replaces `require` with completed-module lookup. Non-Forever profiles still remove `require` from the insecure sandbox and retain their existing secure binding. This does not add Lua `package`, `dofile`, `loadfile`, or host filesystem behavior.

Native Forever was not run. Multiple returns, module-name normalization, duplicate modules, failed-load replacement, and registration annotations remain source-unspecified and are not native-conformance claims.

## Forever import binding — 2026-10-15

The completed-module registry and `require` already work. Vendor `Blizzard_GamepadSmartNavigation` orders `Utility.lua` before consumers `SmartNavigation.lua` and `Debug.lua`; Utility returns a table, while both consumers call `import('.Utility')`. Commit `012df1e45` adds a direct Rust `import` registration using the same backend, only under `client-wowforever` at the existing pre-secure-copy seam. Current runtime-code `c2bf42c4f`, built at `57f8e0b10`, has saved bounded compilation/execution; independent bounded reconciliation PASS. No stub, new loader, autoload, host-package fallback or changed `require` contract.

Reused completion, dependency, identity, grammar and error behavior is **simulator compatibility policy** justified by those vendor calls. Native `import=require` semantics, identity, grammar and error parity are UNKNOWN; native new-test authorship alone is not execution proof.

| Capability | Evidence / remaining gate |
| --- | --- |
| Completed Utility versus absent binding | Actual source `136748f0b` probe first obtains a table through `require('Blizzard_GamepadSmartNavigation.Utility')`, then fails `type(import) == 'function'`: `IMPORT_BINDING_RED: completed Utility table; import=nil`. Process exit0 is not assertion PASS. |
| Existing backend | Historical pre-binding controls remain historical; current groups are separately executed, not additive. |
| New import consumers and restrictions | Saved current loader/sandbox/registry/path execution; normal probe confirms completed Utility identity and callable vendor method returning nil. No gamepad-root completion claim. |
| Separate models/startup | Earlier source136 model/render/GPU evidence remains separate; current startup is captured but failing. |
| Binding acceptance | Independent bounded report accepted; native parity, cross-profile and broader acceptance remain open. |

[Central receipt](../../baselines/README.md#current-importevents-execution--2026-10-15) owns current evidence root, exact counts, binary inventories, probe markers, failing startup and accepted independent bounded report. Historical RED is retained under the earlier receipt linked there. The [separate enum follow-up](../../specs/forever-finite-constants.md#provider-and-experience-enums--source-checkpoint-2026-10-15) owns its pending gates; actual new gamepad root failures stay open. Baseline JSON copying remains main-owned.

## PTR original consumers — 2026-10-25

The [contract](../../specs/addon-module-imports.md#ptr-original-module-consumers) cites the pinned PTR TOC, original imports and original Utility return. PTR previously executed Utility without recording its return and did not install `import`. The repair reuses `ModuleRegistry` rather than creating another model or patching SmartNavigation. Unsupported profiles keep an absent registry; no filesystem fallback is added.

`tests/ptr_module_import.rs` discovers the game-screen addon and loads its original dependency closure. It compares both consumers' captured Utility table with the completed absolute import, checks identity after GC, and calls original `RefreshButtonGroups` before and after removing a real child button. The negative fixture retains byte-identical original source but leaves its on-disk Utility outside the TOC, so the original first-line import must reject it. Nil, empty-table and no-op producers cannot satisfy these assertions. Initial execution reaches both missing-import lines; final paired producer-only RED/GREEN, stock and cross-profile gates remain pending.

C_GamePad publication and event registration are separate gaps, not part of this loader repair. Native import grammar/error parity remains unknown.

## Sources

- [Addon module imports spec](../../specs/addon-module-imports.md) — contract and explicit proof limits.
- [Forever require contract](/tmp/wowforever-require-contract.json) — Warcraft Wiki revision 6879760 capture.
- `78cf08372` — loader completion and profile-exposure integration.
- [Module registry](../../../src/loader/addon_modules.rs) and [resolver](../../../src/loader/module_import.rs) — runtime mechanics.

## See Also

- [[addon-loading]] — TOC/XML file execution.
- [[client-profiles]] — profile-specific publication.
