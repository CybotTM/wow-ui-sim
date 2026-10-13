//! Explicit availability input and unchanged cached-consumer contract.
#![cfg(feature = "retail-12-0-0")]

use std::fs;
use wow_ui_sim::c_api::c_cooldown_viewer::CooldownViewerAvailability;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::paths::default_blizzard_ui_addons_path;

const UNSUPPLIED_REASON: &str = "Availability not supplied";

fn new_env() -> WowLuaEnv {
    WowLuaEnv::new().expect("initialize independent availability environment")
}

fn supply_availability(env: &WowLuaEnv, is_available: bool, failure_reason: &str) {
    env.state().borrow_mut().cooldown_viewer_availability = CooldownViewerAvailability {
        is_available,
        failure_reason: failure_reason.to_owned(),
    };
}

fn query_availability(env: &WowLuaEnv) -> (bool, String) {
    env.eval("return C_CooldownViewer.IsCooldownViewerAvailable()")
        .expect("query live availability")
}

fn read_cached_lua(relative_path: &str) -> String {
    let cache = default_blizzard_ui_addons_path().expect("resolve active profile cache");
    let path = cache.join(relative_path);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn slice_source_block<'a>(source: &'a str, start: &str, next: &str) -> &'a str {
    let start_index = source.find(start).expect("cached consumer start missing");
    let remainder = &source[start_index..];
    let end_index = remainder
        .find(next)
        .expect("cached consumer boundary missing");
    &remainder[..end_index]
}

fn load_settings_predicate(env: &WowLuaEnv) {
    env.exec("NewSettings = {}; NewSettingsPredicates = {}")
        .expect("initialize settings definition tables");
    let source = read_cached_lua("Blizzard_Settings_Shared/Mainline/NewDefinitions.lua");
    env.exec_named(&source, "@cached/NewDefinitions.lua")
        .expect("load unchanged complete settings definitions");
}

fn query_settings_predicate(env: &WowLuaEnv) -> (bool, String) {
    env.eval("return NewSettingsPredicates.cooldownViewerEnabled()")
        .expect("query actual cached settings predicate")
}

fn install_visibility_dependencies(env: &WowLuaEnv) {
    env.exec(
        r#"
        CooldownViewerMixin = {}
        CVarCallbackRegistry = {
            GetCVarValueBool = function(_, name)
                assert(name == 'cooldownViewerEnabled')
                return true
            end,
        }
        CooldownViewerSettings = { IsVisible = function() return false end }
        "#,
    )
    .expect("install bounded visibility dependencies, not query replacement");
}

fn load_bounded_visibility_consumer(env: &WowLuaEnv) {
    let source = read_cached_lua("Blizzard_CooldownViewer/CooldownViewer.lua");
    let cvar = source
        .lines()
        .find(|line| line.starts_with("local cooldownViewerEnabledCVar = "))
        .expect("cached visibility lexical CVar declaration missing");
    let should_be_shown = slice_source_block(
        &source,
        "function CooldownViewerMixin:ShouldBeShown()",
        "\nfunction CooldownViewerMixin:SetIsEditing",
    );
    let is_editing = slice_source_block(
        &source,
        "function CooldownViewerMixin:IsEditing()",
        "\nfunction CooldownViewerMixin:SetHideWhenInactive",
    );
    install_visibility_dependencies(env);
    let chunk = format!("{cvar}\n{should_be_shown}\n{is_editing}");
    env.exec_named(&chunk, "@cached/CooldownViewer.lua:bounded-visibility")
        .expect("load unchanged cached visibility methods");
}

fn query_visibility(env: &WowLuaEnv) -> bool {
    env.eval("return CooldownViewerMixin.ShouldBeShown({ isEditing = false, IsEditing = CooldownViewerMixin.IsEditing })")
        .expect("query bounded cached visibility consumer")
}

#[test]
fn availability_zero_argument_query_returns_exactly_bool_and_string() {
    let env = new_env();
    let result: (i32, String, String) = env
        .eval(
            r#"
        local function inspect(...)
            return select('#', ...), type(select(1, ...)), type(select(2, ...))
        end
        return inspect(C_CooldownViewer.IsCooldownViewerAvailable())
        "#,
        )
        .expect("inspect zero-argument result arity and types");
    assert_eq!(result, (2, "boolean".to_owned(), "string".to_owned()));
}

#[test]
fn availability_unsupplied_state_is_explicitly_unavailable() {
    assert_eq!(
        query_availability(&new_env()),
        (false, UNSUPPLIED_REASON.to_owned())
    );
}

#[test]
fn availability_supplied_true_preserves_empty_reason() {
    let env = new_env();
    supply_availability(&env, true, "");
    assert_eq!(query_availability(&env), (true, String::new()));
}

#[test]
fn availability_supplied_false_preserves_exact_host_reason() {
    let env = new_env();
    let reason = "  Host unavailable: capacité Δ |n  ";
    supply_availability(&env, false, reason);
    assert_eq!(query_availability(&env), (false, reason.to_owned()));
}

#[test]
fn availability_existing_function_reads_live_host_state_on_fresh_queries() {
    let env = new_env();
    env.exec("savedAvailabilityQuery = C_CooldownViewer.IsCooldownViewerAvailable")
        .expect("retain real query without replacing it");
    for (available, reason) in [
        (false, UNSUPPLIED_REASON),
        (true, ""),
        (false, "Host state changed"),
    ] {
        supply_availability(&env, available, reason);
        let result: (bool, String) = env.eval("return savedAvailabilityQuery()").unwrap();
        assert_eq!(result, (available, reason.to_owned()));
    }
}

#[test]
fn availability_state_is_isolated_between_live_and_new_environments() {
    let first = new_env();
    let second = new_env();
    supply_availability(&first, true, "");
    supply_availability(&second, false, "Second environment only");
    assert_eq!(query_availability(&first), (true, String::new()));
    supply_availability(&first, false, "First environment changed");
    assert_eq!(
        query_availability(&first),
        (false, "First environment changed".to_owned())
    );
    assert_eq!(
        query_availability(&second),
        (false, "Second environment only".to_owned())
    );
    assert_eq!(
        query_availability(&new_env()),
        (false, UNSUPPLIED_REASON.to_owned())
    );
}

#[test]
fn availability_tainted_zero_argument_call_projects_supplied_state() {
    let env = new_env();
    for (available, reason) in [(false, "Host denied"), (true, "")] {
        supply_availability(&env, available, reason);
        let result: (bool, i32, bool, String) = env
            .eval(
                r#"
            forceinsecure()
            assert(not issecure(), 'fixture must enter tainted execution')
            local result = { pcall(function()
                local function pack(...) return select('#', ...), ... end
                return pack(C_CooldownViewer.IsCooldownViewerAvailable())
            end) }
            debug.setstacktaint(nil)
            return unpack(result)
            "#,
            )
            .expect("tainted zero-argument access");
        assert_eq!(result, (true, 2, available, reason.to_owned()));
    }
}

#[test]
fn availability_actual_settings_and_bounded_visibility_consume_live_state() {
    let env = new_env();
    load_settings_predicate(&env);
    load_bounded_visibility_consumer(&env);
    for (available, reason) in [
        (false, UNSUPPLIED_REASON),
        (true, ""),
        (false, "Host gate closed: exact reason |n Δ"),
    ] {
        supply_availability(&env, available, reason);
        assert_eq!(
            query_settings_predicate(&env),
            (available, reason.to_owned())
        );
        assert_eq!(query_visibility(&env), available);
    }
}
