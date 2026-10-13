# Pass 2 query models — retained evidence (2026-10-13)

Whole handoff **OPEN**. Current normal startup after SMOOTH **UNRUN**.
This packet documents existing observations only; packet creation ran no build,
test, executable authentication, delegation, commit, or service operation.

## Accounting and history

- Query epoch: nine query modules 46/46 PASS. Initial 14 filters: 78 PASS,
  1 FAIL, 1 ignored helper; 79 unique executed cases. Controls 31/31 PASS
  (lifecycle 6, Vertex probe 1, direct slots 17, Vertex animation 7).
  Missing-method comparison PASS; extra-method comparison FAIL, exit 101.
- Fixture reconciliation 72a8d83a82c73e8ad6371abd4f65faecf26e767c removed
  exactly 16 stale CreateControlPoint rows. One extra-method followup PASS:
  latest 79/79 unique GREEN, 80 execution attempts. Initial FAIL retained.
  Baseline-only correction reused compiled 6b522c595274906b7304dd302123e2e1f3561038;
  this is not proof for arbitrary later source changes.
- SMOOTH RED fcb0b7823: 4 PASS, 2 FAIL, exit 101; setter rejected SMOOTH,
  cached XML setup rejected it and curve assertion failed before motion.
  GREEN compiled 11131c724 / production 0c947a298: six Path PASS, exit 0;
  separate six lifecycle PASS, exit 0. These overlap earlier cases, not new
  additive unique totals. Full small runtime streams and command data retained.
- Historical pre-SMOOTH startup: exit 0, stdout exactly [] plus newline,
  zero reported Lua errors; separately TWELVE AnimSetup errors on stderr
  (original lines 42–53). Original stderr 18,126 bytes / 282 lines.
  Not stderr-error-free. Six inherited manifest deprecation warnings per
  retained compile stderr; aggregate generated-6 line is not a seventh warning.

## Sources and limits

Manifest source paths are relative to <VERIFICATION>, the private verification
root; exact epoch names preserve provenance. query/runtime-summary.md is only
report.md's pre-output preamble; smooth-green/runtime-summary.md is only the
sealed-runtime-verification.md pre-command preamble. Original whole TEXT report
sizes/digests are recorded, but giant aggregate reports/compile stdout are not
copied. Selected small streams/data retain all actual output content; public
copies normalize excess EOF blank lines in addition to path redaction.

Query assertion-reach categories overlap: actual VM secret 5, cached consumer 4,
XML/playback 2, computed layout 3, secret rejection/recovery 4. These are reported
passing test-body counts, not assertion counts or newly instrumented proof.
Simulator contracts only, not native WoW parity or broad UI acceptance.
SMOOTH cached XML is extracted unchanged into a minimal fixture, not full
normal startup; cache is external, not sealed by executable authentication.
Unknown curve and hidden-subtree checks are SOURCE ONLY. SMOOTH Stop uses zero
baseline; changed nonzero replay is tested on NONE. Lifecycle tests use Alpha,
not Path reverse/loop parity. Nonunit scale, competing animations, GPU,
full suites/profiles and broad acceptance remain unproved. Retained readability
finding: interpolate_path 33 lines, above 30-line threshold; not fixed here.

Historical startup guard: read-only root, writable isolated temp, 88s timeout
plus 2s kill grace; CASC/sound off, no third-party addons/SavedVariables;
291 cached Blizzard addons; read-only personal EditMode cache still used.
No personal fixture contents retained. GREEN units: MemoryMax 4G, MemoryHigh
3G, CPUQuota 200%, agents.slice, 88s + 2s, root read-only/private temp/network;
Path 0.78s / 914M peak, lifecycle 0.42s / 65.8M peak.
Default check and PTR remain pending in query report; no completion implied.

## Redactions and integrity

Absolute repo paths become <REPO>; verification paths <VERIFICATION>/epoch;
home paths <HOME>; temp paths <TMP>/basename; other private storage paths
<PRIVATE_STORAGE>. EditMode account/realm/character identity is replaced as
one whole field by <PRIVATE_FIXTURE_IDENTITY>. Basenames retain correlation;
paths in argv are documentary placeholders, not runnable commands.
No binaries, settings, raw personal fixtures or aggregate source diffs copied.

manifest.json inventories every public payload, including this README, with
relative path, byte size and SHA-256. Source-derived entries also record SHA-256
of ORIGINAL TEXT bytes and original size before excerpting/redaction or EOF
blank normalization. Original TEXT sizes/hashes remain unchanged. Each affected
payload records EOF blank normalization in addition to path redaction: remove
only trailing blank LF lines, retaining the final nonblank line's newline and
all preceding content. Private originals remain untouched; normalized public
copies are not byte-identical to unnormalized sources. Digests
cover only text/public payloads; no executable read or rehash. Existing hash
strings in retained reports are historical claims only. Manifest is the inventory
container, excluded from its own digest to avoid self-reference. Local checks
verify payload sizes/digests, JSON parsing, counts and absence of private paths;
they do not rerun behavioral tests or authenticate historical build provenance.
This EOF correction runs only bounded packet `git diff --check` as a formatting
development check, not an independent retention gate. Main commits precede that
independent gate; this correction performs no commit.
