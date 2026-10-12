# Independent bounded metadata verification — PASS

Code revision: `3eab34fb9e13104490906242dd5ded97f896a3c9`.
Observed HEAD: `9e3a142c78380723c098f1f131f0f25f5ac2a2ad`. Scoped committed diff since code revision and working-tree status are empty for `tools/full_suite.py`, `tools/test_full_suite.py`, `deploy.sh`.

## Coverage / proof level

| Contract | Evidence | Result |
|---|---|---|
| Missing-field reproduction | Reused `red.json` / `red.stderr`: exit 1; 3/3 failures specifically missing `command` in serialized step dictionaries | RED confirmed |
| New metadata | Reused `green.json` / `green.stderr`: exit 0; 3/3 pass; inspected committed tests execute real sandbox worker and fake-Cargo subprocesses | PASS |
| Unset / reserved / empty | Tests compare child-recorded capacity to None / "reserved" / "" and exact singleton result environment; json.loads of persisted and stdout results equal. None serializes as null, empty remains distinct; other environment keys excluded | PASS |
| Actual command | Child writes its actual argv; test compares result command to ["cargo", *actual argv]. Shared finish_scopes asserts exact argv ending --offline --locked across integration/prefork/lib | PASS |
| Existing execution controls | Six explicitly named existing cases only; exit 0; 6/6 pass in 1.431s. Exact argv/revision and full stdout/stderr saved in remaining-controls artifacts | PASS |
| Installed controller | `/home/osso/bin/full-suite` directly byte-equal to source, 6312 bytes, uid 1000, mode 0755; existing install.json records main's existing deploy.sh installation | PASS |

## Independent code inspection

`3eab34fb9` adds exactly four lines to run_step's result: command and a singleton environment dictionary using env.get("WOW_SIM_TIMEOUT_FIXTURE_CAPACITY"). They execute after subprocess.run. No source execution command, environment construction, logging, locking, scheduling or subprocess parameters changed. run() stores each step result and serializes it to both disk and stdout: metadata is wired into the existing external contract, not orphaned. Inspected test commit b2cabc0e adds child capacity recording and the three behavioral cases, without adding real Cargo calls. No new anti-pattern markers, placeholders or silent catches in these diffs.

Sandbox controls use TemporaryDirectory HOME and PATH-prefixed fake git/Cargo/systemd-run. Deploy control writes only its temporary HOME. No actual full-suite, build, network, service, production deployment, hashes or formatter invoked. Three new cases were not rerun. Raw child records from reused RED/GREEN are not retained in supplied evidence; passing assertions plus inspected fixture code establish that boundary, not a new independent execution of it.

## Limits

**Metadata fix PASS. Frozen BA3 FAIL; full handoff OPEN.** Future metadata does not recover historical environment or prove causal repair.

Initial inspection git commands exited 128 because Pyrun cwd differed; corrected with explicit cwd. An initial controls evaluation failed on an undefined variable before executing tests; the six controls ran once afterward. Neither incident changed repository or installed state.

## Artifacts

Owned root: `/home/osso/.local/state/wow-ui-sim/verification/full-suite-metadata-current/`

- `independent-report.md`, `independent-proof.json`
- `remaining-controls.json`, `remaining-controls.stdout`, `remaining-controls.stderr`

Reused unchanged: `red.json`, `red.stderr`, `green.json`, `green.stderr`, `install.json`.
