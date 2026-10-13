# Saved Speak profile GREEN runtime verification

Date: 2026-10-13. Method: read-only inspection of the two selected new GREEN epochs; no commands executed, tests replayed, builds, hashes computed, operations, delegation, or canonical source edits. Only this report and sibling proof.json written. Verification skill followed as the assigned verifier using saved command/artifact evidence, not fresh execution.

Base: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current`.

## Actual saved runtime results

| Profile | Epoch / runtime | Actual wait exit | Exact stdout result | Saved service result |
|---|---|---:|---|---|
| WoW Forever | `speak-forever-green-20261013T162416347262Z/runtime` | 0 | 2 passed; 0 failed; 0 ignored; 0 measured; 7998 filtered out; 0.81s | success; exited 0/SUCCESS; 1.026s |
| Mists | `speak-mists-green-20261013T161805959768Z/runtime-retry2` | 0 | 2 passed; 0 failed; 0 ignored; 0 measured; 7859 filtered out; 0.38s | success; exited 0/SUCCESS; 450ms |

Mists retry2 was already present and complete when inspected; no synchronous polling. Each stdout names exactly `voice_speak_for_me_profile_input::documented_profile_query_returns_one_false_boolean` and `voice_speak_for_me_profile_input::documented_profile_retained_query_reads_live_isolated_activity`, both `ok`.

Complete stdout, stderr, launcher.txt, argv.json and result.json inspected. Runtime stderr: Forever 1918 bytes, Mists 1925 bytes; three WowLuaEnv construction traces each, no warning/error/panic/failure markers. These traces are not full Blizzard startup acceptance.

Saved argv executes the selected copied integration binary, filtered to `voice_speak_for_me_profile_input::`, with `--nocapture --test-threads=1`, via actual `systemd-run --wait`, 86-second timeout, bwrap read-only root and saved source overlay, read-only target overlay, unshared network, temporary /tmp, CASC disabled and sound disabled. Launcher/result agree with stdout, not merely submission success. Complete artifacts are embedded in proof.json.

## Reached assertion coverage (both profiles)

Saved `source/tests/voice_speak_for_me_profile_input.rs` is text-identical between epochs. Actual named PASS results plus its sequential `.unwrap()` boundaries establish completion of these assertions; no per-assertion instrumentation is claimed.

| Assertion / behavior | Saved lines | Evidence / limitation |
|---|---|---|
| Direct query arity, type, supplied false | 9–19, especially 14–16 | One return, boolean, false after explicit Rust state assignment. Does not establish native or simulator initial default from this test. |
| Retained query reaches supplied true | 24–39, especially 27, 29, 33–36 | Function captured before mutation; retained call returns exactly one boolean true. Direct query true is not separately asserted. |
| Environment isolation | 24–25, 40–42 | Second environment's direct query remains false while first is true. No reverse-direction/concurrent isolation claim. |
| Retained live false restoration | 43–46 | Same captured function returns false after first state is reset. Restoration arity/type are not separately asserted. |

Saved production `source/src/c_api/c_voice_chat_activity.rs` is also text-identical: lines 11–14 publish for Mists/WoW Forever; line 20 registers `IsSpeakForMeActive`; lines 23–26 read current `voice_chat.is_speak_for_me_active`, push a Bool and return one value. Both saved `src/c_api/registration.rs:56` call registration. Rust state injection is the producer in these tests; transport/event producers and native activity remain outside proof.

## Compiler and receipt linkage

Entire compiler stdout parsed: Forever 387 JSON records (298324 bytes); Mists 385 records (294855 bytes). Each ends with `build-finished.success=true` and has an integration artifact matching the copied binary basename. Full compiler stderr inspected and retained verbatim in proof.json: each has six iced manifest deprecations, seven library warnings, one wow-sim unused-import warning (14 total). Compilation is not warning-clean.

Forever source revision: `4eadd7cf3ef84581ad35967ee6760951939ec438`; Mists: `f835b158a57620662f73ac5c6ed13a68a353532c`. Both seals/results name production query revision `2630c0ccf`. Saved compile summaries/receipt fields marked runtime unrun describe their earlier compilation/authentication phase, not the later actual wait results.

Compiler features establish Forever: `aura-containers`, `aura-instance-enumeration`, `aura-xml-widgets`, `base-spell-relationships`, `client-wowforever`, `forbidden-animation-aspects`, `forbidden-aspects`, `native-duration-formatting`, `numeric-rule-formatters`, `on-update-modes`, `player-cast-durations`, `timed-signal-maps`. Mists: `client-mists`, `prefork-full-ui`. Saved Cargo.toml:156 and :159 define those profile roots. Neither compiler artifact enables sound/gui/casc. Forever compile-result explicitly corrects the original revision.json Mists label; submission argv and actual compiler feature record establish Forever, not that stale label.

For each copied executable, observed current stat equals seal stat, receipt before/after stat and runtime `receipt_stat_reused` exactly:

- Forever: dev 66306; inode 142239676; size 280863016; mtime_ns 1791908883108711621.
- Mists: dev 66306; inode 142117691; size 279025544; mtime_ns 1791908492868605400.

Saved receipt status PASS and saved digest fields agree with each seal; no digest recomputed and no executable bytes read. Stat equality supports receipt reuse, not a fresh content authentication. Source trees, archives, system libraries and external caches/settings are not independently authenticated by this inspection.

## Preservation and verdict boundary

Original RED/setup/compiler failure packets and Mists retry1 denial were not reopened or modified; this report does not supersede or reinterpret them. Selected Forever compile-result also retains its archive-setup correction. New GREEN success applies only to these two tests in each selected runtime, not earlier failures.

**PASS — saved query-only GREEN runtime evidence: Forever 2/2 and Mists retry2 2/2, actual wait exit 0 each.** No CLEAN, native parity/default/event producer, full startup, broader suite, consumer integration, or parent-goal completion claim.
