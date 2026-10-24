# PTR 12.1.7 CDN Content Index

`tools/gen_blizzard_ui_content_index.py` rebuilds the pinned PTR file-to-CDN-range index from authenticated offline Blizzard metadata. It does not download content or use Gethe files as runtime replacement bytes.

## Selected identity and regeneration evidence

Selected target as of **2026-10-23**: product `wowt`, version **12.1.7.70493**, BuildId **70493**. The versions record (sequence `4097283`) advertises build config `fc3eaa61ff0ade6bf35dbab9727bc161`, CDN config `c13b476a09cb3262797a6fdc88103036`, product config `8e1095475b945aa02fd3f2a2d22993b7`. Two small versions/cdns GETs reconfirmed these identities; the handoff receipt is `p1217-source/retarget-70493-metadata.json`.

**Authenticated regeneration completed.** The 70493 index and path manifest contain 4,578 files; limited mapping contains 146,853 rows. Install content key `bb46ab983520a2617d8b2119662ef38b` comes from the hash-verified build config, not ProductConfig. Exact public Gethe revision `2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af` reports 70493 and has the matching full AddOns inventory. Historical 70424 evidence below is unchanged. Runtime exact-identity GREEN and complete isolated provenance sync remain pending. No `wowxptr`, `ptr2`, branch-head or Gethe content fallback.

## Inputs

- Exact public Gethe `ptr` revision whose `version.txt` is `12.1.7.70493`; revision `2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af`.
- Hash-verified 70493 Blizzard build/CDN configs, root and neutral/enUS TVFS files, download manifest and archive indexes. The install content key must come from the authenticated build config, not the product-config hash above.
- Refreshed community listfile and `data/wow-ui-sim-listfile.csv` for FileDataID-to-path mapping. Missing IDs may map only through a unique verified source content key.

## Regenerate

After resume and advertised-host RED→GREEN and authenticated metadata acquisition, bind the exact 70493 Gethe revision in `PINNED_GETHE_REVISION` and runtime expectations, and the authenticated install key in runtime expectations. Then, from the lane checkout:

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

## 70493 authenticated inputs and generation receipt

Core metadata used eight bounded GETs. All 1,422 selected archive footer identities were verified against the new CDN config (126,589,736 bytes); 1,419 identical historical indexes were reused by immutable key and three fetched. Neutral descriptor `0001F3F600000000` selects `vfs-857`, content `e72b48c0960595af517ebe8c3b3a50d6`; enUS descriptor `0000000200000000` selects `vfs-2`, content `552876516b47a8d78cc9a50d55666997`. Both are authenticated 70493 root members; encoded keys and decoded sizes/hashes were verified. Fresh community mapping SHA256: `ff025c8956bfc030cd45d57fb4178c2b132406518873bb288727fa61a03215e1`. All acquisition outputs and generator stdout/stderr/argv receipts remain in the lane verification root `p1217-source-slot1-20261023/acquire-70493/`; source-generation receipt binds the complete generated file hashes. Generator source/cache refresh used a lane-private HOME; no shared source/runtime cache was changed.

| Generated file | SHA256 |
|---|---|
| `data/blizzard-ui-builds/ptr.json` | `e84a8de626166c53d7cccbe0cba65e141488f8cf315d4b03a36c4f181585da00` |
| `data/blizzard-ui-files/ptr.txt` | `ba4414999237ad7b50de73a2a61d7dd18fee31b2f2c3b70d5a21ad3cdc34d328` |
| `data/wow-ui-sim-listfile.csv` | `a86b1ee3173d04cb1e2d60128411d0d38a6bbcbc123af1f8c402be2fe9a9e36f` |

## 70493 source and interrupted-resume proof

At `a9b56d78e`, the exact 70493 identity selector and all 17 pinned producer tests pass; 9 Python generator tests pass. Historical 4cc resume RED and host empty-reply RED precede their respective fixes. All associated PTR compiles report zero warnings.

A sealed CLI (SHA256 `e0c8d685d3b96b3d8e459a41fbdebe215e54fef904b21b637dd0259dd517e295`) performed supported CDN synchronization into a private cache. A 1.030s interrupted launch exited124 with 306 content-verified files, exact schema2 provenance and no completion marker. The next supported launch completed in 8.959s: 4,272 extracted, 306 already present. All 306 prior file mtimes remained unchanged; all 4,578 files matched decoded sizes/content keys, none missing or corrupt. Completion bytes are `ok\n`, provenance SHA256 `5dc36452b60d6f159ca3ad423322ad36a83a30a1881488193aca63bcfc6221b2`. Original persisted CVar stat/hash stayed unchanged across both launches. The separate first full acquisition completed in 14.180s and is not interruption evidence.

Receipts live under verification `p1217-source-slot1-20261023/sync-70493-resume/`; `complete-proof.json` links final integrity/launch receipts. The private cache remains there for integrated startup. Supported sync integrity does not establish startup/panel compatibility; integrated PTR startup follows the master/toc merge, with Retail/Forever preservation still pending.
