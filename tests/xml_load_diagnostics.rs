//! Wrath 12340 file diagnostics; Retail assertions preserve simulator policy only.

use wow_ui_sim::{loader::load_addon, lua_api::WowLuaEnv};

const ADDON: &str = "XmlLoadDiagnosticProbe";

fn write_addon(root: &std::path::Path, xml: &str) -> std::path::PathBuf {
    let addon = root.join(ADDON);
    std::fs::create_dir_all(addon.join("Sub")).unwrap();
    std::fs::write(
        addon.join(format!("{ADDON}.toc")),
        "## Interface: 30300\nRoot.xml\nTail.lua\n",
    )
    .unwrap();
    std::fs::write(addon.join("Root.xml"), xml).unwrap();
    std::fs::write(addon.join("Tail.lua"), "DiagTocTail = true").unwrap();
    addon
}

fn observe_errors(env: &WowLuaEnv) {
    env.exec("DiagErrors = {}; seterrorhandler(function(message) DiagErrors[#DiagErrors + 1] = message end)")
        .unwrap();
}

#[cfg(feature = "client-wrath")]
fn assert_no_lua_errors(env: &WowLuaEnv) {
    let handler_count: i32 = env.eval("return #DiagErrors").unwrap();
    let state = env.state().borrow();
    assert_eq!(
        handler_count, 0,
        "file diagnostics reached Lua handler: {:?}",
        state.lua_errors
    );
    assert!(state.lua_errors.is_empty(), "{:?}", state.lua_errors);
    assert!(state.lua_error_records.is_empty());
    assert!(state.lua_error_counts.is_empty());
}

#[cfg(feature = "client-wrath")]
#[test]
fn wrath_xml_diagnostics_retain_each_failed_child_without_lua_errors() {
    let env = WowLuaEnv::new().unwrap();
    observe_errors(&env);
    let root = tempfile::tempdir().unwrap();
    let addon = write_addon(
        root.path(),
        r#"<Ui>
        <Include file="Sub/Branch.xml"/>
        <Frame name="DiagParentSurvives"/>
        <Script>DiagParentTail = true</Script>
    </Ui>"#,
    );
    std::fs::write(
        addon.join("Sub/Branch.xml"),
        r#"<Ui>
        <Include file="../Missing.xml"/>
        <include file="../Malformed.xml"/>
        <Include file="../Missing.lua"/>
        <Script file="../External.lua">DiagInlineSurvives = true</Script>
        <Include file="../Missing.xml"/>
        <Script>DiagBranchTail = true</Script>
    </Ui>"#,
    )
    .unwrap();
    std::fs::write(
        addon.join("Malformed.xml"),
        r#"<Ui><Frame name="DiagMustNotExist"/><Frame></Ui>"#,
    )
    .unwrap();
    let result = load_addon(&env.loader_env(), &addon.join(format!("{ADDON}.toc"))).unwrap();
    let expected = [
        "Couldn't open Interface\\AddOns\\XmlLoadDiagnosticProbe\\Missing.xml",
        "Couldn't parse XML in Interface\\AddOns\\XmlLoadDiagnosticProbe\\Malformed.xml",
        "Error loading Interface\\AddOns\\XmlLoadDiagnosticProbe\\Missing.lua",
        "Error loading Interface\\AddOns\\XmlLoadDiagnosticProbe\\External.lua",
        "Couldn't open Interface\\AddOns\\XmlLoadDiagnosticProbe\\Missing.xml",
    ];
    assert_eq!(
        result.warnings, expected,
        "retain failures once per attempted file, not propagated parent"
    );
    assert_eq!(result.diagnostics().warnings, expected);
    assert_no_lua_errors(&env);
    env.exec("assert(DiagInlineSurvives and DiagBranchTail and DiagParentTail and DiagTocTail and DiagParentSurvives); assert(DiagMustNotExist == nil)").unwrap();
}

