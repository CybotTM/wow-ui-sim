//! Build 12340 name queries over existing GUID-backed social/party state.
//! Authority: laptop relay `autocomplete-results-12340.md` (2026-10-21),
//! static reconstruction, not native execution. ASCII matching is modeled;
//! native UTF-8 helper/case-map edges and recency timestamps remain unmodeled.

use std::collections::BTreeMap;

use crate::lua_api::game_data::PartyMember;
use crate::lua_api::globals::unit_misc::party_guid_for_index;
use crate::lua_api::state::SimState;
use crate::lua_api::state_types::{SEEDED_LOCAL_CHARACTER_GUID, SocialFriend};

use super::{FLAG_FRIEND, FLAG_IN_GROUP, FLAG_ONLINE};

pub(crate) struct Query {
    pub text: String,
    pub include: u32,
    pub exclude: u32,
    pub max_results: i32,
    pub cursor: usize,
    pub allow_full_match: bool,
}

struct Candidate {
    name: String,
    flags: u32,
}

pub(crate) fn query_names(sim: &SimState, query: &Query) -> Vec<String> {
    let context = cvar_enabled(sim, "autoCompleteUseContext");
    let center = cvar_enabled(sim, "autoCompleteWhenEditingFromCenter");
    let mut candidates = collect_candidates(sim);
    candidates.retain(|candidate| {
        matches_context(candidate, query, context) && matches_text(&candidate.name, query, center)
    });
    // No recency input exists: only the native alphabetical tie ordering is modeled.
    candidates.sort_by_cached_key(|candidate| candidate.name.to_ascii_lowercase());
    if query.max_results > 0 {
        candidates.truncate(query.max_results as usize);
    }
    candidates
        .into_iter()
        .map(|candidate| candidate.name)
        .collect()
}

fn collect_candidates(sim: &SimState) -> Vec<Candidate> {
    let friends = sim.social_friends.iter().filter_map(friend_candidate);
    let party_count = if sim.party_group_active {
        sim.party_members.len()
    } else {
        0
    };
    let party = sim
        .party_members
        .iter()
        .take(party_count)
        .enumerate()
        .filter_map(party_candidate);
    let mut identities: BTreeMap<String, Candidate> = BTreeMap::new();
    for (guid, candidate) in friends.chain(party) {
        identities
            .entry(guid)
            .and_modify(|entry| entry.flags |= candidate.flags)
            .or_insert(candidate);
    }
    // GuildMember has no GUID; do not invent one or infer identity from equal text.
    // Native presence nodes, whisper history and incomplete friend identities are held.
    identities.into_values().collect()
}

fn friend_candidate(friend: &SocialFriend) -> Option<(String, Candidate)> {
    let has_identity = !friend.guid.is_empty();
    let is_self = friend.guid == SEEDED_LOCAL_CHARACTER_GUID;
    if !has_identity || is_self {
        return None;
    }
    let online = if friend.is_online { FLAG_ONLINE } else { 0 };
    let candidate = Candidate {
        name: friend.name.clone(),
        flags: FLAG_FRIEND | online,
    };
    Some((friend.guid.clone(), candidate))
}

fn party_candidate((index, member): (usize, &PartyMember)) -> Option<(String, Candidate)> {
    if !member.name_cached {
        return None;
    }
    let online = if member.connected { FLAG_ONLINE } else { 0 };
    let candidate = Candidate {
        name: member.name.clone(),
        flags: FLAG_IN_GROUP | online,
    };
    Some((party_guid_for_index(index), candidate))
}

fn cvar_enabled(sim: &SimState, name: &str) -> bool {
    sim.cvars
        .get(name)
        .is_some_and(|value| value.parse::<i32>().is_ok_and(|number| number != 0))
}

fn matches_context(candidate: &Candidate, query: &Query, enabled: bool) -> bool {
    if !enabled {
        return true;
    }
    let included = candidate.flags & query.include != 0;
    let excluded = candidate.flags & query.exclude != 0;
    included && !excluded
}

fn matches_text(name: &str, query: &Query, center: bool) -> bool {
    if !query.allow_full_match && name.eq_ignore_ascii_case(&query.text) {
        return false;
    }
    if !center {
        return starts_with(name.as_bytes(), query.text.as_bytes());
    }
    let (prefix, tail) = split_query(query);
    if !starts_with(name.as_bytes(), prefix) {
        return false;
    }
    if name.chars().count() < query.text.chars().count() {
        return false;
    }
    if tail.is_empty() {
        return true;
    }
    matches_tail(name.as_bytes(), tail, prefix.len())
}

fn starts_with(name: &[u8], prefix: &[u8]) -> bool {
    name.get(..prefix.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
}

fn split_query(query: &Query) -> (&[u8], &[u8]) {
    let cursor = query
        .text
        .char_indices()
        .nth(query.cursor)
        .map_or(query.text.len(), |(byte, _)| byte);
    query.text.as_bytes().split_at(cursor)
}

fn matches_tail(name: &[u8], tail: &[u8], cursor: usize) -> bool {
    let Some(remaining) = name.len().checked_sub(tail.len() + cursor) else {
        return false;
    };
    // 12340 never tests the final possible tail position; preserve that bound.
    (0..remaining).any(|offset| {
        let start = cursor + offset;
        name[start..start + tail.len()].eq_ignore_ascii_case(tail)
    })
}
