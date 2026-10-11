# Configuration-specific method baselines

Byte-for-byte copies of reviewed `complete-diff_methods_{extra,missing}.txt` files from [comparison evidence](../../../data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-profile-comparisons/), under the corresponding source `<profile>/` directory. Counts are diff rows relative to the frozen native inventory whose epoch, build, and product are unknown; these are not normative native oracles or historical Retail fixtures.

| Profile | Actual epoch | Forbidden | OnUpdate | Extra rows | Missing rows |
|---|---|---:|---:|---:|---:|
| ptr | retail-12-1-5 | 1 | 1 | 12412 | 33 |
| wrath | no-retail-epoch | 0 | 0 | 11633 | 38 |
| mists | no-retail-epoch | 0 | 0 | 11633 | 38 |
| era | no-retail-epoch | 0 | 0 | 11633 | 38 |
| anniversary | no-retail-epoch | 0 | 0 | 11633 | 38 |
| wowforever | no-retail-epoch | 1 | 1 | 11825 | 38 |

Layout: `<profile>/<actual-epoch>/forbidden-<0or1>-on-update-<0or1>/diff_methods_{extra,missing}.txt`.

Configuration-specific selection follows the user-approved policy; these files retain reviewed simulator comparison results, not native catalogs. Selection is for the current explicit configuration, not automatic execution of the full matrix. Default remains one current Retail baseline: the root [extra](../diff_methods_extra.txt) and [missing](../diff_methods_missing.txt) files. Those files and frozen WowDiscovery remain unchanged by this data-only copy.
