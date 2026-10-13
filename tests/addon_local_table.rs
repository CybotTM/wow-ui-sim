//! Patch 11.1.7 actual opted-in addon namespaces and explicit absence policies.
//! Cached Retail contract; absent/error cases below are chosen simulator policy,
//! not native observations. Each fixture writes two real addon TOCs and Lua files.

#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use std::fs;
use std::path::Path;

use rilua::table_security::wrap_host_secret_string;
use tempfile::TempDir;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::toc::TocFile;

const TARGET: &str = "LocalTableTarget";
const CALLER: &str = "LocalTableCaller";

const TARGET_FIRST: &str = r#"
local name, namespace = ...
assert(name == 'LocalTableTarget')
assert(type(namespace) == 'table')
TargetStarted = true
TargetChunkWasSecure = issecure()
NamespaceWitness = setmetatable({ original = namespace }, { __mode = 'v' })
namespace.firstFileValue = 17
namespace.value = 41
namespace.readValue = function() return namespace.value end
"#;

const TARGET_SECOND: &str = r#"
local name, namespace = ...
assert(name == 'LocalTableTarget')
assert(rawequal(namespace, NamespaceWitness.original), 'same second chunk argument in both files')
assert(namespace.firstFileValue == 17)
namespace.secondFileValue = 23
"#;

const CALLER_OBSERVES_AND_MUTATES: &str = r#"
local name, ownNamespace = ...
assert(name == 'LocalTableCaller')
assert(not issecure(), 'caller must carry real loader addon taint')
local query = C_AddOns.GetAddOnLocalTable
assert(type(query) == 'function', 'missing GetAddOnLocalTable')
local namespace = query('LocalTableTarget')
assert(select('#', query('LocalTableTarget')) == 1)
assert(type(namespace) == 'table')
assert(rawequal(namespace, NamespaceWitness.original), 'must return original chunk namespace')
assert(rawequal(namespace, query('LocalTableTarget')), 'repeat lookup preserves identity')
assert(not rawequal(namespace, ownNamespace), 'must not return caller namespace')
assert(namespace.firstFileValue == 17 and namespace.secondFileValue == 23)
namespace.value = 99
assert(namespace.readValue() == 99, 'original target closure sees caller mutation; not a copy')
CallerObserved = true
"#;

struct AddonFixture {
    directory: TempDir,
}

impl AddonFixture {
    fn new(target_metadata: &str, caller_code: &str) -> Self {
        let directory = tempfile::tempdir().expect("create two-addon fixture root");
        let target = directory.path().join(TARGET);
        let caller = directory.path().join(CALLER);
        fs::create_dir(&target).expect("create target addon folder");
        fs::create_dir(&caller).expect("create caller addon folder");
        fs::write(
            target.join(format!("{TARGET}.toc")),
            format!(
                "## Interface: 120005\n## Title: Target Display Title\n{target_metadata}First.lua\nSecond.lua\n"
            ),
        )
        .expect("write target TOC");
        fs::write(target.join("First.lua"), TARGET_FIRST).expect("write target first chunk");
        fs::write(target.join("Second.lua"), TARGET_SECOND).expect("write target second chunk");
        fs::write(
            caller.join(format!("{CALLER}.toc")),
            "## Interface: 120005\n## Title: Caller Display Title\nCaller.lua\n",
        )
        .expect("write caller TOC");
        fs::write(caller.join("Caller.lua"), caller_code).expect("write caller chunk");
        Self { directory }
    }

    fn root(&self) -> &Path {
        self.directory.path()
    }

    fn load(&self, env: &WowLuaEnv, folder: &str) {
        let toc = self.root().join(folder).join(format!("{folder}.toc"));
        let result = load_addon(&env.loader_env(), &toc).expect("load real fixture addon");
        assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    }
}

fn require_query(env: &WowLuaEnv) {
    env.exec(
        "assert(type(rawget(C_AddOns, 'GetAddOnLocalTable')) == 'function', 'missing GetAddOnLocalTable')",
    )
    .expect("GetAddOnLocalTable must be registered, not an absent pcall target");
}

fn inject_secret_name(env: &WowLuaEnv) {
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    let secret = wrap_host_secret_string(lua.state_mut(), TARGET);
    lua.state_mut().push(secret);
    lua.set_global_val("SecretTargetName", secret)
        .expect("publish genuine VM secret string");
    lua.state_mut().pop();
}

