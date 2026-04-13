# Wiki Log

Chronological record of wiki operations.

## [2026-04-12] ingest | transparent wrapper render-order investigation

Created `investigations/transparent-wrapper-render-order.md` for the world map / quest log render-order fix. Updated it after a follow-up regression to document the depth-aware transparent-wrapper hoist in `state_render.rs`, including both world-map visibility coverage (`world_map_tiles_render_after_tiled_background`) and world-quest pin ordering coverage.

## [2026-04-09] ingest | systems/ pages created (10 pages)

Created all 10 systems/ pages from source docs in docs/:

- systems/layout-system.md — from layout-system.md + anchor-resolution.md
- systems/rendering-pipeline.md — from rendering-pipeline.md
- systems/widget-system.md — from widget-system.md + button-text-rendering.md
- systems/lua-api.md — from lua-api.md
- systems/event-system.md — from event-system.md
- systems/xml-template-system.md — from xml-template-system.md
- systems/addon-loading.md — from addon-loading-pipeline.md
- systems/texture-atlas.md — from texture-atlas-system.md
- systems/frame-data-flow.md — from frame-data-flow.md
- systems/taint-system.md — from protected-frame-enforcement.md + src/lua_api/globals/security_api.rs + src/lua_api/secure_env.rs + src/lua_api/frame/methods/combat_lockdown.rs

Updated index.md systems/ table.

## [2026-04-10] ingest | Initial bulk ingest from 30+ existing docs

Bootstrapped wiki from root-level documentation files. Created pages across systems/, design/, investigations/, and reference/ categories.

## [2026-04-09] ingest | design/ and reference/ pages created

Created 7 pages from DESIGN.md, SCALING.md, docs/debug-tools.md, FUTURE.md, docs/c-api-signature-audit.md, docs/c-api-stub-audit.md, AGENTS.md, and PLAN.md.

Pages created:
- design/architecture-overview.md
- design/scaling-coordinates.md
- design/debug-tools.md
- reference/api-coverage.md
- reference/cli-commands.md
- reference/addon-compatibility.md
- reference/development-phases.md
