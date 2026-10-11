# Mists source-fade phase — retained partial proof

Revision `4e3848a77747bfdfe048f68fca9cc1e231c5750f`, epoch `20261011T155435Z`. [Contract investigation](../../../../docs/wiki/investigations/mists-cast-startup-visibility.md#current-fixture-phase--2026-10-11-unverified); [acceptance authority](../../../../docs/wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md#acceptance-authority--2026-10-11).

- Original exact prefork cast-bar case: **0 PASS / 1 FAIL**, exit1. Normal 0.55s frame advancement reaches hidden state; the next old `UIParent` parent assertion fails. Trace shows the retained bottom manager parent and absent showing-frame membership. Later layout/anchor assertions remain unreached.
- Existing integration source-fade control: **1 PASS execution**, exit0. Nested timeout child/parent summaries describe that same execution, not two tests.
- Cargo compile exit0, two sealed artifact/feature records, thirteen input endpoint hashes unchanged. Runtime uses live CASC and sound disabled, profile `client-mists`; native timing, full startup-error cleanliness and all-profile acceptance are not established.
- [Independent saved-evidence audit](runtime-report.md) authenticates those scopes. [Manifest](retention-manifest.json) inventories twelve exact private metadata/stream/report copies, 89,844 bytes. No executable or vendor Lua payload copied.

The earlier immediate-hidden failure remains historical. Follow-up `83acf239e` changes the parent expectation to source-consistent retained manager identity; this packet does not prove that changed case. The separately [retained parent follow-up](parent-followup/README.md) passes the corrected original case, including remaining lock/layout/anchor checks, and the fade control once each; this historical packet remains the earlier failure scope. No production reparenting or vendor patch follows from this proof.
