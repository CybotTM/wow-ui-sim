//! Valid WoW event names for RegisterEvent validation.
//! Generated from wowless data/products/wow/events.yaml (mainline).
//!
//! The data is split across three submodules (valid_events_a/b/c) to stay
//! within the project's per-file line limit. Each submodule holds a sorted
//! slice covering a portion of the alphabet; `is_valid_event` checks all three.

use super::valid_events_a::EVENTS_A;
use super::valid_events_b::EVENTS_B;
use super::valid_events_c::EVENTS_C;

/// Check if an event name is a known valid WoW event.
///
/// The three chunks together cover A–Z in sorted order. Each chunk is sorted,
/// so we binary-search within the correct chunk based on the first character.
pub fn is_valid_event(name: &str) -> bool {
    // Each chunk's first entry defines the lower bound; use that to route.
    // A–G  → EVENTS_A  (starts with "ACCOUNT_...", ends with "GX_RESTARTED")
    // H–P  → EVENTS_B  (starts with "HANDLE_...", ends with "PVP_WORLDSTATE_UPDATE")
    // Q–Z  → EVENTS_C  (starts with "QUESTLINE_...", ends with "ZONE_CHANGED_NEW_AREA")
    let first = name.as_bytes().first().copied().unwrap_or(0);
    let chunk = if first <= b'G' {
        EVENTS_A
    } else if first <= b'P' {
        EVENTS_B
    } else {
        EVENTS_C
    };
    chunk.binary_search(&name).is_ok()
}
