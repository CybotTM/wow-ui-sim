# Wrath S49 companion projection

## Contract

- Wrath `GetNumCompanions` counts owned companions intersected with explicit host eligibility, independently of modern journal usability.
- Mount info uses that same selected list, sorted case-insensitively by supplied resolved creature names. Missing-name ordering is unspecified.
- Host supplies spell-keyed creature metadata in `wrath_legacy.companion_metadata`. Default metadata is empty. Journal mount IDs and icon fileDataIDs must never stand in for creature IDs or texture paths.
- Present metadata returns exactly five values: numeric creature ID, optional cached name, spell ID, optional texture path, active numeric one or nil. Absent metadata or out-of-range position returns zero values; absence never fabricates a creature ID or removes a known eligible spell from the count.
- Registration uses `client_profile::ACTIVE == Wrath`. Other profiles keep their existing behavior.
- Explicit companion events refresh original cached PetPaperDollFrame buttons inline. Producer never calls the UI consumer directly or mutates button state.

## Evidence and supplied inputs

A5 is the supplied laptop request-5 answer under `handoff/laptop-labels-answers-20261018/`. Native-facing signatures and resolved-name sorting are reconstructed/static evidence, not authenticated native execution.

The S49 test supplies creature IDs 91001/91002 as labelled host fixture data. They are **not** claims about native creatures associated with default catalog spells 458/23338. This supplied-state path is independently testable without authentic default rows. Native default spell-to-creature data remains unresolved (laptop request 11); missing metadata blocks populated **default-catalog** consumers, not explicit host-input tests. The original master oracle remains byte-identical and does not supply metadata.

## Scope and proof limits

This repair covers mount-list projection and explicit-event consumption only. Existing critter counts are preserved; populated critter spell/creature joins, native learn/cache actions, pending-cache gating, event emission/order, complete numeric coercion, paging and native default metadata are not implemented or claimed.

The guarded shared compatibility definition is unchanged for non-Wrath profiles; Wrath installs its real Rust binding before that bootstrap, so it cannot select the single-nil fallback.

Acceptance requires the S49 consumer assertions, the unchanged original oracle, zero test-target warnings, Wrath check and Retail preservation. Independent pre-existing UnitHasRelicSlot errors must remain visible, not patched or hidden by this lane. A blocked assertion or original oracle is reported as blocked, never GREEN.
