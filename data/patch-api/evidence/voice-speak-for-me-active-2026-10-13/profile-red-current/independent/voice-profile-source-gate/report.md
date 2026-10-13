# Voice profile source gate — PASS (source/format/readability only)

Revision: `2630c0ccf71d8b41f0c82b4769beda6f77129ab3`. HEAD verified exact; scoped `git diff --exit-code 2630c0ccf --` over four Rust files, spec and profile test exits0, empty stdout/stderr. No tracked edits.

## Command proof (full outputs)

Working directory `<REPO>`.

- `rustfmt --edition 2024 --check src/c_api/c_voice_chat_activity.rs src/c_api/c_voice_chat_channels.rs src/c_api/mod.rs src/c_api/registration.rs`: exit0, stdout empty, stderr empty. Default child-source traversal enabled, no skip_children.
- `cargo fmt --check`: exit0, stdout empty, stderr empty.

Fresh commands at 2630 replace earlier invalidated proof. Results copied from completed session tool outputs; no reruns to recover logs.

## Artifact gate

[EXIST] PASS — activity 27 lines; channels135; mod426; registration136; spec present/read.
[SUBSTANTIVE] PASS — activity.rs:10–21 exhaustively matches ACTIVE; :23–27 reads only independent voice_chat.is_speak_for_me_active, pushes Val::Bool(active), returns Ok(1). Getter body identical to removed getter. No inferred permission/transcription/channel value.
[WIRED] PASS — globals/register.rs:234 calls missing_surface::register_all. missing_surface.rs:76–82 runs item-trait registration before social registration; :172–176 reaches interaction; :217–218 calls c_api::register_interaction_tables. c_api/mod.rs:357–361 reexports registration; :210 declares ungated activity module. registration.rs:56 calls register. activity.rs:11–14 applies exact profile publication policy.
[ANTI-PATTERN] PASS — TODO/FIXME/HACK/XXX/#[allow each zero across four files; added patch has no logging, fallback/default substitution, suppression or privacy payload.

## Existing surface preservation (source only)

missing_surface.rs:277 later calls voice_chat::register_voice_chat_surface; :278–279 retains retail-12-0-5 gate on channel registration. Core voice_chat.rs:29–60 installs exactly17 distinct keys, none IsSpeakForMeActive. Its namespace helper missing_surface.rs:553–581 reuses existing Val::Table. New activity uses c_api/helpers.rs:49–77, also reusing existing table. table_builder.rs:47–79 installs only the named function key, without clearing/replacing the table. Later17 registrations preserve activity; activity registration preserves existing namespace entries.

Channel file retains IsTranscribing, GetChannelForChannelType, GetChannelForCommunityStream implementations and registrations unchanged. Diff only removes old activity registration/getter. Module gate c_api/mod.rs:211–212 and caller gate missing_surface.rs:278 unchanged. Exactly one explicit Rust activity registration found at activity.rs:20. Commit inventory is five files; no vendor override.

## Coverage matrix

| Profiles | Registration | Proof / missing |
|---|---|---|
| Retail / PTR | cfg!(retail-12-0-5) | Source verified; current runtime preservation not run |
| Mists / WoW Forever | true | Source verified; current GREEN unverified |
| Wrath / Era / Anniversary | false, no new registration | Source verified; native contracts unknown |

No native presence/absence claim for unknown three; generic namespace lookup may still manufacture values. Spec distinguishes simulator false policy from native defaults and records unknown native contracts. Cached documentation not remeasured in this gate.

## Manual readability

No changed-code violations. New register short, exhaustive named decision plus early return; query three body lines. No deep nesting, parameter overload, state accumulation, opaque conditions or unexplained literals. Mutable registration explicit; borrow_state clearly exposes state read. Existing orchestration remains named calls; unrelated pre-existing function lengths not new findings. No new suppression, fallback, logging/private values or vendor changes. Manual review only; no compiler/lint or warning-free claim.

## Historical RED, no current GREEN

Two tests in tests/voice_speak_for_me_profile_input.rs read; gated Mists/Forever. Prior accepted verification/speak-mists-query-red/proof.json binds abb979c4465efcf75c6731dd08077e90abaf5509: actual Mists0 PASS/2 FAIL, wait101. Both arity checks passed then boolean assertions explicitly got nil. False/true-value, isolation and restoration assertions unreached. Initial bwrap mountpoint setup wait1 executed zero tests, not RED. Earlier be2 E0603 compilation not RED. Historical independent report reused, no fresh runtime/authentication.

No GREEN at2630 claimed. Main async runtime gate owns acceptance; current Forever/Retail/PTR runtime preservation remains outside assignment. Earlier Retail evidence revision-scoped.

## Limits / setup errors

No cargo check/test/build/runtime/hash/auth/ops/delegation/commit/tracked edits. Only requested untracked artifacts written. Initial cwd-less git exit128 corrected via explicit cwd; initial formatting evaluation undefined repo failed before executing commands. Glob fd unavailable; known-path read used. Initial report evaluation lost ctx.voice_proofs before writes; completed command outputs transcribed from session without rerunning them.
