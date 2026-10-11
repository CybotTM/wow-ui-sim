# Method-profile retention audit

**PASS — retention only. Capture: 12 actual cases, 1 PASS / 11 FAIL.**

Commit `7d4019472`; accepted private epoch031711 proof composed without replay or binary rehash.

- [PASS] declared small-file hashes: {"actual_files": 65, "declared": 64, "mismatches": [], "unmanifested": ["hash-manifest.json"]}

- [PASS] commit7d4019472 packet binding: {"committed_files": 65, "later_files": [], "mismatches": []}

- [PASS] exact private epoch byte copies: {"count": 50, "mismatches": []}

- [PASS] ptr sanitized receipt: {"artifact_metadata_only_no_binary_rehash": true, "environment_overrides": {"WOW_SIM_CASC": "0"}, "exact_expected_fields": true, "timeout_seconds": 90}

- [PASS] ptr sanitized execution argv: {"exact_other_fields": true, "only_replacement": "argv[2] sealed artifact placeholder", "two_commands": 2}

- [PASS] ptr complete extra: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 12412}

- [PASS] ptr complete missing: {"actual_cases": 1, "exit": 0, "prior_accepted_derivation_composed": true, "rows": 33}

- [PASS] ptr aggregate result: {"executed": 2, "fail": 1, "pass": 1}

- [PASS] wrath sanitized receipt: {"artifact_metadata_only_no_binary_rehash": true, "environment_overrides": {"WOW_SIM_CASC": "0"}, "exact_expected_fields": true, "timeout_seconds": 90}

- [PASS] wrath sanitized execution argv: {"exact_other_fields": true, "only_replacement": "argv[2] sealed artifact placeholder", "two_commands": 2}

- [PASS] wrath complete extra: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 11633}

- [PASS] wrath complete missing: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 38}

- [PASS] wrath aggregate result: {"executed": 2, "fail": 2, "pass": 0}

- [PASS] mists sanitized receipt: {"artifact_metadata_only_no_binary_rehash": true, "environment_overrides": {"WOW_SIM_CASC": "0"}, "exact_expected_fields": true, "timeout_seconds": 90}

- [PASS] mists sanitized execution argv: {"exact_other_fields": true, "only_replacement": "argv[2] sealed artifact placeholder", "two_commands": 2}

- [PASS] mists complete extra: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 11633}

- [PASS] mists complete missing: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 38}

- [PASS] mists aggregate result: {"executed": 2, "fail": 2, "pass": 0}

- [PASS] era sanitized receipt: {"artifact_metadata_only_no_binary_rehash": true, "environment_overrides": {"WOW_SIM_CASC": "0"}, "exact_expected_fields": true, "timeout_seconds": 90}

- [PASS] era sanitized execution argv: {"exact_other_fields": true, "only_replacement": "argv[2] sealed artifact placeholder", "two_commands": 2}

- [PASS] era complete extra: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 11633}

- [PASS] era complete missing: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 38}

- [PASS] era aggregate result: {"executed": 2, "fail": 2, "pass": 0}

- [PASS] anniversary sanitized receipt: {"artifact_metadata_only_no_binary_rehash": true, "environment_overrides": {"WOW_SIM_CASC": "0"}, "exact_expected_fields": true, "timeout_seconds": 90}

- [PASS] anniversary sanitized execution argv: {"exact_other_fields": true, "only_replacement": "argv[2] sealed artifact placeholder", "two_commands": 2}

- [PASS] anniversary complete extra: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 11633}

- [PASS] anniversary complete missing: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 38}

- [PASS] anniversary aggregate result: {"executed": 2, "fail": 2, "pass": 0}

- [PASS] wowforever sanitized receipt: {"artifact_metadata_only_no_binary_rehash": true, "environment_overrides": {"WOW_SIM_CASC": "0"}, "exact_expected_fields": true, "timeout_seconds": 90}

- [PASS] wowforever sanitized execution argv: {"exact_other_fields": true, "only_replacement": "argv[2] sealed artifact placeholder", "two_commands": 2}

- [PASS] wowforever complete extra: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 11825}

- [PASS] wowforever complete missing: {"actual_cases": 1, "exit": 101, "prior_accepted_derivation_composed": true, "rows": 38}

- [PASS] wowforever aggregate result: {"executed": 2, "fail": 2, "pass": 0}

- [PASS] 12-case total and boundary: {"FAIL": 11, "PASS": 1, "actual": 12, "native": "UNKNOWN", "parent": "OPEN", "source_lanes": "KEEP_BLOCKED"}

- [PASS] canonical0042 docs/wow-client-diff/WowDiscovery.lua: {"equals0042_blob": true, "equals_saved_hash": true, "sha256": "4cd7ba572f01c0abf7308fe218230fe004983b0b49d4a752cdfedcf672e4a5e6"}

- [PASS] canonical0042 docs/wow-client-diff/diff_methods_extra.txt: {"equals0042_blob": true, "equals_saved_hash": true, "sha256": "ff9ee51953ef7cbc99cc545c6ac0d185585379035270a40291df4d2ef5a4ddf5"}

- [PASS] canonical0042 docs/wow-client-diff/diff_methods_missing.txt: {"equals0042_blob": true, "equals_saved_hash": true, "sha256": "b26fa8581077721e6925b0bb81266c28ad9c404d60df7edf0f168a9e1c613026"}

- [PASS] canonical0042 tests/method_diff_coverage.rs: {"equals0042_blob": true, "equals_saved_hash": true, "sha256": "a1dc2c865fee737b3746e00d80b44cec5db23b16712ce2ffeda441e1520d324d"}

- [PASS] bounded privacy scan: {"files_scanned": 65, "hits": [], "known_alpha_identity_tokens_derived": 4, "limitations": "Known alpha identity plus private path/sealed-copy/credential-pattern scan; not proof against all conceivable secrets.", "private_values_not_emitted": true}

- [PASS] classification and pending label: {"capture_not_all_pass": true, "pending_label": "Expected at audit time; main must update after inspecting this report. Rehash main-disposition manifest entry if changed."}

## Limits and disposition
Failures compare bare metatable enumeration with shared default-Retail fixtures; not eleven native/profile bugs. Native UNKNOWN, source lanes KEEP_BLOCKED, parent OPEN. Prior Retail passes excluded.
Independent audit pending label is valid until main inspects this report; update afterward and refresh its manifest hash. Any report copy added later changes actual file count and lies outside this audit.
Async1977 documentation not audited. Supply changed documentation source paths for a later bounded audit. No private files mutated; only owned report.md/json written.

Retention note: private verification root redacted; original raw report JSON not copied. Prior audit counts describe the earlier65-file packet.
