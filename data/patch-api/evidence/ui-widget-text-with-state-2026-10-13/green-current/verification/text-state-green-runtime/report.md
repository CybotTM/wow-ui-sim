# TextWithState saved GREEN/runtime/startup/check verification

Date: 2026-10-13. **PASS — bounded Retail/PTR TextWithState GREEN, PTR voice-query preservation, both normal startup probes and default Cargo check. No parent acceptance.**

Verification skill followed as independent verifier. Saved evidence only: no replay, builds, commands, hashes, authentication, operations, delegation, commits or canonical source edits. Writes limited to this report and `proof.json`. Complete runtime stdout/stderr/argv/launcher/results/admissions, complete compiler JSON/stderr, source snapshots and reused authentication metadata are embedded verbatim in the nine-entry proof ledger. Compiler JSON was parsed in full, not sampled.

## Scope and attribution

Base: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current`.

- RetailE: `text-state-retail-green-20261013T165158239706Z`.
- PTRE: `text-state-ptr-green-speak-preservation-20261013T165158437446Z`.
- CheckE: `text-state-default-final-check-20261013T170720415919Z`.

Both runtime source labels are `2bb6cdde16af4f49ada296b7a8f2e2509de38930`. Original `revision.json` incorrectly labels production delta `50f6f0ea8`; each separate `revision-delta-correction.json` identifies TextWithState producer `2bb6cdde1` and voice query `2630c0ccf`. Originals preserved. Compiler/auth records saying runtime pending are earlier phase descriptions, not the later runtime result. Saved test comments saying input-only/unexecuted/unreached/prospective likewise do not override actual executed PASS results.

Attribution is metadata, **not source/archive byte authentication**. Selected saved test/helper source text compares equal across RetailE/PTRE. External caches, settings and libraries are not authenticated.

## Exact completed execution ledger

| Epoch / runtime | Passed / failed | Ignored / measured | Filtered | Harness seconds | Actual wait exit | stdout / stderr bytes | stderr lines |
|---|---:|---:|---:|---:|---:|---:|---:|
| RetailE / runtime-text-state-green | 7 / 0 | 0 / 0 | 10977 | 5.96 | 0 | 942 / 5134 | 88 |
| PTRE / runtime-text-state-green | 7 / 0 | 0 / 0 | 10154 | 5.64 | 0 | 942 / 5127 | 88 |
| PTRE / runtime-speak-query | 2 / 0 | 0 / 0 | 10159 | 1.00 | 0 | 328 / 1926 | 33 |
| PTRE / runtime-transcription | 3 / 0 | 0 / 0 | 10158 | 0.97 | 0 | 403 / 2564 | 44 |
| RetailE / runtime-normal-startup | probe, not Rust tests | — | — | — | 0 | 65 / 14710 | 219 |
| PTRE / runtime-normal-startup | probe, not Rust tests | — | — | — | 0 | 65 / 14686 | 219 |

Exactly **19 named tests passed** across these four test executions; startup probes are not additional tests. Every named result is `ok`; each actual `systemd-run --wait` launcher reports success and process status `0/SUCCESS`. Service runtimes, respectively: 6.148s, 5.803s, 1.098s, 1.019s, 6.764s, 7.238s. Byte lengths match saved results. Full runtime stderr has zero warning, panic, traceback, emitted `[Lua Error]`, budget-error or addon-load-failure matches under the explicit patterns retained in proof. Text-state stderr records eight VM constructions per profile, not eight tests; PTR Speak/transcription record three/four constructions.

All saved admissions say `allowed=true`; runtime limits are 4GiB max / 3GiB high, CPU quota 200%. Caller supplied admission waited0; files contain no wait-duration field, so zero is **not independently timed**. `--wait` is service-completion waiting, not admission latency.

Saved arguments select each epoch's copied executable, filter plus `--nocapture --test-threads=1`, timeout86s with TERM/kill-after2, read-only root/source/target overlays, unshared network, private tmpfs, `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`. Normal runs additionally use `--no-addons --no-saved-vars --exec-lua ... lua-errors`. These are saved restrictions, not newly executed commands.

## Reached TextWithState coverage — both profiles

Proof level: each exact named PASS plus sequential saved Rust/Lua assertions establishes reached behavior; no per-assertion instrumentation claimed. References below are relative to each epoch's saved `source/`.

| Named case / source | Reached behavior |
|---|---|
| queries::text_state_complete21_fresh_snapshots_unknown_nil_replace_remove; `tests/widget_text_state/queries.rs:7-77` | Complete supplied snapshot; distinct fresh table; caller mutation cannot alter original snapshot/map/new result; exactly one unknown nil for 760802 and 1760801; replacement leaves prior snapshot unchanged; explicit visualization deletion returns one nil. Replacement preserves negative padding/order, fractional width/effect ID, query-only kits and timer true without consumer clamping. |
| queries::text_state_numeric_ids_and_strict_widget_id_arguments; `queries.rs:80-112` | IDs -2.5, 0, 617.25, 4294967299 stay distinct, -0 aliases0; -2/617/3 absent. Missing/nil/string/bool/table/function arguments fail with `widgetID must be a number`. No native numeric-range claim. |
| queries::text_state_same_id_inputs_are_environment_local; `queries.rs:115-145` | Empty second environment, independently supplied second value, unchanged complete first snapshot. |
| security::text_state_real_secret_ids_allow_untainted_deny_tainted_recover; `security.rs:9-71` | Actual host secret NUMBER wrappers, supplied and absent IDs. Untainted reads allowed, returned table/keys/values public; tainted calls to both secret IDs rejected with untainted-caller error and no secretunwrap; public numeric IDs still readable; taint clears on return and fresh untainted reads recover without caller mutation affecting retained/future results. |
| feed::text_state_feed_commits_before_events_retains_visualization_on_membership_removal; `feed.rs:46-100` | Typed feed commits record/membership before real UPDATE_UI_WIDGET callback; exact ID760801/set428/type8 payload; initial snapshot immediately observed, then updated snapshot distinct and old text retained; exactly2 updates/0 full updates before removal, then exactly1 full update/2 updates. First removal true, absent removal false, empty membership while visualization survives. |
| feed::text_state_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification; `feed.rs:103-141` | Unknown set429 and mismatched type19 rejected; no input/membership mutation or notification, both event counts0 and no invented set429. This remains the historical RED run's sole passing case. |
| consumer::text_state_cached_type8_activation_update_hidden_restore_membership_removal; `consumer.rs:145-185`, helpers `:11-143` | Actual active-cache dependency closure, no overrides, Blizzard_UIWidgets loaded, type8 enum/registration and real UIWidgetContainerTemplate constructor; real frame-event updates and OnUpdate layout; activation → incremental update → Hidden release → restoration → membership removal. |

`tests/widget_text_state/fixtures.rs:52-88` compares every field's type/value, rejects unexpected keys, and asserts **exactly21** keys (`:82`):

```text
15 numbers: shownState enabledState textSizeType fontType bottomPadding
            tooltipLoc hAlign widgetSizeSetting orderIndex inAnimType outAnimType
            widgetScale layoutDirection modelSceneLayer scriptedAnimationEffectID