#[cfg(feature = "client-wrath")]
#[test]
fn wrath_xml_diagnostics_original_debugtools_consumer_stays_hidden() {
    use std::{cell::RefCell, rc::Rc};
    use wow_ui_sim::render::font::WowFontSystem;
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(Rc::new(RefCell::new(WowFontSystem::new_without_casc())));
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc()
        .expect("pinned Wrath cache required");
    load_addon(&env.loader_env(), &toc).unwrap();
    let debugtools = wow_ui_sim::client_profile::blizzard_ui_addons_dir()
        .join("Blizzard_DebugTools/Blizzard_DebugTools.toc");
    load_addon(&env.loader_env(), &debugtools).unwrap();
    env.exec(r#"
        assert(ScriptErrorsFrame and ScriptErrorsFrame_OnError, 'original DebugTools must be loaded')
        ScriptErrorsFrame:Hide()
        DiagOriginalCount = #ScriptErrorsFrame.order
        seterrorhandler(ScriptErrorsFrame_OnError)
    "#).unwrap();
    let before = env.state().borrow().lua_errors.len();
    let root = tempfile::tempdir().unwrap();
    let addon = write_addon(
        root.path(),
        r#"<Ui><Include file="MissingOriginalConsumer.xml"/><Script>DiagParentTail = true</Script></Ui>"#,
    );
    let result = load_addon(&env.loader_env(), &addon.join(format!("{ADDON}.toc"))).unwrap();
    env.exec("assert(#ScriptErrorsFrame.order == DiagOriginalCount, 'file diagnostic polluted original ScriptErrorsFrame'); assert(not ScriptErrorsFrame:IsShown(), 'file diagnostic opened original error panel'); assert(DiagParentTail and DiagTocTail)").unwrap();
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        before,
        "XML failure polluted lua-errors"
    );
    assert_eq!(
        result.warnings,
        ["Couldn't open Interface\\AddOns\\XmlLoadDiagnosticProbe\\MissingOriginalConsumer.xml"]
    );
    // Positive control: unchanged original consumer must still receive genuine Lua failures.
    let positive = root.path().join("XmlLoadPositiveProbe");
    std::fs::create_dir(&positive).unwrap();
    std::fs::write(positive.join("XmlLoadPositiveProbe.toc"), "Positive.xml\n").unwrap();
    std::fs::write(
        positive.join("Positive.xml"),
        "<Ui><Script>error('diag-genuine-inline-error')</Script></Ui>",
    )
    .unwrap();
    load_addon(
        &env.loader_env(),
        &positive.join("XmlLoadPositiveProbe.toc"),
    )
    .unwrap();
    env.exec(
        "assert(#ScriptErrorsFrame.order > DiagOriginalCount); assert(ScriptErrorsFrame:IsShown())",
    )
    .unwrap();
    assert!(
        env.state()
            .borrow()
            .lua_errors
            .iter()
            .any(|error| error.contains("diag-genuine-inline-error"))
    );
}

#[cfg(feature = "client-wrath")]
#[test]
fn wrath_xml_diagnostics_top_level_missing_malformed_and_unreadable_continue_toc() {
    let env = WowLuaEnv::new().unwrap();
    observe_errors(&env);
    let root = tempfile::tempdir().unwrap();
    let addon = write_addon(
        root.path(),
        r#"<Ui><Include file="Unreadable.xml"/><Script>DiagParentTail = true</Script></Ui>"#,
    );
    std::fs::create_dir(addon.join("Unreadable.xml")).unwrap();
    std::fs::write(addon.join("Malformed.xml"), "<Ui><Frame></Ui>").unwrap();
    std::fs::write(
        addon.join(format!("{ADDON}.toc")),
        "Missing.xml\nMalformed.xml\nRoot.xml\nTail.lua\n",
    )
    .unwrap();
    let result = load_addon(&env.loader_env(), &addon.join(format!("{ADDON}.toc"))).unwrap();
    assert_eq!(
        result.warnings,
        [
            "Couldn't open Interface\\AddOns\\XmlLoadDiagnosticProbe\\Missing.xml",
            "Couldn't parse XML in Interface\\AddOns\\XmlLoadDiagnosticProbe\\Malformed.xml",
            "Couldn't open Interface\\AddOns\\XmlLoadDiagnosticProbe\\Unreadable.xml",
        ]
    );
    assert_no_lua_errors(&env);
    env.exec("assert(DiagParentTail and DiagTocTail)").unwrap();
}

