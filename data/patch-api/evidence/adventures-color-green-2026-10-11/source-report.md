# Bounded source/data gate — 33cd07687

Date: 2026-10-11. Inspected HEAD: `33cd076872da25692cea7f3bd7bc2e466f9442b3`.

**SOURCE/DATA PASS. Runtime GREEN NOT established.** Worker epoch180245 remains outside this source-only gate. No build, check, test execution, runtime, ops, delegation, or repository edits performed. Only this external report was written.

## Literal palette evidence

Retained source: `../wowless-primary-color-discovery/globalcolor-opaque-rows.json`, rows86–89. `colors.rs:28–45` adds precisely these four entries to `NAMED_COLOR_GLOBALS`:

| Row | Global suffix | Raw AARRGGBB | Literal RGBA |
| --- | --- | --- | --- |
| 86 | GREY | ffb9b9b9 | (185/255,185/255,185/255,1) |
| 87 | BLUE | ff4eadd7 | (78/255,173/255,215/255,1) |
| 88 | ORANGE | ffc6644b | (198/255,100/255,75/255,1) |
| 89 | YELLOW | ffffd200 | (1,210/255,0,1) |

All names are prefixed `ADVENTURES_COMBAT_LOG_`. Raw row names/values match exactly; all encoded alpha bytes are ff. This verifies the chosen literal AARRGGBB interpretation, not native API decoding. The retained JSON explicitly labels encoding unverified. The independent-reader artifact reports zero raw structure/name/value mismatches across215 physical rows, explicitly not RGBA/native proof.

Code comment and updated spec identify retained Mists5.5.4.70268 data as **shared simulator palette**, expressly not current Retail-native RGB/RGBA parity. No profile cfg guards surround these four rows.

## Behavioral coverage and wiring

- `tests/garrison_damage_classes.rs:8–34`: new first source-declared test constructs `WowLuaEnv::new()` without addon fixture loading; checks four existing globals, exactly three `GetRGB()` returns, numeric channels and concrete channel values with tolerance1e-12. Behavioral public-output assertions, not source-shape assertions. Declaration order does not guarantee execution order. RGB only: no new alpha runtime assertion.
- `colors.rs:376–381`: existing registration iterates `NAMED_COLOR_GLOBALS`, creates color tables with all four channels, and installs globals. `colors.rs:392–398` and `font_strings_collection/mod.rs:61` retain normal bootstrap wiring.
- `tests/integration.rs:1` invokes `integration_tests!`; Cargo.toml:178–180 declares that target; `crates/test-harness/build.rs:442–446` discovers root Rust test modules. New test is in an existing discovered module.
- Diff from b58020420 to33cd07687 changes this test file only by prepending the28-line RGB test. Authentic consumer test remains unchanged: cached mixin OnLoad and AddCombatEvent, Fire mask4 and Holy mask2, two exact formatted messages and distinct labels. Existing collaborators are UI wiring/actors and spell input; no fixture colors were added.
- Cached `Blizzard_GarrisonUI/Mainline/Blizzard_AdventuresCombatLog.lua:116–129` resolves school mask, selects textColor, formats the entry, then calls `textColor:GetRGB()` at129. No vendor patch is in this commit. Commit33cd07687 touches only colors.rs, this test module and its spec; no unrelated C API changes.

## Finding2293 falsification

Reject “fixture omitted Blizzard_Colors” as explanation for the missing engine globals.

`Blizzard_GarrisonUI_Mainline.toc:4` explicitly declares `Blizzard_Colors` via RequiredDep. Consumer calls `with_blizzard_addon_closure` with empty overrides. `tests/common/blizzard_addon_harness.rs:30–42` delegates to closure construction; `blizzard_addon_closure.rs:24–33` calls canonical discovery. `src/loader/mod.rs:611–631,642–674` builds the TOC map from eager and LoD pools and recursively queues `toc.dependencies()` plus optional dependencies. `src/toc/mod.rs:356` recognizes Dep/RequiredDep/Dependencies/RequiredDeps. Thus it is not a roots-only fixture omitting declared dependencies.

All four cached Blizzard_Colors Lua files (Mainline/ColorConstants.lua, Mainline/ColorManager.lua, Shared/MaterialColors.lua, Shared/ColorConstants.lua) contain zero `ADVENTURES_COMBAT_LOG` references; they consume other engine colors, build derived tables, and expose accessors. Loading those scripts cannot supply these four absent engine-global definitions. Source evidence establishes the bootstrap omission rather than the suggested fixture-color workaround; this is not a fresh runtime closure execution proof.

## Readability / anti-pattern scope

Manual audit limited to four added literal rows and the new28-line test. No readability violations found. Palette bytes/divisor are explained by literal AARRGGBB provenance; test tolerance and three-channel count have clear assertion context. Test has two nested loops, no complex conditionals, no suppressions, no hidden I/O or unrelated blocks. New code has0 TODO/FIXME/HACK/XXX, empty catches or commented-out code blocks. No metrics command or broad audit run.

## Proof ledger / limits

Fresh source evidence: git show33cd07687; git diff b58020420..33cd07687; HEAD identity; empty diff of current colors/test files against33cd07687; retained opaque rows; inspected registration, harness/dependency walk and cached consumer/colors. These cover source/data only.

Caller-provided prior evidence (not rerun here): actual b580 consumer RED at niltextColor line129; two catalog PASS, original load PASS; fmt check exit0. Updated spec records these separately from pending follow-up. Prior PASS does not prove current consumer GREEN; new test execution and formatted Fire/Holy reachability await main-owned runtime worker180245. No current runtime or overall completion claim.
