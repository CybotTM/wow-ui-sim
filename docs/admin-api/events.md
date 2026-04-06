# Events

### A_Admin.FireEvent(event, ...)

Fires a WoW game event directly to all registered listeners.

- **event** `string` -- Event name (e.g., `"ZONE_CHANGED_NEW_AREA"`)
- **...** -- Optional event arguments passed to `OnEvent` handlers
- **Example:**
```lua
-- Fire a simple event
A_Admin.FireEvent("ZONE_CHANGED_NEW_AREA")

-- Fire an event with arguments
A_Admin.FireEvent("ADDON_LOADED", "MyAddon")
A_Admin.FireEvent("CHAT_MSG_SAY", "Hello world", "Arthas", "", "", "Arthas")
A_Admin.FireEvent("UNIT_HEALTH", "player")
```

**Note:** `A_Admin.FireEvent` is a namespaced alias for the internal `FireEvent` global, making it clear in test scripts that this is a simulator-only call with no real WoW equivalent.
