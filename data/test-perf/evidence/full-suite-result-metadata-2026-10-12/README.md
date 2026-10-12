# Full-suite result metadata — pending final proof

Explicit user approval: record only actual executed command and `WOW_SIM_TIMEOUT_FIXTURE_CAPACITY` in future result files; no scheduling or suite rerun.

`steps.<name>.command` is the argv passed to the child, including `--offline --locked`. `steps.<name>.environment` contains only the reservation key: JSON null when absent; explicit empty string and `reserved` preserved. Existing execution argv/environment are unchanged.

New sandbox subprocess tests: RED0/3 atb2cabc0e1 (missing command field); GREEN3/3 at3eab34fb9. Child stubs record actual argv/environment and result JSON is read from disk/stdout. No real Cargo or full suite executed. Existing deploy script installed only the user-owned controller; source-byte equality/mode0755 recorded. Independent remaining-six-regression/install proof pending.

Future metadata cannot reconstruct frozen BA's original environment or prove contention causality. Frozen15,038PASS/3FAIL/19skip/15,041executions and full handoff OPEN unchanged. No simulator/vendor/threshold/service/provider changes.
