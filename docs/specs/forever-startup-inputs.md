# Forever 70532 startup inputs

## Scope

Only `client_profile::ACTIVE == WowForever` publishes this additional surface.
Other profiles retain their existing registrations and behavior. Contracts come
from the copied 1.60.1.70532 `Blizzard_APIDocumentationGenerated` cache.

## Supplied state, not native policy

`SimState.forever_startup` contains host inputs for ammo requirement, cursor
visibility, ping enablement, bank-type purchase permission, match completion,
target spell, TTS settings-save state, temporary CVars, external-event URL state,
and group-buff visual alerts. Empty-world flags and collections are simulator
fixtures, not claims about native defaults. Tests must vary inputs through the
original Blizzard consumers. Native derivation and validation policies remain
unknown.

- `C_PaperDollInfo.AmmoNeeded()` returns one non-null boolean from the host's
  character ammo requirement. PaperDoll hides the AmmoSlot only when false.
- Group-buff rows contain non-null numeric `spellID` and `visualValue` fields;
  `visualValue` uses the documented `VisualAlertType` enum. Linked-rank expansion
  requires host-supplied rows; no spell-rank policy is inferred.
- External-event probes return non-null booleans from the supplied optional URL,
  availability and newness. No network launch is simulated without a modeled
  launch result.
- `C_Spell.GetTargetSpellID()` requires a supplied numeric identifier. An absent
  input is explicit native-unknown, not an invented no-target sentinel.

## Events

Accept the documented `SECURE_TRANSFER_CONFIRM_TAKE_MAIL_MONEY`,
`SHARD_TRANSFER_CANCELLED`, `EXTERNAL_EVENT_LAUNCH_URL_RESULT` and
`SOCIAL_UI_SOCIAL_QUEUE_SYSTEM_STATUS_UPDATED` names for Forever only. URL launch
result has one non-null boolean payload; the other three have no payload. This
adds registration support, not native event-generation policy.

## Unknown residue

`C_AccountServices.GetPreferredDatacenterLocalities` is consumed by the selected
GuildPreferredPlaySettings source but has no declaration in this build's generated
API documentation. Its native contract remains unconfirmed. Do not infer a
catalog or native return from that consumer alone.

## Acceptance

Original selected consumers must reject missing/constant producers and project
varying host inputs. Stock startup captures must preserve exact message bytes,
classify every residual error, and accept a baseline only for documented
native-unknown residue. Record profile checks and CPU evidence in the lane report.
