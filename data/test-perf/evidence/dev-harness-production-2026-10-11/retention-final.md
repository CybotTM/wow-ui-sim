# Independent packet endpoint retention gate

Date: 2026-10-11

Packet: `/home/osso/Projects/wow/wow-ui-sim/data/test-perf/evidence/dev-harness-production-2026-10-11/`

[EXIST] PASS — 12/12 packet files tracked and present; 10/10 intended copies and 10/10 original source paths present.
[SUBSTANTIVE] PASS — 10/10 copies byte-equal to originals; 10/10 SHA-256 and 10/10 size matches; total 587,161 bytes matches manifest.
[WIRED] PASS — 4/4 README links resolve. README preserves ordinary cached production-selection evidence and separates zero helper records from the helper manifest's workspace/path dev-dependency requirement. No manifest-removal, new compilation, or native acceptance inference.
[ANTI-PATTERN] PASS — README/manifest contain 0 TODO, FIXME, HACK, XXX, or `except: pass` markers; copied worker/request are retained evidence, not newly implemented code.
OVERALL: PASS — packet endpoint retention only; no new compilation or native acceptance.

## Exact copy evidence

Each original path is the corresponding `source` entry in `retention-manifest.json`; each was opened directly. No absent paths inferred successful. SHA-256 was calculated on text copies and originals only, never binaries.

| Copy | Bytes | SHA-256 (copy and original) | Equality/manifest |
|---|---:|---|---|
| `admission.json` | 84 | `c80e5f15f01d2482669789de70dee13f934df255f943a6839d5e2d72573620b8` | PASS |
| `build.stderr` | 1205 | `338c6027bea10f18a5dfef6994b0a835804791fff33ee0efcd7b90b3ebdf7250` | PASS |
| `build.stdout` | 563009 | `6f286da3728f8c4eabfb9aaee7c6953239dfb5cf7f6cbd3cf8bfd152566744e4` | PASS |
| `outcome.json` | 950 | `4ce78e93eb601b1e97b97d281a237b1874fae21355086c8b8714864c2b1f5ef8` | PASS |
| `submission.json` | 1128 | `52de4694220e6f98832d15b862e033f50c7c4cf29032dc762bbd27cc3f017e3d` | PASS |
| `worker.py` | 2811 | `9870929945dce281256cd10542003e452281e2a83e3e99e2b97aeb69983531b7` | PASS |
| `request.jsonl` | 2861 | `8051fe13fd88dc240d8ef34a47853ef150c0cdf01232964eb6a33bce666694e7` | PASS |
| `queue-submission.json` | 979 | `72cc73d7b80de1ad5bb1bf87cced36fdceb843443ae31c95f0103ee2dd1b28e5` | PASS |
| `independent-report.md` | 4068 | `9cb8dc3df72665cccbacebb15d788fe560d00a7fd5d30d11f4f2afa959cd7b8e` | PASS |
| `independent-report.json` | 10066 | `20f202c9974a2775d362dcb60b914d5170b8e33f54e905a2f8f239bbd0b7690f` | PASS |

## Links

- `independent-report.md` — present (Independent audit).
- `retention-manifest.json` — present (Retention manifest).
- `../dev-only-harness-2026-10-11/README.md` — present (core extraction).
- `../dev-harness-build-inputs-2026-10-11/README.md` — present (staging input repair).

## Claim boundary

README's historical cached-production and zero-helper statements agree with the retained saved stdout, outcome, submission, and independent report. The independent report explicitly states the helper manifest is still required for workspace resolution; README preserves that distinction and allows explicit workspace selection. Cached build-script output is not execution proof. README's seven-seal statement is historical evidence, not a newly checked current-source seal. Garrison FAIL and native/full handoff OPEN remain recorded context, not newly verified acceptance.

README's “direct caller evidence, not a separate independent retention gate” describes the prior copy-preservation evidence. This external report supplies the requested new independent retention gate without modifying that historical README. Original historical mutation cannot be established retrospectively from endpoint equality; this gate made no original or repository writes.

Only artifact reads, text SHA/size/equality checks, README-link resolution, read-only Git tracked-file inventory, and this external report write were performed. No builds/tests/checks/full-suite/source acquisition, binary hashing, operational actions, execution of retained worker/request, or delegation. No Bash/rg/fd.
