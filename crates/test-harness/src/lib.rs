//! Internal development-only test harness declarations.
//!
//! `integration_tests!()` declares the discovered modules in the caller's scope.
//! `prefork_full_ui_cases!()` expands to the generated case slice expression,
//! resolving `Case` and `crate::...::run` in the caller's scope.

include!(concat!(env!("OUT_DIR"), "/integration_tests.rs"));
include!(concat!(env!("OUT_DIR"), "/prefork_full_ui_cases.rs"));
