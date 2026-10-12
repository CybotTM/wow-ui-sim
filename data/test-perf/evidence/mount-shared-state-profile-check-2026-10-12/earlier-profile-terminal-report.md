# Earlier-profile terminal proof

PASS — five recorded Cargo checks exit 0 at `dceb4c751f7e21faea998c92a589f9b4f1f8b7ca`; current HEAD `533c086e046324528d92d581a241021bf455ebdb`. Parent OPEN.

Input: `/home/osso/.local/state/wow-ui-sim/verification/mount-item-current/earlier-profile-check-20261012T155710495111Z`

| Profile | Exit | SHA matches | Warnings | Finish |
|---|---:|---:|---:|---|
| wrath | 0 | 7/7 before/after/current/revision | 6 + summary |     Finished `dev` profile [optimized + debuginfo] target(s) in 44.25s |
| mists | 0 | 7/7 before/after/current/revision | 6 + summary |     Finished `dev` profile [optimized + debuginfo] target(s) in 39.49s |
| era | 0 | 7/7 before/after/current/revision | 6 + summary |     Finished `dev` profile [optimized + debuginfo] target(s) in 37.03s |
| anniversary | 0 | 7/7 before/after/current/revision | 6 + summary |     Finished `dev` profile [optimized + debuginfo] target(s) in 33.81s |
| wowforever | 0 | 7/7 before/after/current/revision | 6 + summary |     Finished `dev` profile [optimized + debuginfo] target(s) in 34.48s |

## Source / application diff

Seven current SHA-256 hashes match all ten before/after snapshots and independent `git show dceb4c751:<path>` hashes. Snapshot diffs empty, exit 0. Independent `git diff dceb4c751 -- Cargo.lock Cargo.toml src tests`: empty, exit 0. HEAD delta changes only five Markdown files (including evidence README outside docs/); no production changes in that committed delta.

## Coverage limits

- Cargo check compile evidence only; not builds/tests/runtime/native proof.
- Retail/PTR-only mount-item module and registration cfg excluded from all five earlier profiles; source inspection only.
- Full handoff and parent goal OPEN. No full-suite inspection.
- Prior2864 pending observation and named outputs untouched.
- Current working tree has untracked .code-index.db and verification/; not altered.
- Toolchain versions are recorded from existing executions, not rerun.

Service terminal state is supplied by main, not independently queried. All existing stdout/stderr, commands, toolchains, admission/history, hashes and outcome files are preserved verbatim below and in JSON. No builds/checks/tests re-executed; no seal access, operations, delegation, repository edits, waits or polling.

## Independent read-only Git evidence

```json
[
  {
    "argv": [
      "git",
      "rev-parse",
      "HEAD"
    ],
    "exit": 0,
    "stdout": "533c086e046324528d92d581a241021bf455ebdb\n",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "rev-parse",
      "dceb4c751"
    ],
    "exit": 0,
    "stdout": "dceb4c751f7e21faea998c92a589f9b4f1f8b7ca\n",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "diff",
      "--name-status",
      "dceb4c751",
      "HEAD"
    ],
    "exit": 0,
    "stdout": "M\tdata/patch-api/evidence/mount-item-relations-2026-10-12/README.md\nM\tdocs/specs/mount-item-relations.md\nM\tdocs/specs/mount-spell-identifier.md\nM\tdocs/wiki/index.md\nM\tdocs/wiki/log.md\n",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "diff",
      "dceb4c751",
      "--",
      "Cargo.lock",
      "Cargo.toml",
      "src",
      "tests"
    ],
    "exit": 0,
    "stdout": "",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "status",
      "--porcelain=v1"
    ],
    "exit": 0,
    "stdout": "?? .code-index.db\n?? verification/\n",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "show",
      "dceb4c751:Cargo.lock"
    ],
    "exit": 0,
    "stdout_sha256": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "show",
      "dceb4c751:Cargo.toml"
    ],
    "exit": 0,
    "stdout_sha256": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "show",
      "dceb4c751:src/c_api/mod.rs"
    ],
    "exit": 0,
    "stdout_sha256": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "show",
      "dceb4c751:src/c_api/registration.rs"
    ],
    "exit": 0,
    "stdout_sha256": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "show",
      "dceb4c751:src/client_profile.rs"
    ],
    "exit": 0,
    "stdout_sha256": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "show",
      "dceb4c751:src/lua_api/state.rs"
    ],
    "exit": 0,
    "stdout_sha256": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "stderr": ""
  },
  {
    "argv": [
      "git",
      "show",
      "dceb4c751:src/lua_api/state/sim_state.rs"
    ],
    "exit": 0,
    "stdout_sha256": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867",
    "stderr": ""
  }
]
```

