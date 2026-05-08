use super::env;

#[test]
fn test_geterrorhandler_returns_function() {
    let env = env();
    let is_func: bool = env
        .eval("return type(geterrorhandler()) == 'function'")
        .unwrap();
    assert!(is_func);
}

#[test]
fn test_seterrorhandler_accepts_function() {
    let env = env();
    env.eval::<()>("seterrorhandler(function() end)").unwrap();
}

#[test]
fn test_get_current_environment() {
    let env = env();
    let is_global: bool = env.eval("return GetCurrentEnvironment() == _G").unwrap();
    assert!(is_global);
}
