# Supplied challenge run and manual clock

The challenge subsystem owns an optional active run, a supplied map catalogue, ordered criteria per step and explicit world elapsed timers. It does not derive dungeon metadata, penalties or weighted display text. The separate manual environment clock supports long forecast fixtures without changing real-time defaults.

## State and dispatch

`c_api::challenge_run` stores Rust snapshots on each SimState. Admin calls validate input before publishing it, then dispatch the declared events. Criteria and affix readers allocate detached Lua results. The existing scenario step model supplies step metadata and row counts. Run reset/completion removes the active run, criteria and elapsed timers before notifications. Map records remain available.

Publication follows pinned declarations: Retail/PTR/Forever have active map, key, deaths and active flag; Mists/Era/Anniversary have map and active flag. Wrath receives no new members or elapsed-backend routing. Its old `(id, 0, 0)` elapsed placeholder is preserved as an ungrounded gap for a separate lane. The inactive key tuple is deliberately unimplemented pending native evidence, with a named diagnostic instead of invented non-nil values. Mists original criteria rendering is still blocked by the existing legacy C_Scenario info/step constants; this subsystem does not replace them.

## Clock boundary

`SimState::clock_elapsed` and `clock_now` route gameplay timestamps and timer deadlines. A manual clock stores both coordinates, so choosing a new GetTime epoch cannot expire pending timers. Host and admin steps share LuaState-level OnUpdate, animation and timer dispatch. This avoids reborrowing the active Lua VM cell. Timer dispatch restores the previous executing owner, including nested callbacks. The manual queue selects the earliest deadline and ID after each callback; repeat deadlines advance from their previous value. Zero-interval tickers run once per advance.

Duration-object manual clocks, explicit world elapsed timers and the delta-driven encounter timeline retain their own state. Real startup/render scheduling, watchdogs and performance diagnostics do not use manual time. Autonomous party damage still samples real SystemTime randomness; only its death/resurrection timestamp follows the environment clock.

## Evidence status

Initial feature RED is observed on the lane's own binary. Phase A Retail timer, map-icon, admin-event and profile-surface cases pass. The criteria fixture needed the original `ScenarioObjectiveTracker:InitModule` before `UpdateCriteria`; its first failure was an uninitialized parentModule, not a missing producer. Final paired original-consumer RED/GREEN, manual-clock execution, profile checks, stock comparison and external forecast acceptance remain pending at this implementation checkpoint.

## Sources

- [Challenge contract](../../specs/challenge-mode-run.md) — pinned declarations, publication and selected consumers.
- [Clock contract](../../specs/manual-simulation-clock.md) — opt-in, validation and callback order.
- [Admin guide](../../admin-api/challenge-mode.md) — fixture inputs and examples.

## See Also

- [[lua-api]] — environment state and public API registration.
- [[consumer-selection-and-red-proof]] — runtime-selected consumer and paired proof requirements.
