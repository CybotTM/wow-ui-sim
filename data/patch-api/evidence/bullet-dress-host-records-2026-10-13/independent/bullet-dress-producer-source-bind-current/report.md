# Bounded producer SOURCE provenance bind

Date: 2026-10-13
Revision: `69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c`
Disposition: SOURCE gate PASS. Runtime/acceptance NOT ESTABLISHED; reportaccepted=false.

## Revision and tracked-source evidence

- HEAD is the requested revision. Initial whole-repository `git diff --exit-code HEAD --` returned exit 0, empty output. Status shows no tracked changes, but untracked `.code-index.db` and `verification/`; therefore this is not a globally clean checkout claim.
- Both producers and oracle are ancestors of the requested HEAD (three `merge-base --is-ancestor` commands, all exit 0).
- Bullet producer `4b9cb30dcce7daae491b680fa99e123435c187e9`, item producer `6af7108143f524750c674cf0d38b3995a14637a7`, oracle `402ab5a88aa90a6a5fd877f8c906964d29a1cecc` resolved through their commit histories/stats. Scoped diffs from each origin to HEAD are empty, exit 0: producer source and rejection oracle remain unchanged.
- All nine bounded paths are tracked. Current raw file bytes equal the full `git show <requested HEAD>:<path>` output encoded as UTF-8, using direct equality, no hashes. Scoped HEAD diff also exits 0. This strengthens Git-normalized equality with literal source-byte comparison for these paths only.

## Bounded coverage matrix

| Claim | Evidence | Level |
|---|---|---|
| Bullet numeric/secret boundary | queries.rs:29-35 unwraps original stack argument before requiring Num; bullet.rs:26-30 calls it | Source PASS; secret execution UNPROVED |
| Complete snapshot and allocation ordering | bullet.rs:31-48 clones before allocation, nil miss, row pushed before population; 10 numeric + 3 string + lines + bool fields | Source PASS; runtime/GC UNPROVED |
| Ordered separate line table | bullet.rs:82-92 creates/pushes lines, enumerates from 1, attaches and pops temporary root | Source PASS; execution/freshness UNPROVED |
| Item explicit boolean projection | c_item.rs:200-216 authenticates original argument, validates Num/Str, reuses parser, reads only item_dressability, false miss, pushes Bool | Source PASS; parsing/taint/runtime UNPROVED |
| Local publication and storage | bullet.rs:12-23 and c_item.rs:86-99 guard Retail/PTR; queries.rs:24 and c_item.rs:81 wire them; sim_state.rs:732-733 declares plain HashMap, state.rs:625 initializes empty map | Source PASS; exhaustive profile absence UNPROVED |
| Corrected rejection oracle | tests/widget_bullet/feed.rs:84 onward snapshots full existing sets for both rejection paths, compares full maps, empty backing/membership and zero notifications | Oracle source PASS; GREEN UNPROVED |
| Owning specs | Both specs describe explicit supplied inputs, original-secret handling and pending GREEN/native limits | Bounded alignment PASS; historical execution evidence not revalidated |

Prior source observations are supported by these bounded reads. Row/lines are Val values; item backing is a field, not a separate class. No broad readability/anti-pattern gate was run. This gate does not establish complete feature/spec satisfaction.

## Exact commands and results

Initial commands and results plus every subsequent argv/exit/output are recorded in `proof.json`. Full commit-source stdout is consumed for byte comparison but not copied into artifacts; byte lengths and equality results are recorded. Commands below use the fixed absolute repository path; no cwd change.

`git -C <repo> rev-parse HEAD` — exit 0; '69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c\n'

`git -C <repo> status --porcelain=v1` — exit 0; '?? .code-index.db\n?? verification/\n'

`git -C <repo> diff --exit-code HEAD --` — exit 0; ''

`git -C <repo> log -1 --format=fuller 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c` — exit 0; 'Close independent PTR RED classification in producer docs; author/committer Alessio Deiana; 2026-10-13 19:22:57 -0500'

`git -C <repo> show --format=fuller --stat 4b9cb30dc` — exit 0; 'Project supplied BulletTextList records into fresh Lua snapshots; 7 files, 108 insertions, 5 deletions'

`git -C <repo> show --format=fuller --stat 6af710814` — exit 0; 'Model item dressability with explicit supplied booleans; 7 files, 58 insertions, 11 deletions'

`git -C <repo> show --format=fuller --stat 402ab5a88` — exit 0; 'Assert rejected Bullet feeds preserve existing seeded sets; 2 files, 5 insertions, 2 deletions'

`git -C <repo> merge-base --is-ancestor 4b9cb30dc 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c` — exit 0; stdout=''; stderr=''

`git -C <repo> merge-base --is-ancestor 6af710814 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c` — exit 0; stdout=''; stderr=''

`git -C <repo> merge-base --is-ancestor 402ab5a88 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c` — exit 0; stdout=''; stderr=''

