# Mists script-error logging

## Contract

Under `client-mists`, publish `C_Log.LogMessage(message)` and
`C_Log.LogErrorMessage(message)` through the existing Rust console sink.
Each accepts a string, appends it once to `SimState.console_output`, and returns
no values. Other profiles retain their existing publication behavior. Do not
add namespace strictness or synthesize missing namespaces.

Cached Mists `Blizzard_APIDocumentationGenerated/LogDocumentation.lua:5,11-25`
declares this namespace and both string arguments. Original
`Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua:83` logs the formatted error
before delivering registered error callbacks. The console sink models local
logging, not native logging services or priority semantics.

## Tests

`tests/mists_script_error_logging.rs` loads the complete hash-pinned original
ScriptErrors chunk, invokes its installed handler, and asserts exactly one
formatted console message and delivery of the original error to a listener.
Behavioral RED/GREEN and the Mists startup comparison remain pending.

## Limits

LogWarningMessage, LogMessageWithPriority, logging services, and the StoreUI
nil-character root are outside this repair. The unrelated VM diagnostic that
misnames a nil global as `(for generator)` has a separate reproduction.

See [Lua API architecture](../lua-api.md).