#[test]
fn addon_local_table_returns_original_multifile_namespace_to_loaded_caller() {
    let fixture = AddonFixture::new("## AllowAddOnTableAccess: 1\n", CALLER_OBSERVES_AND_MUTATES);
    let env = WowLuaEnv::new().expect("create isolated environment");
    require_query(&env);
    fixture.load(&env, TARGET);
    fixture.load(&env, CALLER);
    env.exec(
        r#"
        assert(CallerObserved == true)
        local namespace = C_AddOns.GetAddOnLocalTable('LocalTableTarget')
        assert(rawequal(namespace, NamespaceWitness.original))
        assert(namespace.value == 99 and namespace.readValue() == 99)
        "#,
    )
    .expect("shared target namespace survives caller's completed chunk");
}

#[test]
fn addon_local_table_retention_survives_gc_before_and_after_later_addon_load() {
    let fixture = AddonFixture::new("## AllowAddOnTableAccess: 1\n", CALLER_OBSERVES_AND_MUTATES);
    let env = WowLuaEnv::new().expect("create isolated environment");
    require_query(&env);
    fixture.load(&env, TARGET);
    // Only a weak global witness exists. The readValue closure is inside the
    // namespace itself, so its cycle cannot serve as an external strong root.
    env.exec(
        r#"
        collectgarbage('collect')
        collectgarbage('collect')
        assert(NamespaceWitness.original ~= nil, 'loader retains namespace after original chunks return')
        local namespace = C_AddOns.GetAddOnLocalTable('LocalTableTarget')
        assert(rawequal(namespace, NamespaceWitness.original))
        assert(namespace.firstFileValue == 17 and namespace.secondFileValue == 23)
        "#,
    )
    .expect("retained namespace is GC-live before a later addon loads");
    fixture.load(&env, CALLER);
    env.exec(
        r#"
        collectgarbage('collect')
        collectgarbage('collect')
        local namespace = C_AddOns.GetAddOnLocalTable('LocalTableTarget')
        assert(rawequal(namespace, NamespaceWitness.original))
        assert(namespace.value == 99 and namespace.readValue() == 99)
        assert(CallerObserved == true)
        "#,
    )
    .expect("GC-live original namespace after later caller chunk returns");
}

#[test]
fn addon_local_table_chosen_error_policy_for_unknown_unloaded_and_title_queries() {
    let fixture = AddonFixture::new(
        "## AllowAddOnTableAccess: 1\n",
        "local name, namespace = ...\n",
    );
    let env = WowLuaEnv::new().expect("create isolated environment");
    env.scan_and_register_addons(fixture.root());
    require_query(&env);
    env.exec(
        r#"
        local query = C_AddOns.GetAddOnLocalTable
        assert(C_AddOns.DoesAddOnExist('LocalTableTarget'))
        assert(not C_AddOns.IsAddOnLoaded('LocalTableTarget'))
        -- Native failure shape unknown: this draft explicitly chooses errors.
        assert(not pcall(query, 'NoSuchLocalTableAddon'))
        assert(not pcall(query, 'LocalTableTarget'), 'registered but unloaded must not expose a table')
        assert(TargetStarted == nil, 'query must not load an addon')
        assert(not C_AddOns.IsAddOnLoaded('LocalTableTarget'))
        "#,
    )
    .expect("unknown/unloaded selected policy and no implicit load");
    fixture.load(&env, TARGET);
    env.exec(
        r#"
        local query = C_AddOns.GetAddOnLocalTable
        assert(type(query('LocalTableTarget')) == 'table')
        assert(not pcall(query, 'Target Display Title'), 'display title must not select canonical folder')
        assert(not pcall(query, 'NoSuchLocalTableAddon'))
        "#,
    )
    .expect("canonical folder lookup, never title lookup");
}

