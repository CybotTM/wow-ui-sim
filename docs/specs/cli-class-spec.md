# CLI class and specialization selection

`wow-sim` accepts class and specialization selections for post-startup scripts and UI capture. See the [CLI reference](../wiki/reference/cli-commands.md).

## What it must do

- [x] Accept case-insensitive class names and one-based indices from `CLASS_LABELS`.
- [x] Accept case-insensitive specialization names and one-based indices from the selected class's specialization catalog.
- [x] Resolve a specialization without `--class` against the class at the post-startup execution point.
- [x] Reject invalid values with valid choices and a nonzero exit status.
- [ ] Reject specialization selection on Wrath, Era, and Anniversary.
- [x] Apply selections through `A_Admin`, then dispatch `PLAYER_SPECIALIZATION_CHANGED` with `player` before `--exec-lua`.
- [x] Do not repeat `PLAYER_ENTERING_WORLD` for a class or specialization change.
- [x] Monk Mistweaver and Windwalker indices resolve to their matching specialization identities in the Blizzard specialization content frame.

## How it works

- [CLI reference](../wiki/reference/cli-commands.md)

## Implementation inventory

- `src/bin/wow_sim/class_spec.rs` resolves and applies the selected context.
- `src/bin/wow_sim/main.rs` accepts and schedules the flags.
- `src/lua_api/env.rs` and `src/lua_api/env_rilua.rs` execute the scheduled hook.
- `crates/sim-data/data/specializations.rs` returns class specializations in `order_index` order.

## Tests asserting this spec

- `tests/cli_class_spec.rs` checks class and specialization pairs through `--exec-lua` and exercises the Blizzard Monk specialization content frame.
- `src/bin/wow_sim/class_spec.rs` tests class and specialization resolution.

## Known gaps (current cycle)

- [ ] Unsupported-profile CLI execution and seven-profile all-target checks are pending.

## Out of scope

Character, realm, and account selection are separate from class and specialization state.
