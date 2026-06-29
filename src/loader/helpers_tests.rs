use super::*;

#[test]
fn test_normalize_path_backslashes() {
    assert_eq!(
        normalize_path("Interface\\Buttons\\UI-Button"),
        "Interface/Buttons/UI-Button"
    );
}

#[test]
fn test_normalize_path_already_forward() {
    assert_eq!(
        normalize_path("Interface/Buttons/UI-Button"),
        "Interface/Buttons/UI-Button"
    );
}

#[test]
fn test_normalize_path_empty() {
    assert_eq!(normalize_path(""), "");
}

#[test]
fn test_resolve_path_with_interface_addons_escape() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let cache_addons = temp.path().join("AddOns");
    let addon_root = cache_addons.join("Blizzard_MapCanvas");
    let target_dir = cache_addons.join("Blizzard_MapCanvas");
    std::fs::create_dir_all(&target_dir).expect("create target dir");
    let target = target_dir.join("Blizzard_MapCanvasDetailLayer.lua");
    std::fs::write(&target, "-- detail layer").expect("write target");

    let resolved = resolve_path_with_fallback(
        &addon_root,
        &addon_root,
        r"..\..\..\Interface\AddOns\Blizzard_MapCanvas\Blizzard_MapCanvasDetailLayer.lua",
    );

    assert_eq!(resolved, target);
}

#[test]
fn test_resolve_lua_escapes_decimal() {
    // \32 = space (ASCII 32)
    assert_eq!(resolve_lua_escapes(r":\32"), ": ");
    assert_eq!(resolve_lua_escapes(r"Say:\32"), "Say: ");
}

#[test]
fn test_resolve_lua_escapes_named() {
    assert_eq!(resolve_lua_escapes(r"\n"), "\n");
    assert_eq!(resolve_lua_escapes(r"\t"), "\t");
    assert_eq!(resolve_lua_escapes(r"\\"), "\\");
}

#[test]
fn test_resolve_lua_escapes_no_escapes() {
    assert_eq!(resolve_lua_escapes("hello"), "hello");
    assert_eq!(resolve_lua_escapes(""), "");
}

#[test]
fn test_resolve_lua_escapes_combined() {
    // \37 = '%', \32 = space
    assert_eq!(resolve_lua_escapes(r"%s\32"), "%s ");
}

#[test]
fn test_escape_lua_string_backslash() {
    assert_eq!(escape_lua_string("a\\b"), "a\\\\b");
}

