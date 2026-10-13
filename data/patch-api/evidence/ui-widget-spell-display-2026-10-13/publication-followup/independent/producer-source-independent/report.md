# SpellDisplay producer: source-only verification

Date: 2026-10-13. OVERALL: PASS within source/readability scope only. No behavioral GREEN, compilation proof, consumer acceptance, GC stress proof, or other-profile runtime proof.

Canonical checkout: `<REPO>`.
Producer/HEAD: `38ecbd8ee5dfae4315ca98001b7e794e324b1066`.
Base: `dbead9e2b94400b526f84280663e00736703bc81`.
Read/followed verify and rust-readability skills; acted as assigned verifier, no delegation. Main owns integration/operations.

## Four source checks

[EXIST] PASS — exactly six changed files present: spec (61 lines), manager module (80), queries (166), producer (116), test root (14), consumer (240). Paths below are canonical-relative.

[SUBSTANTIVE] PASS — `src/c_api/ui_widget_manager/spell_display.rs` implements register/query and four scalar/string writers plus nested projection, not a placeholder. Full five Rust files and current spec inspected; nine existing tests and passive DTO inspected. Spec explicitly leaves fresh-binary execution pending and requirements unchecked; historical RED statements were not independently verified here.

[WIRED] PASS — `src/lua_api/globals/missing_surface.rs:258` calls manager query registration; `src/c_api/ui_widget_manager/queries.rs:23` calls producer registration; `src/c_api/ui_widget_manager.rs:14` declares module. Producer `:17-22` installs `GetSpellDisplayVisualizationInfo` with `query_spell_display`. Test root retains Retail/PTR gate and all existing test modules.

[ANTI-PATTERN] PASS — added-line scan: TODO=0, FIXME=0, HACK=0, XXX=0, `except: pass`=0. Manual changed-source inspection: no empty catch or commented-out implementation. No new warning suppressions, fallbacks, default DTO, clamps, or vendor changes.

## Contract coverage matrix (source proof, not execution)

| Claim | Observed source evidence | Result |
|---|---|---|
| Fresh outer14 + nested16 | Producer `:39-45` allocates/pushes outer per hit; `:78-89` allocates/attaches new nested table. 9 numeric + 3 string + bool + nested = 14; 13 numeric + 2 string + bool = 16. No table reuse/metatable assignment. | PASS source |
| Roots before subsequent allocating work | Outer immediately pushed before writers. Nested passed immediately to `table_set`; `src/lua_api/methods.rs:575-591` roots value before interning key, stores it, applies barrier, then restores stack. Subsequent nested writers run after attachment to rooted outer. String values likewise protected by table setter before key allocation. | PASS source |
| Borrow released before projection | Producer `:30-34` clones owned record; temporary `Ref<SimState>` returned by `methods.rs:86-91` ends at statement semicolon. No state borrow crosses table allocation. | PASS source |
| Strict numeric/secret policy precedes absence | Producer `:26-29` invokes existing validator before map lookup. Queries `:27-33` unwraps secret then accepts only `Val::Num`, no coercion. Pinned rilua source `src/table_security.rs:401-426` rejects secret access without untainted/non-revoked context before yielding payload. | PASS source |
| Numeric identity retained | Existing `WidgetId::from_number`, manager `:36-43`, preserves f64 bits except signed-zero equality; producer passes ID unchanged. No integer cast, range/default/clamp. | PASS source |
| Exactly one nil on absence | Producer `:35-38` pushes `Val::Nil`, returns `Ok(1)`; allocation occurs only after Some. | PASS source |
| No extra attachedUnit | Only documented 14/16 fields serialized. Consumer context test asserts getter attachedUnit absent separately from container/frame context. | PASS source |
| Runtime publication only Retail/PTR | Producer `:11-15` uses `matches!(ACTIVE, ClientProfile::Retail | ClientProfile::Ptr)` before namespace/function registration. Diff adds no other profile gates and changes no existing gates. | PASS source |

## Actual cached schemas and field types

