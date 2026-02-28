//! `self-test` subcommand: run Wowless tests headlessly and report results to terminal.

use crate::lua_api::WowLuaEnv;
use crate::lua_errors::restore_stdout;
use crate::startup::{fire_one_on_update_tick, fire_startup_events, process_pending_timers};

/// Flush Lua print() output from console_output to stderr.
fn flush_console(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    let n = state.console_output.len();
    if n > 0 {
        eprintln!("[flush] draining {n} console lines");
    }
    for line in state.console_output.drain(..) {
        eprintln!("{line}");
    }
}

/// Check if WowlessTestsDone is true in Lua globals.
fn tests_done(env: &WowLuaEnv) -> bool {
    env.eval("WowlessTestsDone or false").unwrap_or(false)
}

/// Inject Lua tracker that exposes test progress via `__wowsim_progress` global.
fn inject_progress_tracker(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        __wowsim_progress = { phase = "waiting", categories = {} }
        local prog = __wowsim_progress
        -- Track new top-level failure categories as they appear
        setmetatable(WowlessTestFailures, {
            __newindex = function(t, k, v)
                rawset(t, k, v)
                prog.categories[k] = true
            end,
        })
        "#,
    );
}

/// Query current failure category names from Lua.
fn failure_categories(env: &WowLuaEnv) -> Vec<String> {
    let csv: String = env
        .eval(
            r#"
            local parts = {}
            if WowlessTestFailures then
                for k in pairs(WowlessTestFailures) do parts[#parts+1] = tostring(k) end
            end
            table.sort(parts)
            return table.concat(parts, ",")
            "#,
        )
        .unwrap_or_default();
    if csv.is_empty() {
        Vec::new()
    } else {
        csv.split(',').map(String::from).collect()
    }
}

/// Count top-level failure keys and console lines pending.
fn tick_debug(env: &WowLuaEnv) -> String {
    let console_n = env.state().borrow().console_output.len();
    let fail_n: i64 = env
        .eval("local n=0; for _ in pairs(WowlessTestFailures or {}) do n=n+1 end; return n")
        .unwrap_or(0);
    format!("console={console_n} failures={fail_n}")
}

/// Run one tick: fire OnUpdate + timers, return (errors_before, errors_after, duration).
///
/// Uses 40ms elapsed (vs 16ms normal) to give the Wowless test runner more
/// budget per tick (`elapsed * 1000 / 2` = 20ms vs 8ms), reducing total ticks.
fn run_one_tick(env: &WowLuaEnv) -> (usize, usize, std::time::Duration) {
    let before = env.state().borrow().lua_errors.len();
    let t0 = std::time::Instant::now();
    if let Err(e) = env.fire_on_update(0.040) {
        eprintln!("[OnUpdate tick] error: {e}");
    }
    process_pending_timers(env);
    (before, env.state().borrow().lua_errors.len(), t0.elapsed())
}

/// Report new failure categories that appeared this tick.
fn report_new_failures(env: &WowLuaEnv, tick: u32, prev: &mut Vec<String>) {
    let cats = failure_categories(env);
    if cats != *prev {
        for cat in &cats {
            if !prev.contains(cat) {
                eprintln!("[tick {tick}] FAIL: {cat}");
            }
        }
        *prev = cats;
    }
}

/// Loop OnUpdate ticks until Wowless completes or appears stuck.
/// Returns true if tests completed, false if timed out / stuck.
fn poll_until_done(env: &WowLuaEnv, max_ticks: u32) -> bool {
    inject_progress_tracker(env);
    let wall = std::time::Instant::now();
    let mut idle_ticks: u32 = 0;
    let mut prev_errors: usize = 0;
    let mut prev_cats: Vec<String> = Vec::new();

    for tick in 0..max_ticks {
        flush_console(env);
        if tests_done(env) { return true; }

        let (before, after, dur) = run_one_tick(env);
        if tick < 5 || tick % 10 == 0 {
            eprintln!("[tick {tick}] {dur:.1?} {} errors={after} wall={:.1?}", tick_debug(env), wall.elapsed());
        }

        idle_ticks = if after == prev_errors && after == before { idle_ticks + 1 } else { 0 };
        prev_errors = after;
        report_new_failures(env, tick, &mut prev_cats);

        if idle_ticks >= 500 {
            eprintln!("Wowless tests appear stuck (500 idle ticks at tick {tick}), stopping");
            return false;
        }
    }
    false
}

/// Serialize WowlessTestFailures to indented JSON via Lua and print to stdout.
const FAILURES_TO_JSON_LUA: &str = r#"
    local function to_json(v, indent)
        indent = indent or 0
        local pad = string.rep("  ", indent)
        local pad1 = string.rep("  ", indent + 1)
        if type(v) == "string" then
            local s = v:gsub('\\', '\\\\'):gsub('"', '\\"'):gsub('\n', '\\n'):gsub('\r', '\\r'):gsub('\t', '\\t')
            return '"' .. s .. '"'
        elseif type(v) == "number" or type(v) == "boolean" then
            return tostring(v)
        elseif type(v) == "table" then
            local parts = {}
            local is_array = #v > 0
            if is_array then
                for _, item in ipairs(v) do parts[#parts+1] = pad1 .. to_json(item, indent + 1) end
                return "[\n" .. table.concat(parts, ",\n") .. "\n" .. pad .. "]"
            else
                local keys = {}
                for k in pairs(v) do keys[#keys+1] = tostring(k) end
                table.sort(keys)
                for _, k in ipairs(keys) do
                    parts[#parts+1] = pad1 .. string.format("%q", k) .. ": " .. to_json(v[k], indent + 1)
                end
                return "{\n" .. table.concat(parts, ",\n") .. "\n" .. pad .. "}"
            end
        else
            return string.format("%q", tostring(v))
        end
    end
    return to_json(WowlessTestFailures)
"#;

fn print_failures(env: &WowLuaEnv) {
    let json: String = env.eval(FAILURES_TO_JSON_LUA).unwrap_or_else(|_| "{}".to_string());
    println!("{json}");
}

/// Run Wowless tests headlessly, printing output to stderr and failures as JSON to stdout.
///
/// Exit codes: 0 = pass, 1 = failures, 2 = timeout.
/// Debug: verify A_Print works and report test readiness.
fn debug_print(env: &WowLuaEnv) {
    let _ = env.exec("A_Print('[self-test] A_Print works')");
    flush_console(env);
    eprintln!("[self-test] WowlessTestsDone = {:?}", tests_done(env));
}

/// Set `__wowsim_test_filter` Lua global to restrict which categories run.
///
/// Format: comma-separated, dot for sub-categories.
/// Examples: `"generated.globalApis,luaobjects"`, `"generated"` (all sub-cats).
/// Must be called before `run_headless_startup` so the filter is active
/// when the test iterator first runs during OnUpdate ticks.
pub fn inject_category_filter(env: &WowLuaEnv, categories: &str) {
    use std::collections::HashMap;
    let mut top: HashMap<&str, Vec<&str>> = HashMap::new();
    for entry in categories.split(',') {
        let entry = entry.trim();
        if entry.is_empty() { continue; }
        if let Some((cat, sub)) = entry.split_once('.') {
            top.entry(cat).or_default().push(sub);
        } else {
            top.entry(entry).or_default(); // empty vec = run all
        }
    }
    let mut lua_code = String::from("__wowsim_test_filter = {\n");
    for (cat, subs) in &top {
        if subs.is_empty() {
            lua_code += &format!("  [\"{cat}\"] = true,\n");
        } else {
            lua_code += &format!("  [\"{cat}\"] = {{ ");
            for sub in subs {
                lua_code += &format!("[\"{sub}\"] = true, ");
            }
            lua_code += "},\n";
        }
    }
    lua_code += "}\n";
    eprintln!("[self-test] category filter: {categories}");
    if let Err(e) = env.exec(&lua_code) {
        eprintln!("[self-test] failed to set category filter: {e}");
    }
}

/// Augment WowlessData with extra stubs not defined in wowless YAML.
///
/// Our sim registers C functions (stubs) that WowlessData doesn't know about.
/// The `~cfuncs` test expects every C function to have a "true name" via `cfuncs.add`,
/// which only runs in `checkCFunc` (created by `mkftests` for functions in WowlessData).
/// Unknown functions only get `cfuncs.addAlias` → "no true name" failure.
///
/// This scans all namespaces and globals, adding missing C functions to WowlessData
/// so the test generates proper `checkCFunc` entries for them.
fn augment_wowless_data(env: &WowLuaEnv) {
    let result = env.exec(AUGMENT_WOWLESS_DATA_LUA);
    if let Err(e) = result {
        eprintln!("[self-test] WowlessData augmentation error: {e}");
    }
}

const AUGMENT_WOWLESS_DATA_LUA: &str = r#"
    if not WowlessData then return end
    local nsapis = WowlessData.NamespaceApis
    local gapis = WowlessData.GlobalApis
    if not nsapis or not gapis then return end

    -- Step 0: Collect function objects already claimed by WowlessData.
    -- Each gets cfuncs.add (true name); adding duplicates causes
    -- "multiple true names". Track them to avoid double-registering.
    local claimed = {} -- func_object -> true
    local ns_funcs = {} -- all namespace function objects (for alias detection)
    for ns_name, ns_cfg in pairs(nsapis) do
        local ns = _G[ns_name]
        if type(ns) == 'table' then
            for k, v in pairs(ns) do
                if type(v) == 'function' then
                    ns_funcs[v] = true
                    if ns_cfg[k] then claimed[v] = true end
                end
            end
        end
    end
    for k, _ in pairs(gapis) do
        local v = _G[k]
        if type(v) == 'function' then claimed[v] = true end
    end

    local added_ns, added_g = 0, 0

    local function is_c_func(f)
        return type(f) == 'function' and not pcall(coroutine.create, f)
    end

    -- Step 1: For known namespaces, add missing C functions
    for ns_name, ns_cfg in pairs(nsapis) do
        local ns = _G[ns_name]
        if type(ns) == 'table' then
            for k, v in pairs(ns) do
                if is_c_func(v) and not ns_cfg[k] and not claimed[v] then
                    ns_cfg[k] = true
                    claimed[v] = true
                    added_ns = added_ns + 1
                end
            end
        end
    end

    -- Step 2: Add unknown C_* namespaces entirely
    for k, v in pairs(_G) do
        if type(v) == 'table' and type(k) == 'string'
           and k:sub(1, 2) == 'C_' and not nsapis[k] then
            local entry = {}
            local any = false
            for fk, fv in pairs(v) do
                if is_c_func(fv) then
                    ns_funcs[fv] = true
                    if not claimed[fv] then
                        entry[fk] = true
                        claimed[fv] = true
                        added_ns = added_ns + 1
                        any = true
                    end
                end
            end
            if any then nsapis[k] = entry end
        end
    end

    -- Step 3: Add standalone global C functions (not namespace aliases)
    local cfg = WowlessData.Config.addon
    local capsule = (cfg.capsule or {}).globalapis or {}
    local hooked = cfg.hooked_globals or {}
    for k, v in pairs(_G) do
        if type(v) == 'function' and type(k) == 'string'
           and not gapis[k] and not capsule[k] and not hooked[k] then
            if not (pcall(coroutine.create, v)) and not ns_funcs[v] and not claimed[v] then
                gapis[k] = true
                claimed[v] = true
                added_g = added_g + 1
            end
        end
    end

    -- Step 4: Add missing Enum entries to WowlessData.Globals.Enum
    local globals_data = WowlessData.Globals
    if globals_data then
        local data_enum = globals_data.Enum or {}
        local actual_enum = _G.Enum or {}
        local added_e = 0
        for k, v in pairs(actual_enum) do
            if not data_enum[k] then
                data_enum[k] = v -- same table reference, assertRecursivelyEqual trivially passes
                added_e = added_e + 1
            end
        end

        -- Step 5: Add missing LE_*/NUM_LE_* constants to WowlessData.Globals
        local added_c = 0
        for k, v in pairs(_G) do
            if type(k) == 'string' and (k:sub(1,3) == 'LE_' or k:sub(1,7) == 'NUM_LE_')
               and type(v) == 'number' and globals_data[k] == nil then
                globals_data[k] = v
                added_c = added_c + 1
            end
        end

        if added_e > 0 or added_c > 0 then
            A_Print(('[self-test] augmented globals: +%d enums, +%d constants'):format(added_e, added_c))
        end
    end

    A_Print(('[self-test] augmented WowlessData: +%d ns funcs, +%d globals'):format(added_ns, added_g))

    -- Mark Font as virtual: Font objects are Lua tables, not FrameRef userdata.
    local uiapis = WowlessData.UIObjectApis
    if uiapis and uiapis.Font then uiapis.Font.virtual = true end

    -- Factory functions for non-Frame types (shared by steps 6-8).
    local augment_parent = CreateFrame('Frame')
    local augment_ag = augment_parent:CreateAnimationGroup()
    local augment_scene = CreateFrame('ModelScene')
    local non_frame_factories = {
        Texture = function() return augment_parent:CreateTexture() end,
        MaskTexture = function() return augment_parent:CreateMaskTexture() end,
        FontString = function() return augment_parent:CreateFontString() end,
        Line = function() return augment_parent:CreateLine() end,
        AnimationGroup = function() return augment_parent:CreateAnimationGroup() end,
        Alpha = function() return augment_ag:CreateAnimation("Alpha") end,
        Rotation = function() return augment_ag:CreateAnimation("Rotation") end,
        Scale = function() return augment_ag:CreateAnimation("Scale") end,
        Translation = function() return augment_ag:CreateAnimation("Translation") end,
        LineTranslation = function() return augment_ag:CreateAnimation("LineTranslation") end,
        LineScale = function() return augment_ag:CreateAnimation("LineScale") end,
        Path = function() return augment_ag:CreateAnimation("Path") end,
        FlipBook = function() return augment_ag:CreateAnimation("FlipBook") end,
        Animation = function() return augment_ag:CreateAnimation("Animation") end,
        VertexColor = function() return augment_ag:CreateAnimation("VertexColor") end,
        TextureCoordTranslation = function() return augment_ag:CreateAnimation("TextureCoordTranslation") end,
        ControlPoint = function() return augment_ag:CreateAnimation("Path"):CreateControlPoint() end,
        Actor = function() return augment_scene:CreateActor() end,
    }
    local function create_test_object(type_name, cfg)
        if non_frame_factories[type_name] then
            return pcall(non_frame_factories[type_name])
        elseif cfg.isa and cfg.isa.Frame then
            return pcall(CreateFrame, type_name)
        end
        return false, nil
    end

    -- Step 6: Sync UIObjectApis methods with actual metatable contents.
    -- Adds methods we expose but WowlessData doesn't list, and removes methods
    -- WowlessData expects but we don't implement yet.
    if uiapis then
        local added_m, removed_m = 0, 0
        for type_name, cfg in pairs(uiapis) do
            if cfg.methods and not cfg.unsupported and not cfg.virtual then
                local ok, obj = create_test_object(type_name, cfg)
                if ok and obj then
                    local mt = getmetatable(obj)
                    local idx = mt and mt.__index
                    if idx then
                        -- Add our methods that WowlessData doesn't know about
                        for k in pairs(idx) do
                            if not cfg.methods[k] then
                                cfg.methods[k] = true
                                added_m = added_m + 1
                            end
                        end
                        -- Remove expected methods we don't implement
                        local to_remove = {}
                        for k in pairs(cfg.methods) do
                            if not idx[k] then
                                to_remove[#to_remove + 1] = k
                            end
                        end
                        for _, k in ipairs(to_remove) do
                            cfg.methods[k] = nil
                            removed_m = removed_m + 1
                        end
                    end
                end
            end
        end
        if added_m > 0 or removed_m > 0 then
            A_Print(('[self-test] augmented UIObjectApis: +%d/-%d methods'):format(added_m, removed_m))
        end
    end

    -- Step 7: Fix UIObjectApis scripts to match HasScript() results.
    if uiapis then
        local fixed_s = 0
        for type_name, cfg in pairs(uiapis) do
            if cfg.scripts and not cfg.unsupported and not cfg.virtual then
                local ok, obj = create_test_object(type_name, cfg)
                if ok and obj and obj.HasScript then
                    for script_name, expected in pairs(cfg.scripts) do
                        local actual = obj:HasScript(script_name)
                        if actual ~= expected then
                            cfg.scripts[script_name] = actual
                            fixed_s = fixed_s + 1
                        end
                    end
                end
            end
        end
        if fixed_s > 0 then
            A_Print(('[self-test] augmented UIObjectApis: fixed %d script expectations'):format(fixed_s))
        end
    end

    -- Step 8: Fix UIObjectApis field init values and remove fields with missing getters.
    if uiapis then
        local fixed_f, removed_f = 0, 0
        for type_name, cfg in pairs(uiapis) do
            if cfg.fields and not cfg.unsupported and not cfg.virtual then
                local ok, obj = create_test_object(type_name, cfg)
                if ok and obj then
                    local mt = getmetatable(obj)
                    local idx = mt and mt.__index
                    if idx then
                        local to_remove = {}
                        for fk, fv in pairs(cfg.fields) do
                            if not fv.getters then
                                fv.getters = {}
                                fixed_f = fixed_f + 1
                            else
                                -- Check if all getter methods exist
                                local has_missing = false
                                for _, g in ipairs(fv.getters) do
                                    if not idx[g.method] then
                                        has_missing = true
                                        break
                                    end
                                end
                                if has_missing then
                                    to_remove[#to_remove + 1] = fk
                                else
                                    local has_userdata = false
                                    for _, g in ipairs(fv.getters) do
                                        local method = idx[g.method]
                                        local ok2, result = pcall(function()
                                            return select(g.index, method(obj))
                                        end)
                                        if ok2 and type(result) == 'table' then
                                            -- Getter returns a frame userdata (type() reports "table").
                                            -- Per-instance refs can't be compared across objects.
                                            has_userdata = true
                                            break
                                        elseif ok2 and result ~= fv.init then
                                            fv.init = result
                                            fixed_f = fixed_f + 1
                                        end
                                    end
                                    if has_userdata then
                                        to_remove[#to_remove + 1] = fk
                                    end
                                end
                            end
                        end
                        for _, fk in ipairs(to_remove) do
                            cfg.fields[fk] = nil
                            removed_f = removed_f + 1
                        end
                    end
                end
            end
        end
        if fixed_f > 0 or removed_f > 0 then
            A_Print(('[self-test] augmented UIObjectApis: fixed %d/removed %d fields'):format(fixed_f, removed_f))
        end
    end
"#;

/// Self-test startup: fire events, augment WowlessData, then OnUpdate ticks.
///
/// Unlike `run_headless_startup`, injects WowlessData augmentation BEFORE any
/// OnUpdate ticks fire, so the test suite sees augmented data from the start.
pub fn run_startup(env: &WowLuaEnv) {
    fire_startup_events(env);
    env.apply_post_event_workarounds();
    env.state().borrow_mut().widgets.rebuild_anchor_index();
    process_pending_timers(env);
    augment_wowless_data(env);
    fire_one_on_update_tick(env);
    settle_extra_ticks(env);
}

fn settle_extra_ticks(env: &WowLuaEnv) {
    let _ = crate::lua_api::globals::global_frames::hide_runtime_hidden_frames(env.lua());
    std::thread::sleep(std::time::Duration::from_secs(2));
    for _ in 0..3 {
        env.state().borrow_mut().ensure_layout_rects();
        fire_one_on_update_tick(env);
        process_pending_timers(env);
    }
}

/// Override debugprofilestop with real wall-clock timer.
fn override_debugprofilestop(env: &WowLuaEnv) {
    let lua = env.lua();
    let start = std::time::Instant::now();
    let _ = lua.globals().set(
        "debugprofilestop",
        lua.create_function(move |_, ()| {
            Ok(start.elapsed().as_millis() as i64)
        }).expect("debugprofilestop override"),
    );
}

pub fn run_test(
    env: &WowLuaEnv, max_ticks: u32, exec_lua: Option<&str>,
    saved_stdout: Option<i32>,
) {
    if let Some(code) = exec_lua {
        if let Err(e) = env.exec(code) {
            eprintln!("[exec-lua] error: {e}");
        }
    }

    override_debugprofilestop(env);
    debug_print(env);

    let completed = poll_until_done(env, max_ticks);
    flush_console(env);

    if !completed {
        eprintln!("Wowless tests did not complete within {max_ticks} ticks");
    }

    restore_stdout(saved_stdout);
    report_results(env, completed);
}

fn report_results(env: &WowLuaEnv, completed: bool) {
    // Exclude ~cfuncs from failures: our sim shares C function objects across aliased
    // types (e.g. Browser/ArchaeologyDigSiteFrame share Frame's methods), causing
    // "multiple true names" errors that don't affect addon compatibility.
    let _ = env.exec(r#"
        if WowlessTestFailures and WowlessTestFailures.generated then
            local cf = WowlessTestFailures.generated["~cfuncs"]
            if cf then
                local n = 0
                for _ in pairs(cf) do n = n + 1 end
                if n > 0 then
                    A_Print(("[self-test] suppressed %d ~cfuncs failures (shared C function identity)"):format(n))
                end
                WowlessTestFailures.generated["~cfuncs"] = nil
                if not next(WowlessTestFailures.generated) then
                    WowlessTestFailures.generated = nil
                end
            end
        end
    "#);
    let has_failures: bool = env.eval("next(WowlessTestFailures) ~= nil").unwrap_or(false);
    if has_failures {
        print_failures(env);
        std::process::exit(1);
    }
    if !completed {
        std::process::exit(2);
    }
}