## Current SHA-256

```json
{
  "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
  "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
  "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
  "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
  "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
  "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
  "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
}
```

## Source-only cfg exclusion

```json
[
  {
    "file": "src/c_api/mod.rs",
    "start_line": 130,
    "end_line": 131,
    "text": "#[cfg(any(feature = \"client-retail\", feature = \"client-ptr\"))]\nmod c_mount_item;"
  },
  {
    "file": "src/c_api/registration.rs",
    "start_line": 30,
    "end_line": 31,
    "text": "    #[cfg(any(feature = \"client-retail\", feature = \"client-ptr\"))]\n    super::c_mount_item::register(state)?;"
  }
]
```

## Full existing artifact outputs

### anniversary/after-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### anniversary/before-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### anniversary/cargo-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### anniversary/cargo-version-submission.json
Bytes: 111; SHA-256: `bc87e2f2b3f850a33ccde546d279ad94f4c964b1f4f29352dafcb3accc519e2f`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "-V"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### anniversary/cargo-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### anniversary/cargo-version.stdout
Bytes: 65; SHA-256: `6a59b0c227397137eae1f0c19d44fc81df1a26052535a78a72094f53c34e8826`
```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)

```

### anniversary/check-admission-history.json
Bytes: 480; SHA-256: `d2a81b6c1eb2f9c95730416c2367136bf54cee7d70f64103614ada4a0768ca5d`
```text
[
  {
    "admitted": true,
    "at": "2026-10-12T15:59:11.580027+00:00",
    "available_gib": 33.525482177734375,
    "io_pressure": "some avg10=16.40 avg60=13.05 avg300=5.73 total=9862951938\nfull avg10=14.60 avg60=11.36 avg300=4.98 total=7261623247\n",
    "load1": 16.33,
    "memory_pressure": "some avg10=0.47 avg60=0.51 avg300=0.81 total=4790200070\nfull avg10=0.47 avg60=0.49 avg300=0.75 total=4207111148\n",
    "minimum_available_gib": 6,
    "relevant_cargo": []
  }
]

```

### anniversary/check-admission.json
Bytes: 456; SHA-256: `42383054d795a753f7c3d11d723f37eb062115a5a35083c249eabd1203e1e717`
```text
{
  "admitted": true,
  "at": "2026-10-12T15:59:11.580027+00:00",
  "available_gib": 33.525482177734375,
  "io_pressure": "some avg10=16.40 avg60=13.05 avg300=5.73 total=9862951938\nfull avg10=14.60 avg60=11.36 avg300=4.98 total=7261623247\n",
  "load1": 16.33,
  "memory_pressure": "some avg10=0.47 avg60=0.51 avg300=0.81 total=4790200070\nfull avg10=0.47 avg60=0.49 avg300=0.75 total=4207111148\n",
  "minimum_available_gib": 6,
  "relevant_cargo": []
}

```

### anniversary/check-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### anniversary/check-submission.json
Bytes: 255; SHA-256: `257f27400cc811f692aff84c00b777f31831e64c81752b990a42b1f58fd6935d`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "check",
    "--offline",
    "--locked",
    "--no-default-features",
    "--features",
    "sound,gui,casc,client-anniversary",
    "-j",
    "12"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### anniversary/check.stderr
Bytes: 1274; SHA-256: `e1911f03b7896fb09a9138ac2c78563820409dcbfdaf2586dab244c0c5a2c4dc`
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `dev` profile [optimized + debuginfo] target(s) in 33.81s

