use crate::encounter_journal_data as data;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const MAX_SEARCH_RESULTS: usize = 200;
const MAX_SEARCH_SECTIONS: usize = 5000;

pub(super) fn ej_set_search(state: &mut LuaState) -> LuaResult<u32> {
    let text = String::from_stack(state, 1).unwrap_or_default();
    let results = compute_search_results(&text);
    let mut sim = borrow_state_mut(state)?;
    sim.encounter_journal.search_text = text;
    sim.encounter_journal.search_finished = true;
    sim.encounter_journal.search_results = results;
    Ok(0)
}

pub(super) fn ej_clear_search(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.encounter_journal.search_text.clear();
    sim.encounter_journal.search_results.clear();
    sim.encounter_journal.search_finished = true;
    Ok(0)
}

pub(super) fn ej_end_search(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.encounter_journal.search_finished = true;
    Ok(0)
}

pub(super) fn ej_get_search_size(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.encounter_journal.search_results.len();
    state.push(Val::Num(n as f64));
    Ok(1)
}

pub(super) fn ej_get_search_progress(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.encounter_journal.search_results.len();
    state.push(Val::Num(n as f64));
    Ok(1)
}

pub(super) fn ej_get_num_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.encounter_journal.search_results.len();
    state.push(Val::Num(n as f64));
    Ok(1)
}

pub(super) fn ej_get_search_result(state: &mut LuaState) -> LuaResult<u32> {
    let index = u32::from_stack(state, 1).unwrap_or(0) as usize;
    let result = {
        let sim = borrow_state(state)?;
        if index == 0 || index > sim.encounter_journal.search_results.len() {
            return Ok(0);
        }
        sim.encounter_journal.search_results[index - 1].clone()
    };
    let link = create_string(state, &result.item_link);
    state.push(Val::Num(result.id as f64));
    state.push(Val::Num(result.kind as f64));
    state.push(Val::Num(result.difficulty_id as f64));
    state.push(Val::Num(result.instance_id as f64));
    state.push(Val::Num(result.encounter_id as f64));
    state.push(link);
    state.push(Val::Num(result.icon as f64));
    Ok(7)
}

pub(super) fn ej_is_search_finished(state: &mut LuaState) -> LuaResult<u32> {
    let finished = borrow_state(state)?.encounter_journal.search_finished;
    state.push(Val::Bool(finished));
    Ok(1)
}

fn compute_search_results(query: &str) -> Vec<crate::lua_api::state::EncounterJournalSearchResult> {
    let needle = query.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return Vec::new();
    }
    let mut results = Vec::new();
    append_instance_results(&needle, &mut results);
    append_encounter_results(&needle, &mut results);
    append_section_results(&needle, &mut results);
    results
}

fn append_instance_results(
    needle: &str,
    results: &mut Vec<crate::lua_api::state::EncounterJournalSearchResult>,
) {
    use crate::lua_api::state::EncounterJournalSearchResult;
    for instance in data::INSTANCES.iter() {
        if results.len() >= MAX_SEARCH_RESULTS {
            return;
        }

        if matches_search_text(instance.name, needle) {
            results.push(EncounterJournalSearchResult {
                id: instance.id,
                kind: 1,
                instance_id: instance.id,
                ..Default::default()
            });
        }
    }
}

fn append_encounter_results(
    needle: &str,
    results: &mut Vec<crate::lua_api::state::EncounterJournalSearchResult>,
) {
    use crate::lua_api::state::EncounterJournalSearchResult;
    for encounter in data::ENCOUNTERS.iter() {
        if results.len() >= MAX_SEARCH_RESULTS {
            return;
        }

        if matches_search_text(encounter.name, needle) {
            results.push(EncounterJournalSearchResult {
                id: encounter.id,
                kind: 2,
                instance_id: encounter.instance_id,
                encounter_id: encounter.id,
                ..Default::default()
            });
        }
    }
}

fn append_section_results(
    needle: &str,
    results: &mut Vec<crate::lua_api::state::EncounterJournalSearchResult>,
) {
    use crate::lua_api::state::EncounterJournalSearchResult;
    for section in data::SECTIONS.iter().take(MAX_SEARCH_SECTIONS) {
        if results.len() >= MAX_SEARCH_RESULTS {
            return;
        }

        if matches_search_text(section.title, needle) {
            results.push(EncounterJournalSearchResult {
                id: section.id,
                kind: 3,
                instance_id: 0,
                encounter_id: section.encounter_id,
                ..Default::default()
            });
        }
    }
}

fn matches_search_text(text: &str, lowercase_needle: &str) -> bool {
    text.to_ascii_lowercase().contains(lowercase_needle)
}
