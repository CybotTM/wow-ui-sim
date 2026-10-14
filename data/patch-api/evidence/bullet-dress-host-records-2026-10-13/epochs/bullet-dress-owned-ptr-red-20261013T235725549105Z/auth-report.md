# Authentication: PASS

- Epoch: `<epochs>/bullet-dress-owned-ptr-red-20261013T235725549105Z`
- New copied PTR integration executable: `<epochs>/bullet-dress-owned-ptr-red-20261013T235725549105Z/artifacts/deps/integration-020202567c2bd5a4`
- SHA256: `704e0f4e0c151799ad35d99c8a9af9827925badb55d514912c3f66d285c36f1f`; matches seal; one sequential hash pass (hash_reads=1).
- Before/after descriptor stats and after path stats exactly match seal: `{"st_dev": 66306, "st_ino": 143142637, "st_mtime_ns": 1791936175039616274, "st_size": 471166488}`.
- Source association: `733399c38f48158206f8b3144cb139a52c537d9f` from seal/revision metadata only, not source-byte authentication.
- Complete compile.stdout parsed: 741 JSON rows, 0 compiler-message diagnostics; build-finished success=true; integration artifact features match PTR seal.
- Complete compile.stderr: 6 manifest deprecation diagnostics plus 1 summary (7 warning lines, not 7 independent diagnostics); no UnitRole; finished in 5m 29s.
- Actual terminal: exited, success, exit 0; peak 7075524608 bytes; CPU 932486125000 ns.
- No builds/runtime or source/archive/library/cache/old-binary reads/authentication. Executable never reread after the hash pass.
- PASS authenticates only the copied executable against its seal; does not assert runtime acceptance or independent source-byte provenance.
