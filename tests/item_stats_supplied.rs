//! Supplied exact-link item-stat contract; behavioral RED not yet observed.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use std::collections::HashMap;
use wow_ui_sim::c_api::ItemStatsState;
use wow_ui_sim::lua_api::WowLuaEnv;

// Same item ID, distinct exact link payloads: item-ID parsing must not alias them.
const LINK_A: &str = "|cff0070dd|Hitem:901001::::::::80:70::1:1:1001:::::|h[Supplied A]|h|r";
const LINK_B: &str = "|cff0070dd|Hitem:901001::::::::80:70::1:1:1002:::::|h[Supplied B]|h|r";

fn seeded_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("fresh item-stat environment");
    let first = HashMap::from([
        ("ITEM_MOD_STRENGTH_SHORT".to_owned(), 120.0),
        ("ITEM_MOD_STAMINA_SHORT".to_owned(), 200.0),
        ("ITEM_MOD_HASTE_RATING_SHORT".to_owned(), 35.5),
        ("ITEM_MOD_VERSATILITY".to_owned(), 0.0),
    ]);
    let second = HashMap::from([
        ("ITEM_MOD_STRENGTH_SHORT".to_owned(), 90.0),
        ("ITEM_MOD_STAMINA_SHORT".to_owned(), 200.0),
        ("ITEM_MOD_CRIT_RATING_SHORT".to_owned(), 18.25),
        ("ITEM_MOD_VERSATILITY".to_owned(), 0.0),
    ]);
    env.state().borrow_mut().item_stats = ItemStatsState {
        by_link: HashMap::from([(LINK_A.to_owned(), first), (LINK_B.to_owned(), second)]),
    };
    env
}

fn install_assertions(env: &WowLuaEnv) {
    // These constants contain no quote/backslash escapes; publish only test inputs.
    env.exec(&format!("ISLinkA = '{LINK_A}'; ISLinkB = '{LINK_B}'"))
        .expect("publish exact fixture links");
    env.exec(
        r#"
        assert(type(C_Item.GetItemStats) == 'function', 'missing GetItemStats')
        assert(type(C_Item.GetItemStatDelta) == 'function', 'missing GetItemStatDelta')
        function ISMap(actual, expected)
            assert(type(actual) == 'table', 'expected stat table')
            for key, value in pairs(expected) do
                assert(type(actual[key]) == 'number' and actual[key] == value,
                    'wrong stat: ' .. key)
            end
            for key in pairs(actual) do
                assert(expected[key] ~= nil, 'unexpected stat: ' .. tostring(key))
            end
        end
        ISExpectedA = {ITEM_MOD_STRENGTH_SHORT=120, ITEM_MOD_STAMINA_SHORT=200,
            ITEM_MOD_HASTE_RATING_SHORT=35.5, ITEM_MOD_VERSATILITY=0}
        ISExpectedB = {ITEM_MOD_STRENGTH_SHORT=90, ITEM_MOD_STAMINA_SHORT=200,
            ITEM_MOD_CRIT_RATING_SHORT=18.25, ITEM_MOD_VERSATILITY=0}
        ISExpectedDelta = {ITEM_MOD_STRENGTH_SHORT=30,
            ITEM_MOD_HASTE_RATING_SHORT=35.5, ITEM_MOD_CRIT_RATING_SHORT=-18.25}
        function ISStringError(query, ...)
            local ok, message = pcall(query, ...)
            assert(not ok and type(message) == 'string', 'invalid type must error')
            assert(message:find('string', 1, true), 'required string error: ' .. message)
            assert(not message:find('untainted caller', 1, true),
                'ordinary or secure invalid input must reach type validation')
        end
        "#,
    )
    .expect("install observable table and error assertions");
}

fn assert_script(env: &WowLuaEnv, script: &str) {
    install_assertions(env);
    env.exec(script).expect("item-stat contract assertion");
}

#[test]
fn item_stats_supplied_links_return_both_exact_numeric_maps() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        ISMap(C_Item.GetItemStats(ISLinkA), ISExpectedA)
        ISMap(C_Item.GetItemStats(ISLinkB), ISExpectedB)
        "#,
    );
}

#[test]
fn item_stats_delta_is_first_minus_second_over_union_in_both_directions() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        ISMap(C_Item.GetItemStatDelta(ISLinkA, ISLinkB), ISExpectedDelta)
        ISMap(C_Item.GetItemStatDelta(ISLinkB, ISLinkA), {
            ITEM_MOD_STRENGTH_SHORT=-30, ITEM_MOD_HASTE_RATING_SHORT=-35.5,
            ITEM_MOD_CRIT_RATING_SHORT=18.25})
        "#,
    );
}

