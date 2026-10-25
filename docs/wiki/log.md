## [2026-10-25] maintenance | Utility command admission and lock rebase

[Findings](investigations/compile-bin-lockwatch.md) record rebase to `b2ca0becd`, current command feature admission, preserved historical evidence, and two offline metadata runs: 781 unchanged package records and stable lock bytes. Master already normalized the lock; no compile or CPU proof.

## [2026-10-25] investigation | Compile binary selection

[Source findings](investigations/compile-bin-lockwatch.md) and [contract](../specs/integration-bin-selection.md): opt-in utility bins preserve wow-sim test consumers and operational builders. Static-only; no slot/build/CPU proof.

## [2026-10-24] design | Generated data crate boundary

[Crate split](design/generated-data-crates.md) preserves the root facade and selected tables while isolating common catalogs from profile features. Same-slot touched-root rebuild 413.80→258.52 s; peak child RSS6,189,056→2,717,272 KiB. Retail/Wrath checks and94 bounded Rust tests pass; all23 generated files retain their hashes. Single warm-dependency samples, not a cold-build or full-profile claim.
