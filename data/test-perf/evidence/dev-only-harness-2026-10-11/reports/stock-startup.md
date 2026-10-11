# Independent saved-artifact verification

Verified: 2026-10-11. Verdict: **PASS, bounded cached retail startup Lua-error capture only**.

Evidence root: `20261011T053952Z/`. Read complete `lua-errors.stdout` (1 line), complete `lua-errors.stderr` (228 lines, paged without omissions), `submission.json`, `admission.json`, and `outcome.json`. No commands, reruns, builds, operations, delegation, or binary rehash performed.

## Result and exact counts

- Saved argv: `/usr/bin/timeout 90 /home/osso/Projects/wow/wow-ui-sim/target/debug/wow-sim --no-addons --no-saved-vars lua-errors`; override `WOW_SIM_CASC=0`. Saved exit: **0**. Stdout exactly `[]\n`.
- Stderr lines 223–228: `=== Final observed Lua error summary ===`, `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`, `Attributed owners: 0`, `Unattributed Lua errors: 0 occurrence(s)`.
- Startup reports **291 Blizzard addons**, **2482 frames**, bytecode cache **1569/1569 hits**, **0 lookup misses**, **0 replay failures**, **0 stored**, **0 store failures** (stderr lines 48–52). These are runtime-reported counts, not independent cache-content validation.
- Non-Lua diagnostics remain: **8 ALSA error diagnostic lines** (21–28), **1 no-audio-device line** (29). Fallback fonts loaded with `casc=false` (15–16). No claim of warning-free startup.
- **0 automatic Blizzard/vendor acquisition messages or acquisition source URLs in the complete startup outputs**. Blizzard loading is reported, but its vendor filesystem source/provenance is not printed. EditMode explicitly reads WTF cache despite disabled SavedVariables (38, 43). Absence of acquisition messages is not proof of absence of unlogged acquisition.

## Artifact identity and cleanup linkage

`submission.json` records first binary SHA-256 `2ab0b0a47cfd98539c07b5226b23f31b1b590c8debbc508f19d8ce32f03e6d88`, 407301768 bytes, mtime_ns 1791696037890126265, inode 139769639. Hash is recorded evidence, not reverified here. `outcome.json` reports unchanged binary metadata and source inputs; all **9/9** recorded before/after source hashes match by inspection.

Linked source: `../dev-harness-new-input-current/20261011T053714Z/cleanup-compile.stdout`, line **737**: production `wow-sim` compiler-artifact, same package/manifest/source/target/profile/executable and identical **21/21 features**, `fresh:true`. Cleanup `cleanup-result.json` also records `wow-sim` fresh. Cleanup `source-cleanup.json` has **8/8** input hashes matching startup's corresponding inputs, `inputs_unchanged:true`, `probe_absent:true`, `tests_directory_mtime_restored:true`. Retail manifest is the ninth startup seal input, not covered by cleanup's eight-input seal.

Features: aura-containers, aura-instance-enumeration, aura-xml-widgets, base-spell-relationships, casc, client-retail, default, forbidden-aspects, gui, native-duration-formatting, numeric-rule-formatters, on-update-modes, player-cast-durations, prefork-full-ui, profile-retail, retail-12-0-0, retail-12-0-5, retail-12-0-7, retail-12-1-0, rodio, sound. Compiled CASC/GUI features do not mean those paths were accepted by this CLI run.

Admission: available_gib 35.63322830200195, load1 3.81, relevant_cargo `[]` (**0**). This is saved admission state, not current host validation.

Cleanup provenance additionally shows compile exit **0**, `build-finished success:true` (line 743), **6 manifest deprecation warnings** plus 1 aggregate warning-summary line in complete cleanup compile stderr. Integration/prefork were `fresh:false`; production `wow-sim` was `fresh:true`. This is cached production artifact linkage, not a fresh production rebuild. Only relevant root/tail records of the 743-line cleanup JSON stream were inspected; no whole-dependency provenance claim.

## Boundaries

PASS does not establish clean-cache behavior, vendor cache validation, native-client parity, native dependency provenance, GUI/render/audio acceptance, full-addon behavior, all-profile acceptance, or test-suite pass counts. No independent cryptographic seal or exhaustive source-tree seal is established by these metadata files. Existing source-blocked Wrath/Era/Anniversary and Mists RED remain unaffected (caller-stated status; not rechecked).
