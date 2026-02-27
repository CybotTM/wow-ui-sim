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

use super::valid_events_a::EVENTS_A;
use super::valid_events_b::EVENTS_B;
use super::valid_events_c::EVENTS_C;

/// Check if an event can be passed to `RegisterEvent()`.
pub fn is_registerable_event(name: &str) -> bool {
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

pub fn callback_events() -> &'static [&'static str] { CALLBACK_EVENTS }
pub fn restricted_events() -> &'static [&'static str] { RESTRICTED_EVENTS }

/// Events that exist in the WoW client but cannot be registered by addons.
/// From wowless events.yaml: registerable = false.
const NON_REGISTERABLE_EVENTS: &[&str] = &[
    "ARENA_REGISTRAR_CLOSED",
    "ARENA_REGISTRAR_SHOW",
    "ARENA_REGISTRAR_UPDATE",
    "ARENA_TEAM_INVITE_REQUEST",
    "ARENA_TEAM_ROSTER_UPDATE",
    "ARENA_TEAM_UPDATE",
    "AUCTION_BIDDER_LIST_UPDATE",
    "AUCTION_ITEM_LIST_UPDATE",
    "AUCTION_OWNED_LIST_UPDATE",
    "CHAT_MSG_ENCOUNTER_EVENT",
    "COMBAT_LOG_APPLY_FILTER_SETTINGS",
    "COMBAT_LOG_REFILTER_ENTRIES",
    "CONFIRM_BARBERS_CHOICE",
    "CORPSE_POSITION_UPDATE",
    "CRAFT_CLOSE",
    "CRAFT_SHOW",
    "CRAFT_UPDATE",
    "ENGRAVING_MODE_CHANGED",
    "ENGRAVING_TARGETING_MODE_CHANGED",
    "FORGE_MASTER_CLOSED",
    "FORGE_MASTER_ITEM_CHANGED",
    "FORGE_MASTER_OPENED",
    "FORGE_MASTER_SET_ITEM",
    "GLYPH_ADDED",
    "GLYPH_REMOVED",
    "GLYPH_UPDATED",
    "HOUSING_CATALOG_SEARCHER_RELEASED",
    "HOUSING_DECOR_NUDGE_STATUS_CHANGED",
    "LEARNED_SPELL_IN_TAB",
    "LFG_LIST_ROLE_UPDATE",
    "LOOT_HISTORY_AUTO_SHOW",
    "LOOT_HISTORY_FULL_UPDATE",
    "LOOT_HISTORY_ROLL_CHANGED",
    "LOOT_HISTORY_ROLL_COMPLETE",
    "MINIMAP_PING",
    "NEW_AUCTION_UPDATE",
    "PET_STABLE_UPDATE_PAPERDOLL",
    "PET_TALENT_UPDATE",
    "PLAYERBANKBAGSLOTS_CHANGED",
    "PLAYER_TARGET_SET_ATTACKING",
    "PREVIEW_PET_TALENT_POINTS_CHANGED",
    "PREVIEW_TALENT_POINTS_CHANGED",
    "PREVIEW_TALENT_PRIMARY_TREE_CHANGED",
    "PRODUCT_CHOICE_UPDATE",
    "QUEST_CHOICE_CLOSE",
    "QUEST_CHOICE_UPDATE",
    "RUNE_UPDATED",
    "STORE_ENTITLEMENT_NOTIFICATION",
    "STORE_PRODUCT_DELIVERED",
    "SUPER_TRACKED_QUEST_CHANGED",
    "TALENT_GROUP_ROLE_CHANGED",
    "TOOLTIP_SHOW_ITEM_COMPARISON",
    "TRADE_SKILL_FILTER_UPDATE",
    "TRADE_SKILL_UPDATE",
    "UNIQUE_QUEST_CHOICE_UPDATE",
    "UNIT_HAPPINESS",
    "UNIT_HEALTH_FREQUENT",
    "UNIT_PET_TRAINING_POINTS",
    "UPDATE_TRADESKILL_RECAST",
    "VOID_DEPOSIT_WARNING",
    "VOID_STORAGE_CONTENTS_UPDATE",
    "VOID_STORAGE_DEPOSIT_UPDATE",
    "VOID_STORAGE_UPDATE",
    "VOID_TRANSFER_DONE",
    "VOID_TRANSFER_SUCCESS",
];
