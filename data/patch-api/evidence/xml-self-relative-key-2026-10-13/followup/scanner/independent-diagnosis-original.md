# XML follow-up scanner counterexample (read-only diagnosis)

## Exact classification

The retry3 projector failed at the default JSON proof `independent/xml-self-relativekey-startup-anchor-current/proof.json` (the fourth proof projection reached after startup report): inside `runtime`, at `runtime-methods-anchor-preservation.files.stderr.complete_text`. The complete string is 21,022 characters / 363 lines. Its first line is `[  0.000s] [Startup] WowLuaEnv::new begin`; its last is `[  8.122s] [Startup] WowLuaEnv::new complete`. This is ordinary captured diagnostic text in a JSON string field, **not serialized JSON or JSONL**. The scanner's line fallback tries `json.loads()` on line 1 and raises `unparseable nested JSON encoding`.

There is a second same-class counterexample at `runtime-template-child-anchor-preservation.files.stderr.complete_text` (1,906 characters / 33 lines; `[  0.000s] [Startup] WowLuaEnv::new begin` through `[  0.964s] [Startup] WowLuaEnv::new complete`). Both are ordinary timestamped startup logs, not nested serialized records. The same proof also contains `runtime-normal-mists-startup.files.stderr.complete_text`, ordinary 220-line timestamped diagnostics; its timestamp prefix is `[  0.000s] [startup] wow-sim process started pid=2298349`.

The failing value comes from retry3's fourth fixed item, the JSON proof projection, not the default-format report. The earlier two failures on timestamp-prefixed strings and `[Startup]` category keys were the same wrong classification family; retry3 fixed those instances too narrowly. Retry2's ordinary-container depth issue is separate and already corrected by not charging ordinary JSON nesting as serialized-decoding depth.

## Minimal parser classification

Preserve the outer JSON document parse. For each nested string, scan the original text and escaped privacy forms first. Only attempt recursive JSON decoding when the *entire trimmed string* is a valid JSON document (or every nonempty line is independently a valid JSON value under the explicitly supported JSONL contract). A failed speculative parse of a JSON-looking string is ordinary text, not a publication error. Do not infer JSONL merely from punctuation at line starts; timestamp/log prefixes remain TEXT. Charge the depth budget only for successful nested serialized-JSON decodes. Continue traversing every ordinary dict key/value and list member without consuming that serialization-depth budget.

This follows the literal requirement—retain complete TEXT and valid embedded JSON/JSONL without reserialization—without requiring every brace/bracket-looking ordinary expression to parse as JSON. Do not change payload bytes, rewrite or reserialize JSON, or weaken escaped privacy scanning.

## Concrete behavioral fixture plan (no implementation)

1. Put a complete 33-line timestamped `[Startup]` log in a JSON field; assert validation accepts it as TEXT and leaves the field byte-for-byte unchanged.
2. Put a Lua/proof expression such as `[key] = { value = "x" }` in a JSON string; assert acceptance as ordinary text, not a nested-JSON error.
3. Put valid embedded JSON object/array strings and supported JSONL strings in JSON fields; assert recursive privacy scanning sees their decoded values while retained outer text is unchanged.
4. Put escaped private identity text inside valid embedded JSON and in ordinary timestamped text; assert both are rejected by the privacy scan, demonstrating classification does not bypass scanning.
5. Nest ordinary objects/lists deeper than eight levels with no serialized-string layers and assert acceptance; then nest valid JSON-encoded strings beyond the configured serialized-decoding limit and assert the depth guard rejects that case.

## Scope / proof

Diagnosis only. No publisher rerun, edits, runtime/build/auth/hash/delegation, or change to original proofs/public-copy gate. Retry3 partial projections and source artifacts remain as found. Initial 77 public-copy gate remains PASS and untouched.
