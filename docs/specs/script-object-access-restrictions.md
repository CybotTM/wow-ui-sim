# Script-object access restrictions

Retail-family 12.1+ and Forever script objects store conditional access-restriction flags separately from their forbidden flag, published through the shared `forbidden-aspects` capability. The contract comes from cached `Blizzard_APIDocumentationGenerated/SimpleFrameScriptObjectAPIDocumentation.lua`; `Blizzard_AuraContainerUtil.lua` applies the flags while creating aura buttons.

## What it must do

- [ ] `AddAccessRestrictions(mask)` accumulates the supplied mask with bitwise OR; zero and repeated additions do not remove existing flags.
- [ ] `GetAccessRestrictions()` returns the object's stored mask, initially zero.
- [ ] `HasAnyAccessRestrictions()` and an explicit nil argument test for any stored restriction; a supplied mask tests for any intersecting bit, with zero matching none.
- [ ] `HasAccessConstraints()` reports true for either the existing forbidden flag or a nonzero conditional-restriction mask.
- [ ] Restrictions remain per-object and do not change the forbidden flag or another object's mask.
- [ ] Native `AuraContainerUtil.ApplyAccessRestrictions` defers pre-login requests until `PLAYER_ENTERING_WORLD`, not `PLAYER_LOGIN`; calls after login apply immediately. Sharing mask methods must preserve both native branches without changing Blizzard Lua.

## How it works

- [Frame data flow](../frame-data-flow.md)
- [Protected frame enforcement](../protected-frame-enforcement.md)

## Implementation inventory

- `src/widget/frame.rs`, `frame_defaults.rs` — zero-default `access_restrictions` mask.
- `src/lua_api/frame/methods/text_attribute_event/access_restrictions.rs` — mask mutation and queries.
- `src/lua_api/frame/methods/text_attribute_event/mod.rs` — registration under `forbidden-aspects`; `ClearScripts` remains Retail-12.1-only and `GetObjectTable` has one shared registration.
- `src/lua_api/frame/methods/text_attribute_event/attributes.rs` — combined `HasAccessConstraints` query.

## Tests asserting this spec

- `tests/forbidden_frames.rs` — shared-capability mask accumulation/filtering, object isolation, combination with forbidden state, and actual cached Blizzard AuraContainer pre-login/world-entry/post-login application.
- `tests/patch_12_1_0_aura_access_restrictions.rs` — Retail aura-secret method access, initialization before restriction, and pre-login accessibility through `PLAYER_LOGIN`.

### Bounded Retail execution — 2026-10-16

Six distinct existing cases PASS: mask/cached-application 3/3 and live aura-secret consumer 3/3, each exit0. Test checkout `4b0ebcde3` links separately sealed normal Retail library `a2d2d2792`; component compilation exits0 with 29 inherited common-fixture warnings. Receipt: `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/access-restrictions-retail-existing-20261016T122607582483Z`; independent full-stream/source/library/seal acceptance: `/home/osso/.local/state/wow-ui-sim/verification/access-restrictions-next/retail-report.md`.

This executes cached Blizzard Lua in the simulator, not native-client conformance. It is bounded Retail proof, not whole-integration, Forever or universal shared-profile acceptance. Requirements above remain unchecked pending remaining profile proof.

## Known gaps (current cycle)

- [ ] Forever and remaining shared-profile behavior proof remain open; bounded Retail execution does not close them. Historical Forever GUI RED: `/tmp/ellesmere-forever/secure-chain-gui.stderr`, cached `Blizzard_AuraContainerUtil.lua:421` reports missing `AddAccessRestrictions` after login. Retail cases do not establish resolution of that Forever GUI failure. No new enforcement or mask semantics are introduced.
- The separate [Forever context-access query](script-object-context-access.md) uses an explicit per-environment aura-secret scenario to evaluate this mask on that query only. It does not derive live aura secrecy or enforce restrictions on other frame methods. No claim of complete security enforcement is made.

## Out of scope

Inventing aura-secrecy policy, context-dependent access checks, new restrictions, or changing addon/vendor Lua. Only `DenyTaintedAccessWhenAurasAreSecret = 1` currently has a named enum value; generic mask algebra does not establish live-client validation rules for unnamed bits.
