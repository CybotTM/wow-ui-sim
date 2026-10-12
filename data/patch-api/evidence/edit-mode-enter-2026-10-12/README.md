# Edit Mode selection and Enter — bounded Retail proof

[Selection contract](../../../../docs/specs/edit-mode-layout-selection.md) · [Chat contract](../../../../docs/specs/enter-chat-binding.md) · [Commands](command-proof.json) · [Retention](retention-manifest.json)

## Coverage

| Boundary | Actual proof | Limit |
|---|---|---|
| Original Enter activation RED | [Default failed; two override/unbind controls passed](enter-red.stdout) | Typing/submission unreached in RED |
| Fresh cache import before whole Game startup RED | [One selected-layout failure](selection-red.stdout) at `653e829f1` | Geometry/final warnings assertion unreached; initial `4fedbf24c` compile error has zero RED credit |
| Whole startup selection GREEN | [Two names](startup-green.stdout) at `7d9d9f621`: native2→Classic/horizontal; one override case executes four explicit name/case/ordinal/ID variants→Widescreen/vertical | Parent/child recaps count once; no PTR execution credit |
| Cache/env/snapshot preservation | [10/10](cache-green.stdout) | Low-level model/parser proof, not native account parity |
| Actual simulator keypress dispatch | [Enter3/3](enter-green.stdout): activate/type/submit, override, unbind | Not native/VNC delivery or intended-user acceptance |

Corrected normal startup [reports Classic329×40 and `[]`](normal-corrected.stdout), exit0/zero Lua errors. The earlier [invalid normal probe failure](invalid-binding-lookup-probe.stdout) is retained: it incorrectly assumed `GetBindingAction` enumerates simulator convenience defaults. Empty lookup does not disprove default dispatch. That probe exit1/two unique errors is not an application regression or a valid acceptance test.

`cargo fmt --check` and default `cargo check --offline --locked -j12` each exit0 once; check26.81s. Independent runtime/spec/readability gate passed at the recorded 16-file source scope. Three newly compiled seals each authenticated once; no old executable/library rehash. Six external iced-wgpu manifest deprecations remain; not warning-free. Compile memory-limit change grants no causal-performance credit.

## GUI and remaining boundaries

Owned viewing service switched from historical bounty artifact to the authenticated `7d9d9f621` artifact. Private owned-PID query observed Classic2, MicroMenu329×40 and native1920×1080 window. Read-only state/window observations do not prove actual key delivery, pixel/native parity or intended-user acceptance. Viewing runs remain bounded90s; VNC listener policy and original WTF files unchanged.

PTR/earlier-profile runtime, native/GUI-user acceptance, broad current-HEAD suite and full handoff remain OPEN. Separate [Friend publication](../friend-note-mutation-2026-10-12/README.md) preserves its own historical/after scopes. New public packet retention/link/privacy gate pending.
