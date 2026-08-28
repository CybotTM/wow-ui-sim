use crate::loader::LoadResult;
use crate::lua_api::LoaderEnv;
use crate::lua_api::state::NilSymbolAccess;
use rilua::Val;
use std::collections::HashSet;
use std::path::PathBuf;

pub(super) fn append_nil_symbol_access_warnings(
    env: &LoaderEnv<'_>,
    addon_index: u16,
    addon_name: &str,
    start_index: usize,
    result: &mut LoadResult,
) {
    let (accesses, published_names) = {
        let state = env.state().borrow();
        let accesses = state.nil_symbol_accesses[start_index..].to_vec();
        let published_names = state
            .global_publications
            .iter()
            .filter(|(owner_index, _)| *owner_index == addon_index)
            .map(|(_, name)| name.clone())
            .collect::<Vec<_>>();
        (accesses, published_names)
    };
    let resolved_names = published_names
        .into_iter()
        .filter(|name| raw_public_global_is_non_nil(env, name))
        .collect::<HashSet<_>>();
    let grouped_accesses = summarize_nil_symbol_accesses(&accesses, addon_index, &resolved_names);
    result.warnings.extend(
        grouped_accesses
            .into_iter()
            .map(|report| format_missing_symbol_report(addon_name, &report)),
    );
}

fn summarize_nil_symbol_accesses(
    accesses: &[NilSymbolAccess],
    addon_index: u16,
    resolved_names: &HashSet<String>,
) -> Vec<MissingSymbolReport> {
    let mut reports = std::collections::BTreeMap::new();
    for access in accesses {
        if access.addon_index != Some(addon_index) || is_resolved_global(access, resolved_names) {
            continue;
        }
        let need = classify_nil_symbol_access(access);
        let location = format_nil_symbol_location(access);
        reports.entry(need).or_insert(location);
    }

    reports
        .into_iter()
        .map(|(need, location)| MissingSymbolReport { need, location })
        .collect()
}

fn is_resolved_global(access: &NilSymbolAccess, resolved_names: &HashSet<String>) -> bool {
    access.container == "_G"
        && !access.key.starts_with("C_")
        && resolved_names.contains(&access.key)
}

fn raw_public_global_is_non_nil(env: &LoaderEnv<'_>, name: &str) -> bool {
    env.with_state(|state| {
        let key = state.gc.intern_string(name.as_bytes());
        let value = state
            .gc
            .tables
            .get(state.global)
            .map(|globals| globals.get_str(key, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
        Ok::<bool, std::convert::Infallible>(!matches!(value, Val::Nil))
    })
    .unwrap_or(false)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MissingSymbolNeed {
    Global(String),
    CNamespace(String),
    CMethod { namespace: String, method: String },
}

fn classify_nil_symbol_access(access: &NilSymbolAccess) -> MissingSymbolNeed {
    if access.container == "_G" {
        return classify_global_nil_access(&access.key);
    }

    if access.container.starts_with("C_") {
        return MissingSymbolNeed::CMethod {
            namespace: access.container.clone(),
            method: access.key.clone(),
        };
    }

    MissingSymbolNeed::Global(format!("{}.{}", access.container, access.key))
}

fn classify_global_nil_access(key: &str) -> MissingSymbolNeed {
    if key.starts_with("C_") {
        return MissingSymbolNeed::CNamespace(key.to_string());
    }

    MissingSymbolNeed::Global(key.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MissingSymbolReport {
    need: MissingSymbolNeed,
    location: Option<String>,
}

fn format_missing_symbol_report(addon_name: &str, report: &MissingSymbolReport) -> String {
    let need = match &report.need {
        MissingSymbolNeed::Global(name) => format!("global {name}"),
        MissingSymbolNeed::CNamespace(namespace) => namespace.clone(),
        MissingSymbolNeed::CMethod { namespace, method } => format!("{namespace}.{method}"),
    };

    match &report.location {
        Some(location) => format!("{addon_name} needs {need} (accessed at {location})"),
        None => format!("{addon_name} needs {need}"),
    }
}

fn format_nil_symbol_location(access: &NilSymbolAccess) -> Option<String> {
    let source = access.source.as_deref()?;
    let line = access.line?;
    Some(format!("{}:{line}", summarize_chunk_source(source)))
}

fn summarize_chunk_source(source: &str) -> String {
    let stripped = source.trim_start_matches(['@', '=']);
    let path = PathBuf::from(stripped);
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| stripped.to_string())
}
