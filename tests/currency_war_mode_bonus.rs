//! Explicit War Mode currency flags; no native/catalog eligibility inference.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

const QUERY_ASSERTIONS: &str = r#"
    local get = C_CurrencyInfo.DoesWarModeBonusApply
    local function absent(...)
        assert(select('#', ...) == 2, 'currency bonus query returns two values')
        local applies, once = ...
        assert(applies == nil and once == nil, 'unknown modeled currency has nil flags')
    end
    local function rejected(...)
        local ok, message = pcall(get, ...)
        assert(not ok, 'required nonnumeric currency input must be rejected')
        assert(type(message) == 'string' and string.find(message, 'number', 1, true),
            'numeric input rejection has type context: ' .. tostring(message))
    end
"#;

fn query_probe(body: &str) {
    let env = WowLuaEnv::new().expect("currency query environment");
    supplied_probe(&env, body);
}

fn supplied_probe(env: &WowLuaEnv, body: &str) {
    env.exec(&format!("{QUERY_ASSERTIONS}\n{FLAG_ASSERTIONS}\n{body}"))
        .expect("public currency War Mode query behavior");
}

#[test]
fn currency_war_mode_bonus_absent_records_return_two_nil_flags() {
    query_probe(
        r#"
        absent(get(90001))
        absent(get(90002))
        absent(get(90001))
        "#,
    );
}

#[test]
fn currency_war_mode_bonus_requires_number_without_string_coercion() {
    query_probe(
        r#"
        rejected()
        rejected(nil)
        for _, value in ipairs({'90001', '', false, true, {}, function() end}) do
            rejected(value)
        end
        absent(get(90001))
        "#,
    );
}

