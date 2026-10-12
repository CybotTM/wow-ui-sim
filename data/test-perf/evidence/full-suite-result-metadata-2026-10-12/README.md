# Full-suite result metadata — verified

[Independent metadata-only PASS](independent-report.md): three new sandbox subprocess/result cases and six existing execution controls. Original three RED failures reproduce the missing command field; GREEN preserves unset/null, empty string and literal `reserved`. No real Cargo/full suite executed.

`steps.<name>.command` records exact argv passed to the child, including `--offline --locked`. `steps.<name>.environment` contains only `WOW_SIM_TIMEOUT_FIXTURE_CAPACITY`; no other inherited environment. Existing execution/scheduling/resource parameters unchanged.

User approved this metadata-only change. Existing `deploy.sh` installed only `/home/osso/bin/full-suite`; source-byte equality/mode0755 independently confirmed. No simulator build/service restart/provider change or push.

Future metadata cannot reconstruct frozen BA's original reservation state or establish contention causality. Frozen15,038PASS/3FAIL/19skip/15,041executions/full handoff OPEN unchanged. Raw new RED/GREEN, six remaining controls and install receipts retained; report copies preserve original private-directory link meaning.
