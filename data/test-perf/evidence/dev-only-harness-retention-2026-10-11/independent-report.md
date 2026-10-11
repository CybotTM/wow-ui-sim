# Independent final artifact/doc retention gate

Verified: 2026-10-11. **PASS — bounded retention/documentation gate only.**

Commit: `a36b8e047a110218324e31390a1e7c3e5a7cd3a4`, observed HEAD. Repository: `/home/osso/Projects/wow/wow-ui-sim`.
Packet: `data/test-perf/evidence/dev-only-harness-2026-10-11/`.

Read/followed `/home/osso/AgentConfig/skills/verify/SKILL.md`; independent verifier, adapted Mode B. No delegation, operations, Bash, rg/fd, builds/tests/checks/fmt/full-suite runs, or binary reads/hashing. Read-only Git metadata queries and Python filesystem/JSON/text comparisons only. Writes limited to this report and `independent-report.json`; no source/private-original edits.

## Gates

- **[EXIST] PASS:** 178 unique sources and 178 unique retained copies present; exactly 180 packet files including README and manifest. No missing/unmanifested extra files. All 180 packet text files match the requested commit's Git blob identities.
- **[SUBSTANTIVE] PASS:** 178/178 source/copy pairs match byte-for-byte, recorded size, and SHA-256; total **13,392,139 bytes**. All 356 source/copy reads decode as UTF-8, without NUL content. Retained reports, compiler JSON streams, listings, execution outputs, worker receipts and generated-source snapshots are substantive evidence, not placeholders. Parsed all 76 retained JSON files plus retained JSONL records; per-file results are in the companion report.
- **[WIRED] PASS:** 23/23 scoped Markdown links resolve, including current heading anchors and wiki Sources links. Audited README, dev-harness contract/current gaps, remote-build generation boundary, integration-size brief, wiki system generation/Sources, and current index/log entries against retained reports. Historical wiki entries are not re-certified by this gate.
- **[ANTI-PATTERN] PASS:** No unsupported completion promotion or actual secret observed. Limited literal secret-signature screen returned zero alerts; not exhaustive secret detection. Allowed source paths, hashes, features and measured CPU metadata are not treated as privacy violations. Intentional historical RED/pending text is preserved evidence, not an implementation placeholder.

## Retained proof cross-checks

| Scope | Concrete retained result | Limit |
|---|---|---|
| Selected execution | Retail 8/8 + Mists 5/5 = 13/13; named stdout and saved exit 0 agree | Not execution of every listed case |
| Original Retail cases | 2/2 named ordinary-prefork cases; exit 0 | Successful prerequisite-conformance output unavailable |
| Temporary input | Exact marker 1/1, exit 0; prefork 2325→2326→2325, integration 10718 unchanged; ordered baseline listings restored, zero duplicates; temporary source absent | One local Retail fixture |
| Stock CLI startup | Exact `[]\n`, saved exit 0; eight ALSA diagnostic lines + one unavailable-audio line | CASC-disabled cached CLI; not GUI/native/audio acceptance |
| Timestamp cache | Before 655 fresh/8 nonfresh, root 0/7 fresh; after 663 fresh/2 nonfresh, root 7/7 fresh | Different revisions/graphs; not normalized speedup |
| New-input compilation | Both stages: 742 JSON records, 666 artifacts, 663 fresh/3 nonfresh, root library + six binaries 7/7 fresh; successful build-finished | Saved artifact evidence, no new compilation |
| Generated snapshots | Original integration 240850 B / helper 240920 B; original registry 666987 B / helper 667061 B; exact expected exported macro envelope contains original bytes in both pairs | Whole-file identity is not claimed |

Retail listings: integration 10718→10718; prefork 2325→2325. Mists: integration 8199→8200, sole addition `achievements_api::test_reward_item_id_tracks_supplied_achievement_state`; prefork 3→3. No other additions/removals/duplicates. Prior reward delta is not harness drift.

CPU arithmetic independently recomputed from saved cgroup counters: before 82.580194 CPU-s / 33.4831080820004 wall-s = 2.4663240281565395 equivalents; after 14.173933 / 14.475307194006746 = 0.9791801175638245. Matches retained figures. Whole-worker scope, before throttling, nonidentical revisions/artifact graphs and jobs/quota ceilings remain caveated; no saturation/per-binary/causal-speedup credit.

Existing format/default-check exit-0 proof remains accepted saved evidence, with six manifest deprecations and seven helper dead-code diagnostics; no separate Mists check or new warning-clean claim. Earlier reports' cache/new-input PENDING findings describe their earlier audit epochs; later retained cache/new-input reports provide distinct bounded proof.

## Exact findings

**No blocking retention/doc findings.**

1. **Explicit pointer caveat, not repaired:** README:19 and `reports/original-cases.md` preserve the wrong original relative `artifact_receipt` pointer. README names the actual `compile-controls/retail/artifacts.json`; absolute executable argv agrees. No private-original metadata edit required or performed.
2. **Numeric directory-mtime proof unavailable:** README:14 and `reports/new-input.md` explicitly distinguish recorded restoration from independent numeric historical equality. Before/after numeric directory timestamps were not persisted; no stronger claim accepted.
3. **Pre-gate pending wording retained:** README:36,38 and spec:7,176 retain pending retention/native boundaries. This new report passes retention; it does not retroactively edit that immutable tracked status or close original Mists RED/native/all-profile/full-suite/parent handoff.

Caller supplies one async pending full-suite submission at `c75ccfe02`; not polled or rerun here. Later documentation/data revisions earn no new code acceptance. Garrison authentic catalog, source-blocked Wrath/Era/Anniversary, desktop/native acceptance and historical suite failures remain open. No provider/model change, vendor acquisition, deployment or push.

## Proof ledger

Fresh evidence for this gate: Python source/copy byte, SHA-256, size and strict UTF-8 comparisons over the full manifest; saved JSONL/listing/stdout parsing; exact macro-envelope comparisons; scoped link/anchor resolution; text-only Git blob checks at `a36b8e047`. Commands queried Git `rev-parse HEAD`, `show --stat --oneline a36b8e047`, and `ls-tree -r -z a36b8e047 -- <packet>`; exit 0. No test/build command invoked. Companion JSON records every pair, packet blob, document digest, checked link, derived count, caveat and boundary. Report writes do not alter the verified source/copy scope.