#[cfg(feature = "client-wrath")]
#[test]
fn wrath_xml_diagnostics_runtime_loadaddon_succeeds_and_retains_attribution() {
    let env = WowLuaEnv::new().unwrap();
    observe_errors(&env);
    let root = tempfile::tempdir().unwrap();
    write_addon(root.path(), r#"<Ui><Include file="Missing.xml"/></Ui>"#);
    env.state().borrow_mut().addon_base_paths = vec![root.path().to_path_buf()];
    env.exec(
        r#"
        DiagLoadedEvents = 0
        local frame = CreateFrame('Frame')
        frame:RegisterEvent('ADDON_LOADED')
        frame:SetScript('OnEvent', function(_, _, name)
            if name == 'XmlLoadDiagnosticProbe' then DiagLoadedEvents = DiagLoadedEvents + 1 end
        end)
        local loaded, reason = LoadAddOn('XmlLoadDiagnosticProbe')
        assert(loaded and reason == nil)
        assert(IsAddOnLoaded('XmlLoadDiagnosticProbe'))
        assert(DiagLoadedEvents == 1 and DiagTocTail)
    "#,
    )
    .unwrap();
    assert_no_lua_errors(&env);
    let diagnostics = env.drain_runtime_addon_diagnostics();
    assert_eq!(
        diagnostics.warnings,
        ["Couldn't open Interface\\AddOns\\XmlLoadDiagnosticProbe\\Missing.xml"]
    );
    assert!(env.drain_runtime_addon_diagnostics().is_empty());
}

#[cfg(feature = "client-wrath")]
#[test]
fn wrath_xml_diagnostics_preserve_lua_compile_runtime_and_onload_errors() {
    let env = WowLuaEnv::new().unwrap();
    observe_errors(&env);
    let root = tempfile::tempdir().unwrap();
    let addon = write_addon(
        root.path(),
        r#"<Ui>
        <Include file="Syntax.lua"/>
        <Script file="Throw.lua"/>
        <Script>DiagParentTail = true</Script>
        <Frame name="DiagOnLoad"><Scripts><OnLoad>error('diag-onload')</OnLoad></Scripts></Frame>
    </Ui>"#,
    );
    std::fs::write(addon.join("Syntax.lua"), "local =").unwrap();
    std::fs::write(addon.join("Throw.lua"), "error('diag-external-runtime')").unwrap();
    load_addon(&env.loader_env(), &addon.join(format!("{ADDON}.toc"))).unwrap();
    let captured: String = env.eval(r"return table.concat(DiagErrors, '\n')").unwrap();
    for marker in ["Syntax.lua", "diag-external-runtime", "diag-onload"] {
        assert!(
            captured.contains(marker),
            "handler lost {marker}: {captured}"
        );
        assert!(
            env.state()
                .borrow()
                .lua_errors
                .iter()
                .any(|error| error.contains(marker))
        );
    }
    env.exec("assert(DiagParentTail and DiagTocTail)").unwrap();
}

#[cfg(feature = "client-retail")]
#[test]
fn retail_xml_diagnostics_preserve_current_handler_and_warning_routing() {
    for (filename, contents) in [
        ("Missing.xml", None),
        ("Malformed.xml", Some("<Ui><Frame></Ui>")),
        ("Missing.lua", None),
    ] {
        let env = WowLuaEnv::new().unwrap();
        observe_errors(&env);
        let root = tempfile::tempdir().unwrap();
        let addon = write_addon(
            root.path(),
            &format!(
                r#"<Ui><Script file="{filename}"/><Script>DiagParentTail = true</Script></Ui>"#
            ),
        );
        // XML failures must use Include; missing Script Lua preserves its current path.
        if filename.ends_with(".xml") {
            std::fs::write(
                addon.join("Root.xml"),
                format!(
                    r#"<Ui><Include file="{filename}"/><Script>DiagParentTail = true</Script></Ui>"#
                ),
            )
            .unwrap();
        }
        if let Some(contents) = contents {
            std::fs::write(addon.join(filename), contents).unwrap();
        }
        let result = load_addon(&env.loader_env(), &addon.join(format!("{ADDON}.toc"))).unwrap();
        let captured: String = env.eval(r"return table.concat(DiagErrors, '\n')").unwrap();
        // Current Retail parse errors omit the child path. Preserve that policy;
        // only Wrath gains the native per-file diagnostic text in this change.
        let expected_error = if filename == "Malformed.xml" {
            "Parse error:"
        } else {
            filename
        };
        assert!(captured.contains(expected_error), "{captured}");
        assert!(
            env.state()
                .borrow()
                .lua_errors
                .iter()
                .any(|error| error.contains(expected_error))
        );
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.contains(expected_error))
        );
        env.exec("assert(DiagParentTail == nil and DiagTocTail)")
            .unwrap();
    }
}
