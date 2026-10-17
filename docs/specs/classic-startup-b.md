# Era/Anniversary startup producers — B

Audited startup roots beyond [A](classic-startup-a.md). [Shared shipped strings](classic-global-strings.md) and [marker active-state](classic-marker-state.md) have separate contracts. Simulator empty-state/selection policies were explicitly approved; they are not native defaults.

## What it must do

- [ ] R5: preserve actual required-LoD dependency loading and ordering, but do not force the Vanilla Transmog LoD addon into startup. Manual Transmog correctness is not claimed.
- [ ] R7/R8/R9: temporary empty product choices return a fresh collection; empty loot history returns numeric zero and original consumers recycle frames; unmodeled tracking returns nil and original consumer hides the icon. No populated behavior is implied.
- [ ] R11: project one-based roster selection over the existing guild member vector; zero means none, invalid/stale selection projects zero. This reset policy is approved simulator policy, not native evidence.
- [ ] R15: Classic CombatLogBase consumes public FrameXML masks without being replayed into the isolated secure environment; original color/filter values exist, public/secure writes remain isolated.

## How it works

- [Addon pipeline](../addon-loading-pipeline.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- R5: `src/loader/mod.rs` — selected-profile implicit startup roots.
- R15: `src/loader/addon.rs` — selected-profile secure replay classification (producer pending).
- R7/R8/R9: `src/lua_api/workarounds/temporary/classic_startup_defaults.rs` — approved temporary empty domains (producers pending).
- R11: `src/lua_api/globals/real/legacy_guild_selection.rs` — existing guild-vector selection (producer pending).

## Tests asserting this spec

`tests/classic_startup_b.rs` contains separate original consumer cases with `classic_startup_b_pins.tsv`. Both-profile RED observed: R5 startup discovers forced Transmog; R7 nil collection length; R8 nil numeric bound; R9 missing global; R11 missing selection API; R15 CombatLogBase replay error. Independent required-LoD preservation passes before producer. Tests/commits remain root-specific; groups share one sealed normal library without repeating logical tests.

## Known gaps (current cycle)

- [ ] GREEN, preservation and final startup capture pending.
- [ ] R10 shared skills and R14 shared zero-result timers await wrath-fixes landing; no duplicate model/producer introduced.
- [ ] Replacing temporary domains with real reward/roll/tracking/timer state is required before populated claims. Retire each endpoint when its backing system exists.
- [ ] Main-classified unmasked quest-material and reputation-label records remain disclosed, not fixed by this spec. R16 Store cursor attribution remains unstaged.

## Out of scope

Blizzard Lua changes, native parity, populated product/loot/tracking/timer domains, complete guild/skill XML lifecycle, speculative Store notification policy, and manual unsupported Transmog loading.
