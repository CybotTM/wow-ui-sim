#!/usr/bin/env bash
# Release-profile proof for the Mists parity contract.
#
# Builds release binaries for the Mists profile, then runs the zero
# `lua-errors` baseline, installed-addon startup matrix, base panel parity
# with visual comparison, installed-addon panel matrix, saved-variable panel
# parity, live GUI smoke, and interaction audit.
# Every step writes a tee-style log while preserving the command exit status.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$REPO_ROOT/target/mists-release-proof"
LOG_DIR="$OUT_DIR/logs"
WOW_SIM_BIN="$REPO_ROOT/target/release/wow-sim"
WOW_CLI_BIN="$REPO_ROOT/target/release/wow-cli"
PANEL_VISUAL_METRICS_BIN="$REPO_ROOT/target/release/panel-visual-metrics"

SKIP_BUILD=0
VALIDATE_ONLY=0
VALIDATE_ARTIFACTS_ONLY=0
SKIP_CLONE=0
PANEL_FILTER=""
ADDON_FILTER=""
LIVE_GUI_SMOKE_TIMEOUT_SECONDS="${MISTS_LIVE_GUI_SMOKE_TIMEOUT_SECONDS:-300}"
INTERACTION_AUDIT_TIMEOUT_SECONDS="${MISTS_INTERACTION_AUDIT_TIMEOUT_SECONDS:-600}"

usage() {
    sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
}

while [ $# -gt 0 ]; do
    case "$1" in
        --panel) PANEL_FILTER="$2"; shift 2 ;;
        --addon) ADDON_FILTER="$2"; shift 2 ;;
        --out-dir) OUT_DIR="$2"; LOG_DIR="$OUT_DIR/logs"; shift 2 ;;
        --skip-build) SKIP_BUILD=1; shift ;;
        --skip-clone) SKIP_CLONE=1; shift ;;
        --validate-only) VALIDATE_ONLY=1; shift ;;
        --validate-artifacts-only) VALIDATE_ARTIFACTS_ONLY=1; shift ;;
        --help|-h) usage; exit 0 ;;
        *) echo "ERROR: unknown argument '$1'" >&2; exit 2 ;;
    esac
done

step_log_path() {
    local name="$1"
    echo "$LOG_DIR/$name.log"
}

run_logged_step() {
    local name="$1"
    shift
    local log_file
    log_file="$(step_log_path "$name")"
    mkdir -p "$(dirname "$log_file")"

    echo ""
    echo "=== $name ==="
    echo "log: $log_file"

    set +e
    "$@" > >(tee "$log_file") 2> >(tee -a "$log_file" >&2)
    local status=$?
    set -e

    if [ "$status" -ne 0 ]; then
        echo "ERROR: $name failed with exit code $status" >&2
        return "$status"
    fi
}

build_release_binaries() {
    cargo build --release \
        --bin wow-sim \
        --bin wow-cli \
        --bin panel-visual-metrics \
        --no-default-features \
        --features "sound,gui,casc,client-mists"
}

zero_lua_errors() {
    local json_file="$OUT_DIR/mists-release-lua-errors.json"
    mkdir -p "$OUT_DIR"

    WOW_SIM_NO_ADDONS=1 WOW_SIM_NO_SAVED_VARS=1 timeout 90 \
        "$WOW_SIM_BIN" --no-addons --no-saved-vars lua-errors \
        > "$json_file"

    local count
    count="$(jq 'length' "$json_file")"
    if [ "$count" != "0" ]; then
        echo "ERROR: Mists release lua-errors baseline is not zero: $json_file" >&2
        jq . "$json_file" >&2
        return 1
    fi

    "$REPO_ROOT/scripts/diff-lua-errors.sh" \
        "$REPO_ROOT/docs/baselines/mists-lua-errors.json" \
        "$json_file" \
        --exit-on-regression
}

installed_addon_matrix() {
    local args=(--profile mists --skip-build --fail-on-addon-errors)
    [ "$SKIP_CLONE" -eq 1 ] && args+=(--skip-clone)
    [ -n "$ADDON_FILTER" ] && args+=("$ADDON_FILTER")

    WOW_SIM_BIN="$WOW_SIM_BIN" \
        "$REPO_ROOT/scripts/test-classic-addons.sh" "${args[@]}"
}

panel_parity_matrix() {
    local args=(--skip-build --out-dir "$OUT_DIR/panel-parity")
    [ -n "$PANEL_FILTER" ] && args+=(--panel "$PANEL_FILTER")

    WOW_SIM_BIN="$WOW_SIM_BIN" \
    PANEL_VISUAL_METRICS_BIN="$PANEL_VISUAL_METRICS_BIN" \
        "$REPO_ROOT/scripts/mists-panel-parity.sh" "${args[@]}"
}

