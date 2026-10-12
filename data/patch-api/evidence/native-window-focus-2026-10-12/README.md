# Native window focus — bounded keyboard acceptance

[Proof](proof.json) · [Retention](retention-manifest.json) · [VNC operation](../private-vnc-simulator-2026-10-12/README.md) · [Earlier Retail dispatch proof](../edit-mode-enter-2026-10-12/README.md)

## Cause and correction

The current authenticated `7d9d9f621` app received no Enter while X11 focus remained PointerRoot. Neither WoW chat nor the simulator `/command` field responded. Assigning native focus to its mapped window, without changing app/VNC code or bindings, produced **five Enter deliveries** (ten receipt/dispatch log lines). User confirmed **“Both now work”**; layout had already been accepted.

Launch setup now waits for the unique simulator title, maps/resizes/focuses the window and verifies focus. The old resize-only setup has a [genuine configuration RED](setup-red.stdout), exit1 ([error](setup-red.stderr)); this is not an application keypress RED. Fresh startup ran the new helper successfully without a subsequent manual focus change. Native XTest Enter plus four controlled fixture letters reached the app: chat focus false→true, length0→4, exact-fixture match true. One Enter delivery has two log lines, not two tests. Independent ownership/socket/focus/geometry/wiring gate PASS.

## Lifetime and limits

User authorized stopping the app after each test instead of automatic90s expiry. Automatic expiry removed; CPU200%, memory4GiB and whole-control-group teardown retained. Main stopped both human and fresh automated test runs after completion; listener/display preserved. No driver, VNC exposure, original WTF, or vendor changes; no new artifact hashing.

Earlier historical-artifact focus-only failure, zero-delivery XTest attempt, invalid binding-lookup normal probe, unsupported diagnostic getter and expiry black screen remain distinct retained observations. They are not discarded or relabeled application GREEN. Earlier public gate’s user-acceptance exclusion remains historical; this later packet supplies only bounded current layout/chat/command-field acceptance.

No universal native/pixel/all-key/all-profile, causal-performance, command-execution or whole-handoff acceptance. Public packet retention/link/privacy gate pending.