#[test]
fn item_stats_missing_exact_link_or_either_delta_operand_returns_nil() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        for _, missing in ipairs({'item:901001', '901001', '', 'item:6948',
            ISLinkA .. ' ', (ISLinkA:gsub('Supplied A', 'Different display name'))}) do
            assert(C_Item.GetItemStats(missing) == nil, 'no inferred/canonicalized stats')
            assert(C_Item.GetItemStatDelta(missing, ISLinkB) == nil, 'missing first')
            assert(C_Item.GetItemStatDelta(ISLinkA, missing) == nil, 'missing second')
            assert(C_Item.GetItemStatDelta(missing, missing) == nil, 'missing both')
        end
        ISMap(C_Item.GetItemStats(ISLinkA), ISExpectedA)
        "#,
    );
}

#[test]
fn item_stats_delta_omits_equal_and_explicit_zero_keys_but_returns_empty_table() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        local delta = C_Item.GetItemStatDelta(ISLinkA, ISLinkB)
        assert(delta.ITEM_MOD_STAMINA_SHORT == nil, 'equal nonzero stats omitted')
        assert(delta.ITEM_MOD_VERSATILITY == nil, 'equal explicit zeros omitted')
        ISMap(C_Item.GetItemStatDelta(ISLinkA, ISLinkA), {})
        ISMap(C_Item.GetItemStatDelta(ISLinkB, ISLinkB), {})
        assert(C_Item.GetItemStats(ISLinkA).ITEM_MOD_VERSATILITY == 0,
            'zero omission applies to delta, not supplied stats')
        "#,
    );
}

#[test]
fn item_stats_results_are_fresh_and_mutation_does_not_change_later_queries() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        local first = C_Item.GetItemStats(ISLinkA)
        local second = C_Item.GetItemStats(ISLinkA)
        local delta = C_Item.GetItemStatDelta(ISLinkA, ISLinkB)
        local otherDelta = C_Item.GetItemStatDelta(ISLinkA, ISLinkB)
        assert(not rawequal(first, second) and not rawequal(delta, otherDelta),
            'every call must return an independent table')
        first.ITEM_MOD_STRENGTH_SHORT = -999
        first.ITEM_MOD_STAMINA_SHORT = nil
        first.injected = 777
        delta.ITEM_MOD_STRENGTH_SHORT = 999
        delta.ITEM_MOD_HASTE_RATING_SHORT = nil
        delta.injected = 888
        ISMap(second, ISExpectedA)
        ISMap(otherDelta, ISExpectedDelta)
        ISMap(C_Item.GetItemStats(ISLinkA), ISExpectedA)
        ISMap(C_Item.GetItemStats(ISLinkB), ISExpectedB)
        ISMap(C_Item.GetItemStatDelta(ISLinkA, ISLinkB), ISExpectedDelta)
        "#,
    );
}

#[test]
fn item_stats_supplied_state_is_isolated_between_environments() {
    let first = seeded_env();
    let second = WowLuaEnv::new().expect("independent empty item-stat environment");
    assert_script(
        &second,
        r#"
        assert(C_Item.GetItemStats(ISLinkA) == nil)
        assert(C_Item.GetItemStats(ISLinkB) == nil)
        assert(C_Item.GetItemStatDelta(ISLinkA, ISLinkB) == nil)
        "#,
    );
    second.state().borrow_mut().item_stats.by_link.insert(
        LINK_A.to_owned(),
        HashMap::from([("ITEM_MOD_STRENGTH_SHORT".to_owned(), 7.0)]),
    );
    assert_script(
        &second,
        r#"
        ISMap(C_Item.GetItemStats(ISLinkA), {ITEM_MOD_STRENGTH_SHORT=7})
        assert(C_Item.GetItemStats(ISLinkB) == nil)
        assert(C_Item.GetItemStatDelta(ISLinkA, ISLinkB) == nil)
        "#,
    );
    assert_script(
        &first,
        r#"
        ISMap(C_Item.GetItemStats(ISLinkA), ISExpectedA)
        ISMap(C_Item.GetItemStats(ISLinkB), ISExpectedB)
        ISMap(C_Item.GetItemStatDelta(ISLinkA, ISLinkB), ISExpectedDelta)
        "#,
    );
}

#[test]
fn item_stats_plain_invalid_types_error_in_every_required_position() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        ISStringError(C_Item.GetItemStats)
        ISStringError(C_Item.GetItemStats, nil)
        ISStringError(C_Item.GetItemStatDelta)
        ISStringError(C_Item.GetItemStatDelta, ISLinkA)
        ISStringError(C_Item.GetItemStatDelta, nil, ISLinkB)
        ISStringError(C_Item.GetItemStatDelta, ISLinkA, nil)
        for _, value in ipairs({901001, false, true, {}, function() end,
            coroutine.create(function() end)}) do
            ISStringError(C_Item.GetItemStats, value)
            ISStringError(C_Item.GetItemStatDelta, value, ISLinkB)
            ISStringError(C_Item.GetItemStatDelta, ISLinkA, value)
        end
        "#,
    );
}

