//! Shared shape-only assertions. No API replacement or native-behavior oracle.
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn load(roots: &[&str]) -> WowLuaEnv {
    let ui = super::super::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, loaded) =
        super::super::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
            &ui,
            roots,
            &[],
        );
    for root in roots {
        assert!(loaded.iter().any(|name| name == root), "load {root}");
    }
    env
}

pub fn check(env: &WowLuaEnv, script: &str) {
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(&format!("{ASSERTIONS}\n{script}"))
        .expect("documented shape and original consumer acceptance (native semantics unknown)");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

const ASSERTIONS: &str = r#"
local function pack(...)
    return { n = select('#', ...), ... }
end
local function scalar(kind, ...)
    local result = pack(...)
    assert(result.n == 1, 'documented single return: '..result.n)
    assert(type(result[1]) == kind, 'documented non-nil '..kind)
    return result[1]
end
local function original(fn, suffix)
    local info = debug.getinfo(fn, 'S')
    local source = info.source:gsub('\\', '/')
    assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
        'runtime-selected original consumer required: '..suffix)
end
local function enum(actual, declared)
    for name, value in pairs(declared) do
        assert(actual[name] == value, 'documented enum '..name)
    end
    for name in pairs(actual) do
        -- Some runtime enum tables publish metadata, not additional members.
        if name ~= 'NumValues' and name ~= 'MinValue' and name ~= 'MaxValue' then
            assert(declared[name] ~= nil, 'undocumented enum member '..name)
        end
    end
end
local function member(value, declared)
    local found = false
    for _, candidate in pairs(declared) do
        found = found or value == candidate
    end
    assert(found, 'documented enum membership')
end
local function fields(record, schema)
    assert(type(record) == 'table', 'documented record')
    for _, field in ipairs(schema) do
        local value = record[field[1]]
        if not (field[3] and value == nil) then
            assert(value ~= nil, 'non-nil field '..field[1])
            if field[2] ~= 'opaque' then
                assert(type(value) == field[2], 'field type '..field[1])
            end
        end
    end
end
"#;
