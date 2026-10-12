# Later retention independent verification — 2026-10-12

**FAIL — bounded retention only.** HEAD `a9855840b4294d76c7a12bb9351a0624e8756c04`. No runtime/source-current acceptance claim.

| Manifest | Exact rows | Exact destination bytes | Byte/declared-size equal |
|---|---:|---:|---:|
| `final-stream-retention-manifest.json` | 86 | 1406560 | 86/86 |
| `profile-gate-source-retention.json` | 2 | 137152 | 2/2 |

New comparisons: 88 rows / 1,543,712 bytes; source mismatches 0. Each source/destination pair compared once, both byte lengths checked against declaration.

Reused prior independent proof: `/home/osso/.local/state/wow-ui-sim/verification/quest-currency-war-mode-current/through-precursor-retention-proof.json` — 108 copies / 2,889,750 bytes at `7e136499558625b6dcdc6ac6bda186b4caf52d5a`; zero repeated prior comparisons. Combined independent scope: **196 copies / 4,433,462 bytes**. README's **198 retained copies / 4,516,214 bytes** additionally includes 2 earlier main-only followup reports / 82,752 bytes; their manifest arithmetic reused, not newly independently byte-verified.

## README / links

Final README: 5284 bytes; SHA-256 `8fb3f28a8ad9b9093bf9f5f7971237a8e99e7b6c861cd1514a0f09144719e24e`; unchanged immediately before report creation: True. All 7 direct README links/anchors checked, plus complete linked contract and currency/QuestUtils wiki sections: 23/23 links pass. 4 inline packet-file references exist. Per-link paths, anchors and line numbers recorded in proof. No unrelated wiki sections recursively audited.

## Privacy / preservation

All 88 new text copies / 1,543,712 bytes scanned for actual credential/key/token/cookie signatures and embedded binary payloads; 0 findings. Both sides UTF-8/NUL/header checked; destination credential assignments, private-key markers, cookie formats, long base64/data URIs and control characters checked. Artifact binary paths and digest values are metadata, not secret/binary payloads. Signature scan does not prove exhaustive absence of every possible secret.

Existing proof.json, reports and authentication receipts untouched. No binary read/hash/authentication; no builds/tests/runtime/service operations, delegation, questions, repository edits or commits. Only new report/proof created exclusively outside repository. Main owns commits.

## Failures and mismatches

{
  "failures": [
    {
      "kind": "revision",
      "actual": "a9855840b4294d76c7a12bb9351a0624e8756c04",
      "expected": "b41e779ac"
    }
  ],
  "source_mismatches": [],
  "privacy_findings": []
}

Scope excludes runtime, latest-source acceptance, native parity and parent full handoff. Initial implicit-cwd git probe failed (128); explicit git -C established requested HEAD. No broad checks repeated.