#[test]
fn currency_war_mode_bonus_unrepresented_numbers_have_no_flags() {
    query_probe(
        r#"
        -- Positive state controls must separately prove identity non-aliasing.
        for _, value in ipairs({0, -1, 90001.5, 0/0, math.huge,
            -math.huge, 1e308, -1e308}) do
            absent(get(value))
        end
        "#,
    );
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn currency_war_mode_bonus_untainted_secrets_preserve_wrapper_and_caller() {
    query_probe(
        r#"
        debug.settaintmode(true)
        local first, second = secretwrap(90001), secretwrap(90002)
        assert(debug.getstacktaint() == nil, 'untainted entry')
        absent(get(first))
        absent(get(second))
        assert(issecretvalue(first) and issecretvalue(second), 'wrappers preserved')
        assert(secretunwrap(first) == 90001 and secretunwrap(second) == 90002,
            'wrapped values preserved')
        assert(debug.getstacktaint() == nil, 'caller context preserved')
        "#,
    );
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn currency_war_mode_bonus_tainted_secrets_denied_before_validation() {
    query_probe(
        r#"
        debug.settaintmode(true)
        local valid, invalid = secretwrap(90001), secretwrap('not-a-number')
        local function addon()
            assert(debug.getstacktaint() == 'CurrencyWarModeProbe', 'tainted entry')
            for _, value in ipairs({valid, invalid}) do
                local ok, message = pcall(get, value)
                assert(not ok and type(message) == 'string', 'secret denied')
                assert(string.find(message, 'untainted caller', 1, true),
                    'authentication precedes type/lookup: ' .. tostring(message))
                assert(issecretvalue(value), 'denial preserves wrapper')
                assert(not pcall(secretunwrap, value), 'no declassification')
                assert(debug.getstacktaint() == 'CurrencyWarModeProbe', 'taint preserved')
            end
            absent(get(90001))
        end
        debug.setobjecttaint(addon, 'CurrencyWarModeProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'caller restored')
        absent(get(valid))
        "#,
    );
}

const FLAG_ASSERTIONS: &str = r#"
    local function flags(expectedApplies, expectedOnce, ...)
        assert(select('#', ...) == 2, 'supplied query returns exactly two values')
        local applies, once = ...
        assert(applies == expectedApplies, 'supplied applies flag preserved')
        assert(once == expectedOnce, 'supplied once flag preserved')
    end
"#;

#[test]
fn currency_war_mode_bonus_supplied_independent_nullable_combinations() {
    let env = WowLuaEnv::new().expect("supplied currency environment");
    assert!(env.state().borrow().currency_war_mode_bonus.is_empty());
    let nullable_flags = [None, Some(false), Some(true)];
    for (first_index, applies) in nullable_flags.into_iter().enumerate() {
        for (second_index, once) in nullable_flags.into_iter().enumerate() {
            let id = 90001 + (first_index * 3 + second_index) as i32;
            env.state()
                .borrow_mut()
                .currency_war_mode_bonus
                .insert(id, (applies, once));
        }
    }
    supplied_probe(
        &env,
        r#"
        flags(nil, nil, get(90001))
        flags(nil, false, get(90002))
        flags(nil, true, get(90003))
        flags(false, nil, get(90004))
        flags(false, false, get(90005))
        flags(false, true, get(90006))
        flags(true, nil, get(90007))
        flags(true, false, get(90008))
        flags(true, true, get(90009))
        absent(get(90010))
    "#,
    );
}

#[test]
fn currency_war_mode_bonus_owned_updates_removal_and_currency_isolation() {
    let env = WowLuaEnv::new().expect("supplied currency environment");
    env.state().borrow_mut().currency_war_mode_bonus.extend([
        (90001, (Some(true), Some(false))),
        (90002, (Some(false), Some(true))),
    ]);
    supplied_probe(
        &env,
        r#"
        originalApplies, originalOnce = get(90001)
        flags(true, false, originalApplies, originalOnce)
        flags(false, true, get(90002))
    "#,
    );
    env.state()
        .borrow_mut()
        .currency_war_mode_bonus
        .insert(90001, (None, Some(true)));
    supplied_probe(
        &env,
        r#"
        flags(nil, true, get(90001))
        flags(true, false, originalApplies, originalOnce)
        flags(false, true, get(90002))
    "#,
    );
    env.state()
        .borrow_mut()
        .currency_war_mode_bonus
        .remove(&90001);
    supplied_probe(
        &env,
        r#"
        absent(get(90001))
        flags(true, false, originalApplies, originalOnce)
        flags(false, true, get(90002))
    "#,
    );
}

#[test]
fn currency_war_mode_bonus_positive_identity_does_not_alias_numeric_edges() {
    let env = WowLuaEnv::new().expect("supplied currency environment");
    env.state().borrow_mut().currency_war_mode_bonus.extend([
        (90001, (Some(true), Some(false))),
        (i32::MIN, (Some(false), Some(true))),
        (i32::MAX, (Some(true), Some(true))),
    ]);
    supplied_probe(
        &env,
        r#"
        flags(true, false, get(90001))
        flags(false, true, get(-2147483648))
        flags(true, true, get(2147483647))
        for _, value in ipairs({90001.5, 90000.5, 2147483647.5,
            -2147483648.5, 2147483648, -2147483649, 4295057297,
            0/0, math.huge, -math.huge, 1e308, -1e308}) do
            absent(get(value))
        end
        rejected('90001')
        flags(true, false, get(90001))
    "#,
    );
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn currency_war_mode_bonus_supplied_secret_identity_and_invalid_type() {
    let env = WowLuaEnv::new().expect("supplied currency environment");
    env.state()
        .borrow_mut()
        .currency_war_mode_bonus
        .insert(90001, (Some(true), Some(false)));
    supplied_probe(
        &env,
        r#"
        debug.settaintmode(true)
        local id, fractional = secretwrap(90001), secretwrap(90001.5)
        flags(true, false, get(id))
        absent(get(fractional))
        rejected(secretwrap('90001'))
        assert(issecretvalue(id) and issecretvalue(fractional), 'wrappers remain secret')
        assert(secretunwrap(id) == 90001 and secretunwrap(fractional) == 90001.5,
            'wrapped identity unchanged')
        assert(debug.getstacktaint() == nil, 'untainted caller preserved')
    "#,
    );
}
