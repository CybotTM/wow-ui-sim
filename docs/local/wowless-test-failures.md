# Wowless Self-Test Status

Results from `wow-sim --no-saved-vars self-test --max-ticks 20000`.

**Total: 97,282 sync + 4 async = 97,286 tests**
**Passing: 92,319 sync + 1 async = 92,320 (94.9%)**
**Failing: 4,962 sync + 3 async = 4,965 (5.1%)**

## Category Filter

Use `--categories` to run a subset:

```bash
# Run only failing categories (fast iteration)
wow-sim --no-saved-vars self-test --categories "generated.globalApis,generated.globals,generated.impltests,generated.uiobjects,generated.~cfuncs,luaobjects,async"

# Run a single sub-category
wow-sim --no-saved-vars self-test --categories "generated.globalApis"
```

## Category Breakdown

### Fully Passing (skip with --categories)

| Category | Tests | Status |
|---|---|---|
| `sync` | ~1,000 | PASS |
| `xml` | ~100 | PASS |
| `uiobjects` (top-level) | ~500 | PASS |
| `generated.apiNamespaces` | ~5,000 | PASS |
| `generated.cvars` | ~1,000 | PASS |
| `generated.events` | ~2,000 | PASS |

### Failing Categories

#### `generated.globals` (failures)

Missing or incorrect global values/functions. Includes:
- Missing LE_* enum constants (partially fixed)
- Constants values from wowless YAML

#### `generated.impltests` (failures)

Implementation-specific tests for frame methods and behaviors. Various widget method issues.

#### `generated.uiobjects` (bulk of failures)

Frame type method tests. Major failure patterns:

- **StatusBar methods**: `want "function", got "nil"` — many StatusBar-specific methods not on our StatusBar type (SetMinMaxValues, SetValue, SetOrientation, SetStatusBarColor, etc.)
- **TabardModel/UnitPositionFrame**: Wrong GetObjectType results — our CreateFrame maps these to simpler types
- **Texture**: `want false, got true` — Texture creation test expects something different
- **Animation types** (TextureCoordTranslation, Translation, VertexColor): `attempt to index field 'AnimationGroup' (a nil value)` — animation system gaps
- **Script handlers**: OnChar, OnGamePadButtonDown/Up, OnHyperlinkClick/Enter/Leave report `want true, got false` — HasScript check failures

#### `generated.~cfuncs` (failures)

C function uniqueness checker. Tests that each C function has exactly one "true name". Our Lua-implemented stubs (closures) don't satisfy this — `debug.getinfo` can't find a canonical name for them.

~100 functions fail, mostly C_* namespace stubs that are Lua closures rather than native C functions.

#### `luaobjects` (failures)

- **LuaDurationObject**: `GetClockTime` method missing, `methodsunique` check fails
- **UnitHealPredictionCalculator**: 17 methods missing (GetCurrentHealth, GetMaximumHealth, etc.), `methodsunique` check fails

These are specialized Lua object types we haven't implemented yet.

#### `async` (3 of 4 failing)

| Test | Status | Issue |
|---|---|---|
| RequestTimePlayed | PASS | |
| event registration and dispatch order | FAIL | Our dispatch is sequential (t1..t32), WoW interleaves by registration order within event buckets |
| individual event reg before all | FAIL | RegisterAllEvents dispatch order: want a1,a2 got a2,a1 |
| C_Timer.NewTimer | FAIL | Timer callback receives wrong LuaFunctionContainer pointer — tostring mismatch |

## Previously Fixed

- SetAllPoints implicit parent, anchor cycles, GetNumPoints, $parent substitution
- CreateFrame with frame in name position, taint error messages
- Frame level recalculation on SetParent
- SetAllPoints implicitscreen, anchor cycle error messages
- SetColorTexture/GetTexture round-trip, Slider SetThumbTexture fileID
- Font object (CreateFont registry, numeric name, cycle detection)
- StatusBar GetStatusBarTexture returns nil, WorldFrame quirks
- Animation target validation, Texture SetTexture numeric ID
- Region rect (GetLeft/Right/Top/Bottom/Center, IsRectValid)
- Font vfs (GetFont returns nil on fresh CreateFont)
- OnShow/OnHide mutual recursion (12-invocation limit)
- RegisterEventCallback stub, OnShow/OnHide children-first ordering
- Button states, default children, parent keys
- All 72 generated.cvars failures (synced cvars.yaml from wowless)
- string.format.impltype (rewritten patch in Rust)
- apiNamespaces (regenerated stubs, fixed _tpath null handling)
- Constants values (from wowless YAML)
- generated.globalApis (5 impltype failures: newsecurefunction wrapping)

## A_Print Taint Bypass

`print()` is intercepted by Elune's taint system after `debug.settaintmode('rw')`. Use `A_Print(...)` which reads the original print function from Lua registry, bypassing taint.

## Test Runner Notes

- test.lua creates an OnUpdate frame that iterates sync tests with a budget (~half frame time)
- Sync tests run ~1000/tick in release mode, ~1.3s/tick
- Full run takes ~8 minutes in release
- `--categories` filter reduces to ~2 minutes for failing categories only
- Two OnUpdate frames run (test.lua loaded during addon load, registers twice) — cosmetic duplicate output, doesn't affect results
