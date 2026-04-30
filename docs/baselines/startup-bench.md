# Startup-time benchmarks (Phase 9.1)

Wall-clock from `wow-sim lua-errors` invocation to JSON emit. This covers
the full startup path:

1. rilua VM init
2. Blizzard UI load (vendor `Interface/BlizzardUI/<Profile>/`)
3. Third-party addon load (none in the bundled `Interface/AddOns/`)
4. Profile compat bootstraps (`src/{wrath,mists,era}/compat_bootstrap.lua`)
5. Startup events dispatched
6. Lua-error capture serialized to JSON

Captured via `scripts/bench-startup.sh` (5 runs per profile; min / median /
max wall-clock reported).

## Latest run

See `docs/baselines/startup-bench.tsv` for the most-recent numbers committed
alongside this doc. Initial baselines on the dev box, debug build, with
SavedVariables loading suppressed (`WOW_SIM_NO_SAVED_VARS=1`):

| Profile | Build | Min   | Median | Max   |
|---------|-------|------:|-------:|------:|
| retail | dev | ~15s | ~17s | ~31s |
| wrath | dev | ~10s | ~11-15s | ~16s |
| mists | dev | ~13s | ~17-19s | ~21s |
| era | dev | ~14s | ~15-17s | ~21s |
| anniversary | dev | ~13s | ~14-21s | ~22s |

Variance is high (~30%) run-to-run — concurrent system load, page-cache
warmth, and CPU thermal state all visibly move the numbers. Treat this
table as a regression *floor* rather than precise budget; track the
TSV file in git for diffable history.

## Expected vs current

Plan numbers (`PLAN.classic.md`) targeted retail ~5s, wrath ~12s, mists
~12s. The dev-box reality is roughly 3× retail's planned target — likely
because (a) measurement runs include rilua VM init + compile + JSON emit
that the plan estimates excluded, and (b) the dev box is a desktop under
moderate concurrent load. The relative ordering is consistent:
retail > mists ≈ era ≈ anniversary > wrath. Anniversary is occasionally
the slowest because its vendor source is the multi-flavor Gethe repo
(2.5.5) carrying every expansion's TOC variants, even if only `_Vanilla`
is used.

## How to re-run

```bash
# All profiles, default 5 runs
./scripts/bench-startup.sh

# Subset
./scripts/bench-startup.sh wrath mists

# More runs for tighter median
./scripts/bench-startup.sh --runs 10

# Release mode (much faster, but not the canonical mode)
./scripts/bench-startup.sh --release

# Machine-readable TSV for diffing
./scripts/bench-startup.sh --tsv > /tmp/now.tsv
diff docs/baselines/startup-bench.tsv /tmp/now.tsv
```

## Things this benchmark intentionally does NOT measure

- **First-time vendor clone time** (`scripts/setup-blizzard-ui.sh`). The
  one-time setup cost is out of scope.
- **cargo build time**. Build is excluded; the script builds once before
  the loop and times only the binary execution.
- **Per-addon timings**. The simulator emits `(io=X xml=X lua=X sv=X)`
  per addon at boot; aggregate those manually if a regression points to
  a specific addon.
- **3D / GPU rendering**. lua-errors mode runs headless; full GUI startup
  with frame rendering is a different number. Add a `--gui-startup` mode
  if/when that becomes a regression target.

## Tracking regressions

When a PR changes startup-relevant code (loader, lua_api/env_init,
compat bootstraps, frame methods), re-run the bench and diff the TSV:

```bash
./scripts/bench-startup.sh --tsv > /tmp/proposed.tsv
diff docs/baselines/startup-bench.tsv /tmp/proposed.tsv
```

If a profile's median regresses by more than the ~30% noise floor, treat
it as a real regression and investigate before merging.
