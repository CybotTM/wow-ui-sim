use wow_ui_sim::lua_api::{AddonInfo, WowLuaEnv};

fn env_with_dep_addons() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    {
        let mut state = env.state().borrow_mut();
        state.addons.push(AddonInfo {
            folder_name: "ParentAddon".into(),
            title: "Parent".into(),
            enabled: true,
            ..Default::default()
        });
        state.addons.push(AddonInfo {
            folder_name: "ChildAddon".into(),
            title: "Child".into(),
            enabled: true,
            dependencies: vec!["ParentAddon".into()],
            ..Default::default()
        });
        state.addons.push(AddonInfo {
            folder_name: "Sibling".into(),
            title: "Sibling".into(),
            enabled: true,
            ..Default::default()
        });
        state.addons.push(AddonInfo {
            folder_name: "MultiDepAddon".into(),
            title: "MultiDep".into(),
            enabled: true,
            dependencies: vec!["ParentAddon".into(), "ChildAddon".into(), "Sibling".into()],
            ..Default::default()
        });
    }
    env
}

#[test]
fn test_get_addon_dependencies_empty_when_none_declared() {
    let env = env_with_dep_addons();
    let count: i32 = env
        .eval("return select('#', C_AddOns.GetAddOnDependencies('ParentAddon'))")
        .unwrap();
    assert_eq!(count, 0, "addon with no deps should return zero values");
}

#[test]
fn test_get_addon_dependencies_returns_single_string_value() {
    let env = env_with_dep_addons();
    let (count, first): (i32, String) = env
        .eval(
            r#"
            local count = select('#', C_AddOns.GetAddOnDependencies('ChildAddon'))
            local first = (C_AddOns.GetAddOnDependencies('ChildAddon'))
            return count, first
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(first, "ParentAddon");
}

#[test]
fn test_get_addon_dependencies_returns_variadic_not_table() {
    let env = env_with_dep_addons();
    let is_string: bool = env
        .eval("return type((C_AddOns.GetAddOnDependencies('ChildAddon'))) == 'string'")
        .unwrap();
    assert!(
        is_string,
        "first return must be a plain string, never a table"
    );
}

#[test]
fn test_get_addon_dependencies_multiple_returns_in_order() {
    let env = env_with_dep_addons();
    let (count, a, b, c): (i32, String, String, String) = env
        .eval(
            r#"
            return select('#', C_AddOns.GetAddOnDependencies('MultiDepAddon')),
                   select(1, C_AddOns.GetAddOnDependencies('MultiDepAddon')),
                   select(2, C_AddOns.GetAddOnDependencies('MultiDepAddon')),
                   select(3, C_AddOns.GetAddOnDependencies('MultiDepAddon'))
            "#,
        )
        .unwrap();
    assert_eq!(count, 3);
    assert_eq!(a, "ParentAddon");
    assert_eq!(b, "ChildAddon");
    assert_eq!(c, "Sibling");
}

#[test]
fn test_get_addon_dependencies_by_index() {
    let env = env_with_dep_addons();
    let (count, first): (i32, String) = env
        .eval(
            r#"
            local idx
            for i = 1, C_AddOns.GetNumAddOns() do
                if C_AddOns.GetAddOnName(i) == "MultiDepAddon" then idx = i end
            end
            return select('#', C_AddOns.GetAddOnDependencies(idx)),
                   (C_AddOns.GetAddOnDependencies(idx))
            "#,
        )
        .unwrap();
    assert_eq!(count, 3);
    assert_eq!(first, "ParentAddon");
}

#[test]
fn test_get_addon_dependencies_unknown_addon_returns_empty() {
    let env = env_with_dep_addons();
    let count: i32 = env
        .eval("return select('#', C_AddOns.GetAddOnDependencies('NoSuchAddon'))")
        .unwrap();
    assert_eq!(count, 0);

    let count_bad_index: i32 = env
        .eval("return select('#', C_AddOns.GetAddOnDependencies(9999))")
        .unwrap();
    assert_eq!(count_bad_index, 0);
}

#[test]
fn test_get_addon_dependencies_flows_into_variadic_callee() {
    let env = env_with_dep_addons();
    let joined = join_multi_dep_addon_dependencies(&env);
    assert_eq!(joined, "ParentAddon,ChildAddon,Sibling");
}

#[test]
fn test_get_addon_dependencies_walks_all_loaded_state() {
    let env = env_with_dep_addons();
    mark_foundation_dep_addons_loaded(&env);
    assert!(multi_dep_addon_dependencies_loaded(&env));

    mark_child_addon_unloaded(&env);
    assert!(
        !multi_dep_addon_dependencies_loaded(&env),
        "marking ChildAddon unloaded should make the walk return false"
    );
}

fn join_multi_dep_addon_dependencies(env: &WowLuaEnv) -> String {
    env.eval(
        r#"
        local function joinAll(...)
            return table.concat({...}, ",")
        end
        return joinAll(C_AddOns.GetAddOnDependencies('MultiDepAddon'))
        "#,
    )
    .unwrap()
}

fn mark_foundation_dep_addons_loaded(env: &WowLuaEnv) {
    let mut sim = env.state().borrow_mut();
    for addon in sim.addons.iter_mut() {
        if matches!(
            addon.folder_name.as_str(),
            "ParentAddon" | "ChildAddon" | "Sibling"
        ) {
            addon.loaded = true;
        }
    }
}

fn mark_child_addon_unloaded(env: &WowLuaEnv) {
    let mut sim = env.state().borrow_mut();
    if let Some(addon) = sim
        .addons
        .iter_mut()
        .find(|addon| addon.folder_name == "ChildAddon")
    {
        addon.loaded = false;
    }
}

fn multi_dep_addon_dependencies_loaded(env: &WowLuaEnv) -> bool {
    env.eval(
        r#"
        local deps = { C_AddOns.GetAddOnDependencies('MultiDepAddon') }
        for _, dep in ipairs(deps) do
            if not C_AddOns.IsAddOnLoaded(dep) then return false end
        end
        return #deps > 0
        "#,
    )
    .unwrap()
}
