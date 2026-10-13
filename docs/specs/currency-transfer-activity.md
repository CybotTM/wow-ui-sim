# Currency transfer activity query

`C_CurrencyInfo.IsCurrencyTransferInProgress()` returns explicit environment-local transfer activity. Cached API metadata specifies one boolean; the cached transfer menu consumes it separately from request/readiness/transaction APIs. [C API architecture](../lua-api.md) describes ownership.

## What it must do

- [ ] Return exactly one non-nil boolean, modeled false when unsupplied.
- [ ] Project supplied false/true/false unchanged without modifying input or inferring activity from currency metadata/readiness.
- [ ] Read live state through retained function references; isolate existing and new environments.
- [ ] Drive unchanged cached immediate spinner/content methods with real frame state and the actual query, without mocking request/lifecycle APIs.

## How it works

- [Lua API and simulator state](../lua-api.md).
- [Client profiles](../wiki/systems/client-profiles.md).

## Implementation inventory

- `src/lua_api/state/sim_state.rs`: explicit activity input.
- `src/lua_api/state.rs`: false default.
- `src/c_api/item_spell/c_currency.rs`: existing namespace owner; query registration pending.

## Tests asserting this spec

- `tests/currency_transfer_activity.rs`: three default/live/isolation and bounded cached-consumer cases.

## Known gaps (current cycle)

- [ ] Input/tests staged, getter unchanged; genuine behavioral RED unrun.
- [ ] Runtime/profile/startup and independent verification pending.

## Out of scope

Transfer requests, account roster, balances, costs, transaction log, readiness, timers, completion/failure/events and persistence require separate contracts. A supplied boolean does not implement transfer lifecycle or prove native semantics; consumer proof is immediate spinner methods, not full addon or timer behavior.