`git -C <repo> ls-files --error-unmatch -- src/c_api/ui_widget_manager/bullet.rs src/c_api/ui_widget_manager/queries.rs src/c_api/ui_widget_manager.rs src/c_api/item_spell/c_item.rs src/lua_api/state/sim_state.rs src/lua_api/state.rs tests/widget_bullet/feed.rs docs/specs/ui-widget-bullet-text-list.md docs/specs/item-dressability.md` — exit 0; stdout='docs/specs/item-dressability.md\ndocs/specs/ui-widget-bullet-text-list.md\nsrc/c_api/item_spell/c_item.rs\nsrc/c_api/ui_widget_manager.rs\nsrc/c_api/ui_widget_manager/bullet.rs\nsrc/c_api/ui_widget_manager/queries.rs\nsrc/lua_api/state.rs\nsrc/lua_api/state/sim_state.rs\ntests/widget_bullet/feed.rs\n'; stderr=''

`git -C <repo> log --format=%H %s -8 -- src/c_api/ui_widget_manager/bullet.rs src/c_api/ui_widget_manager/queries.rs src/c_api/ui_widget_manager.rs src/c_api/item_spell/c_item.rs src/lua_api/state/sim_state.rs src/lua_api/state.rs tests/widget_bullet/feed.rs docs/specs/ui-widget-bullet-text-list.md docs/specs/item-dressability.md` — exit 0; stdout='69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c Close independent PTR RED classification in producer docs\n6af7108143f524750c674cf0d38b3995a14637a7 Model item dressability with explicit supplied booleans\n4b9cb30dcce7daae491b680fa99e123435c187e9 Project supplied BulletTextList records into fresh Lua snapshots\n402ab5a88aa90a6a5fd877f8c906964d29a1cecc Assert rejected Bullet feeds preserve existing seeded sets\n33da6693e15d762fccba74ddbf1b9defa9137023 Record owned-input Retail RED and corrected copy frontier\n733399c38f48158206f8b3144cb139a52c537d9f Retain whole initial Bullet and dressability RED evidence\nf89518e49aecfbbb190d5be719f78e85e4b35556 Close independent initial dressability RED classification\n76e8b830931d1489ad5a89a75306bcbaa2bc61f7 Add passive BulletTextList records and real consumer RED tests\n'; stderr=''

`git -C <repo> diff --exit-code 4b9cb30dc HEAD -- src/c_api/ui_widget_manager/bullet.rs src/c_api/ui_widget_manager/queries.rs src/c_api/ui_widget_manager.rs` — exit 0; stdout=''; stderr=''

`git -C <repo> diff --exit-code 6af710814 HEAD -- src/c_api/item_spell/c_item.rs src/lua_api/state/sim_state.rs src/lua_api/state.rs` — exit 0; stdout=''; stderr=''

`git -C <repo> diff --exit-code 402ab5a88 HEAD -- tests/widget_bullet/feed.rs` — exit 0; stdout=''; stderr=''

`git -C <repo> diff --no-ext-diff --no-textconv --exit-code HEAD -- src/c_api/ui_widget_manager/bullet.rs src/c_api/ui_widget_manager/queries.rs src/c_api/ui_widget_manager.rs src/c_api/item_spell/c_item.rs src/lua_api/state/sim_state.rs src/lua_api/state.rs tests/widget_bullet/feed.rs docs/specs/ui-widget-bullet-text-list.md docs/specs/item-dressability.md` — exit 0; stdout=''; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:src/c_api/ui_widget_manager/bullet.rs` — exit 0; current 3105 bytes / committed 3105 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:src/c_api/ui_widget_manager/queries.rs` — exit 0; current 5609 bytes / committed 5609 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:src/c_api/ui_widget_manager.rs` — exit 0; current 2477 bytes / committed 2477 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:src/c_api/item_spell/c_item.rs` — exit 0; current 26068 bytes / committed 26068 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:src/lua_api/state/sim_state.rs` — exit 0; current 40109 bytes / committed 40109 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:src/lua_api/state.rs` — exit 0; current 50964 bytes / committed 50964 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:tests/widget_bullet/feed.rs` — exit 0; current 5753 bytes / committed 5753 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:docs/specs/ui-widget-bullet-text-list.md` — exit 0; current 12333 bytes / committed 12333 bytes; exact equal=True; stderr=''

`git -C <repo> show 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c:docs/specs/item-dressability.md` — exit 0; current 8707 bytes / committed 8707 bytes; exact equal=True; stderr=''

## Preserved failures and limits

Original `failed4126` remains unresolved; prior report/proof are untouched. No runtime, acceptance, GREEN, native, consumer, GC, broader surface or harness repair claim. No builds/tests/formatter/hashing, old binary/archive/cache reads, cwd-switch tools, Bash, delegation or repository edits. Historical RED/formatting claims are context only. Observations are sequential, not atomic. Two Pyrun context errors are recorded in proof.json; one lost membership result was repeated to obtain valid evidence.

## Artifact persistence

New files created exclusively with mode 0600; each file fsynced, destination directory and parent fsynced, full bytes read back and checked equal, modes checked. Fsync completion is OS persistence evidence, not a guarantee against every storage/hardware failure.