#[test]
fn test_escape_lua_string_quotes() {
    assert_eq!(escape_lua_string(r#"say "hello""#), r#"say \"hello\""#);
}

#[test]
fn test_escape_lua_string_newlines() {
    assert_eq!(
        escape_lua_string("line1\nline2\rline3"),
        "line1\\nline2\\rline3"
    );
}

#[test]
fn test_escape_lua_string_combined() {
    assert_eq!(escape_lua_string("a\\b\n\"c\""), "a\\\\b\\n\\\"c\\\"");
}

#[test]
fn test_resolve_child_name_with_parent() {
    let name = resolve_child_name(Some("$parentTitle"), "MyFrame", "anon_");
    assert_eq!(name, "MyFrameTitle");
}

#[test]
fn test_resolve_child_name_no_parent_placeholder() {
    let name = resolve_child_name(Some("ExplicitName"), "MyFrame", "anon_");
    assert_eq!(name, "ExplicitName");
}

#[test]
fn test_resolve_child_name_none_generates_prefix() {
    let name = resolve_child_name(None, "MyFrame", "anon_");
    assert!(
        name.starts_with("anon_"),
        "Should start with prefix, got: {}",
        name
    );
}

#[test]
fn test_resolve_relative_key_simple_name() {
    let result = resolve_relative_key("ScrollFrame", "parent");
    assert_eq!(result, "ScrollFrame");
}

#[test]
fn test_resolve_relative_key_parent() {
    let result = resolve_relative_key("$parent", "parent");
    assert_eq!(result, "parent");
}

#[test]
fn test_resolve_relative_key_parent_child() {
    let result = resolve_relative_key("$parent.ScrollFrame", "parent");
    assert_eq!(result, r#"parent["ScrollFrame"]"#);
}

#[test]
fn test_resolve_relative_key_double_parent() {
    let result = resolve_relative_key("$parent.$parent.ScrollFrame", "parent");
    assert_eq!(result, r#"parent:GetParent()["ScrollFrame"]"#);
}

#[test]
fn test_resolve_relative_key_parent_key_alias() {
    // $parentKey is treated identically to $parent
    let result = resolve_relative_key("$parentKey", "parent");
    assert_eq!(result, "parent");
}

#[test]
fn test_resolve_relative_key_parent_key_with_child() {
    let result = resolve_relative_key("$parentKey.Foo", "parent");
    assert_eq!(result, r#"parent["Foo"]"#);
}

#[test]
fn test_resolve_relative_key_capital_parent() {
    // $Parent (capital P) is treated identically to $parent
    let result = resolve_relative_key("$Parent", "parent");
    assert_eq!(result, "parent");
}

#[test]
fn test_resolve_relative_key_capital_parent_with_child() {
    let result = resolve_relative_key("$Parent.Foo", "parent");
    assert_eq!(result, r#"parent["Foo"]"#);
}

#[test]
fn test_resolve_relative_key_parent_prefix_suffix() {
    // $parentFoo -> parent["Foo"] (prefix substitution)
    let result = resolve_relative_key("$parentPanelContainer", "parent");
    assert_eq!(result, r#"parent["PanelContainer"]"#);
}

#[test]
fn test_resolve_relative_key_capital_parent_prefix_suffix() {
    // $ParentFoo -> parent["Foo"] (capital P prefix substitution)
    let result = resolve_relative_key("$ParentPanelContainer", "parent");
    assert_eq!(result, r#"parent["PanelContainer"]"#);
}

#[test]
fn test_resolve_relative_key_triple_chained_parent() {
    let result = resolve_relative_key("$parent.$parent.$parent.Bar", "parent");
    assert_eq!(result, r#"parent:GetParent():GetParent()["Bar"]"#);
}

#[test]
fn test_resolve_relative_key_empty_key() {
    // Empty key returns the key itself (no $parent marker)
    let result = resolve_relative_key("", "parent");
    assert_eq!(result, "");
}

#[test]
fn test_resolve_relative_key_just_dollar_parent_prefix_no_suffix() {
    // "$parent" as a prefix with empty suffix after strip is equivalent to bare $parent.
    let result = resolve_relative_key("$parent", "myframe");
    assert_eq!(result, "myframe");
}

#[test]
fn test_resolve_relative_key_parent_prefix_in_chained() {
    // $parent.$parentFoo -> parent:GetParent()["Foo"]
    let result = resolve_relative_key("$parent.$parentFoo", "parent");
    assert_eq!(result, r#"parent:GetParent()["Foo"]"#);
}

#[test]
fn test_get_size_values_direct() {
    let size = crate::xml::SizeXml {
        x: Some(100.0),
        y: Some(200.0),
        abs_dimension: None,
    };
    assert_eq!(get_size_values(&size), (Some(100.0), Some(200.0)));
}

#[test]
fn test_get_size_values_abs_dimension() {
    let size = crate::xml::SizeXml {
        x: None,
        y: None,
        abs_dimension: Some(crate::xml::AbsDimensionXml {
            x: Some(50.0),
            y: Some(75.0),
        }),
    };
    assert_eq!(get_size_values(&size), (Some(50.0), Some(75.0)));
}

#[test]
fn test_get_size_values_empty() {
    let size = crate::xml::SizeXml {
        x: None,
        y: None,
        abs_dimension: None,
    };
    assert_eq!(get_size_values(&size), (None, None));
}
