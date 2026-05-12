#![cfg(feature = "client-mists")]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const PANEL_BASELINE: &str = include_str!("../docs/baselines/mists-panels.md");
const INTERACTION_AUDIT: &str = include_str!("../docs/baselines/mists-panel-interactions.md");

#[test]
fn mists_interaction_audit_covers_every_pass_panel_with_tests() {
    let baseline_panels = pass_panels(PANEL_BASELINE);
    let audit_rows = audit_rows(INTERACTION_AUDIT);

    assert_eq!(
        audit_rows.len(),
        baseline_panels.len(),
        "interaction audit row count must match pass panel count"
    );

    for panel in &baseline_panels {
        let Some(row) = audit_rows.get(panel) else {
            panic!("missing interaction audit row for panel: {panel}");
        };
        assert_ne!(
            row.status, "Missing",
            "panel still needs a Mists parity test: {panel}"
        );
        assert!(
            row.coverage.contains("tests/"),
            "panel audit row must name concrete tests: {panel}"
        );
        assert_test_references_exist(&row.coverage);
    }

    let audited_panels = audit_rows.keys().cloned().collect::<BTreeSet<_>>();
    assert_eq!(
        audited_panels, baseline_panels,
        "interaction audit must not drift from docs/baselines/mists-panels.md"
    );
}

fn pass_panels(markdown: &str) -> BTreeSet<String> {
    markdown
        .lines()
        .filter_map(parse_table_row)
        .filter(|row| row[1] == "Pass")
        .map(|row| row[0].to_string())
        .collect()
}

fn audit_rows(markdown: &str) -> BTreeMap<String, AuditRow> {
    markdown
        .lines()
        .filter_map(parse_table_row)
        .map(|row| {
            let audit = AuditRow {
                coverage: row[2].to_string(),
                status: row[3].to_string(),
            };
            (row[0].to_string(), audit)
        })
        .collect()
}

fn parse_table_row(line: &str) -> Option<Vec<String>> {
    if !line.starts_with('|') {
        return None;
    }
    let fields = line
        .split('|')
        .skip(1)
        .map(str::trim)
        .map(str::to_string)
        .collect::<Vec<_>>();
    if fields.len() < 4 || fields[0] == "Panel" || fields[0].starts_with("---") {
        return None;
    }
    Some(fields)
}

fn assert_test_references_exist(coverage: &str) {
    for reference in extract_test_references(coverage) {
        let (file_path, test_name) = reference
            .split_once("::")
            .unwrap_or_else(|| panic!("invalid test reference: {reference}"));
        let source = std::fs::read_to_string(file_path)
            .unwrap_or_else(|err| panic!("reading {file_path}: {err}"));
        assert!(
            Path::new(file_path).is_file(),
            "referenced test file is missing: {file_path}"
        );
        assert!(
            source.contains(&format!("fn {test_name}(")),
            "referenced test function is missing: {reference}"
        );
    }
}

fn extract_test_references(coverage: &str) -> Vec<&str> {
    coverage
        .split('`')
        .filter(|part| part.starts_with("tests/") && part.contains("::"))
        .collect()
}

struct AuditRow {
    coverage: String,
    status: String,
}