panel_parity_with_saved_vars() {
    local args=(--skip-build --with-saved-vars --out-dir "$OUT_DIR/panel-parity-with-saved-vars")
    [ -n "$PANEL_FILTER" ] && args+=(--panel "$PANEL_FILTER")

    WOW_SIM_BIN="$WOW_SIM_BIN" \
    PANEL_VISUAL_METRICS_BIN="$PANEL_VISUAL_METRICS_BIN" \
        "$REPO_ROOT/scripts/mists-panel-parity.sh" "${args[@]}"
}

installed_addon_panel_matrix() {
    local args=(--skip-build --out-dir "$OUT_DIR/addon-panel-parity")
    [ -n "$PANEL_FILTER" ] && args+=(--panel "$PANEL_FILTER")
    [ -n "$ADDON_FILTER" ] && args+=(--addon "$ADDON_FILTER")

    WOW_SIM_BIN="$WOW_SIM_BIN" \
    PANEL_VISUAL_METRICS_BIN="$PANEL_VISUAL_METRICS_BIN" \
        "$REPO_ROOT/scripts/test-mists-addon-panels.sh" "${args[@]}"
}

installed_addon_panel_matrix_with_saved_vars() {
    local args=(--skip-build --with-saved-vars --out-dir "$OUT_DIR/addon-panel-parity-with-saved-vars")
    [ -n "$PANEL_FILTER" ] && args+=(--panel "$PANEL_FILTER")
    [ -n "$ADDON_FILTER" ] && args+=(--addon "$ADDON_FILTER")

    WOW_SIM_BIN="$WOW_SIM_BIN" \
    PANEL_VISUAL_METRICS_BIN="$PANEL_VISUAL_METRICS_BIN" \
        "$REPO_ROOT/scripts/test-mists-addon-panels.sh" "${args[@]}"
}

live_gui_smoke() {
    local args=(--skip-build --out-dir "$OUT_DIR/live-gui-smoke")

    WOW_SIM_BIN="$WOW_SIM_BIN" \
    WOW_CLI_BIN="$WOW_CLI_BIN" \
        timeout "$LIVE_GUI_SMOKE_TIMEOUT_SECONDS" \
        "$REPO_ROOT/scripts/mists-live-gui-smoke.sh" "${args[@]}"
}

interaction_audit() {
    timeout "$INTERACTION_AUDIT_TIMEOUT_SECONDS" \
        cargo test --release \
        --no-default-features \
        --features "sound,gui,casc,client-mists" \
        --test mists_panel_interaction_audit
}

artifact_file_exists() {
    local path="$1"
    if [ ! -s "$path" ]; then
        echo "ERROR: missing artifact file: ${path#$OUT_DIR/}" >&2
        return 1
    fi
}

validate_lane_logs() {
    local log_name missing=0
    for log_name in \
        build-release.log \
        zero-lua-errors.log \
        installed-addon-matrix.log \
        panel-parity-and-visual-comparison.log \
        installed-addon-panel-matrix.log \
        panel-parity-with-saved-vars.log \
        installed-addon-panel-matrix-with-saved-vars.log \
        live-gui-smoke.log \
        interaction-audit.log
    do
        artifact_file_exists "$LOG_DIR/$log_name" || missing=1
    done
    return "$missing"
}

validate_base_lua_errors_artifact() {
    artifact_file_exists "$OUT_DIR/mists-release-lua-errors.json"
}

parse_pass_panel_rows() {
    local line panel status artifacts slug
    while IFS= read -r line || [ -n "$line" ]; do
        [[ "$line" == \|* ]] || continue
        IFS='|' read -r _ panel status artifacts _ <<< "$line"
        panel="$(trim_artifact_cell "$panel")"
        status="$(trim_artifact_cell "$status")"
        artifacts="$(trim_artifact_cell "$artifacts")"
        [ "$panel" = "Panel" ] && continue
        [ "$panel" = "---" ] && continue
        [ "$status" = "Pass" ] || continue
        slug="${artifacts#*target/mists-panel-parity/}"
        slug="${slug%%/*}"
        [ -n "$slug" ] || continue
        matches_artifact_panel_filter "$panel" "$slug" && printf '%s\n' "$slug"
    done < "$REPO_ROOT/docs/baselines/mists-panels.md"
}