#[test]
fn item_stats_secure_genuine_secrets_unwrap_then_validate_each_position() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'secure caller required')
        local first, second = secretwrap(ISLinkA), secretwrap(ISLinkB)
        local missing = secretwrap('PRIVATE-ItemStats-missing')
        assert(issecretvalue(first) and issecretvalue(second) and issecretvalue(missing))
        ISMap(C_Item.GetItemStats(first), ISExpectedA)
        ISMap(C_Item.GetItemStats(second), ISExpectedB)
        ISMap(C_Item.GetItemStatDelta(first, ISLinkB), ISExpectedDelta)
        ISMap(C_Item.GetItemStatDelta(ISLinkA, second), ISExpectedDelta)
        ISMap(C_Item.GetItemStatDelta(first, second), ISExpectedDelta)
        assert(C_Item.GetItemStats(missing) == nil)
        assert(C_Item.GetItemStatDelta(missing, second) == nil)
        assert(C_Item.GetItemStatDelta(first, missing) == nil)
        for _, value in ipairs({secretwrap(901001), secretwrap(false), secretwrap({}),
            secretwrap(nil), secretwrap(function() end)}) do
            assert(issecretvalue(value), 'VM-owned invalid secret')
            ISStringError(C_Item.GetItemStats, value)
            ISStringError(C_Item.GetItemStatDelta, value, second)
            ISStringError(C_Item.GetItemStatDelta, first, value)
            assert(issecretvalue(value), 'validation must not destroy wrapper')
        end
        assert(debug.getstacktaint() == nil and issecretvalue(first) and issecretvalue(second))
        "#,
    );
}

#[test]
fn item_stats_tainted_secrets_are_denied_before_parsing_any_original_argument() {
    let env = seeded_env();
    assert_script(
        &env,
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'secure setup')
        local first, second = secretwrap(ISLinkA), secretwrap(ISLinkB)
        local values = {first, second, secretwrap('PRIVATE-ItemStats-missing'),
            secretwrap(901001), secretwrap(false), secretwrap({}), secretwrap(nil)}
        for _, value in ipairs(values) do assert(issecretvalue(value), 'genuine secret') end
        local function denied(query, ...)
            local ok, message = pcall(query, ...)
            assert(not ok and type(message) == 'string', 'secret must be denied')
            assert(message:find('untainted caller', 1, true),
                'authentication before parsing/lookup: ' .. message)
            assert(not message:find('PRIVATE-ItemStats', 1, true), 'no private payload leak')
            assert(debug.getstacktaint() == 'ItemStatsDraft', 'preserve tainted caller')
        end
        local function addon()
            assert(debug.getstacktaint() == 'ItemStatsDraft', 'genuinely tainted closure')
            for _, value in ipairs(values) do
                denied(C_Item.GetItemStats, value)
                denied(C_Item.GetItemStatDelta, value, ISLinkB)
                denied(C_Item.GetItemStatDelta, ISLinkA, value)
                denied(C_Item.GetItemStatDelta, false, value)
                denied(C_Item.GetItemStatDelta, nil, value)
                denied(C_Item.GetItemStatDelta, value, false)
                assert(issecretvalue(value), 'denial retains original wrapper')
                assert(not pcall(secretunwrap, value), 'tainted caller cannot unwrap')
            end
            denied(C_Item.GetItemStatDelta, first, second)
            ISMap(C_Item.GetItemStats(ISLinkA), ISExpectedA)
            ISMap(C_Item.GetItemStatDelta(ISLinkA, ISLinkB), ISExpectedDelta)
            ISStringError(C_Item.GetItemStats, false)
            ISStringError(C_Item.GetItemStatDelta, false, ISLinkB)
            ISStringError(C_Item.GetItemStatDelta, ISLinkA, false)
            assert(debug.getstacktaint() == 'ItemStatsDraft')
        end
        debug.setobjecttaint(addon, 'ItemStatsDraft')
        addon()
        assert(debug.getstacktaint() == nil, 'outer secure caller restored')
        for _, value in ipairs(values) do assert(issecretvalue(value)) end
        ISMap(C_Item.GetItemStats(first), ISExpectedA)
        ISMap(C_Item.GetItemStatDelta(first, second), ISExpectedDelta)
        "#,
    );
}
