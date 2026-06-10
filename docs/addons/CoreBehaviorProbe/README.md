# CoreBehaviorProbe

Captures real-client behavior for core Frame/UIObject APIs where simulators may diverge:

- `SetForbidden` / `IsForbidden`
- `CreateForbiddenFrame` and `EnumerateFrames`
- `RegisterUnitEvent` / `IsEventRegistered`
- wildcard `GetAttribute(prefix, name, suffix)` with false values
- `Raise` / `GetRaisedFrameLevel`

Install under the live retail AddOns directory, enable it, log in or `/reload`, then `/reload` or logout once more so WoW flushes `CoreBehaviorProbeDB`.
