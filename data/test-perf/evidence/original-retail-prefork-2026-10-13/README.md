# Original Retail prefork — bounded checkpoint, 2026-10-13

[Independent saved-evidence report](independent/report.md) and [structured proof](independent/proof.json) record **4 PASS / 0 FAIL** in three original Retail fixture groups. Recorded compiled source: `a6723200be40145ec352666b538998bf4035a608`; production: `50f6f0ea8`. These are saved provenance labels, not fresh revision authentication.

## Exact selector matrix

Each saved invocation sets `PREFORK_EXACT_FIXTURE_GROUP` and uses `--nocapture`, not a substring filter or controller fixture. The chat rows share one invocation.

| Exact group selector | Original case | Saved result | Complete streams / result |
|---|---|---|---|
| `chat` | `chat_frame::test_chat_editbox_click_type_and_submit` | PASS | [stdout](runtime-original/chat.stdout), [stderr](runtime-original/chat.stderr), [result](runtime-original/chat.result.json) |
| `chat` | `chat_frame::test_chat_editbox_text_color_after_activation` | PASS | [stdout](runtime-original/chat.stdout), [stderr](runtime-original/chat.stderr), [result](runtime-original/chat.result.json) |
| `cast-bar` | `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` | PASS | [stdout](runtime-original/cast-bar.stdout), [stderr](runtime-original/cast-bar.stderr), [result](runtime-original/cast-bar.result.json) |
| `spellbook` | `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` | PASS | [stdout](runtime-original/spellbook.stdout), [stderr](runtime-original/spellbook.stderr), [result](runtime-original/spellbook.result.json) |

[Aggregate results](runtime-original/results.json) record chat 2/2, cast-bar 1/1 and spellbook 1/1. Actual `systemd-run --wait` launcher exits are **0 / 0 / 0**, separately recorded from test PASS: [chat launcher](runtime-original/chat.launcher.txt), [cast-bar launcher](runtime-original/cast-bar.launcher.txt), [spellbook launcher](runtime-original/spellbook.launcher.txt). Saved argv: [chat](runtime-original/chat.argv.json), [cast-bar](runtime-original/cast-bar.argv.json), [spellbook](runtime-original/spellbook.argv.json).

Full captured runtime stderr has no observed warning/error/panic/failure diagnostics; informational startup/workaround/EditMode traces remain. This is captured-channel evidence, not proof that every runtime error is absent. [Compiler stderr](compile.stderr) retains **six inherited manifest lint-name deprecations**, not warning-free compilation. [Compiler result](compile-result.json) and [terminal state](compile-terminal.txt) record successful compilation; the independent report distinguishes process classification from exit status.

## Retention and limits

[Retention manifest](retention-manifest.json) records **32 complete selected TEXT payloads, 104,446 public bytes**, with consistent redactions and LF normalization. Completeness applies to selected text, not the entire original evidence tree. Giant compiler stdout, executable/library/archive/source-tree/helper/image and private mapping are excluded; saved seal/receipt digest strings are text, not fresh authentication. Current bounded retention PASS after `4308beb6d` historical-path normalization; original verifier FAIL preserved. Prior 32 TEXT transforms/104,446 public bytes, 20 JSON equivalents and 37 links reused without rehash. Local saved followup: `verification/original4-retention/privacy-followup.md`.

[Runtime inputs](runtime-inputs.json) retain active Retail cache as a separate read-only runtime input, not an immutable cache archive or cache-equality proof. No new assertion-body audit, conformance run, broad suite, owning/publication repeat, native GUI/rendering/visual parity, performance/resource comparison, Mists/all-profile or full-parent acceptance. Logged `FIRST_FRAME_RENDERED` does not prove a native rendered frame. Historical RED, Mists cast-bar failure, broad-suite failures and native/full-parent gaps remain unchanged. No build/runtime/operational action accompanies this documentation checkpoint.

[Prefork contract](../../../../docs/specs/prefork-test-harness.md) and [system page](../../../../docs/wiki/systems/prefork-test-harness.md) keep this checkpoint separate from historical proof epochs.