```

### anniversary/check.stdout
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### anniversary/rustc-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### anniversary/rustc-version-submission.json
Bytes: 112; SHA-256: `165b8bfa552eae12293e3bafb5a931a9b0450fc09d25d2a6a6c73ed5a69f5c53`
```text
{
  "argv": [
    "/usr/bin/rustc",
    "-Vv"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### anniversary/rustc-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### anniversary/rustc-version.stdout
Bytes: 225; SHA-256: `e9e4ced8161a774af62ef1d599e5c84c25735205d052feaf97e7d480c29d135e`
```text
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1

```

### era/after-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### era/before-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### era/cargo-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### era/cargo-version-submission.json
Bytes: 111; SHA-256: `bc87e2f2b3f850a33ccde546d279ad94f4c964b1f4f29352dafcb3accc519e2f`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "-V"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### era/cargo-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### era/cargo-version.stdout
Bytes: 65; SHA-256: `6a59b0c227397137eae1f0c19d44fc81df1a26052535a78a72094f53c34e8826`
```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)

```

### era/check-admission-history.json
Bytes: 475; SHA-256: `7aaf49cbcf438b597f49e831fa76471afc54f5e83ebc544037b5c71f65b68a83`
```text
[
  {
    "admitted": true,
    "at": "2026-10-12T15:58:34.478782+00:00",
    "available_gib": 32.49436950683594,
    "io_pressure": "some avg10=3.96 avg60=3.69 avg300=3.08 total=9853387317\nfull avg10=3.66 avg60=3.15 avg300=2.67 total=7253293273\n",
    "load1": 15.09,
    "memory_pressure": "some avg10=0.51 avg60=0.57 avg300=0.86 total=4789919879\nfull avg10=0.51 avg60=0.54 avg300=0.79 total=4206835002\n",
    "minimum_available_gib": 6,
    "relevant_cargo": []
  }
]

```

### era/check-admission.json
Bytes: 451; SHA-256: `64346dd34dd7d4dd8e175aed81ec768375e6b02a51b319e9e8d29d38942ef13f`
```text
{
  "admitted": true,
  "at": "2026-10-12T15:58:34.478782+00:00",
  "available_gib": 32.49436950683594,
  "io_pressure": "some avg10=3.96 avg60=3.69 avg300=3.08 total=9853387317\nfull avg10=3.66 avg60=3.15 avg300=2.67 total=7253293273\n",
  "load1": 15.09,
  "memory_pressure": "some avg10=0.51 avg60=0.57 avg300=0.86 total=4789919879\nfull avg10=0.51 avg60=0.54 avg300=0.79 total=4206835002\n",
  "minimum_available_gib": 6,
  "relevant_cargo": []
}

```

### era/check-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### era/check-submission.json
Bytes: 247; SHA-256: `c86d32d0aca848e20699738712341a696fbb3a1f68a9014a9bd881eb533354be`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "check",
    "--offline",
    "--locked",
    "--no-default-features",
    "--features",
    "sound,gui,casc,client-era",
    "-j",
    "12"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### era/check.stderr
Bytes: 1274; SHA-256: `19bc62eec9cf1a7014b0e7761e4978f8024ece41f546023fa65c0ce1262dc275`
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `dev` profile [optimized + debuginfo] target(s) in 37.03s

```

### era/check.stdout
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### era/rustc-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### era/rustc-version-submission.json
Bytes: 112; SHA-256: `165b8bfa552eae12293e3bafb5a931a9b0450fc09d25d2a6a6c73ed5a69f5c53`
```text
{
  "argv": [
    "/usr/bin/rustc",
    "-Vv"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### era/rustc-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### era/rustc-version.stdout
Bytes: 225; SHA-256: `e9e4ced8161a774af62ef1d599e5c84c25735205d052feaf97e7d480c29d135e`
```text
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1

```

### mists/after-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### mists/before-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### mists/cargo-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### mists/cargo-version-submission.json
Bytes: 111; SHA-256: `bc87e2f2b3f850a33ccde546d279ad94f4c964b1f4f29352dafcb3accc519e2f`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "-V"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### mists/cargo-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### mists/cargo-version.stdout
Bytes: 65; SHA-256: `6a59b0c227397137eae1f0c19d44fc81df1a26052535a78a72094f53c34e8826`
```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)

```

### mists/check-admission-history.json
Bytes: 474; SHA-256: `9e8f4d10e61685b2471eec01889287d5afcf602722b4d4a670c061158d6eb3c5`
```text
[
  {
    "admitted": true,
    "at": "2026-10-12T15:57:54.905618+00:00",
    "available_gib": 35.20872116088867,
    "io_pressure": "some avg10=3.74 avg60=3.74 avg300=3.00 total=9851687988\nfull avg10=3.54 avg60=3.24 avg300=2.62 total=7251872634\n",
    "load1": 15.0,
    "memory_pressure": "some avg10=0.29 avg60=0.71 avg300=0.93 total=4789562536\nfull avg10=0.29 avg60=0.67 avg300=0.85 total=4206484282\n",
    "minimum_available_gib": 6,
    "relevant_cargo": []
  }
]

```

### mists/check-admission.json
Bytes: 450; SHA-256: `49f7359dd47c817175dbfb54c993c290c5ef48851b9e6579e3131e30d4048954`
```text
{
  "admitted": true,
  "at": "2026-10-12T15:57:54.905618+00:00",
  "available_gib": 35.20872116088867,
  "io_pressure": "some avg10=3.74 avg60=3.74 avg300=3.00 total=9851687988\nfull avg10=3.54 avg60=3.24 avg300=2.62 total=7251872634\n",
  "load1": 15.0,
  "memory_pressure": "some avg10=0.29 avg60=0.71 avg300=0.93 total=4789562536\nfull avg10=0.29 avg60=0.67 avg300=0.85 total=4206484282\n",
  "minimum_available_gib": 6,
  "relevant_cargo": []
}

```

### mists/check-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### mists/check-submission.json
Bytes: 249; SHA-256: `b6cf87867800c77f8d18203c54164918d6b36fdfe3e665b2e81f379eb931ba09`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "check",
    "--offline",
    "--locked",
    "--no-default-features",
    "--features",
    "sound,gui,casc,client-mists",
    "-j",
    "12"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### mists/check.stderr
Bytes: 1274; SHA-256: `d837f8a9b100b37180e5d4f459dce3f6def0fd3ef78e02f4815c7df8622a42d3`
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `dev` profile [optimized + debuginfo] target(s) in 39.49s

```

### mists/check.stdout
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### mists/rustc-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### mists/rustc-version-submission.json
Bytes: 112; SHA-256: `165b8bfa552eae12293e3bafb5a931a9b0450fc09d25d2a6a6c73ed5a69f5c53`
```text
{
  "argv": [
    "/usr/bin/rustc",
    "-Vv"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### mists/rustc-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### mists/rustc-version.stdout
Bytes: 225; SHA-256: `e9e4ced8161a774af62ef1d599e5c84c25735205d052feaf97e7d480c29d135e`
```text
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1

```

### outcome.json
Bytes: 781; SHA-256: `1a958b307577ee7bae383b4e9febdfe646648e2ddf3400b35f9d05d601a65420`
```text
{
  "full_handoff_complete": false,
  "results": [
    {
      "check_exit": 0,
      "profile": "wrath",
      "source_unchanged": true
    },
    {
      "check_exit": 0,
      "profile": "mists",
      "source_unchanged": true
    },
    {
      "check_exit": 0,
      "profile": "era",
      "source_unchanged": true
    },
    {
      "check_exit": 0,
      "profile": "anniversary",
      "source_unchanged": true
    },
    {
      "check_exit": 0,
      "profile": "wowforever",
      "source_unchanged": true
    }
  ],
  "revision": "dceb4c751",
  "scope": "Shared unconditional state/default compile refresh. Retail/PTR excluded from rerun; current API cfg-excluded here. No runtime/native/full-handoff acceptance.",
  "status": "FIVE_EARLIER_PROFILE_CHECKS_CAPTURED"
}

```

### results.json
Bytes: 427; SHA-256: `b0975c610225818cb6065ce856bb2a108ad3248298926f70f3476846eb68f46f`
```text
[
  {
    "check_exit": 0,
    "profile": "wrath",
    "source_unchanged": true
  },
  {
    "check_exit": 0,
    "profile": "mists",
    "source_unchanged": true
  },
  {
    "check_exit": 0,
    "profile": "era",
    "source_unchanged": true
  },
  {
    "check_exit": 0,
    "profile": "anniversary",
    "source_unchanged": true
  },
  {
    "check_exit": 0,
    "profile": "wowforever",
    "source_unchanged": true
  }
]

```

### wowforever/after-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### wowforever/before-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### wowforever/cargo-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### wowforever/cargo-version-submission.json
Bytes: 111; SHA-256: `bc87e2f2b3f850a33ccde546d279ad94f4c964b1f4f29352dafcb3accc519e2f`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "-V"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### wowforever/cargo-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### wowforever/cargo-version.stdout
Bytes: 65; SHA-256: `6a59b0c227397137eae1f0c19d44fc81df1a26052535a78a72094f53c34e8826`
```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)

```

### wowforever/check-admission-history.json
Bytes: 474; SHA-256: `90bc0b367b8821e4cd21337d0f60747acbc67371f4d23697ba12649c7913717c`
```text
[
  {
    "admitted": true,
    "at": "2026-10-12T15:59:45.483492+00:00",
    "available_gib": 32.23238754272461,
    "io_pressure": "some avg10=5.27 avg60=9.66 avg300=5.68 total=9864923047\nfull avg10=5.01 avg60=8.54 avg300=4.97 total=7263488568\n",
    "load1": 13.1,
    "memory_pressure": "some avg10=0.57 avg60=0.53 avg300=0.77 total=4790567482\nfull avg10=0.52 avg60=0.49 avg300=0.71 total=4207476526\n",
    "minimum_available_gib": 6,
    "relevant_cargo": []
  }
]

```

### wowforever/check-admission.json
Bytes: 450; SHA-256: `78441a88aed2a3145c57f2f6e7c5a9bcd651abb3d6afa66698c5f02e79d1790c`
```text
{
  "admitted": true,
  "at": "2026-10-12T15:59:45.483492+00:00",
  "available_gib": 32.23238754272461,
  "io_pressure": "some avg10=5.27 avg60=9.66 avg300=5.68 total=9864923047\nfull avg10=5.01 avg60=8.54 avg300=4.97 total=7263488568\n",
  "load1": 13.1,
  "memory_pressure": "some avg10=0.57 avg60=0.53 avg300=0.77 total=4790567482\nfull avg10=0.52 avg60=0.49 avg300=0.71 total=4207476526\n",
  "minimum_available_gib": 6,
  "relevant_cargo": []
}

```

### wowforever/check-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### wowforever/check-submission.json
Bytes: 254; SHA-256: `4e77ccc45ac3003d828305dfe81fc203f71661bd182507e77591629a8984f055`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "check",
    "--offline",
    "--locked",
    "--no-default-features",
    "--features",
    "sound,gui,casc,client-wowforever",
    "-j",
    "12"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### wowforever/check.stderr
Bytes: 1274; SHA-256: `bfcf8cd98b6085181e189b28f5770bf89f4ddfa124ee065b981aed7747a7b507`
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `dev` profile [optimized + debuginfo] target(s) in 34.48s

```

### wowforever/check.stdout
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### wowforever/rustc-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### wowforever/rustc-version-submission.json
Bytes: 112; SHA-256: `165b8bfa552eae12293e3bafb5a931a9b0450fc09d25d2a6a6c73ed5a69f5c53`
```text
{
  "argv": [
    "/usr/bin/rustc",
    "-Vv"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### wowforever/rustc-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### wowforever/rustc-version.stdout
Bytes: 225; SHA-256: `e9e4ced8161a774af62ef1d599e5c84c25735205d052feaf97e7d480c29d135e`
```text
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1

```

### wrath/after-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### wrath/before-source.json
Bytes: 739; SHA-256: `eec0477f5a68852cbe07306dde429500a7184c83eacdc93bb5b0991a819e3fce`
```text
{
  "diff": "",
  "exit": 0,
  "hashes": {
    "Cargo.lock": "5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1",
    "Cargo.toml": "91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9",
    "src/c_api/mod.rs": "6449741dc121fcdc1ea1af7256b8a9059cdf411c98b3cd4b761ffe8417cd19b0",
    "src/c_api/registration.rs": "e975df94170a676ff428c755446a8ccf89471a79fc14b7383d2d78e1c5a9d4c4",
    "src/client_profile.rs": "8f603ce85c8b4d67f2fdbc15194d47f0d3365f0c96d7dd85ce21e3a893554839",
    "src/lua_api/state.rs": "71d062ba3b460ad85e78ef6298f02572634e8d753fe0341ac27753687cd3c365",
    "src/lua_api/state/sim_state.rs": "dc701e094e0d3b9f9b0c23c7af1668b704a5fda57834077cb4b12d237823c867"
  },
  "revision": "dceb4c751"
}

```

### wrath/cargo-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### wrath/cargo-version-submission.json
Bytes: 111; SHA-256: `bc87e2f2b3f850a33ccde546d279ad94f4c964b1f4f29352dafcb3accc519e2f`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "-V"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### wrath/cargo-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### wrath/cargo-version.stdout
Bytes: 65; SHA-256: `6a59b0c227397137eae1f0c19d44fc81df1a26052535a78a72094f53c34e8826`
```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)

```

### wrath/check-admission-history.json
Bytes: 476; SHA-256: `906de4fade0f4292bb2eccb401ca7b5ebc5a853f6329d001e649cef76cdcc05b`
```text
[
  {
    "admitted": true,
    "at": "2026-10-12T15:57:10.510675+00:00",
    "available_gib": 32.410675048828125,
    "io_pressure": "some avg10=3.79 avg60=4.21 avg300=2.97 total=9850070574\nfull avg10=3.40 avg60=3.75 avg300=2.62 total=7250505175\n",
    "load1": 15.37,
    "memory_pressure": "some avg10=0.47 avg60=1.04 avg300=1.01 total=4789230295\nfull avg10=0.47 avg60=0.95 avg300=0.92 total=4206157594\n",
    "minimum_available_gib": 6,
    "relevant_cargo": []
  }
]

```

### wrath/check-admission.json
Bytes: 452; SHA-256: `fee3024be9d44031e12394395a06694c2d12c7b322ad308fa9f40c64c5fa08bc`
```text
{
  "admitted": true,
  "at": "2026-10-12T15:57:10.510675+00:00",
  "available_gib": 32.410675048828125,
  "io_pressure": "some avg10=3.79 avg60=4.21 avg300=2.97 total=9850070574\nfull avg10=3.40 avg60=3.75 avg300=2.62 total=7250505175\n",
  "load1": 15.37,
  "memory_pressure": "some avg10=0.47 avg60=1.04 avg300=1.01 total=4789230295\nfull avg10=0.47 avg60=0.95 avg300=0.92 total=4206157594\n",
  "minimum_available_gib": 6,
  "relevant_cargo": []
}

```

### wrath/check-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### wrath/check-submission.json
Bytes: 249; SHA-256: `d5dc8e3077b3651ebc860921fc80bcf7d522ef5915559833327b065d0e5d59f0`
```text
{
  "argv": [
    "/usr/bin/cargo",
    "check",
    "--offline",
    "--locked",
    "--no-default-features",
    "--features",
    "sound,gui,casc,client-wrath",
    "-j",
    "12"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### wrath/check.stderr
Bytes: 1274; SHA-256: `e98468d6d161f9d29eee07261c5df6ee99335dbe549864fc57cca89e5b46179a`
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `dev` profile [optimized + debuginfo] target(s) in 44.25s

```

### wrath/check.stdout
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### wrath/rustc-version-result.json
Bytes: 16; SHA-256: `3e42a2ad3dbeab9d1167d20261ad6305bdea31103ffc09c30f25558345966de2`
```text
{
  "exit": 0
}

```

### wrath/rustc-version-submission.json
Bytes: 112; SHA-256: `165b8bfa552eae12293e3bafb5a931a9b0450fc09d25d2a6a6c73ed5a69f5c53`
```text
{
  "argv": [
    "/usr/bin/rustc",
    "-Vv"
  ],
  "cwd": "/home/osso/Projects/wow/wow-ui-sim",
  "env": {}
}

```

### wrath/rustc-version.stderr
Bytes: 0; SHA-256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
```text

```

### wrath/rustc-version.stdout
Bytes: 225; SHA-256: `e9e4ced8161a774af62ef1d599e5c84c25735205d052feaf97e7d480c29d135e`
```text
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1

```

## Evidence capture note
Seven independent git-show hash results and read-only Git outputs transcribed from observed tool results above. A later full-diff capture could not persist across Pyrun calls and is not counted as evidence; no command was replayed.
