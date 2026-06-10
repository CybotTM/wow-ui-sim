# AttributeDispatchProbe

Retail-client probe for `SetAttribute` dispatch semantics.

It records:

- whether `OnAttributeChanged` fires twice for two identical scalar writes;
- whether explicit `false` values also refire on repeated writes;
- whether sequential `ShowUIPanel()` calls using Blizzard's repeated `panel-show=true` pulse keep the second panel in the `CloseAllWindows()` stack.

Install under the live retail AddOns directory, enable it, log in or `/reload`, then `/reload` or logout once more so WoW flushes `AttributeDispatchProbeDB`.