#[test]
fn addon_local_table_chosen_error_policy_for_loaded_unopted_target() {
    // Missing opt-in and an explicit zero are separate real TOC cases.
    for metadata in ["", "## AllowAddOnTableAccess: 0\n"] {
        let fixture = AddonFixture::new(metadata, "local name, namespace = ...\n");
        let env = WowLuaEnv::new().expect("create isolated environment");
        require_query(&env);
        fixture.load(&env, TARGET);
        env.exec(
            r#"
            assert(TargetStarted == true and NamespaceWitness.original ~= nil)
            assert(C_AddOns.IsAddOnLoaded('LocalTableTarget'))
            -- Opt-in requirement is sourced; its error shape is chosen, native unknown.
            assert(not pcall(C_AddOns.GetAddOnLocalTable, 'LocalTableTarget'))
            "#,
        )
        .expect("loaded unopted target does not expose its real namespace");
    }
}

#[test]
fn addon_local_table_vm_allowed_when_untainted_secret_name_policy() {
    let caller = r#"
        assert(not issecure(), 'real caller chunk is tainted')
        assert(issecretvalue(SecretTargetName), 'host-injected VM secret required')
        assert(not pcall(C_AddOns.GetAddOnLocalTable, SecretTargetName))
        assert(not issecure(), 'rejected secret lookup must not clear caller taint')
        local namespace = C_AddOns.GetAddOnLocalTable('LocalTableTarget')
        assert(rawequal(namespace, NamespaceWitness.original), 'tainted plain-name lookup remains allowed')
        assert(not issecure(), 'successful lookup must not clear caller taint')
        CallerObserved = true
    "#;
    let fixture = AddonFixture::new("## AllowAddOnTableAccess: 1\n", caller);
    let env = WowLuaEnv::new().expect("create isolated environment");
    require_query(&env);
    fixture.load(&env, TARGET);
    inject_secret_name(&env);
    env.exec(
        r#"
        assert(issecure(), 'untainted control caller required')
        assert(issecretvalue(SecretTargetName))
        assert(rawequal(C_AddOns.GetAddOnLocalTable(SecretTargetName), NamespaceWitness.original))
        assert(issecure(), 'accepted secret lookup preserves untainted state')
        "#,
    )
    .expect("genuine VM secret name accepted from untainted code");
    fixture.load(&env, CALLER);
    env.exec("assert(CallerObserved == true)")
        .expect("real tainted caller exercised secret denial and plain-name success");
}

#[test]
fn addon_local_table_blizzard_plain_name_denied_to_tainted_caller_but_allowed_untainted() {
    // Cached Blizzard_CombatAudioAlerts.toc uses both declarations. The actual
    // TocFile::loads_as_blizzard_code policy recognizes AllowLoad, independent
    // of folder spelling; do not mutate AddonInfo.security or fake a state flag.
    let metadata = "## AllowLoad: game\n## AllowAddOnTableAccess: 1\n";
    let caller = r#"
        assert(not issecure(), 'real loader-stamped caller required')
        local name = 'LocalTableTarget'
        assert(not issecretvalue(name), 'test Blizzard denial independently of secret-input rejection')
        assert(not pcall(C_AddOns.GetAddOnLocalTable, name), 'insecure code cannot query Blizzard namespace')
        assert(not issecure(), 'Blizzard denial must not clear caller taint')
        CallerObserved = true
    "#;
    let fixture = AddonFixture::new(metadata, caller);
    let target_dir = fixture.root().join(TARGET);
    let toc_text = fs::read_to_string(target_dir.join(format!("{TARGET}.toc")))
        .expect("read actual target TOC");
    let toc = TocFile::parse(&target_dir, &toc_text);
    assert!(
        toc.loads_as_blizzard_code(),
        "fixture must follow real loader classification"
    );
    let env = WowLuaEnv::new().expect("create isolated environment");
    require_query(&env);
    fixture.load(&env, TARGET);
    env.exec(
        r#"
        assert(TargetChunkWasSecure == true, 'classified target actually executed untainted')
        assert(issecure(), 'untainted control required')
        assert(rawequal(C_AddOns.GetAddOnLocalTable('LocalTableTarget'), NamespaceWitness.original))
        "#,
    )
    .expect("untainted caller may query opted-in loader-classified Blizzard target");
    fixture.load(&env, CALLER);
    env.exec(
        r#"
        assert(CallerObserved == true)
        assert(issecure())
        assert(rawequal(C_AddOns.GetAddOnLocalTable('LocalTableTarget'), NamespaceWitness.original))
        "#,
    )
    .expect("tainted plain-name denial did not revoke untainted access");
}
