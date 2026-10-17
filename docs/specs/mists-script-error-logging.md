# Classic script-error logging

## Contract

Under `client-mists`, `client-era` and `client-anniversary`, publish
`C_Log.LogMessage(message)` and
`C_Log.LogErrorMessage(message)` through the existing Rust console sink.
Each accepts a string, appends it once to `SimState.console_output`, and returns
no values. Other profiles retain their existing publication behavior. Do not
add namespace strictness or synthesize missing namespaces.

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
Behavioral RED failed at original line 83 on the missing namespace. GREEN
passes against producer `c9594ed03`: exactly one console message, callback
delivery, and exactly the deliberate rendered error, with no secondary error.
Five namespace controls also pass. Era/Anniversary extension proof is pending.
Mists startup returns to the merge-base
capture's exact two StoreUI records/two occurrences; the secondary ScriptErrors
record disappears. This pre-rebase result does not claim current-master parity.

## Limits

LogWarningMessage, LogMessageWithPriority, logging services, and the StoreUI
nil-character root are outside this repair. The unrelated VM diagnostic that
misnames a nil global as `(for generator)` has a separate reproduction.

See [Lua API architecture](../lua-api.md).
