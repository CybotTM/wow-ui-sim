//! Integration tests for the global `assertsafe(cond, message)`.
//!
//! `assertsafe` is Blizzard's non-throwing assertion. The simulator must
//! route the failure message through the same `lua_error_records` pipeline
//! that captures raised Lua errors, but it must NOT raise — successful
//! callers should see `cond` returned, and code after the call should keep
//! running.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn last_assertsafe_record(env: &WowLuaEnv) -> String {
    let state = env.state().borrow();
    let record = state
        .lua_error_records
        .iter()
        .rev()
        .find(|record| record.message.contains("assertsafe:"))
        .expect("expected at least one assertsafe-tagged error record");
    record.message.clone()
}

fn count_assertsafe_records(env: &WowLuaEnv) -> usize {
    env.state()
        .borrow()
        .lua_error_records
        .iter()
        .filter(|record| record.message.contains("assertsafe:"))
        .count()
}

#[test]
fn assertsafe_global_is_a_function() {
    let env = env();
    let kind: String = env.eval("return type(assertsafe)").unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn assertsafe_truthy_cond_is_a_no_op() {
    let env = env();
    env.exec(r#"assertsafe(true, "should not appear")"#)
        .unwrap();
    env.exec(r#"assertsafe(1, "should not appear")"#).unwrap();
    env.exec(r#"assertsafe("hi", "should not appear")"#)
        .unwrap();
    env.exec(r#"assertsafe({}, "should not appear")"#).unwrap();

    assert_eq!(
        count_assertsafe_records(&env),
        0,
        "truthy cond should never push to lua_error_records"
    );
}

#[test]
fn assertsafe_falsy_cond_records_message() {
    let env = env();
    env.exec(r#"assertsafe(false, "Invalid addon performance msg.")"#)
        .unwrap();

    let recorded = last_assertsafe_record(&env);
    assert!(
        recorded.starts_with("assertsafe: Invalid addon performance msg."),
        "expected assertsafe-prefixed message, got {recorded:?}"
    );
}

#[test]
fn assertsafe_nil_cond_records_message() {
    let env = env();
    env.exec(r#"assertsafe(nil, "nil branch")"#).unwrap();

    let recorded = last_assertsafe_record(&env);
    assert!(
        recorded.starts_with("assertsafe: nil branch"),
        "expected nil-cond branch to log the message, got {recorded:?}"
    );
}

#[test]
fn assertsafe_does_not_raise_so_caller_keeps_running() {
    let env = env();
    env.exec(
        r#"
        _G.__post_assertsafe = false
        assertsafe(false, "do not raise")
        _G.__post_assertsafe = true
        "#,
    )
    .unwrap();

    let ran: bool = env.eval("return _G.__post_assertsafe").unwrap();
    assert!(
        ran,
        "code after assertsafe should keep running when cond is falsy"
    );
}

#[test]
fn assertsafe_returns_the_input_cond() {
    let env = env();
    let truthy: bool = env.eval(r#"return assertsafe(true, "x") == true"#).unwrap();
    assert!(truthy, "assertsafe should return its cond input on success");

    let falsy: bool = env
        .eval(r#"return assertsafe(false, "x") == false"#)
        .unwrap();
    assert!(falsy, "assertsafe should return its cond input on failure");
}

#[test]
fn assertsafe_missing_message_uses_default_text() {
    let env = env();
    env.exec("assertsafe(false)").unwrap();

    let recorded = last_assertsafe_record(&env);
    assert!(
        recorded.starts_with("assertsafe: non-fatal assertion failed"),
        "expected default fallback text, got {recorded:?}"
    );
}

#[test]
fn assertsafe_function_message_is_invoked() {
    let env = env();
    env.exec(
        r#"
        assertsafe(false, function()
            return "lazy " .. "message"
        end)
        "#,
    )
    .unwrap();

    let recorded = last_assertsafe_record(&env);
    assert!(
        recorded.starts_with("assertsafe: lazy message"),
        "expected function-returned message, got {recorded:?}"
    );
}

#[test]
fn assertsafe_routes_to_active_error_handler() {
    let env = env();
    env.exec(
        r#"
        _G.__captured = nil
        seterrorhandler(function(msg)
            _G.__captured = msg
        end)
        assertsafe(false, "handler reaches me")
        "#,
    )
    .unwrap();

    let captured: String = env.eval("return _G.__captured").unwrap();
    assert!(
        captured.starts_with("assertsafe: handler reaches me"),
        "expected error handler to receive the prefixed message, got {captured:?}"
    );
}

#[test]
fn assertsafe_records_one_entry_per_call() {
    let env = env();
    env.exec(
        r#"
        assertsafe(false, "first")
        assertsafe(false, "second")
        assertsafe(false, "third")
        "#,
    )
    .unwrap();

    let messages: Vec<String> = env
        .state()
        .borrow()
        .lua_error_records
        .iter()
        .filter(|record| record.message.contains("assertsafe:"))
        .map(|record| record.message.clone())
        .collect();

    assert_eq!(messages.len(), 3);
    assert!(messages[0].starts_with("assertsafe: first"));
    assert!(messages[1].starts_with("assertsafe: second"));
    assert!(messages[2].starts_with("assertsafe: third"));
}