trim_artifact_cell() {
    local value="$*"
    value="${value#"${value%%[![:space:]]*}"}"
    value="${value%"${value##*[![:space:]]}"}"
    printf '%s' "$value"
}

matches_artifact_panel_filter() {
    local panel="$1" slug="$2"
    [ -z "$PANEL_FILTER" ] && return 0
    [ "$PANEL_FILTER" = "$slug" ] && return 0
    [[ "${panel,,}" == *"${PANEL_FILTER,,}"* ]]
}

parse_mists_addon_names() {
    local name profile url ref subpath
    while IFS=$'\t' read -r name profile url ref subpath; do
        [[ "$name" =~ ^# ]] && continue
        [ -z "$name" ] && continue
        [ "$name" = "name" ] && continue
        [ "$profile" = "mists" ] || continue
        [ -n "$ADDON_FILTER" ] && [ "$ADDON_FILTER" != "$name" ] && continue
        printf '%s\n' "$name"
    done < "$REPO_ROOT/tools/classic-addon-manifest.tsv"
}

validate_panel_artifact_dir() {
    local matrix_dir="$1" slug="$2"
    local missing=0
    artifact_file_exists "$matrix_dir/$slug/screenshot.webp" || missing=1
    artifact_file_exists "$matrix_dir/$slug/dump-tree.txt" || missing=1
    artifact_file_exists "$matrix_dir/$slug/lua-errors.json" || missing=1
    return "$missing"
}

validate_panel_matrix_artifacts() {
    local matrix_dir="$1" slug missing=0
    while IFS= read -r slug; do
        validate_panel_artifact_dir "$matrix_dir" "$slug" || missing=1
    done < <(parse_pass_panel_rows)
    return "$missing"
}

validate_addon_panel_matrix_artifacts() {
    local matrix_dir="$1" addon slug missing=0
    while IFS= read -r addon; do
        while IFS= read -r slug; do
            validate_panel_artifact_dir "$matrix_dir/$addon" "$slug" || missing=1
        done < <(parse_pass_panel_rows)
    done < <(parse_mists_addon_names)
    return "$missing"
}

validate_release_artifacts() {
    local missing=0
    validate_lane_logs || missing=1
    validate_base_lua_errors_artifact || missing=1
    validate_panel_matrix_artifacts "$OUT_DIR/panel-parity" || missing=1
    validate_panel_matrix_artifacts "$OUT_DIR/panel-parity-with-saved-vars" || missing=1
    validate_addon_panel_matrix_artifacts "$OUT_DIR/addon-panel-parity" || missing=1
    validate_addon_panel_matrix_artifacts "$OUT_DIR/addon-panel-parity-with-saved-vars" || missing=1

    if [ "$missing" -ne 0 ]; then
        return 1
    fi
    echo "Mists release proof artifacts are complete: $OUT_DIR"
}

validate_plan() {
    "$REPO_ROOT/scripts/mists-panel-parity.sh" --validate-only
    "$REPO_ROOT/scripts/mists-live-gui-smoke.sh" --validate-only
    local addon_panel_args=(--validate-only)
    [ -n "$ADDON_FILTER" ] && addon_panel_args+=(--addon "$ADDON_FILTER")
    "$REPO_ROOT/scripts/test-mists-addon-panels.sh" "${addon_panel_args[@]}"

    echo "release proof lane: zero-lua-errors"
    echo "release proof lane: installed-addon-matrix"
    echo "release proof lane: panel-parity"
    echo "release proof lane: visual-comparison"
    [ "${MISTS_PANEL_SIGNAL_ONLY:-0}" = "1" ] && echo "release proof lane: signal-only-visuals"
    echo "release proof lane: installed-addon-panel-matrix"
    echo "release proof lane: panel-parity-with-saved-vars"
    echo "release proof lane: installed-addon-panel-matrix-with-saved-vars"
    echo "release proof lane: live-gui-smoke"
    echo "release proof lane: interaction-audit"
    echo "release proof lane: artifact-completeness"
}

if [ "$VALIDATE_ONLY" -eq 1 ]; then
    validate_plan
    exit 0
fi

if [ "$VALIDATE_ARTIFACTS_ONLY" -eq 1 ]; then
    validate_release_artifacts
    exit 0
fi

mkdir -p "$LOG_DIR"

if [ "$SKIP_BUILD" -eq 0 ]; then
    run_logged_step build-release build_release_binaries
fi
run_logged_step zero-lua-errors zero_lua_errors
run_logged_step installed-addon-matrix installed_addon_matrix
run_logged_step panel-parity-and-visual-comparison panel_parity_matrix
run_logged_step installed-addon-panel-matrix installed_addon_panel_matrix
run_logged_step panel-parity-with-saved-vars panel_parity_with_saved_vars
run_logged_step installed-addon-panel-matrix-with-saved-vars installed_addon_panel_matrix_with_saved_vars
run_logged_step live-gui-smoke live_gui_smoke
run_logged_step interaction-audit interaction_audit
run_logged_step artifact-completeness validate_release_artifacts

echo ""
echo "Mists release proof passed. Artifacts: $OUT_DIR"
