use std::io::Write;

use wow_ui_sim::loader::{LoadDiagnosticChannel, classify_load_diagnostic, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn load_diagnostics_keep_observations_requirements_and_failures_distinct() {
    assert_eq!(
        classify_load_diagnostic("Addon needs global OptionalFrame"),
        LoadDiagnosticChannel::Observation
    );
    assert_eq!(
        classify_load_diagnostic("Addon needs C_Container.MissingMethod"),
        LoadDiagnosticChannel::Requirement
    );
    assert_eq!(
        classify_load_diagnostic("Addon.lua: load failed"),
        LoadDiagnosticChannel::Failure
    );
}

fn create_test_addon_with_missing_symbol_accesses() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let addon_dir = dir.path();

    let toc_path = addon_dir.join("TestNilSymbols.toc");
    let mut toc = std::fs::File::create(&toc_path).unwrap();
    writeln!(toc, "## Title: TestNilSymbols").unwrap();
    writeln!(toc, "TestNilSymbols.lua").unwrap();

    let lua_path = addon_dir.join("TestNilSymbols.lua");
    let mut lua = std::fs::File::create(&lua_path).unwrap();
    writeln!(
        lua,
        r#"local _ = MissingGlobalSymbol
local _ = C_MissingNamespace
local _ = C_Container.MissingMethod
local _ = C_Container.MissingMethod
local _ = _G.OptionalMissingGlobal
local _ = _G["OptionalMissingGlobal"]
local _ = _G.DynamicThenDirectMissingGlobal
local _ = DynamicThenDirectMissingGlobal
local _ = _G.C_ExplicitMissingNamespace
local _ = _G.C_Container.ExplicitMissingMethod
"#
    )
    .unwrap();

    dir
}

fn create_test_addon_with_late_symbol_publication() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let addon_dir = dir.path();

    let toc_path = addon_dir.join("TestLatePublication.toc");
    let mut toc = std::fs::File::create(&toc_path).unwrap();
    writeln!(toc, "## Title: TestLatePublication").unwrap();
    writeln!(toc, "Early.lua").unwrap();
    writeln!(toc, "Published.xml").unwrap();
    writeln!(toc, "Late.lua").unwrap();

    let mut early = std::fs::File::create(addon_dir.join("Early.lua")).unwrap();
    writeln!(
        early,
        r#"local _ = PublishedByLua
local _ = PublishedByXml
local _ = StillMissingGlobal
local _ = C_Container.StillMissingMethod
"#
    )
    .unwrap();

    let mut published = std::fs::File::create(addon_dir.join("Published.xml")).unwrap();
    writeln!(
        published,
        r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
    <Frame name="PublishedByXml"/>
</Ui>"#
    )
    .unwrap();

    let mut late = std::fs::File::create(addon_dir.join("Late.lua")).unwrap();
    writeln!(late, "PublishedByLua = true").unwrap();

    dir
}

fn create_test_secure_addon_with_late_symbol_publication() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let addon_dir = dir.path();

    let mut toc = std::fs::File::create(addon_dir.join("TestSecureLatePublication.toc")).unwrap();
    writeln!(toc, "## Title: TestSecureLatePublication").unwrap();
    writeln!(toc, "## UseSecureEnvironment: 1").unwrap();
    writeln!(toc, "PublicMiss.lua [LoadIntoEnvironment global]").unwrap();
    writeln!(toc, "Frames.xml").unwrap();
    writeln!(toc, "Late.lua").unwrap();

    std::fs::write(
        addon_dir.join("PublicMiss.lua"),
        "local _ = SecurePublicationMustNotResolvePublic\n",
    )
    .unwrap();
    std::fs::write(
        addon_dir.join("Frames.xml"),
        r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
    <Frame name="LateSecureFunctionTemplate" virtual="true">
        <Scripts>
            <OnShow function="LaterSecureFunction"/>
        </Scripts>
    </Frame>
    <Frame name="EarlySecureFunctionFrame" inherits="LateSecureFunctionTemplate"/>
    <Frame name="MissingSecureFunctionFrame">
        <Scripts>
            <OnShow function="StillMissingSecureFunction"/>
        </Scripts>
    </Frame>
</Ui>"#,
    )
    .unwrap();
    std::fs::write(
        addon_dir.join("Late.lua"),
        r#"function LaterSecureFunction(self)
    self.lateSecureHandlerRan = true
end
function SecurePublicationMustNotResolvePublic()
end
local frame = CreateFrame("Frame", "LateSecureFunctionFrame", nil, "LateSecureFunctionTemplate")
frame:Hide()
frame:Show()
SecureLateHandlerExecuted = frame.lateSecureHandlerRan == true
"#,
    )
    .unwrap();

    dir
}

