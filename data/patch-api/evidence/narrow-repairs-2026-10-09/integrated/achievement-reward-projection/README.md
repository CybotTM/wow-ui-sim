# Achievement reward update and return-arity receipt

Tests-only revision `069f1cfbd9ae3b184c81b4fa898cca4cd098510f`; saved epoch `20261011T044347Z`. Producer unchanged. Existing basic reward-value tests remain in `tests/c_achievement_info_probes.rs`; this is not the first reward projection proof or a reproduced bug repair.

New selector: `achievements_api::test_reward_item_id_tracks_supplied_achievement_state`.

| Boundary | Saved result | Scope |
|---|---|---|
| Exact selected case | 1 passed, 0 failed, 0 ignored; 10717 filtered; exit 0 | Default Retail, bare environment with CASC disabled |
| Two supplied records before/after updates | Assertions pass in that one case | Some(180001)/None become None/Some(180002); four exact one-return observations |
| Formatting | exit 0 | `cargo fmt --all -- --check` |
| Compilation/check | exit 0 each | Offline/locked/default integration build and `cargo check --offline --locked --tests`, jobs12; existing warnings remain |
| Input stability | Six recorded hashes unchanged | Test, existing producer/registration, Cargo manifest/lock, build script; not all dependency provenance |

`retention-manifest.json` names ten exact copies with hashes/sizes and untouched private source paths. Full compiler/check streams and sealed binary remain private. [Final independent gate](final-report.md) accepted: 1/1 case, compilation/format/check exit 0, ten exact retained copies and scoped documentation links. Check emitted 13 existing warning diagnostics; not warning-free.

Synthetic IDs are explicitly inserted fixtures, not catalog assertions. Both nil observations query existing records; unknown-ID behavior is outside this test. No native reward mapping, secrecy/taint contract, other-profile GREEN, full suite or whole-handoff acceptance follows. No production code changed and no RED/GREEN bug-fix claim is made.
