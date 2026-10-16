# Wrath unit-popup and frame-binding state

Scoped original Wrath consumers exercise RAF links, late XML arguments, and
missing key labels. This does not establish clean FrameXML startup or a native
initial loot method. See [XML templates](../xml-template-system.md) and
[Lua API](../lua-api.md).

## What it must do

- [x] The original PARTY menu hides both RAF actions without a represented RAF link,
  and follows explicit link insertion/removal. The empty link snapshot and GUID
  storage are simulator input policies; summon/level-grant eligibility is separate.
- [x] Original MainMenuBar and VehicleMenuBar OnLoad declarations evaluate their
  single global argument at invocation, after child creation. Rebinding that
  global affects the next call, not an old captured object.
- [x] The original VehicleMenuBar handler continues into its real OnLoad, registering
  its events and resizing all six action-button textures.
- [x] Original CharacterMicroButton OnLoad and UPDATE_BINDINGS show `(C)` using the
  original GetBindingText fallback. Missing C_KEY_TEXT remains nil on repeated reads;
  registered C_CVar remains callable. Unknown C_ prefixes alone cannot create APIs.

## How it works

- [XML template system](../xml-template-system.md)
- [Frame data flow](../frame-data-flow.md)

## Implementation inventory

- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs`: passive Wrath RAF input set.
- `src/lua_api/globals/real/recruit_a_friend.rs`: modeled legacy predicate; strict
  string validation is simulator policy, not an established native error contract.
- `src/lua_api/globals/real/mod.rs`, `src/lua_api/globals/register.rs`: Wrath registration.
- `src/lua_api/globals/create_frame/template_chain/builder_function_family/global_args.rs`:
  invocation-time single-global-argument handler.
- `src/lua_api/env_init/runtime_surface_bootstrap.lua`: removes prefix-only synthesis;
  shares the existing internal factory with explicit later namespace initializers.
  No new compatibility path or namespace inventory is invented.

## Tests asserting this spec

`tests/wrath_unitpopup_state.rs` loads SHA256-pinned original cached Lua files and
parsed original XML OnLoad declarations. Passive geometry scaffolds retain the
installation-before-child-creation boundary. This is not complete original XML
initialization. On 2026-10-16 all five tests failed behaviorally before producers
and passed afterward. The corrected binding-event test resets its tooltip first;
a no-op dispatch mutation fails while the original event dispatch refreshes it.
Passive vehicle texture geometry matches the pinned inherited original template.

## Known gaps (current cycle)

- [x] Wrath-profile RED then GREEN: five passing regressions plus four panel and
  eight cursor preservation tests. Combined startup observation: 44 unique /
  55 occurrences becomes 37 / 51; startup still fails.
- [ ] Establish the exact initial Wrath loot token from public/native evidence.
- [ ] Original PartyFrame.xml sequential-sibling creation and RaidFrame recovery
  remain blocked acceptance, not credited by the direct RAF menu tests.

## Out of scope

Fresh loot defaults, full startup, other argument shapes, RAF server updates,
summoning/granting levels, native malformed-argument/security parity, and vendor
changes. Removing generic namespace fabrication is shared simulator behavior;
this cycle proves only the original Wrath key-label consumer and a real C control.
