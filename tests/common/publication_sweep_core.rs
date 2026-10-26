//! Cache-independent register-driven publication/absence sweep.
//! Publication/absence only: no signature, output, security, or behavior parity claim.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use serde_json::{Value, json};
use wow_ui_sim::lua_api::WowLuaEnv;

#[path = "publication_probe.rs"]
mod publication_probe;

pub(crate) use publication_probe::{Entry, probe_entry};

/// One patch's sweep inputs. Env var names select a scratch register / results file.
pub(crate) struct SweepSpec {
    pub register: &'static str,
    pub known_gaps: &'static str,
    pub row_count: usize,
    pub register_env: &'static str,
    pub out_env: &'static str,
    /// Later patches, oldest first, within the same client line. A later add/remove
    /// overrides expected publication (recorded as `superseded_by`). Registers from
    /// another client history never supersede this patch.
    pub later_registers: &'static [&'static str],
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ClientLine {
    #[default]
    Retail,
    MistsClassic,
    ClassicEra,
    WrathClassic,
}

impl ClientLine {
    fn matches_profile(self, profile: wow_ui_sim::client_profile::ClientProfile) -> bool {
        use wow_ui_sim::client_profile::ClientProfile;
        matches!(
            (self, profile),
            (Self::Retail, ClientProfile::Retail | ClientProfile::Ptr)
                | (Self::MistsClassic, ClientProfile::Mists)
                | (Self::WrathClassic, ClientProfile::Wrath)
                | (
                    Self::ClassicEra,
                    ClientProfile::Era | ClientProfile::Anniversary
                )
        )
    }
}

#[derive(Deserialize)]
struct Register {
    #[serde(default)]
    client_line: ClientLine,
    entries: Vec<Entry>,
}

/// Expected publication after applying later-patch supersession.
struct Expectation<'a> {
    removed: bool,
    superseded_by: Option<&'a str>,
}

fn classify_entry(
    env: &WowLuaEnv,
    entry: &Entry,
    expectation: &Expectation,
    aliases: &BTreeMap<String, (String, String)>,
) -> Value {
    let (kind, detail, ok, value, default) = probe_entry(env, entry, expectation.removed, aliases);
    let default_mismatch = entry.section == "cvars"
        && entry.kind.as_deref() != Some("command")
        && !expectation.removed
        && entry.page_default.is_some()
        && entry.page_default != default;
    if default_mismatch {
        eprintln!(
            "CVar default mismatch {}: page={:?}, observed={default:?}",
            entry.id, entry.page_default
        );
    }
    json!({
        "expected": {
            "section": entry.section,
            "direction": entry.direction,
            "symbol": entry.symbol,
            "publication": if expectation.removed { "absent" } else { "published" },
            "page_default": entry.page_default,
            "superseded_by": expectation.superseded_by,
        },
        "observed": {
            "kind": kind, "detail": detail, "value": value, "default": default,
            "default_mismatch": default_mismatch,
        },
        "ok": ok,
    })
}

fn parse_register(source: &str, row_count: Option<usize>) -> Register {
    let register: Register = serde_json::from_str(source).expect("parse wikitext register");
    let ids: BTreeSet<_> = register.entries.iter().map(|entry| &entry.id).collect();
    assert_eq!(ids.len(), register.entries.len(), "duplicate source IDs");
    if let Some(row_count) = row_count {
        assert_eq!(
            register.entries.len(),
            row_count,
            "register row count changed"
        );
    }
    for entry in &register.entries {
        assert!(matches!(
            entry.section.as_str(),
            "global-api"
                | "framexml"
                | "scriptobjects"
                | "widgets"
                | "events"
                | "cvars"
                | "commands"
        ));
        assert!(matches!(
            entry.direction.as_str(),
            "added" | "removed" | "changed"
        ));
    }
    register
}

fn read_register(spec: &SweepSpec) -> Register {
    // A full scratch register allows negative controls without changing committed data.
    let source = match std::env::var_os(spec.register_env) {
        Some(path) => std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {} {path:?}: {error}", spec.register_env)),
        None => spec.register.to_owned(),
    };
    parse_register(&source, Some(spec.row_count))
}

/// Latest later-patch add/remove per symbol; `changed` rows keep publication as-is.
fn later_publication(spec: &SweepSpec, client_line: ClientLine) -> BTreeMap<String, Entry> {
    let mut latest = BTreeMap::new();
    for source in spec.later_registers {
        let register = parse_register(source, None);
        if register.client_line != client_line {
            continue;
        }
        for entry in register.entries {
            if entry.direction != "changed" {
                latest.insert(entry.symbol.clone(), entry);
            }
        }
    }
    latest
}

fn expectation_for<'a>(entry: &Entry, later: &'a BTreeMap<String, Entry>) -> Expectation<'a> {
    let own_removed = entry.direction == "removed";
    match later.get(&entry.symbol) {
        Some(newer) if (newer.direction == "removed") != own_removed => Expectation {
            removed: !own_removed,
            superseded_by: Some(&newer.id),
        },
        _ => Expectation {
            removed: own_removed,
            superseded_by: None,
        },
    }
}

fn write_results_if_requested(out_env: &str, results: &BTreeMap<String, Value>) {
    if let Some(path) = std::env::var_os(out_env) {
        let json = serde_json::to_vec_pretty(results).expect("serialize sweep results");
        std::fs::write(&path, json)
            .unwrap_or_else(|error| panic!("write {out_env} {path:?}: {error}"));
    }
}

pub(crate) fn run_sweep(
    env: &WowLuaEnv,
    spec: &SweepSpec,
    read_aliases: impl FnOnce() -> BTreeMap<String, (String, String)>,
) {
    let register = read_register(spec);
    assert!(
        register
            .client_line
            .matches_profile(wow_ui_sim::client_profile::ACTIVE),
        "register client line does not match active profile"
    );
    let later = later_publication(spec, register.client_line);
    let known: BTreeSet<String> =
        serde_json::from_str(spec.known_gaps).expect("parse known-gap IDs");
    let aliases = read_aliases();
    let results = register
        .entries
        .iter()
        .map(|entry| {
            let expectation = expectation_for(entry, &later);
            (
                entry.id.clone(),
                classify_entry(env, entry, &expectation, &aliases),
            )
        })
        .collect::<BTreeMap<_, _>>();
    // Persist every result before the mismatch assertion, including the first RED run.
    write_results_if_requested(spec.out_env, &results);
    let non_ok: BTreeSet<String> = results
        .iter()
        .filter(|(_, result)| result["ok"] == false)
        .map(|(id, _)| id.clone())
        .collect();
    let new_gaps: Vec<_> = non_ok.difference(&known).collect();
    let resolved_gaps: Vec<_> = known.difference(&non_ok).collect();
    assert_eq!(
        non_ok, known,
        "new gaps: {new_gaps:?}; resolved/stale gaps: {resolved_gaps:?}"
    );
}
