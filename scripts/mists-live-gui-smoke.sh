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

DIRECT_PROBE_ROWS=(
    "idle-hud|UIParent"
    "specialization-learn|PlayerTalentFrame"
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
    validate_direct_probes
}

validate_direct_probes() {
    local count=0 row probe root
    for row in "${DIRECT_PROBE_ROWS[@]}"; do
        IFS='|' read -r probe root <<<"$row"
        [ -n "$probe" ] || { echo "ERROR: missing direct probe name" >&2; return 2; }
        [ -n "$root" ] || { echo "ERROR: missing expected root for direct probe $probe" >&2; return 2; }
        count=$((count + 1))
    done

    echo "$count Mists live GUI direct probe(s) validated"
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

write_direct_probe_lua() {
    local path="$1" probe="$2"
    case "$probe" in
        idle-hud) write_idle_hud_lua "$path" ;;
        specialization-learn) write_specialization_learn_lua "$path" ;;
        *) echo "ERROR: unknown direct probe '$probe'" >&2; return 2 ;;
    esac
}

write_idle_hud_lua() {
    local path="$1"
    cat >"$path" <<'LUA'
local function fail(message)
    error("SMOKE_FAIL idle-hud: " .. tostring(message), 0)
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

local before = #(__mists_live_smoke_errors or {})
for _, name in ipairs({
    "UIParent",
    "PlayerFrame",
    "MainMenuBar",
    "CharacterMicroButton",
    "MainMenuMicroButton",
    "MainMenuBarBackpackButton",
}) do
    if not is_visible(_G[name]) then
        fail(name .. " missing or hidden before panel input")
    end
end

for _, name in ipairs({
    "CharacterFrame",
    "SpellBookFrame",
    "PlayerTalentFrame",
}) do
    if is_visible(_G[name]) then
        fail(name .. " should not be visible in idle HUD state")
    end
end

local after = #(__mists_live_smoke_errors or {})
if after > before then
    fail("lua error count changed from " .. before .. " to " .. after)
end

print("SMOKE_PROBE_OK", "idle-hud", "UIParent")
LUA
}

write_specialization_learn_lua() {
    local path="$1"
    cat >"$path" <<'LUA'
local function fail(message)
    error("SMOKE_FAIL specialization-learn: " .. tostring(message), 0)
end

local function dispatch_click(button)
    if not button then
        fail("missing TalentMicroButton")
    end
    if type(button.Click) == "function" then
        button:Click("LeftButton")
        return
    end
    local on_click = button.GetScript and button:GetScript("OnClick")
    if type(on_click) ~= "function" then
        fail("TalentMicroButton has no clickable dispatch")
    end
    local ok, err = pcall(on_click, button, "LeftButton", false)
    if not ok then
        fail("TalentMicroButton click failed: " .. tostring(err))
    end
end

local function click_button(button, label)
    if not button then
        fail("missing " .. label)
    end
    if button.IsEnabled and not button:IsEnabled() then
        fail(label .. " disabled")
    end
    if type(button.Click) == "function" then
        button:Click("LeftButton")
        return
    end
    local on_click = button.GetScript and button:GetScript("OnClick")
    if type(on_click) ~= "function" then
        fail(label .. " has no OnClick script")
    end
    local ok, err = pcall(on_click, button, "LeftButton", false)
    if not ok then
        fail(label .. " click failed: " .. tostring(err))
    end
end

local before = #(__mists_live_smoke_errors or {})
if A_Admin and A_Admin.SetSpec then
    A_Admin.SetSpec(4)
end

dispatch_click(TalentMicroButton)
if not PlayerTalentFrame or not PlayerTalentFrame:IsShown() then
    fail("PlayerTalentFrame did not open from TalentMicroButton")
end

if type(PlayerTalentTab_OnClick) == "function" and PlayerTalentFrameTab1 then
    PlayerTalentTab_OnClick(PlayerTalentFrameTab1)
end
if type(PlayerTalentFrame_UpdateSpecFrame) == "function" then
    PlayerTalentFrame_UpdateSpecFrame(PlayerTalentFrameSpecialization, 2)
end

local learn_button = PlayerTalentFrameSpecialization and PlayerTalentFrameSpecialization.learnButton
click_button(learn_button, "specialization Learn button")

local dialog = StaticPopup_FindVisible and StaticPopup_FindVisible("CONFIRM_LEARN_SPEC")
if not dialog or not dialog.button1 then
    fail("specialization Learn confirmation did not open")
end
click_button(dialog.button1, "specialization Learn confirmation")

if C_SpecializationInfo.GetSpecialization() ~= 2 or GetSpecialization() ~= 2 then
    fail("specialization Learn did not activate the previewed spec")
end

if type(PlayerTalentTab_OnClick) == "function" and PlayerTalentFrameTab2 then
    PlayerTalentTab_OnClick(PlayerTalentFrameTab2)
end
if not PlayerTalentFrameTalents or not PlayerTalentFrameTalents:IsShown() then
    fail("talents tab was not reachable after learning specialization")
end

local row = PlayerTalentFrameTalents.tier1
local talent = row and row.talent1
if not talent or not talent:IsShown() or not talent.icon or not talent.icon:GetTexture() then
    fail("learned specialization did not expose a populated talent row")
end

local after = #(__mists_live_smoke_errors or {})
if after > before then
    fail("lua error count changed from " .. before .. " to " .. after)
end

print("SMOKE_PROBE_OK", "specialization-learn", "PlayerTalentFrame")
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

run_direct_probes() {
    local row probe root probe_file output
    for row in "${DIRECT_PROBE_ROWS[@]}"; do
        IFS='|' read -r probe root <<<"$row"
        echo "=== probe:$probe -> $root ==="
        probe_file="$OUT_DIR/probe-$probe.lua"
        write_direct_probe_lua "$probe_file" "$probe"
        output="$(run_lua_file "$probe_file")" || {
            echo "$output" >&2
            return 1
        }
        echo "$output"
        if echo "$output" | grep -q 'SMOKE_FAIL\|SMOKE_LUA_ERROR'; then
            return 1
        fi
        if ! echo "$output" | grep -q "SMOKE_PROBE_OK.*$probe"; then
            echo "ERROR: $probe did not report a direct probe result" >&2
            return 1
        fi
    done
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
run_direct_probes
click_micro_buttons
assert_log_has_no_lua_errors

echo "Mists live GUI smoke passed"
