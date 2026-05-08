use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use wow_ui_sim::lua_api::WowLuaEnv;

const METHOD_SNAPSHOT_LUA: &str = r#"
local parent = CreateFrame("Frame", "MethodDiffCoverageParent")

local factories = {
    Button = function() return CreateFrame("Button") end,
    CheckButton = function() return CreateFrame("CheckButton") end,
    ColorSelect = function() return CreateFrame("ColorSelect") end,
    Cooldown = function() return CreateFrame("Cooldown") end,
    EditBox = function() return CreateFrame("EditBox") end,
    FontString = function() return parent:CreateFontString() end,
    Frame = function() return CreateFrame("Frame") end,
    GameTooltip = function() return GameTooltip end,
    MessageFrame = function() return CreateFrame("MessageFrame") end,
    Minimap = function() return CreateFrame("Minimap") end,
    Model = function() return CreateFrame("Model") end,
    PlayerModel = function() return CreateFrame("PlayerModel") end,
    ScrollFrame = function() return CreateFrame("ScrollFrame") end,
    SimpleHTML = function() return CreateFrame("SimpleHTML") end,
    Slider = function() return CreateFrame("Slider") end,
    StatusBar = function() return CreateFrame("StatusBar") end,
    Texture = function() return parent:CreateTexture() end,
}

local snapshot = {}
for type_name, factory in pairs(factories) do
    local ok, obj = pcall(factory)
    assert(ok and obj, "failed to create object for " .. type_name)

    local mt = getmetatable(obj)
    assert(mt and type(mt.__index) == "table", "missing __index table for " .. type_name)

    local method_names = {}
    for method_name in pairs(mt.__index) do
        method_names[#method_names + 1] = method_name
    end
    table.sort(method_names)
    snapshot[type_name] = table.concat(method_names, "\n")
end

return snapshot
"#;

const FRAME_METHODS_SECTION_START: &str = "[\"frame_methods\"] = {";

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn diff_path(file_name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs")
        .join("wow-client-diff")
        .join(file_name)
        .display()
        .to_string()
}

fn load_method_snapshot(env: &WowLuaEnv) -> BTreeMap<String, BTreeSet<String>> {
    let raw_snapshot: BTreeMap<String, String> = env
        .eval(METHOD_SNAPSHOT_LUA)
        .expect("failed to build metatable method snapshot");

    raw_snapshot
        .into_iter()
        .map(|(type_name, methods)| {
            let parsed_methods = methods
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(str::to_string)
                .collect();
            (type_name, parsed_methods)
        })
        .collect()
}

fn parse_diff_file(path: &str) -> BTreeMap<String, BTreeSet<String>> {
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read diff file: {path}"));
    let mut entries = BTreeMap::<String, BTreeSet<String>>::new();

    for line in contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let (type_name, method_name) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("invalid diff entry, expected Type:Method: {line}"));
        entries
            .entry(type_name.to_string())
            .or_default()
            .insert(method_name.to_string());
    }

    entries
}

fn parse_discovery_method_name(line: &str) -> Option<&str> {
    line.strip_prefix('"')
        .and_then(|value| value.strip_suffix("\","))
}

fn parse_discovery_type_name(line: &str) -> Option<&str> {
    line.strip_prefix("[\"")
        .and_then(|value| value.strip_suffix("\"] = {"))
}

fn reset_discovery_type_on_section_end(current_type: &mut Option<String>, line: &str) -> bool {
    if line != "}," {
        return false;
    }
    *current_type = None;
    true
}

fn insert_discovery_method_entry(
    parsed: &mut BTreeMap<String, BTreeSet<String>>,
    current_type: &Option<String>,
    line: &str,
) -> bool {
    let Some(type_name) = current_type else {
        return false;
    };
    let Some(method_name) = parse_discovery_method_name(line) else {
        return false;
    };

    parsed
        .entry(type_name.clone())
        .or_default()
        .insert(method_name.to_string());
    true
}

fn start_discovery_type_section(
    parsed: &mut BTreeMap<String, BTreeSet<String>>,
    current_type: &mut Option<String>,
    line: &str,
) -> bool {
    let Some(type_name) = parse_discovery_type_name(line) else {
        return false;
    };

    let type_name = type_name.to_string();
    *current_type = Some(type_name.clone());
    parsed.entry(type_name).or_default();
    true
}

fn parse_discovery_frame_methods(path: &str) -> BTreeMap<String, BTreeSet<String>> {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("failed to read discovery file: {path}"));
    let mut in_frame_methods = false;
    let mut current_type: Option<String> = None;
    let mut parsed = BTreeMap::<String, BTreeSet<String>>::new();

    for line in contents.lines().map(str::trim) {
        if !in_frame_methods {
            in_frame_methods = line == FRAME_METHODS_SECTION_START;
            continue;
        }
        if current_type.is_none() && line == "}," {
            break;
        }
        if reset_discovery_type_on_section_end(&mut current_type, line) {
            continue;
        }
        if insert_discovery_method_entry(&mut parsed, &current_type, line) {
            continue;
        }
        if start_discovery_type_section(&mut parsed, &mut current_type, line) {
            continue;
        }
    }

    parsed
}

