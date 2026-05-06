use crate::loader::LoadResult;
use crate::lua_api::LoaderEnv;
use crate::lua_api::state::NilSymbolAccess;
use std::path::PathBuf;

pub(super) fn append_nil_symbol_access_warnings(
    env: &LoaderEnv<'_>,
    addon_name: &str,
    start_index: usize,
    result: &mut LoadResult,
) {
    let grouped_accesses = {
        let state = env.state().borrow();
        summarize_nil_symbol_accesses(&state.nil_symbol_accesses[start_index..])
    };
    result.warnings.extend(
        grouped_accesses
            .into_iter()
            .map(|report| format_missing_symbol_report(addon_name, &report)),
    );
}

fn summarize_nil_symbol_accesses(accesses: &[NilSymbolAccess]) -> Vec<MissingSymbolReport> {
    let mut reports = std::collections::BTreeMap::new();
    for access in accesses {
        let need = classify_nil_symbol_access(access);
        let location = format_nil_symbol_location(access);
        reports.entry(need).or_insert(location);
    }

    reports
        .into_iter()
        .map(|(need, location)| MissingSymbolReport { need, location })
        .collect()
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
