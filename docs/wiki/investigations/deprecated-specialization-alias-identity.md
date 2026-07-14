# Deprecated Specialization Alias Identity Clobbering

`Blizzard_DeprecatedSpecialization` copies `C_SpecializationInfo` methods into
plain globals by value (`GetSpecialization = C_SpecializationInfo.GetSpecialization`),
so Blizzard code expects the global aliases to stay identity-equal to the
namespace methods. The simulator's post-cleanup restore broke that identity by
re-registering the namespace after addons loaded.

## Content

### Symptoms

`blizzard_deprecated_specialization_direct_aliases_are_identity_equal_to_c_spec_info`
failed: all four direct aliases (`GetSpecialization`, `GetSpecializationInfo`,
`GetNumSpecializationsForClassID`, `GetActiveSpecGroup`) compared non-equal to
their `C_SpecializationInfo` counterparts after a full game-UI load.

### Root cause

`apply_post_load_workarounds` → `restore_post_cleanup_globals`
(`src/lua_api/workarounds/temporary/environment_cleanup_restore.rs`) calls
`c_api::register_utility_bootstrap_tables`, which re-runs
`register_c_specialization_info` after addons have loaded. Every
`table_set_rust_fn_static` call allocates a **fresh** Rust closure value, so
the re-registration replaced the table's methods with new function values while
the deprecated addon's global aliases still pointed at the originals.
`Blizzard_EnvironmentCleanup` never nils `C_SpecializationInfo` (it only
removes store/auth surfaces), so the re-registration was pure churn.

### Fix

`register_c_specialization_info_methods` (`src/c_api/c_spec.rs`) now skips a
method when the table already holds a Rust closure under that name
(`holds_rust_fn`). First registration still replaces Lua-closure gap fillers
installed by workaround bootstraps; re-registration becomes a no-op, preserving
closure identity across the post-cleanup restore.

### Related boundary cleanup (same change)

`c_spec.rs` also carried duplicate copies of the legacy specialization globals
and `UIWidgetContainerMixin` that had already been moved to
`src/lua_api/globals/real/specialization_legacy.rs` and
`real/ui_widget_container.rs`. The duplicates were deleted;
`GetInspectSpecialization`, `GetSpecializationRoleByID`, and legacy instant
`SetSpecialization` moved into `specialization_legacy.rs` so `c_api` contains
only `C_*` surfaces (enforced by `tests/specialization_legacy_boundary.rs` and
`tests/ui_widget_container_boundary.rs`, which failed until the duplicates were
removed).

## Sources

- `src/c_api/c_spec.rs` — idempotent method registration (`holds_rust_fn`)
- `src/lua_api/workarounds/temporary/environment_cleanup_restore.rs` — the re-registration path
- `tests/blizzard_deprecated_specialization_loads.rs` — identity regression test

## See Also

- [[specialization-mastery-spells]] — mastery spell modeling that retired the namespace's last gap-filler shim
