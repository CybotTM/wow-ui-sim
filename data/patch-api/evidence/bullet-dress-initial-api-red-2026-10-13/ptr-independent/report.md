# PTR initial RED: bounded independent classification

Epoch: `TOKEN_PTR_ROOT`
Selected Git source: `TOKEN_IDENTIFIER_06` (source13a).

## Verdict
Both recorded runs reached behavioral assertions, not setup failures. Combined: 4 tests executed, 1 passed, 3 failed; both launcher exits 101. Neither run is pending. This classifies initial RED only, not parent completion or full feature acceptance.

## Evidence and reached boundaries

| Case | Classification | Exact boundary and limitations |
|---|---|---|
| Bullet unknown numeric IDs | PASS, 1 case | stdout: `bullet_text_list_empty_default_unknown_numeric_id_returns_one_nil_simulator_policy ... ok`. Source tests/ui_widget_bullet_text_list_api_red.rs:44-60 checks exactly one nil for 7601001, -2.5, 0, 617.25, 4294967299, then helper checks recorded Lua errors. Simulator policy only; not native proof. |
| Bullet malformed widgetID | Genuine behavioral RED, 1 case | stderr: `nil: expected widgetID validation error`; Rust panic at tests/ui_widget_bullet_text_list_api_red.rs:8:33. First reject(nil) at line 35 reaches assertion line 27: pcall succeeded instead of throwing. Context/type/message assertions lines 28-33, string/bool/table inputs lines 36-38, and post-eval error check are unreached in this case. |
| Dressability empty host default | Genuine behavioral RED, 1 case | stderr: `empty supplied classification must return boolean false`; Rust panic at tests/item_dressability.rs:28:6. Public function and generated hyperlink setup checks passed (lines 12, 20-21). First query(6948), line 22, passed exact-one-return assertion line 14, then failed boolean-false assertion lines 16-17. Actual value/type is not printed. Inputs 901001, item:6948, generated hyperlink at lines 23-25 are unreached. |
| Dressability required input/security case | Genuine behavioral RED at ordinary argument boundary, 1 case | stderr: `required ItemInfo must reject input`; Rust panic at tests/item_dressability.rs:100:6. Untainted setup and public function checks passed (lines 36-39). First no-argument call at line 46 failed combined assertion line 42 (not ok AND string message): output does not distinguish successful return from non-string error. Nil/bool/table/function inputs lines 47-50, argument diagnostic check lines 43-44, and entire later secret/taint section lines 52-97 are unreached. Test name does not establish VM-secret authentication or AllowedWhenUntainted behavior. |

Recorded summaries:
- Bullet: `test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 10182 filtered out; finished in 0.76s`.
- Dressability: `test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 10182 filtered out; finished in 0.39s`.

## Launch/setup classification
Both admission.json records report allowed=true and profile=ptr. Both argv.json records select the same epoch integration executable with their exact module filters, --nocapture, --test-threads=1, an 86-second timeout, bwrap, WOW_SIM_CASC=0 and WOW_SIM_NO_SOUND=1. Launcher records show service result exit-code/status 101, not timeout/OOM/setup failure (880ms/135.1M bullet; 433ms/67.9M dressability). Each test environment logs WowLuaEnv::new complete before the observed assertion failure. No live-service inspection was needed. Service unit names contain ptr-green; actual test results, not unit naming, determine RED classification.

## Scope/provenance limits
Read receipt-integration.json and revision.json metadata; receipt reports PASS for a prior executable check and explicitly disclaims source-byte provenance. This verifier did not authenticate that claim. Read the six requested runtime files for each run and only the two requested test files through git show at source13a. Initial git show without explicit cwd failed before reading source; explicit repository cwd succeeded. Output truncation was resolved by reading captured output/selected evidence, not by rerunning tests.

No build or runtime launched; no executable read/hash, source/archive/library/cache/payload authentication, old acceptance payload opened, production inspection, or source edit. No native item classification, supplied-record behavior, consumer behavior, or complete security coverage proven. Historical reports preserved; this report is bounded classification, not parent completion.
