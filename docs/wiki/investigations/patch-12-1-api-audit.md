# Patch 12.1 API Audit

Patch 12.1 API surface work in wow-ui-sim is split between compatible bridges that can be safely modeled as inert/additive simulator behavior and paused items that require exact Blizzard PTR observations before implementation.

## Content

### Completed compatible bridge work

The 12.1 compatibility work is currently captured by these commits:

- `856bd7c6d` — bridged the first 12.1 PTR API surface pass.
- `088be550c` — fixed 12.1 bridge edge cases.
- `390086d31` — hid removed 12.1 PTR symbols.
- `bda32ebd4` — gated 12.1 API surface behind cumulative retail epoch features.
- `ed34635c5` — moved strict 12.1 removals after startup so Blizzard UI can still load current EditMode code.
- `16b7d85d6` — modeled 12.1 forbidden aspect inheritance for compatible frame/object behavior.
- `4b5fc502d` — bridged remaining inert social, Discord, Battle.net title-friend, encounter-journal, and housing/blueprint probes.

Key implementation locations:

- `Cargo.toml`, `src/client_profile.rs` — `retail-12-1-0` epoch feature and interface-version mapping.
- `src/ptr/compat_bootstrap.rs`, `src/ptr/strict_removals.lua` — 12.1 compatibility bootstrap and post-startup strict removals.
- `src/lua_api/workarounds/temporary/patch_12_1_inert_defaults.rs` — temporary inert defaults for additive 12.1 APIs without a backing simulator model.
- `src/loader/tests/wow_api_globals/startup_globals.rs` — safe bridge and strict removal regression coverage.
- `src/loader/tests/wow_api_globals/frames_and_attributes.rs`, `src/loader/tests/xml_basics_extra.rs`, `src/loader/tests/runtime_templates.rs`, `src/loader/tests/runtime_template_misc.rs` — 12.1 widget/XML/private-partition behavior coverage.

Verified proof logs for the final compatible bridge pass:

- `/tmp/wow_12_1_inert_full_fmt-check.out`
- `/tmp/wow_12_1_inert_full_check-default.out`
- `/tmp/wow_12_1_inert_full_check-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_check-ptr.out`
- `/tmp/wow_12_1_inert_full_test-safe-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_test-safe-ptr.out`
- `/tmp/wow_12_1_inert_full_test-strict-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_build-retail-12-1.out`
- `/tmp/wow_12_1_inert_full_lua-retail-12-1.out`

Rust readability metrics for the final bridge are under `/tmp/rust_readability_12_1_inert` with no high-complexity findings.

### Paused / blocked items

Do not implement these as guesses. They need real Blizzard PTR probes, generated Blizzard docs, or exact behavior captures before simulator changes:

- **UnitAura secrecy and errors** — PTR notes say `C_UnitAura`/`C_TooltipInfo` aura access by index/slot/instance ID Lua-errors for addons while auras are secret, spell-ID/name access remains callable, `UNIT_AURA` payloads become fully secret, and `AuraData` structs are fully secret. The simulator must not approximate this without knowing exact taint/addon vs Blizzard call-site behavior and error shapes.
- **Private Script Objects / Forbidden Partition** — compatible XML/private-table mechanics are modeled, but the full object partition contract is not proven. Need live behavior for public/forbidden table identity, inaccessible key paths, child object visibility, hooks, script storage, and delegate edge cases.
- **Forbidden Aspects enforcement** — inheritance and query/add APIs are modeled, but exact restrictions for `UntrustedScriptExecution`, `UntrustedLayoutScriptExecution`, `EventRegistrations`, `AlwaysPropagateInput`, `ScriptedInput`, and `QueryFocus` need probes before blocking frame methods, focus/input queries, event registration, hooks, or script execution.
- **AuraContainer / AuraButton / ManagedAuraContainer** — object names and compatible creation/XML paths are bridged, but full aura assignment, filtering, sorting, forbidden partition placement, automatic button management, tooltip behavior, and secret `IsShown` behavior are not modeled.
- **DurationTextBinding and RadialProgress script objects** — texture/statusbar radial bar methods are bridged, but the standalone script-object APIs (`RadialProgress:*`, `DurationTextBinding:*`) need exact object lifetime and return semantics.
- **Changed structure payloads with real data** — inert compatibility fields were added where safe, but exact payloads for Battle.net, Discord, housing, cooldown viewer, pet journal, LFG, player choice, tiered entrance, and private aura structures require backing models or live captures before claiming behavioral fidelity.
- **Deprecated wrappers vs strict removals timing** — strict removed symbols are hidden for addon-facing 12.1 checks after startup. Current Blizzard UI still reads some removed/changed values during load, so moving removals earlier can break startup. Revisit only with current PTR Blizzard UI that no longer needs those load-time values.

### Practical next step

If this work resumes, create probe addons first. Target the uncertain areas above with live PTR captures, then update this page and implement only behavior that has concrete evidence.

## Sources

- `/tmp/warcraft_patch_12_1_api_changes.txt` — source patch-note/API-change list used for the audit.
- `src/loader/tests/wow_api_globals/startup_globals.rs` — regression coverage for safe bridges and strict removals.
- `src/lua_api/workarounds/temporary/patch_12_1_inert_defaults.rs` — inert additive 12.1 bridge surface.
- `src/ptr/strict_removals.lua` — post-startup hiding of removed 12.1 symbols.
- `src/lua_api/frame/methods/forbidden_aspects.rs` — compatible forbidden-aspect query/inheritance implementation.
- `docs/wiki/systems/client-profiles.md` — retail epoch feature model.

## See Also

- [[client-profiles]] — retail epoch features used to gate 12.1 API surface.
- [[xml-template-system]] — private/forbidden XML partition and mixin behavior.
- [[lua-api]] — Lua runtime surface and C API bridge context.
- [[taint-system]] — secure/taint behavior that overlaps with forbidden aspects and aura secrecy.
