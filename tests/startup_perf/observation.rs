use std::time::Duration;

use serde_json::{Value, json};
use wow_ui_sim::loader::LoadResult;

#[derive(Debug)]
pub struct AddonLoadObservation {
    pub expected_name: String,
    pub wall_elapsed: Duration,
    pub result: LoadResult,
}

/// Encodes loader observations without treating overlapping timings as exclusive.
/// `ordinal` is the caller-supplied, 1-based startup load position.
pub fn encode_addon_observation(ordinal: usize, observation: &AddonLoadObservation) -> Value {
    let result = &observation.result;
    let timing = &result.timing;
    json!({
        "kind": "startup_addon_load",
        "ordinal": ordinal,
        "expected_name": observation.expected_name,
        "actual_name": result.name,
        "lua_files": result.lua_files,
        "xml_files": result.xml_files,
        "load_addon_elapsed_ns": observation.wall_elapsed.as_nanos(),
        "timing_ns": {
            "io": timing.io_time.as_nanos(),
            "xml_parse": timing.xml_parse_time.as_nanos(),
            "xml_process": timing.xml_process_time.as_nanos(),
            "lua_compile": timing.lua_compile_time.as_nanos(),
            "lua_call": timing.lua_call_time.as_nanos(),
            "lua_exec": timing.lua_exec_time.as_nanos(),
            "savedvars": timing.saved_vars_time.as_nanos(),
            "total": timing.total().as_nanos(),
        },
        "cache": {
            "hits": timing.cache_hits,
            "misses": timing.cache_misses,
            "lookup_misses": timing.cache_lookup_misses,
            "replay_failures": timing.cache_replay_failures,
            "store_successes": timing.cache_store_successes,
            "store_failures": timing.cache_store_failures,
        },
        "diagnostics": {
            "warnings": result.warnings.len(),
            "nil_symbol_observations": result.nil_symbol_observations.len(),
            "missing_requirements": result.missing_requirements.len(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wow_ui_sim::loader::{
        LoadDiagnosticAttribution, LoadTiming, MissingRequirement, MissingRequirementKind,
        NilSymbolEnvironment, NilSymbolObservation, NilSymbolObservationKind,
    };

    #[test]
    fn addon_observation_roundtrip_preserves_values_and_only_diagnostic_counts() {
        let attribution = LoadDiagnosticAttribution {
            addon_name: "ActualAddon".into(),
            environment: NilSymbolEnvironment::Public,
            source: Some("private-source.lua".into()),
            line: Some(23),
        };
        let missing = MissingRequirement {
            kind: MissingRequirementKind::CMethod {
                namespace: "C_PrivateFixture".into(),
                method: "PrivateMethod".into(),
            },
            attribution: attribution.clone(),
        };
        let observation = AddonLoadObservation {
            expected_name: "ExpectedAddon".into(),
            wall_elapsed: Duration::from_nanos(61),
            result: LoadResult {
                name: "ActualAddon".into(),
                lua_files: 4,
                xml_files: 9,
                timing: LoadTiming {
                    io_time: Duration::from_nanos(2),
                    xml_parse_time: Duration::from_nanos(3),
                    xml_process_time: Duration::from_nanos(5),
                    lua_compile_time: Duration::from_nanos(13),
                    lua_call_time: Duration::from_nanos(17),
                    lua_exec_time: Duration::from_nanos(30),
                    saved_vars_time: Duration::from_nanos(7),
                    cache_hits: 11,
                    cache_misses: 12,
                    cache_lookup_misses: 13,
                    cache_replay_failures: 14,
                    cache_store_successes: 15,
                    cache_store_failures: 16,
                    ..LoadTiming::default()
                },
                warnings: vec!["private warning one".into(), "private warning two".into()],
                nil_symbol_observations: vec![NilSymbolObservation {
                    kind: NilSymbolObservationKind::Global {
                        name: "PrivateGlobal".into(),
                    },
                    attribution,
                }],
                missing_requirements: vec![missing.clone(), missing.clone(), missing],
            },
        };

        let encoded = encode_addon_observation(7, &observation);
        let serialized = serde_json::to_string(&encoded).expect("serialize addon observation");
        let decoded: Value =
            serde_json::from_str(&serialized).expect("decode addon observation JSON");

        assert_eq!(decoded, encoded);
        assert_eq!(
            decoded,
            json!({
                "kind": "startup_addon_load",
                "ordinal": 7,
                "expected_name": "ExpectedAddon",
                "actual_name": "ActualAddon",
                "lua_files": 4,
                "xml_files": 9,
                "load_addon_elapsed_ns": 61,
                "timing_ns": {
                    "io": 2,
                    "xml_parse": 3,
                    "xml_process": 5,
                    "lua_compile": 13,
                    "lua_call": 17,
                    "lua_exec": 30,
                    "savedvars": 7,
                    "total": 47,
                },
                "cache": {
                    "hits": 11,
                    "misses": 12,
                    "lookup_misses": 13,
                    "replay_failures": 14,
                    "store_successes": 15,
                    "store_failures": 16,
                },
                "diagnostics": {
                    "warnings": 2,
                    "nil_symbol_observations": 1,
                    "missing_requirements": 3,
                },
            })
        );
    }
}
