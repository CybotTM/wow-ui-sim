# Shared Vanilla global strings

Era and Anniversary select one complete shipped enUS Classic dataset through `src/lib.rs`, not a Retail table with guessed overlays. Both selected profiles use installed `wow_classic_era` build 1.15.9.70003, active build key `2417084b8b041ee120d02d4adf31b0bc`. [Acquisition and generation](../updating-blizzard-ui-to-a-new-patch.md#classic-eraanniversary-globalstrings).

## What it must do

- [ ] Select all 19,624 shipped rows for both profiles, independently of Retail/Forever string tables. No fallback to another profile's dataset.
- [ ] Original quest count formats three and zero out of twenty with the shipped two-placeholder template; missing-string restore uses the same selected table and preserves vendor-owned text.
- [ ] Original microbutton tooltip and original HD/nameplate setting validators consume the shipped labels.
- [ ] Do not invent `OPTION_TOOLTIP_HD_TEXTURES`: it is absent from the shipped base DB2. Retain any resulting consumer error as remaining.

## How it works

- [Profile architecture](../wiki/systems/client-profiles.md)
- [Patch update workflow](../updating-blizzard-ui-to-a-new-patch.md#classic-eraanniversary-globalstrings)

## Implementation inventory

- `data/global_strings_classic.rs`: complete generated PHF table from local shipped GlobalStrings DB2 FDID 1394440.
- `src/lib.rs`: build-profile selection; existing registration/restore consume that selected module unchanged.

## Tests asserting this spec

- `tests/classic_startup_b.rs`: four pinned original-consumer R2 cases, including public post-load restoration. Both profiles RED 0/4 before data selection; no passive ingress was added.

## Known gaps (current cycle)

- [ ] GREEN, preservation and startup reconciliation pending.
- [ ] `OPTION_TOOLTIP_HD_TEXTURES` absent; no tooltip substitution authorized.
- [ ] Native global publication/hotfix application not measured. Installed base DB2 text is provenance, not native runtime parity.

## Out of scope

Wrath-reference overlays, invented labels, format relaxation, another profile's data, hotfix application, and unrelated startup roots.
