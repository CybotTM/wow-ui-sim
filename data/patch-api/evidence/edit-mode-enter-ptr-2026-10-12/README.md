# PTR Edit Mode and Enter — bounded runtime proof

[Proof](proof.json) · [Retention](retention-manifest.json) · [Enter contract](../../../../docs/specs/enter-chat-binding.md) · [Selection contract](../../../../docs/specs/edit-mode-layout-selection.md)

Compiled `f3145424c`; wrappers `2764c7edd`. Independent PTR gate PASS:

| Boundary | Executed proof |
|---|---|
| Original Enter bodies | [3/3](01-chat-enter.stdout): default activation/type/submit, override, explicit unbind |
| Fresh file import before unchanged whole Game startup | [2/2](02-cache-startup.stdout): native2→Classic/horizontal; four explicit Widescreen/vertical variants inside one name |
| Cache/parser/env/snapshot controls | [10/10](03-cache-controls.stdout) |
| Normal startup | [Marker and `[]`](04-normal-ptr.stdout), zero Lua errors; manager/API active-index equality, no forced Classic in actual PTR cache |
| Format / PTR offline locked check | Each ran once, exit0; [check diagnostics](06-ptr-check.stderr) preserve six external manifest deprecations; source path explicitly sanitized |

15 test names executed; parent/child recaps are not extra cases. Runtime tests clear only the ambient manual layout override, with CASC/sound disabled; internal environment-precedence tests remain real. Fresh whole Game constructor invokes the identical original Enter bodies; it is not the Retail prefork constructor or a new prefork contract. Existing Retail proof reused, not rerun. Source14 stable through compilation and gate. Two new seals authenticated once each; no old/new seal rehash or library hashing. Changed-Rust readability audit clean.

PTR execution does not establish native/PTR GUI acceptance, earlier-profile compatibility, prefork cache/process invariants, broad-current-suite or causal-performance acceptance. Whole handoff OPEN. Separate [Retail native-focus/user acceptance](../native-window-focus-2026-10-12/README.md) is not PTR GUI credit. Public retention/link/privacy gate pending.
