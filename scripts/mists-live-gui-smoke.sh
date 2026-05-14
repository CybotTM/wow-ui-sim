#!/usr/bin/env bash
# Run a connected-GUI Mists smoke test through wow-cli.
#
# Starts a real Mists wow-sim GUI, connects to its Lua socket, dispatches every
# Mists micro-menu panel opener, and fails if real script dispatch records any
# Lua errors.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$REPO_ROOT/target/mists-live-gui-smoke"
WOW_SIM_BIN="${WOW_SIM_BIN:-$REPO_ROOT/target/debug/wow-sim}"
WOW_CLI_BIN="${WOW_CLI_BIN:-$REPO_ROOT/target/debug/wow-cli}"
SKIP_BUILD=0
VALIDATE_ONLY=0
BUTTON_FILTER=""
SIM_PID=""
SOCKET=""
LOG_FILE=""
WOW_CLI_LUA_TIMEOUT_SECONDS="${MISTS_LIVE_GUI_CLI_TIMEOUT_SECONDS:-30}"

MICRO_BUTTON_ROWS=(
    "CharacterMicroButton|CharacterFrame|mouse"
    "SpellbookMicroButton|SpellBookFrame|click"
    "TalentMicroButton|PlayerTalentFrame|click"
    "AchievementMicroButton|AchievementFrame|click"
    "QuestLogMicroButton|QuestLogFrame|click"
    "SocialsMicroButton|FriendsFrame|click"
    "GuildMicroButton|CommunitiesFrame|click"
    "EJMicroButton|EncounterJournal|click"
    "CollectionsMicroButton|CollectionsJournal|click"
    "PVPMicroButton|PVEFrame|mouse"
    "LFGMicroButton|PVEFrame|mouse"
    "MainMenuMicroButton|GameMenuFrame|mouse"
    "HelpMicroButton|HelpFrame|click"
    "StoreMicroButton|probe:StoreFrame_IsShown|click"
)

usage() {
    sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
}

while [ $# -gt 0 ]; do
    case "$1" in
        --button) BUTTON_FILTER="$2"; shift 2 ;;
        --out-dir) OUT_DIR="$2"; shift 2 ;;
        --skip-build) SKIP_BUILD=1; shift ;;
        --validate-only) VALIDATE_ONLY=1; shift ;;
        --help|-h) usage; exit 0 ;;
        *) echo "ERROR: unknown argument '$1'" >&2; exit 2 ;;
    esac
done

selected_rows() {
    local row button
    for row in "${MICRO_BUTTON_ROWS[@]}"; do
        button="${row%%|*}"
        [ -n "$BUTTON_FILTER" ] && [ "$BUTTON_FILTER" != "$button" ] && continue
        printf '%s\n' "$row"
    done
}

validate_rows() {
    local count=0 row button root mode
    while IFS='|' read -r button root mode; do
        [ -n "$button" ] || continue
        [ -n "$root" ] || { echo "ERROR: missing expected root for $button" >&2; return 2; }
        case "$mode" in
            click|mouse) ;;
            *) echo "ERROR: $button has unsupported dispatch mode '$mode'" >&2; return 2 ;;
        esac
        count=$((count + 1))
    done < <(selected_rows)

    if [ "$count" -eq 0 ]; then
        echo "ERROR: no Mists micro-button rows matched filter '$BUTTON_FILTER'" >&2
        return 2
    fi
    echo "$count Mists live GUI micro-button row(s) validated"
}

cleanup() {
    if [ -n "$SIM_PID" ] && kill -0 "$SIM_PID" 2>/dev/null; then
        kill "$SIM_PID" 2>/dev/null || true
        wait "$SIM_PID" 2>/dev/null || true
    fi
}

build_binaries() {
    cargo build --bin wow-sim --bin wow-cli --no-default-features --features "sound,gui,casc,client-mists"
}

start_gui() {
    mkdir -p "$OUT_DIR"
    LOG_FILE="$OUT_DIR/wow-sim.log"
    WOW_SIM_NO_ADDONS=1 WOW_SIM_NO_SAVED_VARS=1 \
        "$WOW_SIM_BIN" --no-addons --no-saved-vars >"$LOG_FILE" 2>&1 &
    SIM_PID="$!"
    SOCKET="/tmp/wow-lua-$SIM_PID.sock"
}

wait_for_gui_socket() {
    local attempt
    for attempt in $(seq 1 90); do
        if ! kill -0 "$SIM_PID" 2>/dev/null; then
            echo "ERROR: wow-sim exited before the Lua socket was ready" >&2
            tail -n 80 "$LOG_FILE" >&2 || true
            return 1
        fi
        if WOW_LUA_SOCKET="$SOCKET" "$WOW_CLI_BIN" lua -e 'print("READY")' 2>/dev/null | grep -q '^READY$'; then
            return 0
        fi
        sleep 1
    done
    echo "ERROR: timed out waiting for $SOCKET" >&2
    tail -n 80 "$LOG_FILE" >&2 || true
    return 1
}

