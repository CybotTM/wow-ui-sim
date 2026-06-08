# ServerSnapshot

ServerSnapshot is a small World of Warcraft addon that records server-backed character state into `SavedVariables`. The main target is action bar slot contents, because those are stored by Blizzard servers and are not present in local WTF files until the client asks the server.

## Install

Copy this folder to the matching WoW client AddOns directory:

```text
World of Warcraft/_retail_/Interface/AddOns/ServerSnapshot/
```

The folder should contain:

```text
ServerSnapshot/ServerSnapshot.toc
ServerSnapshot/ServerSnapshot.lua
ServerSnapshot/README.md
```

## Use

Log into the character whose data you want to capture. The addon snapshots automatically on login and after action bar, spell, talent, macro, and specialization changes.

Slash commands:

```text
/serversnapshot
/ssnap
```

Both commands take a fresh snapshot and print the character key plus action slot count.

## Saved Data

WoW writes the database at logout or `/reload`:

```text
World of Warcraft/_retail_/WTF/Account/<ACCOUNT>/SavedVariables/ServerSnapshot.lua
```

The global table is:

```lua
ServerSnapshotDB
```

Snapshots are stored by character key:

```lua
ServerSnapshotDB.characters["Realm/Character"]
```

Each snapshot includes metadata, action bar slot contents, spellbook data when available, macros when available, and talent/loadout details where Blizzard exposes a public API.

## wow-ui-sim Import

When wow-ui-sim starts with SavedVariables enabled, it looks for:

```text
WTF/Account/<ACCOUNT>/SavedVariables/ServerSnapshot.lua
```

If present, the simulator loads `ServerSnapshotDB`, picks `lastCharacterKey` when available, falls back to the newest captured character snapshot, clears the simulator action bars, and seeds spell action slots from the captured action bar data before Blizzard addons load.

Only spell slots are imported today. Empty slots and non-spell entries such as macros are ignored.

## Notes

- Empty action slots are recorded as `{ empty = true }`.
- Missing APIs are skipped instead of breaking the addon.
- `## Interface` may need to be updated for the exact WoW client build. In game, run `/run print(select(4, GetBuildInfo()))`.
