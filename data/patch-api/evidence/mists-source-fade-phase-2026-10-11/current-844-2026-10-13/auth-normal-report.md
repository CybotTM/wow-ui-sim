# Normal executable authentication

PASS — only `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/mists-original-cast-prefork-current-20261013T195457944782Z/artifacts/wow-sim-normal` authenticated against `seal-wow-sim-normal.json`.

- SHA256: `d4b61e9809a40609e9443fcaed0e0247974715478fe142514840c6f43331f63c`; matches seal. Exactly one independent streaming hash pass (`hash_reads=1`), 398088896 bytes.
- Pre/post path and descriptor stats match seal exactly: `{"st_dev": 66306, "st_ino": 147198538, "st_mtime_ns": 1791921555412436844, "st_size": 398088896}`.
- `receipt-wow-sim-normal.json` absent before authentication; exclusively created, file and directory fsynced, full persisted readback equals receipt. Required status=PASS, sha256_matches_seal=true, hash_reads=1, pathstat_after verified.

## Metadata-only build association

`normal-deps-selection.json` matches exactly one saved main binary compiler-artifact record: wow-sim, kind bin, src/bin/wow_sim/main.rs, profile.test=false, fresh=false. Selected deps/fingerprint identifier: 500d73632104a659.

Mists profile; exact features: casc, client-mists, gui, prefork-full-ui, rodio, sound. Recorded source revision: 844a63adb95c7111e1286fa668e632f7cd237806 (metadata only).

Saved submission/output mtime build epoch bounds (ns): 1791921298062740529–1791921581176094238. Referenced deps mtime 1791921555412436844 and fingerprint mtime 1791921555653442994 lie within bounds. Referenced target and fingerprint were statted only, never opened. Association is metadata-only, not source-byte authentication or independent source-to-binary provenance.

## Prior saved compile evidence

Prior auth-report.md records full saved compile evidence: 741 JSONL records (665 compiler-artifact, 75 build-script-executed, 1 successful build-finished), zero compiler-message records, terminal ExecMainStatus=0/Result=success. Six manifest deprecation warnings plus one source dead-code warning at src/lua_api/globals/real/unit_roles.rs:17:5 (Tank, Healer, Damage never constructed). Zero JSONL diagnostics does not mean warning-free stderr. No new compile or source inspection.

## Scope

Old prefork artifacts/wow-sim is distinct and was not opened or hashed. No prefork/old binary, library, archive, or source bytes read or authenticated. No runtime, build, delegation, or tracked edits. Receipt and report are the only new outputs. Runtime acceptance is outside scope.

An initial persistence evaluation failed with undefined os before any executable open/hash or output creation; this successful evaluation performed the only streaming hash pass.