write_setup_lua() {
    local path="$1"
    cat >"$path" <<'LUA'
__mists_live_smoke_errors = {}
debug.getregistry()["__error_handler"] = function(message)
    local text = tostring(message)
    __mists_live_smoke_errors[#__mists_live_smoke_errors + 1] = text
    print("SMOKE_LUA_ERROR", text)
end
print("SMOKE_SETUP_OK")
LUA
}

write_click_lua() {
    local path="$1" button="$2" root="$3" mode="$4"
    cat >"$path" <<LUA
local buttonName = "$button"
local rootName = "$root"
local dispatchMode = "$mode"
local button

local function fail(message)
    error("SMOKE_FAIL " .. buttonName .. ": " .. tostring(message), 0)
end

local function is_visible(frame)
    if not frame then
        return false
    end
    if frame.IsVisible then
        return frame:IsVisible()
    end
    if frame.IsShown then
        return frame:IsShown()
    end
    return true
end

local function expected_root_is_visible()
    local probe = rootName:match("^probe:(.+)$")
    if probe then
        local fn = _G[probe]
        return type(fn) == "function" and fn()
    end
    return is_visible(_G[rootName])
end

local function script(name)
    local fn = button.GetScript and button:GetScript(name)
    if type(fn) ~= "function" then
        return nil
    end
    return fn
end

local function run_script(name)
    local fn = script(name)
    if not fn then
        fail("missing " .. name .. " script")
    end
    local ok, err = pcall(fn, button, "LeftButton", false)
    if not ok then
        fail(name .. " failed: " .. tostring(err))
    end
end

local function dispatch_click()
    if dispatchMode == "click" then
        if type(button.Click) == "function" then
            button:Click("LeftButton")
            return
        end
        run_script("OnClick")
        return
    end

    A_Admin.SetMouseOverFrame(button)
    run_script("OnMouseDown")
    run_script("OnMouseUp")
    A_Admin.SetMouseOverFrame(nil)
end

button = _G[buttonName]
if not button then
    fail("missing button")
end
if button.IsShown and not button:IsShown() then
    print("SMOKE_SKIPPED", buttonName, "hidden")
    return
end
if button.IsEnabled and not button:IsEnabled() then
    print("SMOKE_SKIPPED", buttonName, "disabled")
    return
end

local before = #(__mists_live_smoke_errors or {})
dispatch_click()
if type(UpdateMicroButtons) == "function" then
    pcall(UpdateMicroButtons)
end
local after = #(__mists_live_smoke_errors or {})
if after > before then
    fail("lua error count changed from " .. before .. " to " .. after)
end

if not expected_root_is_visible() then
    fail("expected visible root " .. rootName)
end

print("SMOKE_CLICK_OK", buttonName, rootName)
LUA
}

run_lua_file() {
    local file="$1"
    WOW_LUA_SOCKET="$SOCKET" timeout "$WOW_CLI_LUA_TIMEOUT_SECONDS" "$WOW_CLI_BIN" lua -f "$file"
}

install_error_probe() {
    local setup="$OUT_DIR/setup.lua"
    write_setup_lua "$setup"
    local output
    output="$(run_lua_file "$setup")"
    echo "$output" | grep -q 'SMOKE_SETUP_OK' || {
        echo "ERROR: failed to install connected Lua error probe" >&2
        echo "$output" >&2
        return 1
    }
}

click_micro_buttons() {
    local row button root mode click_file output
    while IFS='|' read -r button root mode; do
        [ -n "$button" ] || continue
        echo "=== $button -> $root ==="
        click_file="$OUT_DIR/$button.lua"
        write_click_lua "$click_file" "$button" "$root" "$mode"
        output="$(run_lua_file "$click_file")" || {
            echo "$output" >&2
            return 1
        }
        echo "$output"
        if echo "$output" | grep -q 'SMOKE_FAIL\|SMOKE_LUA_ERROR'; then
            return 1
        fi
        if ! echo "$output" | grep -q 'SMOKE_CLICK_OK\|SMOKE_SKIPPED'; then
            echo "ERROR: $button did not report a smoke result" >&2
            return 1
        fi
    done < <(selected_rows)
}

assert_log_has_no_lua_errors() {
    if grep -q 'Lua error:' "$LOG_FILE"; then
        echo "ERROR: live GUI log contains Lua errors" >&2
        grep 'Lua error:' "$LOG_FILE" >&2 || true
        return 1
    fi
}

validate_rows
if [ "$VALIDATE_ONLY" -eq 1 ]; then
    exit 0
fi

if [ "$SKIP_BUILD" -eq 0 ]; then
    build_binaries
fi

trap cleanup EXIT
trap 'cleanup; exit 130' INT TERM

start_gui
wait_for_gui_socket
install_error_probe
click_micro_buttons
assert_log_has_no_lua_errors

echo "Mists live GUI smoke passed"