5 strings:  text tooltip textureKit frameTextureKit widgetTag
1 boolean:  hasTimer
```

Cached consumer assertions include real font measurement >1, width180 then206.25, text/tooltip, LEFT→RIGHT alignment, White→Green enabled state/color, normal→shadow font path/size/flags, Top tooltip location, scale1, vertical layout, order12.5→9.25. Height uses unchanged template `textHeight + Clamp(padding,0,textHeight-1)` including supplied negative padding -3.75. Incremental update retains frame identity and pre-consumer snapshot; Hidden removes/hides frame while getter reports Hidden; restoration shows real pooled template with restored text; membership removal hides/releases frame while getter retains restored record. Pool-active and widgets-showing counts are exactly **0→1→1→0→1→0**; every layout flush asserts not dirty, numTimers0, ticker nil. These are simulator Lua/layout properties, not rendered pixels/native behavior.

Saved `tests/common/blizzard_addon_closure.rs` loads discovered cache dependencies into isolated Game environment; it does not clear errors in this path. Shared `tests/widget_discrete/fixtures.rs:40-48` asserts the **entire `env.state().lua_errors` vector** empty after every successful assertion phase. Consumer thus checks that collection after cached registration/setup and each lifecycle/layout step; not merely a filtered widget error subset. This is not a separate assertion of `lua_error_records`/`lua_error_counts`, nor full normal startup acceptance.

## PTR voice preservation and historical Retail boundary

PTRE Speak2PASS reaches default exactly one false boolean, retained live query and stale retained value, independent environments, independence from supplied transcription/channel/connection changes, and false restoration. PTRE transcription3PASS reaches exactly one false boolean, retained live input/environment isolation, independent permission/channel snapshots including snapshot mutation/community stream, channel clearing and disconnected-input preservation. Saved source: `tests/voice_speak_for_me_active_input.rs`, `tests/voice_transcribing_input.rs`.

**Those PTR voice tests do not assert the whole recorded Lua-error collection**: they use eval/exec unwrap. No whole-collection or current PTR cached voice-consumer acceptance inferred. Full stderr inspection establishes only observed diagnostic output.

Prior accepted Retail voice **6PASS at64a2a57b2**, caller reference3819, remains a separate revision scope: 2 Speak queries +3 transcription +1 cached consumer. Its epoch-local `verification/retail-speak-preservation/report.md` is referenced/retained in proof, not rerun or promoted to a cached voice-consumer claim at2bb. Genuine TextWithState RED **1PASS/6FAIL, wait101 at64a** remains unchanged in `verification/text-state-genuine-red/`; GREEN does not retroactively erase those failures.

## Separate normal CLI startup probes

Exact asserted query is `C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801)` on empty supplied input: function exists, **one return and nil**. `C_VoiceChat.IsSpeakForMeActive()` and `C_VoiceChat.IsTranscribing()` each exist, return **one boolean false**. Both full65-byte stdout streams are exactly:

```text
TEXT_SPEAK_STARTUP\tfunction\t1\tnil\tboolean\tfalse\tboolean\tfalse
[]
```

Here `\t` displays literal tab separators in the saved stream. Full stderr for each ends explicitly:

```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