fn compute_method_diff(
    snapshot: &BTreeMap<String, BTreeSet<String>>,
    discovered: &BTreeMap<String, BTreeSet<String>>,
) -> (
    BTreeMap<String, BTreeSet<String>>,
    BTreeMap<String, BTreeSet<String>>,
) {
    let mut missing = BTreeMap::new();
    let mut extra = BTreeMap::new();

    for type_name in discovered.keys() {
        let discovered_methods = discovered
            .get(type_name)
            .unwrap_or_else(|| panic!("missing discovery methods for widget type: {type_name}"));
        let actual_methods = snapshot.get(type_name).unwrap_or_else(|| {
            panic!("missing method snapshot for widget type listed in discovery data: {type_name}")
        });

        let missing_methods: BTreeSet<String> = discovered_methods
            .difference(actual_methods)
            .cloned()
            .collect();
        if !missing_methods.is_empty() {
            missing.insert(type_name.clone(), missing_methods);
        }

        let extra_methods: BTreeSet<String> = actual_methods
            .difference(discovered_methods)
            .cloned()
            .collect();
        if !extra_methods.is_empty() {
            extra.insert(type_name.clone(), extra_methods);
        }
    }

    (missing, extra)
}

fn flatten_diff_entries(diff: &BTreeMap<String, BTreeSet<String>>) -> BTreeSet<String> {
    diff.iter()
        .flat_map(|(type_name, methods)| {
            methods
                .iter()
                .map(move |method_name| format!("{type_name}:{method_name}"))
        })
        .collect()
}

fn assert_diff_file_matches_current_surface(
    label: &str,
    expected: &BTreeMap<String, BTreeSet<String>>,
    actual: &BTreeMap<String, BTreeSet<String>>,
) {
    if expected == actual {
        return;
    }

    let expected_entries = flatten_diff_entries(expected);
    let actual_entries = flatten_diff_entries(actual);

    let stale_entries: Vec<String> = expected_entries
        .difference(&actual_entries)
        .cloned()
        .collect();
    let untracked_entries: Vec<String> = actual_entries
        .difference(&expected_entries)
        .cloned()
        .collect();

    let mut message = format!("{label} is out of sync with the current metatable surface.");
    if !stale_entries.is_empty() {
        message.push_str("\n\nRemove stale entries:\n");
        message.push_str(&stale_entries.join("\n"));
    }
    if !untracked_entries.is_empty() {
        message.push_str("\n\nAdd new entries:\n");
        message.push_str(&untracked_entries.join("\n"));
    }

    panic!("{message}");
}

fn serialize_diff_file(diff: &BTreeMap<String, BTreeSet<String>>) -> String {
    let mut lines: Vec<String> = flatten_diff_entries(diff).into_iter().collect();
    lines.push(String::new());
    lines.join("\n")
}

#[test]
fn diff_methods_missing_snapshot_matches_current_metatable_surface() {
    let env = env();
    let snapshot = load_method_snapshot(&env);
    let discovered = parse_discovery_frame_methods(&diff_path("WowDiscovery.lua"));
    let expected_missing = parse_diff_file(&diff_path("diff_methods_missing.txt"));
    let (actual_missing, _) = compute_method_diff(&snapshot, &discovered);

    assert_diff_file_matches_current_surface(
        "diff_methods_missing.txt",
        &expected_missing,
        &actual_missing,
    );
}

#[test]
fn diff_methods_extra_snapshot_matches_current_metatable_surface() {
    let env = env();
    let snapshot = load_method_snapshot(&env);
    let discovered = parse_discovery_frame_methods(&diff_path("WowDiscovery.lua"));
    let expected_extra = parse_diff_file(&diff_path("diff_methods_extra.txt"));
    let (_, actual_extra) = compute_method_diff(&snapshot, &discovered);

    assert_diff_file_matches_current_surface(
        "diff_methods_extra.txt",
        &expected_extra,
        &actual_extra,
    );
}

#[test]
#[ignore = "helper for intentionally refreshing diff_methods_missing.txt and diff_methods_extra.txt"]
fn refresh_method_diff_files() {
    let env = env();
    let snapshot = load_method_snapshot(&env);
    let discovered = parse_discovery_frame_methods(&diff_path("WowDiscovery.lua"));
    let (actual_missing, actual_extra) = compute_method_diff(&snapshot, &discovered);

    fs::write(
        diff_path("diff_methods_missing.txt"),
        serialize_diff_file(&actual_missing),
    )
    .expect("failed to write diff_methods_missing.txt");
    fs::write(
        diff_path("diff_methods_extra.txt"),
        serialize_diff_file(&actual_extra),
    )
    .expect("failed to write diff_methods_extra.txt");
}
