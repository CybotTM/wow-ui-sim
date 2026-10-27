# Wrath unit tap queries

`UnitIsTapped`, `UnitIsTappedByPlayer` and `UnitIsTappedByAllThreatList` read separate supplied tap flags keyed by unit GUID. An existing unit without supplied flags is untapped in the simulator's default scenario. Each returns 1 for a set flag and nil otherwise; non-string input raises a usage error.

Wrath `FrameXML.toc` loads `TargetFrame.xml`, which loads `TargetFrame.lua`; line 262 uses these queries for the target portrait and name tint. `FocusFrame.lua` uses the same queries. Exact Wrath Classic 3.4.x semantics are unknown. Return and input conventions use historical 3.3.5a comparison evidence, not proof of Classic parity; untapped defaults are simulator scenario inputs.