fn create_test_addon_with_cleared_publication() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let addon_dir = dir.path();

    let mut toc = std::fs::File::create(addon_dir.join("TestClearedPublication.toc")).unwrap();
    writeln!(toc, "## Title: TestClearedPublication").unwrap();
    writeln!(toc, "TestClearedPublication.lua").unwrap();

    let mut lua = std::fs::File::create(addon_dir.join("TestClearedPublication.lua")).unwrap();
    writeln!(lua, "local _ = PublishedThenCleared").unwrap();
    writeln!(lua, "PublishedThenCleared = true").unwrap();
    writeln!(lua, "PublishedThenCleared = nil").unwrap();

    dir
}

fn write_runtime_event_warning_addon(
    root: &std::path::Path,
    addon_name: &str,
    lua_source: &str,
) {
    let addon_dir = root.join(addon_name);
    std::fs::create_dir_all(&addon_dir).unwrap();
    std::fs::write(
        addon_dir.join(format!("{addon_name}.toc")),
        format!("## Title: {addon_name}\n{addon_name}.lua\n"),
    )
    .unwrap();
    std::fs::write(addon_dir.join(format!("{addon_name}.lua")), lua_source).unwrap();
}

fn write_runtime_event_warning_addon_fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    write_runtime_event_warning_addon(
        dir.path(),
        "OuterEventLoader",
        r#"local frame = CreateFrame("Frame")
frame:RegisterEvent("ADDON_LOADED")
frame:SetScript("OnEvent", function(_, _, addonName)
    if addonName == "OuterEventLoader" then
        local loaded, reason = C_AddOns.LoadAddOn("RuntimeParent")
        assert(loaded, tostring(reason))
    end
end)
"#,
    );
    write_runtime_event_warning_addon(
        dir.path(),
        "RuntimeParent",
        r#"local _ = RuntimeParentMissingGlobal
local _ = C_Container.RuntimeParentMissingMethod
local frame = CreateFrame("Frame")
frame:RegisterEvent("ADDON_LOADED")
frame:SetScript("OnEvent", function(_, _, addonName)
    if addonName == "RuntimeParent" then
        local loaded, reason = C_AddOns.LoadAddOn("RuntimeChild")
        assert(loaded, tostring(reason))
    end
end)
"#,
    );
    write_runtime_event_warning_addon(
        dir.path(),
        "RuntimeChild",
        r#"local _ = RuntimeChildMissingGlobal
local _ = C_Container.RuntimeChildMissingMethod
"#,
    );
    dir
}

fn create_nested_publication_addons() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let outer_dir = dir.path().join("OuterConsumer");
    let nested_dir = dir.path().join("NestedPublisher");
    std::fs::create_dir_all(&outer_dir).unwrap();
    std::fs::create_dir_all(&nested_dir).unwrap();

    let mut outer_toc = std::fs::File::create(outer_dir.join("OuterConsumer.toc")).unwrap();
    writeln!(outer_toc, "## Title: OuterConsumer").unwrap();
    writeln!(outer_toc, "OuterConsumer.lua").unwrap();
    let mut outer_lua = std::fs::File::create(outer_dir.join("OuterConsumer.lua")).unwrap();
    writeln!(
        outer_lua,
        r#"local _ = NestedPublishedGlobal
local loaded, reason = C_AddOns.LoadAddOn("NestedPublisher")
assert(loaded, tostring(reason))
local recordPublication = rawget(_G, "__wow_record_public_global_publication")
if type(recordPublication) == "function" then
    recordPublication("NestedPublishedGlobal")
end
"#
    )
    .unwrap();

    let mut nested_toc = std::fs::File::create(nested_dir.join("NestedPublisher.toc")).unwrap();
    writeln!(nested_toc, "## Title: NestedPublisher").unwrap();
    writeln!(nested_toc, "NestedPublisher.lua").unwrap();
    let mut nested_lua = std::fs::File::create(nested_dir.join("NestedPublisher.lua")).unwrap();
    writeln!(
        nested_lua,
        r#"local _ = C_Container.NestedMissingMethod
local _ = NestedResolvedGlobal
NestedResolvedGlobal = true
NestedPublishedGlobal = true
"#
    )
    .unwrap();

    dir
}

