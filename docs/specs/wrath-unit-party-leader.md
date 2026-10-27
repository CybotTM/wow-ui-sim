# Wrath unit party leader

`UnitIsPartyLeader(unit)` reads the simulator party leader and resolves target and focus by GUID. It returns 1 for the leader and nil otherwise; non-string input raises a usage error. A solo player is not a leader in the simulator's default scenario.

Wrath `FrameXML.toc` loads `TargetFrame.xml`, which loads `TargetFrame.lua`; line 117 uses the result for the target leader icon. Exact Wrath Classic 3.4.x semantics are unknown. The return and input conventions use historical 3.3.5a comparison evidence, not proof of Classic parity; the solo value is a scenario default.
