# Dropdown Intrinsic Script Chain

Reputation filter dropdowns failed to open because intrinsic `DropdownButton` XML scripts were replaced by style-template scripts. The fix keeps Blizzard's intrinsic handler in the runtime script chain and removes the simulator's fake descriptor/materialized-row fallback path.

## Content

### Symptoms

`ReputationFrame.filterDropdown` had correct menu data when the generator was intercepted: radios for All, Warband, the character name, a divider, and the legacy reputations checkbox. The visible UI still did not behave like the real menu path because the dropdown click did not open a Blizzard `Menu.GetManager()` menu.

### Root Cause

`DropdownButton.xml` registers `DropdownButton` as an intrinsic template with `OnMouseDown method="OnMouseDown_Intrinsic"`. `WowStyle1DropdownTemplate` then contributes its own `OnMouseDown method="OnMouseDown"`. The runtime template chain applied the intrinsic template first, then treated the derived style script as a normal replacement, so the intrinsic click handler was lost.

The issue was in the simulator XML/template script application, not in the ReputationFrame generator or popup row rendering.

### Fix

When a template chain has already applied an intrinsic base, later default scripts are chained with existing handlers instead of replacing them. This preserves Blizzard's intrinsic event path while still allowing the derived style handler to run first.

The fake menu path was retired:

- `MENU_DESCRIPTOR_FALLBACK_LUA` and `ensure_menu_descriptor_fallback` were removed.
- The post-load `Blizzard_Menu` fallback install hook was removed.
- Runtime bootstrap dropdown materialization and style mouse-down patches were removed.
- `tests/menu_fallback.rs` was deleted.

### Coverage

- `intrinsic_dropdown_scripts_chain_with_style_template_scripts` asserts a minimal intrinsic `DropdownButton` plus style template runs both handlers in order.
- `reputation_filter_dropdown_opens_with_blizzard_menu_renderer` loads Blizzard UI, runs `ReputationFrame`'s real `OnShow`, clicks the real dropdown script, and asserts `Menu.GetManager()` tracks the opened menu.

## Sources

- [DropdownButton.xml](../../../Interface/BlizzardUI/Blizzard_Menu/DropdownButton.xml) — intrinsic template scripts
- [MenuTemplates.xml](../../../Interface/BlizzardUI/Blizzard_Menu/Mainline/MenuTemplates.xml) — style dropdown scripts
- [ReputationFrame.lua](../../../Interface/BlizzardUI/Blizzard_UIPanels_Game/Mainline/ReputationFrame.lua) — menu generator
- [template_chain.rs](../../../src/lua_api/globals/create_frame/template_chain.rs) — runtime template script application
- [helpers.rs](../../../src/loader/helpers.rs) — slow-path XML script chaining
- [startup_api_stubs.rs](../../../tests/startup_api_stubs.rs) — Reputation dropdown regression test
- [registry.rs](../../../tests/xml_templates/registry.rs) — intrinsic/style chaining regression test

## See Also

- [[xml-template-system]] — template registration and inheritance chain behavior
- [[frame-data-flow]] — Lua/Rust frame state and script dispatch
