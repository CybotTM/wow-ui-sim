# PTR 12.1.7 CDN Content Index

`tools/gen_blizzard_ui_content_index.py` rebuilds the pinned PTR 12.1.7 file-to-CDN-range index from already downloaded Blizzard metadata. It does not download content and never uses Gethe files as runtime bytes.

## Inputs

- Gethe checkout at `b87363e0770c2f15b1e2f154de8ad3b0cf3ea90a` on `ptr`, version `12.1.7.70424`.
- Blizzard `wowt` build config, CDN config, decoded neutral/enUS TVFS files, download manifest, and verified archive indexes.
- `data/wow-ui-sim-listfile.csv` for FileDataID-to-path mapping. New IDs fall back only when one source path has the verified content key.

The generator verifies MD5 identity of the downloaded build/CDN configs, root TVFS, neutral/enUS leaf TVFS files referenced by that root, and download manifest before parsing them. It then verifies the exact Gethe revision/version, `WOW-70424patch12.1.7_PTR`, `wowt`, TVFS content keys, download encoding keys, archive-index footer hashes, and complete source coverage.

## Regenerate

```text
python3 tools/gen_blizzard_ui_content_index.py \
  --metadata-prefix /tmp/ptr-metadata \
  --source-dir ~/.cache/wow-ui-sim/wow-ui-source-git/ptr/Interface/AddOns \
  --output data/blizzard-ui-builds/ptr.json
```

The committed output is `data/blizzard-ui-builds/ptr.json`. Its versioned schema has top-level `product`, `version`, `build_key`, `cdn_key`, `install_key`, `gethe_revision`, and per-file path, FileDataID, content/encoding keys, archive, range offset, and size.

## Selected identity and regeneration evidence

Selected build key `328e394e14935f6e1017674de430603e`, CDN key
`ccc89ec4bcc03f6cc1cc1c7861d9ed0f`, install content key
`531a6f831c609097179d4bc49731007b`. No `wowxptr`/`ptr2` fallback.

On 2026-10-22, authenticated offline metadata generated 4,570 indexed paths
(+793/-248 against 69594). The refreshed limited listfile contains 146,843
rows. Neutral TVFS content key `76cf6489e8564ef5b4246362b8ab0bbc` is selected
by root locale mask `0001F3F6`, content flags zero; enUS-only TVFS key
`552876516b47a8d78cc9a50d55666997` uses mask `00000002`, flags zero.
Both leaf keys occur in the authenticated root. All 1,422 downloaded archive
index footer identities match the CDN config; every indexed path matches
source content (including CRLF normalization), download encoding identity
and an archive range. This proves index generation, not runtime acquisition,
startup or full locale-selection compatibility. The previous 69594 startup
baseline remains historical until the epoch lane captures 70424.

## Tests

```text
python3 -m unittest tools.tests.test_gen_blizzard_ui_content_index
```