#[test]
fn load_addon_reports_missing_global_and_namespace_symbol_accesses() {
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_addon_with_missing_symbol_accesses();
    let toc_path = dir.path().join("TestNilSymbols.toc");

    let result = load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");

    assert!(
        result.warnings.contains(
            &"TestNilSymbols needs global MissingGlobalSymbol (accessed at TestNilSymbols.lua:1)"
                .to_string()
        ),
        "expected missing global gap warning, got {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestNilSymbols needs C_MissingNamespace (accessed at TestNilSymbols.lua:2)"
                .to_string()
        ),
        "expected missing C_* namespace gap warning, got {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestNilSymbols needs C_Container.MissingMethod (accessed at TestNilSymbols.lua:3)"
                .to_string()
        ),
        "expected missing C_* method gap warning, got {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestNilSymbols needs C_ExplicitMissingNamespace (accessed at TestNilSymbols.lua:9)"
                .to_string()
        ),
        "explicit _G C_* namespace probes must remain strict diagnostics: {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestNilSymbols needs C_Container.ExplicitMissingMethod (accessed at TestNilSymbols.lua:10)"
                .to_string()
        ),
        "explicit _G C_* member probes must remain strict diagnostics: {:?}",
        result.warnings
    );
    assert!(
        !result
            .warnings
            .iter()
            .any(|warning| warning.contains("OptionalMissingGlobal")),
        "explicit _G table probes are not statically named global requirements: {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestNilSymbols needs global DynamicThenDirectMissingGlobal (accessed at TestNilSymbols.lua:8)"
                .to_string()
        ),
        "a prior explicit _G probe must not hide a later direct-global requirement: {:?}",
        result.warnings
    );
}

#[test]
fn load_addon_omits_regular_globals_published_before_addon_completion() {
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_addon_with_late_symbol_publication();
    let toc_path = dir.path().join("TestLatePublication.toc");

    let result = load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");

    assert!(
        !result
            .warnings
            .iter()
            .any(|warning| warning.contains("PublishedByLua")),
        "late Lua publication should resolve its early nil access: {:?}",
        result.warnings
    );
    assert!(
        !result
            .warnings
            .iter()
            .any(|warning| warning.contains("PublishedByXml")),
        "named XML frame publication should resolve its early nil access: {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestLatePublication needs global StillMissingGlobal (accessed at Early.lua:3)"
                .to_string()
        ),
        "regular global still nil at addon completion must remain warned: {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestLatePublication needs C_Container.StillMissingMethod (accessed at Early.lua:4)"
                .to_string()
        ),
        "C_* method gaps must remain warned after fallback publication: {:?}",
        result.warnings
    );
}

#[test]
fn secure_publication_resolves_only_secure_same_addon_accesses() {
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_secure_addon_with_late_symbol_publication();
    let toc_path = dir.path().join("TestSecureLatePublication.toc");

    let result = load_addon(&env.loader_env(), &toc_path).expect("secure addon load should succeed");

    let (public_type, secure_type, handler_executed): (String, String, bool) = env
        .eval(
            r#"
            return type(rawget(_G, "LaterSecureFunction")),
                   type(rawget(__secureenv, "LaterSecureFunction")),
                   rawget(__secureenv, "SecureLateHandlerExecuted") == true
            "#,
        )
        .expect("secure publication state should be queryable");
    assert_eq!(public_type, "nil", "secure publication must not leak into _G");
    assert_eq!(secure_type, "function");
    assert!(handler_executed, "late secure XML function handler should execute");

    assert!(
        !result
            .warnings
            .iter()
            .any(|warning| warning.contains("LaterSecureFunction")),
        "late secure publication should resolve its secure-origin nil access: {:?}",
        result.warnings
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|warning| warning.contains("StillMissingSecureFunction")),
        "a genuinely missing secure function must remain diagnostic: {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"TestSecureLatePublication needs global SecurePublicationMustNotResolvePublic (accessed at PublicMiss.lua:1)"
                .to_string()
        ),
        "a secure publication must not resolve a public-origin miss: {:?}",
        result.warnings
    );
    let state = env.state().borrow();
    assert!(
        state.global_publications.is_empty(),
        "completed addon loads must clear public publication records"
    );
    assert!(
        state.secure_global_publications.is_empty(),
        "completed addon loads must clear secure publication records"
    );
}

