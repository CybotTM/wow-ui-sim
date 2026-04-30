//! WoW event name validation.
//! Generated from wowless data/products/wow/events.yaml (mainline).
//!
//! Two concepts:
//! - **Registerable**: events that addons can pass to `RegisterEvent()`.
//!   Split across valid_events_a/b/c submodules.
//! - **Non-registerable**: valid events that exist in the client but
//!   `RegisterEvent()` rejects them (e.g. CHAT_MSG_ENCOUNTER_EVENT).
//!
//! `is_valid_event` = registerable OR non-registerable (for C_EventUtils).
//! `is_registerable_event` = only registerable (for RegisterEvent).

#[cfg(feature = "client-retail")]
use super::valid_events_a::EVENTS_A;
#[cfg(feature = "client-retail")]
use super::valid_events_a_tail::EVENTS_A_TAIL;
#[cfg(feature = "client-retail")]
use super::valid_events_b::EVENTS_B;
#[cfg(feature = "client-retail")]
use super::valid_events_c::EVENTS_C;

/// Check if an event can be passed to `RegisterEvent()`.
///
/// Under non-retail client profiles the validator is permissive: the
/// wrath/mists/era/anniversary event lists predate the events.yaml dataset
/// (which is mainline-only), so rejecting unknown events would break
/// legitimate WotLK/MoP/Vanilla code paths. The retail profile keeps strict
/// validation against the generated event tables.
#[cfg(any(
    feature = "client-wrath",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]
pub fn is_registerable_event(name: &str) -> bool {
    crate::wrath::is_registerable_event(name)
}

#[cfg(feature = "client-retail")]
pub fn is_registerable_event(name: &str) -> bool {
    let first = name.as_bytes().first().copied().unwrap_or(0);
    if first <= b'G' {
        return EVENTS_A.contains(&name) || EVENTS_A_TAIL.contains(&name);
    }
    let chunk = if first <= b'P' { EVENTS_B } else { EVENTS_C };
    chunk.contains(&name)
}

/// Check if an event name is known to the WoW client (registerable or not).
pub fn is_valid_event(name: &str) -> bool {
    is_registerable_event(name) || NON_REGISTERABLE_EVENTS.binary_search(&name).is_ok()
}

/// Restricted events cannot be registered by addons (returns false as second value).
const RESTRICTED_EVENTS: &[&str] = &[
    "COMBAT_LOG_APPLY_FILTER_SETTINGS",
    "COMBAT_LOG_EVENT",
    "COMBAT_LOG_EVENT_UNFILTERED",
    "COMBAT_LOG_REFILTER_ENTRIES",
    "MINIMAP_PING",
    "TUTORIAL_COMBAT_EVENT",
];

pub fn is_restricted_event(name: &str) -> bool {
    RESTRICTED_EVENTS.binary_search(&name).is_ok()
}

/// Events that support RegisterEventCallback (from wowless events.yaml callback: true).
const CALLBACK_EVENTS: &[&str] = &[
    "CLASS_TALENTS_SWITCH_TO_LOADOUT_BY_INDEX",
    "CLASS_TALENTS_SWITCH_TO_LOADOUT_BY_NAME",
    "CLASS_TALENTS_SWITCH_TO_SPECIALIZATION_BY_INDEX",
    "CLASS_TALENTS_SWITCH_TO_SPECIALIZATION_BY_NAME",
    "COMBAT_LOG_APPLY_FILTER_SETTINGS",
    "COMBAT_LOG_EVENT",
    "COMBAT_LOG_EVENT_UNFILTERED",
    "COMBAT_LOG_REFILTER_ENTRIES",
    "ENCOUNTER_STATE_CHANGED",
    "MINIMAP_PING",
    "TOOLTIP_SHOW_ITEM_COMPARISON",
];

pub fn is_callback_event(name: &str) -> bool {
    CALLBACK_EVENTS.binary_search(&name).is_ok()
}

pub fn callback_events() -> &'static [&'static str] {
    CALLBACK_EVENTS
}
pub fn restricted_events() -> &'static [&'static str] {
    RESTRICTED_EVENTS
}

/// Events that exist in the WoW client but cannot be registered by addons.
/// From wowless events.yaml: registerable = false.
const NON_REGISTERABLE_EVENTS: &[&str] = &[];
