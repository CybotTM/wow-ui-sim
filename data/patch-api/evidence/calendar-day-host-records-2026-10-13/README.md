# Host-only Calendar day records — bounded evidence

Producer: `396f0c9a6`. No normal day-event seeds, Lua editor or Club producer; existing numeric event-index defaults and filters remain unchanged. [Owning contract](../../../../docs/specs/calendar-day-event-records.md#current-bounded-acceptance--2026-10-13).

## Runtime coverage

| Profile | Day queries / real cached consumers / secrets | Numeric-index regressions | Separate normal CLI |
| --- | --- | --- | --- |
| Retail | 9/9 PASS | 11/11 PASS | Marker + `[]`, exit0, CLEAN, zero Lua errors |
| PTR | 9/9 PASS | 11/11 PASS | Marker + `[]`, exit0, CLEAN, zero Lua errors |

Independent [Retail gate](gates-current/independent/calendar-day-retail-producer-runtime-current/report.md) and [PTR gate](gates-current/independent/calendar-day-ptr-producer-runtime-current/report.md). Forty Rust cases total; startup observations are not extra tests. Both runtimes disabled CASC/sound and third-party addons/SavedVariables; existing WTF EditMode cache was read-only and still read. Native parity, output secrecy, full Calendar lifecycle, rendering, installed CASC and full-parent acceptance remain unproved.

[Default lib/binaries check](gates-current/independent/calendar-day-default-check-current/report.md): 733 records, exited0, zero source compiler messages; six manifest deprecations retained. [Source/spec/readability/format gate](green-retail/source-gate/report.md) remains separate from runtime proof.

## Complete selected TEXT

- [Retail RED](red-retail/README.md) / [PTR RED](red-ptr/README.md): 21 texts each; genuine pre-producer 0/9, exit101. Downstream consumer/security assertions unreached.
- [Retail GREEN](green-retail/README.md) / [PTR GREEN](green-ptr/README.md): 37 + 39 texts. PTR's three receipt-schema blocks are zero-launch records, separate from successful retries.
- [Runtime/check gates](gates-current/README.md): 19 texts; original erroneous README-size FAIL and additive correction preserved whole.
- [Final copy-retention reports](retention-current/README.md): four texts; preceding 76-text GREEN and 19-text gate-copy checks passed. Independent verification of these four new copies remains PENDING.

141 complete selected TEXT payloads, plus generated metadata. Historical packet PENDING labels describe their publication/selection checkpoint, not current runtime results. Exact ordered literal privacy substitutions and LF only; no clipping or historical JSON/JSONL reserialization. Private originals and maps remain mode0600. Copy integrity does not establish source/native provenance.
