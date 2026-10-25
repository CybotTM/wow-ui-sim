# Forever 70532 startup gaps

Verified 2026-10-25. The locally authenticated 1.60.1.70532 UI reaches simulator
registration gaps. Fixes publish documented shapes from explicit host inputs,
not guessed native character/account policy.

## Reproduction and selection

The lane's copied runtime cache retains all source bytes. Stock
`wow-sim --no-addons --no-saved-vars lua-errors` reproduced 76 raw messages:
38 underlying errors and 38 Blizzard_ScriptErrors reports. PaperDoll's
`[Game]` entry is allowed for camelot; the excluded Mainline alternative is not
its consumer. `PaperDollItemSlotButton_OnLoad` uses `AmmoNeeded() == false` to
hide AmmoSlot. Returning nil would not implement this condition.

The runtime's API documentation confirms all initial symbols/events except
`C_AccountServices.GetPreferredDatacenterLocalities`. The selected
GuildPreferredPlaySettings source calls that undeclared method, but that alone
does not establish its native signature or catalog.

## Supplied input boundary

The [contract](../../specs/forever-startup-inputs.md) distinguishes fixture state
from native policy. Publication checks `client_profile::ACTIVE`; registration
on other profiles is untouched. Tests use selected cached Blizzard functions
and vary host state. Ammo requirement is an explicit character input, not an
inferred class/item rule. Group-buff rows are supplied already expanded; no
native rank-expansion rule is claimed.

Eight original-consumer selectors reached GREEN at `4429dde8b`: ammo visibility,
event registration, group-buff rows, external URL state, cursor visibility,
bank purchase permissions, PvP completion and target spell. Their exact RED and
GREEN listings/executions are in the lane report. Subsequent source changes can
invalidate intersecting proof.

## Current unknowns and exposed consumers

`GetTargetSpellID` documents a non-null number but no absent-target sentinel.
Absent supplied input raises an explicit native-unknown error. Repairing earlier
PaperDoll/BagSlot errors lets more bag consumers reach this query; do not hide
these reports by returning an invented zero. Native facts were requested from
the laptop. Locality declaration/behavior is also pending that evidence.

The first repaired stock capture had 40 raw messages. It exposed
`C_Bank.FetchBankLockedReason` (nullable documented enum) and
`SOCIAL_UI_RAID_LIST_SYSTEM_STATUS_UPDATED` (documented payload-free event).
The completed supplied models include Discord, ping/restriction, spatial
housing, TTS save/bool settings, temporary scalar CVars and optional bank offers.
All 21 exact selectors pass at `1fcbddf67`. The [current baseline and per-record
classification](../../baselines/README.md#forever-lua-errors) retain only the two
approved native-unknown boundaries. Startup is not clean; no native idle value
or locality catalog was invented. Profile checks and release evidence belong to
the lane report.

## Excluded and unmodeled branches

The user approved correcting the positive housing expectation after proving
`Blizzard_HousingControls.toc` declares `## AllowLoadGameType: standard`. That
excludes camelot and the sole `HousingControls_LoadUI` bootstrap publisher. The
test retains both boolean projection branches and asserts the exact missing
publisher error and unloaded state; no loading or vendor changes are made.

Optional bank offers are covered as absent and supplied paid five-field rows.
Native pricing, affordability and free-offer purchase actions are not modeled.
External URL network launch, native buff-rank expansion and TTS persistence also
remain outside this startup/shape boundary. Laptop evidence disposition for
both retained baseline symbols: **laptop aso source-only audit 2026-10-25:
native-unknown**.

## Sources

- [Startup input contract](../../specs/forever-startup-inputs.md).
- [Profile selection](../systems/client-profiles.md).
- [Consumer-selection proof rules](consumer-selection-and-red-proof.md).
- Lane evidence: `/home/osso/.local/state/wow-ui-sim/handoff/forever-startup-gaps/REPORT.md`.

## See Also

- [[c-factory-startup-followup]] — historical frozen startup member accounting.
- [[casc-asset-cache]] — byte-preserving source-cache provenance.