#[test]
fn publication_guard_cleared_global_remains_warned() {
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_addon_with_cleared_publication();
    let toc_path = dir.path().join("TestClearedPublication.toc");

    let result = load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");

    assert!(
        result.warnings.contains(
            &"TestClearedPublication needs global PublishedThenCleared (accessed at TestClearedPublication.lua:1)"
                .to_string()
        ),
        "global cleared before addon completion must remain warned: {:?}",
        result.warnings
    );
}

#[test]
fn runtime_addon_event_warnings_are_finalized_once_with_their_owners() {
    let env = WowLuaEnv::new().unwrap();
    let dir = write_runtime_event_warning_addon_fixture();
    env.state().borrow_mut().addon_base_paths = vec![dir.path().to_path_buf()];
    let toc_path = dir.path().join("OuterEventLoader/OuterEventLoader.toc");

    let result = load_addon(&env.loader_env(), &toc_path).expect("outer addon load should succeed");
    assert!(
        result.warnings.is_empty(),
        "runtime warnings must not appear before the outer ADDON_LOADED event: {:?}",
        result.warnings
    );

    env.fire_event_with_args("ADDON_LOADED", &[env.lua_string("OuterEventLoader")])
        .expect("outer ADDON_LOADED event should load the runtime addon chain");

    let warnings = env.drain_runtime_addon_warnings();
    let expected = [
        "RuntimeParent needs global RuntimeParentMissingGlobal (accessed at RuntimeParent.lua:1)",
        "RuntimeParent needs C_Container.RuntimeParentMissingMethod (accessed at RuntimeParent.lua:2)",
        "RuntimeChild needs global RuntimeChildMissingGlobal (accessed at RuntimeChild.lua:1)",
        "RuntimeChild needs C_Container.RuntimeChildMissingMethod (accessed at RuntimeChild.lua:2)",
    ];
    for expected_warning in expected {
        assert_eq!(
            warnings
                .iter()
                .filter(|warning| warning.as_str() == expected_warning)
                .count(),
            1,
            "finalized runtime warning should retain its owner and appear exactly once: {expected_warning}; got {warnings:?}"
        );
    }
    assert_eq!(
        warnings.len(),
        expected.len(),
        "runtime warning drain should contain only the four expected warnings: {warnings:?}"
    );
    assert!(
        env.drain_runtime_addon_warnings().is_empty(),
        "runtime warning drain should consume finalized warnings"
    );
}

#[test]
fn publication_guard_nested_addon_does_not_resolve_outer_warning() {
    let env = WowLuaEnv::new().unwrap();
    let dir = create_nested_publication_addons();
    env.state().borrow_mut().addon_base_paths = vec![dir.path().to_path_buf()];
    let toc_path = dir.path().join("OuterConsumer/OuterConsumer.toc");

    let result = load_addon(&env.loader_env(), &toc_path).expect("outer addon load should succeed");
    let nested_global: bool = env
        .eval("return NestedPublishedGlobal == true")
        .expect("nested publication should be readable");

    assert!(nested_global, "nested addon should publish its global");
    let nested_method_warning =
        "NestedPublisher needs C_Container.NestedMissingMethod (accessed at NestedPublisher.lua:1)";
    assert_eq!(
        result
            .warnings
            .iter()
            .filter(|warning| warning.as_str() == nested_method_warning)
            .count(),
        1,
        "nested addon warning should propagate exactly once: {:?}",
        result.warnings
    );
    assert!(
        !result
            .warnings
            .iter()
            .any(|warning| warning.contains("NestedResolvedGlobal")),
        "nested addon's resolved global should stay reconciled: {:?}",
        result.warnings
    );
    assert!(
        result.warnings.contains(
            &"OuterConsumer needs global NestedPublishedGlobal (accessed at OuterConsumer.lua:1)"
                .to_string()
        ),
        "nested addon publication must not resolve the outer addon's warning: {:?}",
        result.warnings
    );
}
