use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn formal_arg_parameter_in_vararg_function_is_not_shadowed() {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    let (arg, arg_index, vararg_count, first_vararg): (String, i64, i64, String) = env
        .eval(
            r##"
            local function check(arg, argIndex, ...)
                return arg, argIndex, select("#", ...), select(1, ...)
            end
            return check("value", 7, "function")
            "##,
        )
        .unwrap();

    assert_eq!(arg, "value");
    assert_eq!(arg_index, 7);
    assert_eq!(vararg_count, 1);
    assert_eq!(first_vararg, "function");
}
