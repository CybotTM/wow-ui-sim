//! I4 staging: publication and original-consumer SHAPES, never native value parity.
//! Explicit profile gates keep these selectors out of unrelated client histories.
#![cfg(any(
    feature = "client-ptr",
    feature = "client-retail",
    feature = "client-wowforever"
))]

#[path = "patch_12_1_7_inherited_i4/runtime.rs"]
mod runtime;

#[test]
#[cfg(feature = "client-ptr")]
fn ptr_discovery() {
    runtime::run(false);
}

#[test]
#[cfg(feature = "client-ptr")]
fn ptr_preservation() {
    runtime::run(true);
}

#[test]
#[cfg(feature = "client-retail")]
fn retail_preservation() {
    runtime::run(true);
}

#[test]
#[cfg(feature = "client-wowforever")]
fn forever_preservation() {
    runtime::run(true);
}

/// Measurement only. Capture at the pre-repair SHA, not after a producer repair.
/// This selector does not assert compatibility and must never be called GREEN.
#[test]
fn capture_profile_baseline() {
    runtime::capture();
}
