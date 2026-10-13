# Integration authentication

PASS — new copied integration executable only.

- Path: `TOKEN_RETAIL_ROOT/artifacts/deps/TOKEN_IDENTIFIER_09`
- SHA256: `TOKEN_IDENTIFIER_01`; matches seal; hash_reads=1.
- Stat before and after (including open descriptor): exact match to seal for st_dev, st_ino, st_size, st_mtime_ns.
- Source label: `TOKEN_IDENTIFIER_07`; profile: `retail`. Label only; no source-byte provenance claim.
- Complete compile.stderr and all 741 compile.stdout JSON rows read: 0 compiler diagnostics; six manifest deprecations plus one manifest warning summary; no UnitRole in stderr. JSON build-finished success=true.
- Terminal evidence caller-confirmed, not independently inspected: SubState=exited, ExecMainStatus=0, peak=4359192576 bytes.
- Scope: new copied executable; external libraries/cache/settings and source archive not independently authenticated. No build or runtime launch; runtime acceptance not verified.

Stat before/after:
```json
{
  "st_dev": "TOKEN_ST_DEV_1",
  "st_ino": "TOKEN_ST_INO_2",
  "st_size": 471300104,
  "st_mtime_ns": 1791934848436960034
}
```
