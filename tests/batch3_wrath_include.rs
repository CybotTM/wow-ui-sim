//! Real temporary TOC/XML/Lua files exercise the loader failure boundary.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::{loader::load_addon, lua_api::WowLuaEnv};

fn nested_failure(filename: &str, contents: Option<&str>, expected: &str) {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        Batch3Order, Batch3Errors = {}, {}
        seterrorhandler(function(message) table.insert(Batch3Errors, tostring(message)) end)
    "#,
    )
    .unwrap();
    let root = tempfile::tempdir().unwrap();
    let addon = root.path().join("Batch3IncludeProbe");
    std::fs::create_dir(&addon).unwrap();
    let files = [
        (
            "Batch3IncludeProbe.toc",
            "## Interface: 30300\nRoot.xml\nTail.lua\n".to_string(),
        ),
        (
            "Root.xml",
            r#"<Ui>
            <Script>table.insert(Batch3Order, 'root-before')</Script>
            <Include file="Branch.xml"/>
            <Frame name="Batch3RootAfter" parent="UIParent"/>
            <Script>table.insert(Batch3Order, 'root-after')</Script>
        </Ui>"#
                .to_string(),
        ),
        (
            "Branch.xml",
            format!(
                r#"<Ui>
            <Script>table.insert(Batch3Order, 'branch-before')</Script>
            <Include file="{filename}"/>
            <Frame name="Batch3BranchAfter" parent="UIParent"/>
            <Script>table.insert(Batch3Order, 'branch-after')</Script>
        </Ui>"#
            ),
        ),
        (
            "Tail.lua",
            "table.insert(Batch3Order, 'toc-tail')".to_string(),
        ),
    ];
    for (name, data) in files {
        std::fs::write(addon.join(name), data).unwrap();
    }
    if let Some(data) = contents {
        std::fs::write(addon.join(filename), data).unwrap();
    }
    load_addon(&env.loader_env(), &addon.join("Batch3IncludeProbe.toc")).unwrap();
    let order: String = env.eval("return table.concat(Batch3Order, ',')").unwrap();
    assert_eq!(
        order, expected,
        "nested child failure must not abort parent siblings"
    );
    env.exec(
        r#"
        assert(Batch3BranchAfter and Batch3RootAfter, "both parent frames survive")
        assert(Batch3MalformedChild == nil, "malformed document executes no children")
    "#,
    )
    .unwrap();
    // Deliberately no error-count, inter-error ordering, or exact wording oracle.
    // Missing Lua includes currently only warn; diagnostic channel is not specified here.
}

#[test]
fn x2_nested_missing_xml_continues_branch_root_and_toc() {
    nested_failure(
        "Missing.xml",
        None,
        "root-before,branch-before,branch-after,root-after,toc-tail",
    );
}

#[test]
fn x2_nested_malformed_xml_executes_no_child_then_continues_parents() {
    nested_failure(
        "Malformed.xml",
        Some(
            r#"<Ui>
        <Frame name="Batch3MalformedChild"/>
        <Script>table.insert(Batch3Order, 'must-not-run')</Script>
        <Frame></Ui>"#,
        ),
        "root-before,branch-before,branch-after,root-after,toc-tail",
    );
}

#[test]
fn x2_nested_missing_lua_include_preserves_parent_siblings() {
    nested_failure(
        "Missing.lua",
        None,
        "root-before,branch-before,branch-after,root-after,toc-tail",
    );
}

#[test]
fn x2_nested_throwing_lua_include_preserves_prior_effects_and_continues() {
    nested_failure(
        "Throw.lua",
        Some(
            "table.insert(Batch3Order, 'child-before'); error('batch3-stop'); table.insert(Batch3Order, 'must-not-run')",
        ),
        "root-before,branch-before,child-before,branch-after,root-after,toc-tail",
    );
}

#[test]
fn x2_nested_syntax_error_lua_include_executes_nothing_then_continues() {
    nested_failure(
        "Syntax.lua",
        Some("table.insert(Batch3Order, 'must-not-run'); local ="),
        "root-before,branch-before,branch-after,root-after,toc-tail",
    );
}

#[test]
fn x2_external_script_failure_does_not_abort_later_xml_siblings() {
    let env = WowLuaEnv::new().unwrap();
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("Batch3Script.toc"), "Root.xml\n").unwrap();
    std::fs::write(root.path().join("Fail.lua"), "error('batch3-script-stop')").unwrap();
    std::fs::write(
        root.path().join("Root.xml"),
        r#"<Ui>
        <Script file="Fail.lua"/>
        <Frame name="Batch3AfterExternalScript" parent="UIParent"/>
        <Script>Batch3ExternalTail = true</Script>
    </Ui>"#,
    )
    .unwrap();
    load_addon(&env.loader_env(), &root.path().join("Batch3Script.toc")).unwrap();
    env.exec("assert(Batch3AfterExternalScript and Batch3ExternalTail, 'Script failure must not abort XML siblings')").unwrap();
}
