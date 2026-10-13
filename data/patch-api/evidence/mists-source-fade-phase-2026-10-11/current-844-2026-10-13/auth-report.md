# Executable authentication

PASS — only `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/mists-original-cast-prefork-current-20261013T195457944782Z/artifacts/wow-sim` authenticated.

- SHA256: `7f17b10f569a0028b1ebbfea58d33a089a5ab815451393838d98238425962605`; matches seal. Exactly one independent streaming SHA256 pass (`hash_reads=1`), 344540168 bytes.
- Pre/post open-descriptor and path stats each match seal: `{"st_dev": 66306, "st_ino": 147211156, "st_mtime_ns": 1791921580666081223, "st_size": 344540168}`.
- Receipt absent before authentication; `receipt-prefork_full_ui.json` created exclusively, file and directory fsynced, full readback equals persisted receipt. Required status/boolean/count/path/stat fields verified.

## Saved compile evidence

Full `compile.stdout` JSONL read and parsed: 741 records (665 compiler-artifact, 75 build-script-executed, 1 successful build-finished). Zero compiler-message records. Full `compile.stderr` and terminal metadata read. Terminal: ActiveState=active, SubState=exited, Result=success, ExecMainStatus=0. Six manifest deprecation warnings observed.

**Requested zero source diagnostics cannot be confirmed across all saved output:** stderr reports one source dead-code warning at `src/lua_api/globals/real/unit_roles.rs:17:5`: variants Tank, Healer, Damage never constructed; wow-ui-sim lib generated 1 warning. JSONL has zero source diagnostic records, but stderr is not source-warning-free. `compile-result.json`'s empty source_compiler_messages list must not be interpreted as zero source warnings across both streams.

Profile: Mists (`seal.profile=mists`, artifact feature `client-mists`). Artifact target `prefork_full_ui`, kind test, required feature prefork-full-ui; fresh=false. Observed features: casc, client-mists, gui, prefork-full-ui, rodio, sound. Cargo test profile: opt_level=1, debuginfo=line-tables-only, debug_assertions=true, overflow_checks=true, test=true. Saved submission uses no-default-features and sound,gui,casc,client-mists; effective artifact metadata includes prefork-full-ui and rodio.

## Scope

No old executable, library, archive, or source authentication. No build, runtime execution, delegation, or tracked writes. Saved compile metadata is evidence only, not an independent source-to-binary provenance authentication. Main owns runtime acceptance. Initial Pyrun authentication attempt failed on undefined base before any executable open or hash; successful attempt above is the only streaming SHA256 pass.
