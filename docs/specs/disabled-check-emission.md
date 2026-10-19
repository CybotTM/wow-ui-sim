# Disabled checked texture emission

Draft contract for CheckButton texture selection exercised through original Blizzard LFG consumers. State selection lives in `src/lua_api/frame/methods/button_anchor_hierarchy/`; rendered requests come from `src/iced_app/`. Architectural context: [rendering pipeline](../rendering-pipeline.md). All requirements remain unverified: this handoff stages tests without executing them.

## What it must do

- [ ] A checked, enabled original LFG checkbox emits its checked texture, not its distinct disabled-checked texture.
- [ ] Disabling a checked checkbox selects only its disabled-checked texture; enabling it restores only the normal checked texture.
- [ ] Original LFG reassignment from ordinary to mixed selection while already disabled replaces the emitted request with the mixed disabled-check path; old check paths disappear.
- [ ] A disabled unchecked checkbox emits neither check slot; a hidden ancestor suppresses both slots even while checked.
- [ ] Reassignment and enable transitions preserve both texture child objects and their checkbox parent.

## How it works

- [Widget system](../widget-system.md).
- [Rendering pipeline](../rendering-pipeline.md).

## Implementation inventory

- `src/lua_api/frame/methods/button_anchor_hierarchy/textures.rs` — slot assignment and enabled/checked visibility predicate.
- `src/lua_api/frame/methods/button_anchor_hierarchy/buttons.rs` — enable transition synchronization.
- `src/iced_app/button_vis.rs` — renderer visibility/ancestor filtering.
- `src/iced_app/strata_emit.rs` — emitted texture request boundary.

## Tests asserting this spec

- Proposed `tests/c12_disabled_check_consumer.rs` — unchanged Retail LFGDungeonListButton_SetDungeon and Wrath LFDQueueFrameSpecificListButton_SetDungeon, distinct emitted requests, reassignment, ownership and hidden ancestor.
- Existing `tests/methods_button_textures.rs` — setter/storage controls; not emitted-output acceptance.
- Existing `tests/checkbutton_render_order.rs` — ordinary check ordering control; not disabled-check acceptance.

## Known gaps (current cycle)

- [ ] Compile and execute staged test separately under Retail and Wrath; expected GREEN from source inspection, not a measured pass.
- [ ] Treat missing cache, consumer load errors, missing fixture dependencies or compilation errors as setup failures, never behavioral RED.
- [ ] PTR/Mists/Forever original-consumer execution remains unproved; Mists listing has a different caller and needs its own fixture.

## Out of scope

- Nil-slot fallback semantics: unspecified; do not introduce compatibility behavior.
- Font precedence, AddonList desaturation and pixel/GPU screenshots: different contracts.
- Native-client parity or complete LFG startup: this is a bounded consumer/renderer integration proof using supplied widget/data fixtures.
