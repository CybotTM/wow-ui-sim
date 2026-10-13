# TextWithState RED — complete selected TEXT retention

Date: 2026-10-13. Canonical HEAD: `877895e2da5723700e4e26c1e2c957cadb7fbc16`.

**PASS — bounded new33 TEXT retention/privacy/link gate. Parent remains OPEN.**

## Coverage and proof

| Boundary | Result | Concrete evidence |
|---|---|---|
| Complete original/public retention | PASS | 33/33 full strict-UTF8 pairs; original bytes/digests, public bytes/digests, exact ordered privacy projection and terminal-LF equality all checked. Originals 2,172,634 bytes; public 2,075,282 bytes. |
| Inventory | PASS | Exact 33 payload paths plus manifest; no extra/missing files. |
| Recursive JSON | PASS | 21 JSON payloads; 2581 nodes. Every projected value/key/container/pointer compared; zero JSON-pointer omissions. |
| No clipping | PASS | Input-source proof retained whole: original 1,946,515 bytes, public 1,852,927 bytes; full 49-entry ledger and embedded source strings compared, not capped at 512KiB. All other selected proofs complete. |
| Privacy | PASS | 3325 original raw/decoded leaf/key streams and 3746 public streams. All mapping literals, private prefixes, numeric account component and WTF/Account patterns scanned. 0 literal leaks; 0 pattern hits. Mapping mode0600, not published. |
| Links/anchors | PASS | 13 Markdown links inspected across all selected MD plus entire owning README/spec; exact targets/anchors listed in proof. |

Original selected streams contain **zero WTF/Account path matches**; six mapping account/realm/player identity literals also checked without assuming their presence. Numeric account component checked separately. This is bounded pattern/identity evidence, not a universal privacy claim.

Four original empty format stdout/stderr streams are 0 bytes and become exactly one LF (1 byte), as disclosed by manifest/README normalization. No substituted summaries, omitted JSON source/ledger fields, or arbitrary cap.

## Preserved truth boundaries

- Historical input-spec consistency **FAIL** remains FAIL. Separate `1be01af09` correction is retained with prior independent verification (caller reference3815, distinct from historical input gate3807); negative-padding clamp assertion unchanged and unreached at RED. No new git-source authentication of that correction.
- Producer `2bb6cdde1` source/format/readability **PASS** remains source-only, **not GREEN**.
- Actual saved RED: **7 executed / 1 PASS / 6 FAIL; wait101; 0.72s**. Five direct supplied-table failures and callback/immediate feed failure preserved; cached consumer/security prerequisites and later assertions unreached. Passing rejected-feed control is not producer GREEN.
- Original stale Discrete `50f6f0ea8` metadata preserved verbatim after privacy projection; separate correction records actual input `64a2a57b2` and voice `2630c0ccf`.

## Limits and ledger

`proof.json` retains per-payload full TEXT digests/sizes/results, recursive pointer/type inventory, exact path inventory, raw/decoded privacy coverage, link targets, saved boundary evidence, command/scope/result validity ledger and inspection incidents. These digests authenticate selected **TEXT retention only**, not embedded source trees or executable/library/archive content. Copied old digest strings remain text.

No builds/checks/formats, runtime replay, new auth, operations, delegation, source-tree/binary/library/archive byte hashing, old-packet verification, canonical source/config/docs edits, or parent closure. Only requested `verification/text-state-red-retention/report.md` and `proof.json` written. Owning retention-PENDING wording was observed, not edited.
