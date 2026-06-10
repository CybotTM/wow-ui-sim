# DevToolsDumpProbe

Small retail-client probe for the output of dumping a frame after writing an array entry:

```lua
local f = CreateFrame("Frame")
tinsert(f, "foo")
DevTools_Dump(f)
```

The addon registers a `DevTools_AddMessageHandler` callback, runs the snippet on `PLAYER_LOGIN`, and writes the captured dump lines plus slot/type metadata to `DevToolsDumpProbeDB`.

Install it under the live client AddOns directory, enable it, log in or `/reload`, then `/reload` or logout once more so WoW flushes SavedVariables.