**CLEAN is reported by inspected stderr, not inferred from [] or exit0.** Saved profile/features/normal-deps association establish Retail/PTR labels; the stdout marker itself carries no profile tag. Full stderr markers show Retail **291 Blizzard addons**, bytecode cache **1569/1569 hits**; PTR **283 Blizzard addons**, **1568/1568 hits**. Both say lookup_miss0/replay_fail0/stored0/store_fail0, Sound disabled, casc=false, SavedVariables disabled and third-party Addon loading disabled. Counts are 291/283, not presumed281. Cache contents/settings are external and unauthenticated; these probes do not prove all addon combinations/full-parent acceptance.

## Compilation, default check and earlier format gate

| Scope | Full JSON records | Record composition | Successful build-finished | Source compiler messages | Terminal evidence |
|---|---:|---|---|---:|---|
| RetailE no-run integration compile | 741 | 665 artifacts,75 build scripts,1 finish | true | 0 | Existing saved auth systemctl: Result success, SubState exited, MainPID0, ExecMainCode1, ExecMainStatus0 |
| PTRE no-run integration compile | 741 | 665 artifacts,75 build scripts,1 finish | true | 0 | Same saved terminal fields |
| CheckE default library/binary check | 733 | 658 artifacts,74 build scripts,1 finish | true | 0 | Saved main-owned terminal observation: SubState exited, MainPID0, ExecMainStatus0; main notified completion |

Default check completed, not deferred: `cargo check --offline --locked -j4 --message-format=json-render-diagnostics`, source label `91b93e78efb13edb0ce96f0a5029d7e23c84f7fb`, production delta2bb; stderr finish28.56s, saved peak4750823424bytes. Launch exit0 alone was not treated as completion. No polling/systemctl/check rerun by verifier. Separate default check does not replace PTR/test-profile coverage.

Each full compile/check stderr retains **six iced-wgpu-patched deprecated manifest keys** (`large-enum-variant`, `map-entry`, `match-wildcard-for-single-variants`, `redundant-closure-for-method-calls`, `trivially-copy-pass-by-ref`, `type-complexity`) plus one aggregate six-warning line: seven warning lines, six deprecations, zero JSON compiler messages. Not warning-clean. Retail/PTR compiles each also record three package-cache lock waits; PTR additionally one build-directory wait. Toolchain metadata: Arch cargo/rustc1.99.0, x86_64-unknown-linux-gnu. Source format PASS3821 (`verification/text-state-producer-source/`) is reused at its bounded source scope; no formats rerun.

## Authentication reuse and exclusions

Existing3828 receipt evidence covers four new copied executables, each previously hash-read once. This verifier performed **zero new hashes, byte-authentication or executable byte reads**. Saved runtime receipt-stat tuples match saved before/after receipts. Unique normal-deps associations: Retail `wow_sim-a69cd69216e5522c`; PTR `wow_sim-616324da1af8e7f2`. Matching compiler features, fingerprint, size/nanosecond mtime are **metadata association**, not cryptographic source/original-binary binding. Saved ldd exited0 for all four copies; library bytes are not authenticated. No mutable target/debug/wow-sim authentication performed.

PASS applies only to these saved executions/checks and reached assertions. No native parity, CASC asset resolution, audio, rendered/pixel behavior, full suite, other-profile TextWithState publication, current Retail cached voice-consumer preservation or parent-goal completion claim.
