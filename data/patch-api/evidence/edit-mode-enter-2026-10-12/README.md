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

Owned viewing service switched from historical bounty artifact to the authenticated `7d9d9f621` artifact. Private owned-PID query observed Classic2, MicroMenu329×40 and native1920×1080 window. Those read-only observations alone did not prove key delivery or user acceptance. Subsequent user test: layout accepted; both keyboard fields initially failed with zero app Enter deliveries and native PointerRoot focus. Focusing the same current-artifact window produced five actual Enter deliveries; user confirmed chat and `/command` both worked. Native window setup now assigns/verifies focus; fresh-launch gate independently PASS ([later native-focus packet](../native-window-focus-2026-10-12/README.md)); main stopped the test app. User authorized stopping after each test instead of automatic90s expiry; resource limits, VNC listener policy and original WTF files unchanged.

Current GUI layout/keyboard has bounded user confirmation after native focus assignment. Later [PTR simulator runtime](../edit-mode-enter-ptr-2026-10-12/README.md) passes the same3/2/10 bodies/controls at `f3145424c`; no PTR GUI/prefork-cache/process credit. Earlier-profile runtime, universal native/pixel parity, broad current-HEAD suite and full handoff remain OPEN. Separate [Friend publication](../friend-note-mutation-2026-10-12/README.md) preserves its own historical/after scopes. [Independent public retention/link/privacy gate](public-retention-gate.json) PASS through `b1c7c4348`; bounded proof only.
