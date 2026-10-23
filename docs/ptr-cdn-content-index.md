# PTR 12.1.7 CDN Content Index

`tools/gen_blizzard_ui_content_index.py` rebuilds the pinned PTR file-to-CDN-range index from authenticated offline Blizzard metadata. It does not download content or use Gethe files as runtime replacement bytes.

## Selected identity and regeneration evidence

Selected target as of **2026-10-23**: product `wowt`, version **12.1.7.70493**, BuildId **70493**. The versions record (sequence `4097283`) advertises build config `fc3eaa61ff0ade6bf35dbab9727bc161`, CDN config `c13b476a09cb3262797a6fdc88103036`, product config `8e1095475b945aa02fd3f2a2d22993b7`. Two small versions/cdns GETs reconfirmed these identities; the handoff receipt is `p1217-source/retarget-70493-metadata.json`.

**Regeneration pending.** `data/blizzard-ui-builds/ptr.json` and `data/blizzard-ui-files/ptr.txt` still contain the authenticated **70424** snapshot. They are deliberately not relabeled as 70493. The expected version/build/CDN constants now require 70493, so the stale index is rejected and `pinned_ptr_selects_exact_70493_source_identity` is expected RED until regeneration; no test was executed during this edit-only retarget. The generator and runtime still retain superseded Gethe/install pins, explicitly marked pending replacement. Resolve both from authenticated 70493 evidence before generating/accepting the new index. No `wowxptr`, `ptr2`, or branch-head fallback.

## Inputs

- Exact public Gethe `ptr` revision whose `version.txt` is `12.1.7.70493`; revision not yet established.
- Hash-verified 70493 Blizzard build/CDN configs, root and neutral/enUS TVFS files, download manifest and archive indexes. The install content key must come from the authenticated build config, not the product-config hash above.
- Refreshed community listfile and `data/wow-ui-sim-listfile.csv` for FileDataID-to-path mapping. Missing IDs may map only through a unique verified source content key.

## Regenerate

After the pending resume regression RED/GREEN and authenticated metadata acquisition, bind the exact 70493 Gethe revision in `PINNED_GETHE_REVISION` and runtime expectations, and the authenticated install key in runtime expectations. Then, from the lane checkout:

```text
python3 tools/gen_blizzard_ui_content_index.py --metadata-prefix <verified-70493-prefix> --source-dir <exact-70493-checkout>/Interface/AddOns --output data/blizzard-ui-builds/ptr.json
python3 tools/gen_blizzard_ui_manifest.py ptr
python3 tools/gen_limited_listfile.py
```

The manifest generator reads the newly generated index's exact Gethe pin; never run it against the stale index expecting 70493. Refresh community mapping first; regenerate the limited listfile again after the new manifest if required. Commit the real index, path manifest, mapping and resolved identity constants together. Full continuation, metadata filenames, slot admission and proof order live in the source lane's `SLOT-RECIPE.md` handoff.

The versioned schema retains `product`, `version`, `build_key`, `cdn_key`, `install_key`, `gethe_revision` and per-file path, FileDataID, content/encoding keys, archive, offset and size. Verify configs, TVFS root/leaf membership, download identity, archive footer identities and complete source coverage; do not fabricate entries or carry forward old ranges without regeneration.

## Historical 70424 generation (not selected-target proof)

On 2026-10-22, build config `328e394e14935f6e1017674de430603e`, CDN config `ccc89ec4bcc03f6cc1cc1c7861d9ed0f`, install key `531a6f831c609097179d4bc49731007b` and Gethe revision `b87363e0770c2f15b1e2f154de8ad3b0cf3ea90a` produced 4,570 indexed paths (+793/-248 against 69594) and 146,843 limited-listfile rows. Neutral TVFS key `76cf6489e8564ef5b4246362b8ab0bbc` used mask `0001F3F6`; enUS key `552876516b47a8d78cc9a50d55666997` used mask `00000002`; flags were zero. Both were authenticated root members. All 1,422 archive-index footers matched; indexed content/ranges were verified. These counts/keys do not describe 70493. Historical partial acquisition and the 69594 startup baseline do not establish 70493 startup compatibility.

## Tests

```text
python3 -m unittest tools.tests.test_gen_blizzard_ui_content_index
```

Runtime identity and range/content tests: [source contract](specs/ptr-blizzard-ui-source.md). No builds/tests or cache sync occurred during the retarget.
