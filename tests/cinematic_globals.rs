use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn cinematic_query_globals_exist_and_return_false() {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");

    let (in_cinematic_type, in_scene_type): (String, String) = env
        .eval("return type(InCinematic), type(IsInCinematicScene)")
        .expect("type checks should not error");
    assert_eq!(in_cinematic_type, "function");
    assert_eq!(in_scene_type, "function");

    let (in_cinematic, in_scene): (bool, bool) = env
        .eval("return InCinematic(), IsInCinematicScene()")
        .expect("cinematic query globals should be callable");
    assert!(!in_cinematic, "InCinematic should default to false");
    assert!(!in_scene, "IsInCinematicScene should default to false");
}

#[test]
fn cinematic_list_namespace_exists_and_returns_empty_table() {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");

    let (namespace_type, method_type): (String, String) = env
        .eval("return type(C_CinematicList), type(C_CinematicList.GetUICinematicList)")
        .expect("type checks should not error");
    assert_eq!(namespace_type, "table");
    assert_eq!(method_type, "function");

    let count: i64 = env
        .eval(
            r#"
            local movies = C_CinematicList.GetUICinematicList()
            assert(type(movies) == "table", "GetUICinematicList must return a table")
            return #movies
            "#,
        )
        .expect("GetUICinematicList should return a Lua table");
    assert_eq!(
        count, 0,
        "GetUICinematicList should default to an empty table"
    );
}
