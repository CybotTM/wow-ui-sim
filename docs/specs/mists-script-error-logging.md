# Classic script-error logging

## Contract

Under `client-mists`, `client-era` and `client-anniversary`, publish
`C_Log.LogMessage(message)` and
`C_Log.LogErrorMessage(message)` through the existing Rust console sink.
Each accepts a string, appends it once to `SimState.console_output`, and returns
no values. Retail/PTR and Wrath retain their existing publication behavior.
[Forever quest-dialog logging](quest-readiness-dialog.md#supplied-major-faction-reputation-and-error-logging)
adds an explicit caller of this same sink without changing the classic predicate.
Do not add namespace strictness or synthesize missing namespaces.

Cached Mists, Era and Anniversary
`Blizzard_APIDocumentationGenerated/LogDocumentation.lua:5,11-25` declare this
namespace and both string arguments. Original
`Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua:83` logs the formatted error
before delivering registered error callbacks. The console sink models local
logging, not native logging services or priority semantics.

## Tests

`tests/mists_script_error_logging.rs` loads the complete hash-pinned original
ScriptErrors chunk, invokes its installed handler, and asserts exactly one
formatted console message and delivery of the original error to a listener.
Behavioral RED failed at original line 83 on Mists, Era and Anniversary.
GREEN against shared producer `b37d2a509` asserts exactly one console message,
callback delivery, and exactly the deliberate rendered error with no secondary
error. Original ScriptErrors bytes are identical across these three caches.

[Integrated profile receipts](startup-explicit-namespaces.md#proof-level) own the
post-rebase test/startup matrix. The real producer eliminates the reporter failure;
it does not suppress the original error being formatted. Mists's StoreUI root is
fixed separately on comparison master `9c0251fca`.

## Limits

LogWarningMessage, LogMessageWithPriority, logging services, and the StoreUI
nil-character root are outside this repair. The unrelated VM diagnostic that
misnames a nil global as `(for generator)` has a separate reproduction.

See [Lua API architecture](../lua-api.md).
