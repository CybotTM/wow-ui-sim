# ServerSnapshot Action Bars

ServerSnapshot action-bar import lets wow-ui-sim seed its action bar model from real WoW SavedVariables captured by the bundled `ServerSnapshot` addon. This replaces empty or synthetic simulator action slots with spell slots returned by the live client/server before Blizzard action-bar UI code loads.

## Content

`docs/addons/ServerSnapshot/` contains the capture addon. In live WoW, it writes account SavedVariables to:

```text
WTF/Account/<ACCOUNT>/SavedVariables/ServerSnapshot.lua
```

The simulator startup path uses the normal `SavedVariablesManager` WTF import source. After SavedVariables are configured and EditMode cache is loaded, but before Blizzard addons load, `server_snapshot_import::load_from_saved_variables` loads the `ServerSnapshot` SavedVariables file and applies the captured action bars.

The importer chooses `ServerSnapshotDB.lastCharacterKey` when that character snapshot exists. If not, it falls back to the newest `capturedAt` snapshot. It clears the simulator action bars and imports only entries with `type = "spell"`, using `spellID` or `id` as the spell id. Empty slots and non-spell actions are ignored.

The result populates `SimState.action_bars`, so existing APIs such as `HasAction`, `GetActionInfo`, and action button setup see the same spell slots as any other simulator-seeded action bar.

## Sources

- [ServerSnapshot addon README](../../addons/ServerSnapshot/README.md) — capture addon and import behavior
- [server_snapshot_import.rs](../../../src/server_snapshot_import.rs) — startup import logic
- [main.rs](../../../src/bin/wow_sim/main.rs) — startup hook placement
- [server_snapshot_import.rs tests](../../../tests/server_snapshot_import.rs) — coverage for in-memory and WTF-file imports

## See Also

- [[addon-loading]] — SavedVariables loading and startup sequence
- [[lua-api]] — `WowLuaEnv` and action-bar API surface
