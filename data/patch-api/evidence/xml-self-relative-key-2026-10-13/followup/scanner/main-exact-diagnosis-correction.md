# XML follow-up scanner: exact retry3 correction

Main read-only diagnostic inspected `xml-self-relativekey-default-format-current/proof.json` with the actual retry3 classifiers. Failed paths are `epochs.default.terminal.ExecStart` and `epochs.format.terminal.ExecStart`. Both hold ordinary systemd property text, not serialized JSON:

```
{ path=/usr/bin/env ; argv[]=/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo check --offline --locked --lib --bins -j4 --message-format=json-render-diagnostics ; ignore_errors=no ; start_time=[Tue 2026-10-13 17:37:54 CDT] ; stop_time=[Tue 2026-10-13 17:38:18 CDT] ; pid=2304584 ; code=exited ; status=0 }
{ path=/usr/bin/env ; argv[]=/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo fmt --check ; ignore_errors=no ; start_time=[Tue 2026-10-13 17:38:14 CDT] ; stop_time=[Tue 2026-10-13 17:38:32 CDT] ; pid=2305791 ; code=exited ; status=0 }
```

Agent4057's exact retry3 attribution to the startup gate is rejected: partial retry3 projections already contain its complete report AND proof, followed by the default-format report. The actual timestamp/log-tag controls already passed. Preserve the complete original agent report and every historical prepublication failure; its general classification advice remains applicable, its exact remaining-file/value claim does not.

Root cause: punctuation was used as proof of serialization. Keep outer JSON/JSONL parsing strict according to selected format. For arbitrary nested string values, scan complete plain/escaped text first; recurse only after successful whole JSON or all-record JSONL decoding. Speculative parse failures classify ordinary text, not malformed declared JSON. Count serialization layers separately from ordinary object nesting. This is not a compatibility fallback or acceptance of invalid declared JSON.

Behavioral controls must accept byte-unchanged systemd ExecStart and timestamp/tag/Lua-expression text; reject escaped private values in both plain and valid encoded text; inspect valid nested JSON/JSONL; accept deep legitimate structure; reject excessive serialized layers. No public writes or executable/library/archive/source authentication occurred in diagnosis. Initial77 acceptance remains unchanged; this correction does not revalidate it.
