# Memory + widget-count snapshot per profile (Phase 9.3)

Three numbers per profile:

- **`heap_kb`** — `collectgarbage("count")` after startup events fire.
  This is the rilua-managed Lua heap only (tables, strings, closures).
- **`rss_peak_kb`** — peak resident-set-size of the wow-sim process,
  sampled every 100ms via `/proc/<pid>/status` while it runs. Includes
  the rilua heap plus the Rust-side widget tree, texture atlas, asset
  cache, and library mappings.
- **`widgets_total`** — count of `[<WidgetType>]` entries in the
  `dump-tree` output (Frame, Button, Texture, FontString, EditBox,
  StatusBar, MessageFrame, CheckButton, ScrollFrame, GameTooltip,
  ModelScene, WorldFrame, Slider).

Captured via `scripts/bench-memory.sh`. Latest numbers in
`docs/baselines/memory-bench.tsv`. Sample baseline (debug build,
`WOW_SIM_NO_SAVED_VARS=1`):

| Profile | heap_kb | rss_peak_kb | widgets_total |
|---------|--------:|------------:|--------------:|
| retail | ~32,400 (32 MB) | ~862,000 (842 MB) | 7,743 |
| wrath | ~13,200 (13 MB) | ~525,000 (513 MB) | 14,934 |
| mists | ~23,500 (23 MB) | ~800,000 (781 MB) | 17,221 |
| era | ~18,400 (18 MB) | ~559,000 (546 MB) | 9,519 |
| anniversary | ~19,500 (19 MB) | ~581,000 (567 MB) | 9,679 |

Observations from the initial snapshot:

- **rilua heap and RSS scale independently.** Retail has the largest
  Lua heap (32 MB) but only 7.7K widgets; wrath has the smallest heap
  (13 MB) but 15K widgets. Most retail addon code is method tables and
  string-keyed lookups (heap-heavy); wrath FrameXML is template-heavy
  (widget-heavy).
- **`rss − heap_kb` is dominated by Rust-side state**: widget tree,
  glyph + texture atlas, CASC asset resolver mappings. Lua memory
  is a small share of total process RSS.
- **Mists has the most widgets** (17K) — biggest UI surface across
  these profiles.
- **Era and Anniversary are close**: both serve vanilla content;
  Anniversary's slightly higher RSS reflects its larger source-repo
  build (multi-flavor 2.5.5 vs era's 1.15.8 single-flavor).

## How to re-run

```bash
# All profiles, human-readable
./scripts/bench-memory.sh

# Subset
./scripts/bench-memory.sh wrath mists

# TSV for diff/dashboard
./scripts/bench-memory.sh --tsv > /tmp/now.tsv
diff docs/baselines/memory-bench.tsv /tmp/now.tsv

# Without third-party addons
./scripts/bench-memory.sh --no-addons
```

## Tracking regressions

Treat the committed `memory-bench.tsv` as the regression floor. A PR
that bumps `rss_peak_kb` by more than ~10% on any profile should
explain why before merging — possible causes include a new always-on
data table, a leaked atlas slot, or an addon that pulls in extra
modules.

## What's deliberately NOT measured

- **Pre-startup memory.** The `heap_kb` reading happens once after
  startup events; we don't track ramp-up. If addon-load peak > end-of-
  startup matters, instrument the loader directly.
- **Per-addon attribution.** The Rust loader emits per-addon timings
  but not per-addon memory. A PR that wants to know "did adding addon
  X cost N MB" should run with vs without the addon and diff the TSV.
- **Asset resolver / CASC cache footprint** — counted as part of RSS
  but not broken out. Substantial when CASC is enabled (default).
