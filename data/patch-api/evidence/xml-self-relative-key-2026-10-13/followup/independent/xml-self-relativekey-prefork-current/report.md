# Saved-only named Mists cast-bar proof after XML480

**PASS, narrow scope: 1/1 named case; 0 failed; no ignored case; launcher/main exit 0.** Existing runtime evidence, not a fresh run. No native, rendering, full-prefork or full-parent acceptance.

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/xml-selfkey-mists-prefork-20261013T223641907291Z`.
Declared source: `2d4b7e56a06ea0fbb400a37f2aabb819337e5872`. Production delta: `480b9f934`. Do not relabel the source as 480: local Git metadata shows two subsequent documentation commits (ed3d7b711, 2d4b7e56), with eight Markdown documentation/evidence files changed and no code delta. No source-to-binary reauthentication.

## Saved selection and counts

`runtime-named-cast-bar/argv.json`: PREFORK_EXACT_FIXTURE_GROUP=cast-bar, WOW_PRODUCT=wow_classic, --exact --nocapture --test-threads=1, --unshare-pid and --unshare-net; CASC=0, sound disabled; timeout 86s. Read-only saved-source overlay. `artifacts/wow-sim` is the copied prefork_full_ui test executable, not proof of normal wow-sim execution.

Full stdout: 148 bytes, four lines:
```
running 1 tests
test spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix ... ok

test result: ok. 1 passed; 0 failed; 1 total
```
`launcher.txt`: success; main exited status=0/SUCCESS; service runtime 16.313s. `result.json` launcher_exit=0. Saved admission alone is not test proof.

Original `summaries: []` is preserved verbatim in proof.json's original_result. Helper runtime-sealed-command-resumed.py matches standard libtest summaries requiring ignored/measured/filtered fields, while tests/common/prefork.rs:345 prints passed/failed/total. This mismatch is a parser gap, not missing execution. Added actual_parsed_proof: 1 passed, 0 failed, 1 total. No ignored case is source-inferred, not a fabricated stdout field: Case has only name/test (:23-31), exact selection chooses the registered case, and the runner invokes its body without an ignore branch. Helper and original receipts remain untouched.

## Source/harness contract and capability matrix

All source references are to epoch source/, inspected read-only.

| Capability | Exact coverage | Proof |
|---|---|---|
| Actual fixture/body | prefork_full_ui.rs:238-241 registry; :301-321 main env-group dispatch; :370-382 cast-bar constructor; :455-485 prerequisite conformance/setup. spell_casting.rs:138-163 original full-UI constructor. common/mod.rs:115-131 callable macro; common/prefork.rs:447-475 executes selected.case.test then CHILD_PASS | Saved named PASS, not setup/conformance count |
| Unlocked startup layout | spell_casting.rs:470-474 lock=0, unattached | Selected body returned; assertions passed |
| Settled idle state | :449-454 Mists fire_on_update(0.55) BEFORE assertions; :494-506 hidden, parent and layoutParent identity equal UIParentBottomManagedFrameContainer, absent from showingFrames | Body PASS plus stderr:109 matching phase probe |
| Anchor behavior | :526-530 player anchor pcall succeeds and remains unattached; :557-565 locked cast anchor pcall succeeds, attached, parent name PlayerFrame | Body PASS; no individual assertion logs |
| Missing/excluded | Immediate-hidden acceptance, other cast/spell tests, native/render/full-prefork/full-parent | Unproved; historical immediate-hidden FAIL remains |

Eleven applicable Mists assertions. Non-Mists branch and normal integration wrapper not separately executed. Source comment describes 0.2s delay + 0.3s fade, not independently authenticated native timing. No forced Hide/replayed anchors were introduced by this verification. Prerequisite conformance is not aggregated into the one-case count.

## Complete stderr inventory

Read all 12,387 bytes / 109 lines in contiguous ranges 1-85 and 86-109. Exactly 103 Startup/Workarounds/EditMode timing lines and six MISTS_CAST_PARENT probes; zero unclassified lines.

Probe inventory: :12 after-addon-load/before-post-load-workarounds; :91 after-post-load-workarounds/before-startup-events; :106 before-EDIT_MODE_LAYOUTS_UPDATED; :107 after-EDIT_MODE_LAYOUTS_UPDATED; :108 after-startup-events; :109 cast-case-before-original-assertions.

Three READ_ERROR observations (:12,91,106) concern systemInfo.isInDefaultPosition while systemInfo=nil. Probe pcall captures these; they are not body panics. Before initialization: shown=true, parent UIParent, layoutParent bottom container, no showingFrames membership. After edit-mode event and startup (:107-108): initialized/default-position, shown=true, parent/layoutParent bottom container, membership present. After 0.55 settlement (:109): shown=false, unattached, same parent/layoutParent, membership absent.

Zero Lua-error headers, self-anchor emissions or panic headers in this saved stderr. Do not call all startup clean: constructor discards load_addon results and several event results. Three diagnostic read errors remain. No compile warning inventory claimed by this runtime-only audit.

## Historical comparison — scopes preserved

844a63adb95c7111e1286fa668e632f7cd237806 epoch runtime-original-cast: same selector, 1/1 PASS, launcher0, stdout148, stderr13170. Prior complete saved-proof report records the same 0.55 settlement before hidden assertions and two self-anchor setup-error emissions. Current stream has none. This is an observed saved-log difference after XML480, not independent causal/native proof.

845bea6d5 epoch 20261010T084353Z: original immediate-hidden named case remains FAIL, 0 passed / 1 failed / 1 total. Direct saved stdout says FAILED (panic: Mists startup cast bar should remain hidden); stderr:120-121 locates tests/spell_casting.rs:489:13. Before that assertion, IsShown=true. Current phase-adjusted PASS does NOT convert this unchanged historical immediate-hidden contract to PASS, erase it, or establish original 3/3 acceptance.

## Actions, provenance and outputs

Read saved runtime/source/metadata and existing historical reports; local Git show/log/diff only. Initial Git metadata calls lacked explicit cwd and failed without mutation; corrected read-only calls succeeded. No build, launch, auth/rehash of executables or old epochs, replay, source edit, commit, delegation, cache mutation or operational action. Saved authentication receipt was read, not recreated; independently authenticated external inputs remain excluded.

Only report.md and proof.json written under the requested directory. Both mode0600, individually fsynced, directory fsynced, then completely read back with exact byte equality and mode checks. Original result summaries[] preserved; parsed 1/1 added separately. No broader acceptance claim.
