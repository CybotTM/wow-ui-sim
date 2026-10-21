# Native Wrath action callback fixture (F04/F05)

`common::action_bar_fixture::build_wrath_env` reuses `load_profile_framexml` to load the active native FrameXML TOC, then fires production startup, ACTIONBAR_SHOWGRID and UPDATE_BINDINGS. `wrath_env` holds the normal shared-workload permit. F05 can extend this fixture; do not add modern aliases/mixins/parentKeys or replace callback functions.

F04 keeps original test IDs. Wrath assertions live in `common/wrath_action_assertions.rs`; non-Wrath bodies are unchanged. Assertions use original named regions, events, callbacks and secure input dispatch. Native drag runs XML OnDragStart/OnReceiveDrag without manually assigning the icon. Wrath key down pushes only; key up calls SecureActionButton_OnClick and casts.

## Evidence-backed gates (Wrath only)

- `action_button_input_dispatch::try_use_action_button_only_fires_from_key_down`: 12340 `ActionButton.lua:15-43` has no TryUseActionButton; native key-release behavior remains exercised by both original generic input and keybind cases.
- `behavior_attach_detach`: EditMode attach/detach methods absent from native MainMenuBar/ActionButton/MultiActionBars code. Generic fixed native bar layout stays covered.
- `behavior_main_bar_endcaps`: HideBarArt setting absent; native `MainMenuBar.xml:291-314` creates fixed end-cap textures. Native visibility, texture and size stay covered by `action_bar`.
- `behavior_battlefield_hides_xp_bar`: native MainMenuBar/ReputationFrame has XP/reputation stacking/replacement but no Honor status-tracking bar. Native XP/reputation callbacks stay covered.

Partial modern-only assertions remain non-Wrath: extra-action button (generic multibar kept), paragon/renown (standard watched faction kept), taxi early-landing (native vehicle exit kept), modern range event/router (native OnUpdate range consumer kept). These are not claimed as native 12340 features.

Sources: profile cache `wrath/FrameXML/{ActionButton.lua,ActionButtonTemplate.xml,ActionBarFrame.xml,MainMenuBar.xml,MainMenuBar.lua,MultiActionBars.lua,BonusActionBarFrame.lua,PetActionBarFrame.lua,ReputationFrame.lua}`. Laptop 12340 query contracts: `handoff/laptop-labels-answers-20261018/{batch3.md,action-inrange-12340.md,pet-shapeshift-info-12340.md}`; reconstructed static facts, not native execution.