Inspected BOTH `<HOME>/.cache/wow-ui-sim/blizzard-ui/{retail,ptr}/AddOns/Blizzard_APIDocumentationGenerated/` files, not legacy/cache-root aliases.

`UIWidgetManagerDocumentation.lua:353-365`: getter widgetID number, SecretArguments=AllowedWhenUntainted, nullable single widgetInfo return.

`:1746-1764`: exact outer fields:
- Table: spellInfo.
- Numbers/enums: tooltipLoc, widgetSizeSetting, orderIndex, inAnimType, outAnimType, widgetScale, layoutDirection, modelSceneLayer, scriptedAnimationEffectID.
- Strings/textureKit: textureKit, frameTextureKit, widgetTag.
- Bool: hasTimer.

`:2138-2158`: exact nested fields:
- Numbers/enums: spellID, shownState, enabledState, stackDisplay, iconSizeType, iconDisplayType, textShownState, borderColor, textFontType, textSizeType, hAlignType, tint, showGlowState.
- Strings: tooltip, text.
- Bool: showAsEarned.

All fields non-nilable. Both schemas match producer spelling, DTO scalar representation and recursive fixture expectation. `borderColor`/`tint` are numeric enum fields, not color objects. `UIWidgetManagerSharedDocumentation.lua:148-149` declares SpellDisplay=13, DoubleStateIconRow=14 in both profiles.

Checked all 19 consumer fixture enum-name/value assertions against actual cached documentation in each profile: 19/19 matches each (38 total); no missing declarations. Values: Shown=1, Hidden=0, White=3, Large=2, Buff=0, text Shown=0, Blue=8, tint None=0/Red=1, HideGlow=0, Normal=0, Medium16Pt=1, Left=0, Top=4, animation None=0, OneHundred=0, widget layout Vertical=1, model layer None=0, set layout Vertical=0.

## Nine tests: source inspected, not run

Existing tests: queries.rs:7/73/103 (freshness/replacement/deletion, numeric policy, isolation); feed.rs:46/114 (event ordering/retention, rejected feed preservation); security.rs:9 (real-secret access/recovery); consumer.rs:106/191/209 (lifecycle, attached context, zero-spell polling).

Recursive fixture checks exact outer14/nested16 types/values and first supplied-table boundary before cached construction/security helper setup. Producer corresponds to these assertions; no assertion weakened. Consumer diff contains ONLY helper declaration and three call-site renames. Test-root diff ONLY updates stale scaffold comment. No claim these tests pass.

## Readability

No new changed-code violations found under rust-readability categories. New producer functions remain below 30 body lines, use direct guards and named query/write steps, with shallow control flow and no mutable collection accumulation, parameter overload, suppression, or opaque casts. Numeric field lists are explicit contract mappings, not duplicated branching.

Accepted spelling `load_and_register_spell_container_env` reveals cached load/registration effects and resolves supplied prior 3850 naming finding. Existing helper body and first-result ordering unchanged; no extraction or code weakening requested/performed.

## Fresh command proof

Working directory explicitly canonical for both checks:

1. `rustfmt --check --edition 2024 --config skip_children=true src/c_api/ui_widget_manager.rs src/c_api/ui_widget_manager/queries.rs src/c_api/ui_widget_manager/spell_display.rs tests/ui_widget_manager_spell_display.rs tests/widget_spell_display/consumer.rs` — exit 0; full stdout empty; full stderr empty.
2. `git diff --check dbead9e2b 38ecbd8ee` — exit 0; full stdout empty; full stderr empty.

`git diff --exit-code 38ecbd8ee -- <six files>` also exit 0, empty output; inspected working files equal producer revision. Initial command setup encountered non-repo Pyrun cwd and nonpersistent variable; explicit canonical cwd corrected setup before proof commands. No proof command rerun.

No Cargo/build/test/runtime/delegation/commit/tracked writes or sealed artifact/archive/hash reads. Existing untracked `.code-index.db` and `verification/` untouched. Only private report/proof files written at requested absolute destination (directory 0700, files 0600). Remaining behavioral verification belongs to main.
