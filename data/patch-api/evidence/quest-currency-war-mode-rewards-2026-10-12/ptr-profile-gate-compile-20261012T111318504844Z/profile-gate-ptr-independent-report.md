# NEW PTR profile-gate independent verification

**PENDING — partial evidence only; no current PASS.**

Epoch: `ptr-profile-gate-compile-20261012T111318504844Z`.
Source: `91324b10380f9be3e8d196491e9a2d20bc597fa1`; build/docs: `298caf47a53eac18960c1cef6716489e54a351cc`.

| Coverage | Evidence | Result |
|---|---|---|
| New integration authentication | Hashed once, receipt immediately persisted; expected/actual size **467628400 bytes**, SHA256 **dc342f072d3c65ed7b31ca215a55ec9bc90c63520ee735ff984284906f7a3921**, both match | PASS authentication only |
| PTR compile | Entire 574600-byte stdout parsed: 741 records = 665 compiler-artifact +75 build-script-executed +1 successful build-finished. Integration Cargo record equals manifest, client-ptr enabled/client-retail absent. Full1275-byte stderr read. Exit0, zero compiler-error diagnostics; six iced manifest deprecations, plus aggregate warning line | PASS compile, warnings remain |
| Quest caller13 | Epoch outcome `COMPILED_NOT_EXECUTED`; no runtime streams/outcome produced. **0 executed, 0 passed, 0 failed**; runtime error count unknown, not zero | PENDING |
| Normal four raw getters/startup | No normal-artifact.json, wow-sim-sealed, normal submission/streams/outcome. **0/4 getters verified**; startup error count/exit unknown. Compiler's target/debug wow-sim entry is not a sealed/authenticated normal result; no receipt fabricated | PENDING |
| Source applicability/reuse | Accepted913 source equivalence/manual readability reused. Independent913→298 name-status diff:105 evidence paths +3 docs, zero source/build/test changes. Full three doc patches inspected; docs explicitly preserve a5e proof and pending913 acceptance | PASS bounded applicability only |

Runtime admission history contains three rejected admissions, latest11:16:16.515999Z, load29.77. This is retained metadata, not service polling or a controller/test success claim.

Unchanged old flag-query9 proof reused only within its accepted helper/query/test scope: **zero new flag executions**. Other profiles receive **zero credit**. No historical runs count toward current13. Native catalogs, other rewards, lifecycle, earlier profiles and full handoff remain outside scope.

No builds/tests/runtime commands, service polls, long-build waits, delegation, questions, repeated source gates or repository edits. Old root proof/auth untouched. Integration receipt reused after first hash; no further binary reads/hashes. Normal authentication pending because new normal seal absent at snapshot.
