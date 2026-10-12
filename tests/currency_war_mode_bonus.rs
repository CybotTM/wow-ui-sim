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
    env.exec(&format!("{QUERY_ASSERTIONS}\n{body}"))
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
