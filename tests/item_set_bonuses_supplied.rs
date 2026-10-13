//! Supplied item-set list contract; behavioral RED not yet observed.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

const ASSERTIONS: &str = r#"
    local query = C_Item.GetSetBonusesForSpecializationByItemID
    assert(type(query) == 'function', 'missing exact Retail C_Item query')
    function ISQuery(...) return query(...) end
    function ISList(expected, ...)
        assert(select('#', ...) == 1, 'supplied mapping returns one table')
        local result = ...
        assert(type(result) == 'table', 'spell ID list')
        assert(#result == #expected, 'exact supplied length')
        local count = 0
        for key, value in pairs(result) do
            count = count + 1
            assert(type(key) == 'number' and key >= 1 and key <= #expected,
                'only supplied sequence entries')
            assert(type(value) == 'number' and value == expected[key], 'exact ID/order')
        end
        assert(count == #expected, 'no additional result fields')
        for index, value in ipairs(expected) do assert(result[index] == value) end
        return result
    end
    function ISMiss(spec, item)
        assert(ISQuery(spec, item) == nil, 'absent supplied pair returns nil')
    end
    function ISNumberError(...)
        local ok, message = pcall(ISQuery, ...)
        assert(not ok and type(message) == 'string', 'invalid type must error')
        assert(message:find('number', 1, true), 'required-number validation')
        assert(not message:find('untainted caller', 1, true), 'not permission denial')
    end
"#;

fn environment() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("item-set draft environment");
    env.exec(ASSERTIONS).expect("behavioral assertion helpers");
    env
}

fn supplied_environment() -> WowLuaEnv {
    let env = environment();
    // Explicit artificial fixtures, not claims about these items' native bonuses.
    env.state().borrow_mut().item_set_bonuses.extend([
        ((70, 211995), vec![453729, 453728, 453729]),
        ((65, 211995), vec![454000]),
        ((70, 212000), vec![455001, 455002]),
        ((66, 211995), vec![]),
    ]);
    env
}

fn probe(env: &WowLuaEnv, script: &str) {
    let supplied = env.state().borrow().item_set_bonuses.clone();
    env.exec(script).expect("actual item-set query behavior");
    assert_eq!(
        env.state().borrow().item_set_bonuses,
        supplied,
        "query preserves supplier state"
    );
}

#[test]
fn item_set_absent_pairs_return_nil_without_derived_defaults() {
    let env = environment();
    probe(
        &env,
        "ISMiss(70, 211995); ISMiss(65, 6948); ISMiss(66, 901001)",
    );
}

#[test]
fn item_set_pair_lookup_preserves_exact_spell_ids_order_and_duplicates() {
    let env = supplied_environment();
    probe(
        &env,
        r#"
        ISList({453729, 453728, 453729}, ISQuery(70, 211995))
        ISList({454000}, ISQuery(65, 211995))
        ISList({455001, 455002}, ISQuery(70, 212000))
        ISMiss(65, 212000)
        ISMiss(211995, 70)
    "#,
    );
}

#[test]
fn item_set_supplied_empty_list_is_distinct_from_absence() {
    let env = supplied_environment();
    probe(
        &env,
        r#"
        local empty = ISList({}, ISQuery(66, 211995))
        assert(next(empty) == nil)
        ISMiss(66, 212000)
    "#,
    );
}

#[test]
fn item_set_results_are_fresh_and_mutation_cannot_change_later_queries() {
    let env = supplied_environment();
    probe(
        &env,
        r#"
        local first = ISList({453729, 453728, 453729}, ISQuery(70, 211995))
        local second = ISList({453729, 453728, 453729}, ISQuery(70, 211995))
        assert(not rawequal(first, second), 'fresh result per call')
        first[1] = 1; first[2] = nil; first[4] = 2; first.marker = 'caller-owned'
        ISList({453729, 453728, 453729}, second)
        ISList({453729, 453728, 453729}, ISQuery(70, 211995))
        local empty = ISList({}, ISQuery(66, 211995))
        local another = ISList({}, ISQuery(66, 211995))
        assert(not rawequal(empty, another), 'empty results also fresh')
        empty[1] = 999
        ISList({}, another)
        ISList({}, ISQuery(66, 211995))
    "#,
    );
}

#[test]
fn item_set_supplied_pairs_are_environment_local() {
    let first = supplied_environment();
    let second = environment();
    probe(&second, "ISMiss(70, 211995)");
    second
        .state()
        .borrow_mut()
        .item_set_bonuses
        .insert((70, 211995), vec![999001]);
    probe(
        &first,
        "ISList({453729, 453728, 453729}, ISQuery(70, 211995))",
    );
    probe(&second, "ISList({999001}, ISQuery(70, 211995))");
    let third = environment();
    probe(&third, "ISMiss(70, 211995)");
}

#[test]
fn item_set_numeric_selectors_do_not_truncate_or_saturate_to_supplied_pairs() {
    let env = supplied_environment();
    env.state().borrow_mut().item_set_bonuses.extend([
        ((0, 211995), vec![999010]),
        ((i32::MIN, 211995), vec![999011]),
        ((i32::MAX, 211995), vec![999012]),
        ((70, 0), vec![999013]),
        ((70, i32::MIN), vec![999014]),
        ((70, i32::MAX), vec![999015]),
    ]);
    probe(
        &env,
        r#"
        ISList({999010}, ISQuery(0, 211995))
        ISList({999011}, ISQuery(-2147483648, 211995))
        ISList({999012}, ISQuery(2147483647, 211995))
        ISList({999013}, ISQuery(70, 0))
        ISList({999014}, ISQuery(70, -2147483648))
        ISList({999015}, ISQuery(70, 2147483647))
        for _, value in ipairs({70.5, 211995.5, -0.5, 0.5,
            -2147483648.5, 2147483647.5, -2147483649, 2147483648,
            0/0, 1/0, -1/0}) do
            ISMiss(value, 211995)
            ISMiss(70, value)
        end
    "#,
    );
}

#[test]
fn item_set_both_arguments_require_numbers_without_string_coercion() {
    let env = supplied_environment();
    probe(
        &env,
        r#"
        ISNumberError()
        ISNumberError(70)
        ISNumberError(nil, 211995)
        ISNumberError(70, nil)
        for _, value in ipairs({false, true, '70', '211995', 'item:211995', {},
            function() end, coroutine.create(function() end)}) do
            ISNumberError(value, 211995)
            ISNumberError(70, value)
        end
        ISList({453729, 453728, 453729}, ISQuery(70, 211995))
    "#,
    );
}

#[test]
fn item_set_untainted_callers_accept_genuine_secrets_in_either_original_argument() {
    let env = supplied_environment();
    probe(
        &env,
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local spec, item = secretwrap(70), secretwrap(211995)
        assert(issecretvalue(spec) and issecretvalue(item), 'real VM wrappers')
        ISList({453729, 453728, 453729}, ISQuery(spec, 211995))
        ISList({453729, 453728, 453729}, ISQuery(70, item))
        ISList({453729, 453728, 453729}, ISQuery(spec, item))
        ISList({}, ISQuery(secretwrap(66), item))
        ISMiss(secretwrap(65), secretwrap(212000))
        for _, value in ipairs({secretwrap('70'), secretwrap(false),
            secretwrap({}), secretwrap(nil)}) do
            assert(issecretvalue(value))
            ISNumberError(value, 211995)
            ISNumberError(70, value)
            assert(issecretvalue(value), 'validation retains original wrapper')
        end
        assert(issecretvalue(spec) and issecretvalue(item))
        assert(secretunwrap(spec) == 70 and secretunwrap(item) == 211995)
        assert(debug.getstacktaint() == nil, 'secure caller preserved')
    "#,
    );
}

#[test]
fn item_set_tainted_secrets_deny_before_either_argument_validation_or_miss() {
    let env = supplied_environment();
    probe(
        &env,
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local spec, item = secretwrap(70), secretwrap(211995)
        local secretMiss = secretwrap(901001)
        local malformed = {secretwrap('70'), secretwrap(false), secretwrap({}), secretwrap(nil)}
        local function denied(a, b, original)
            assert(issecretvalue(original), 'real original VM secret')
            local ok, message = pcall(ISQuery, a, b)
            assert(not ok and type(message) == 'string', 'permission denial')
            assert(message:find('untainted caller', 1, true), 'authenticate BOTH originals first')
            assert(not message:find('must be a number', 1, true), 'no earlier type error')
            assert(issecretvalue(original), 'wrapper retained')
            assert(not pcall(secretunwrap, original), 'caller cannot declassify')
            assert(debug.getstacktaint() == 'ItemSetDraft', 'denial preserves caller taint')
        end
        local function addon()
            assert(debug.getstacktaint() == 'ItemSetDraft', 'genuinely tainted closure')
            denied(spec, 211995, spec)
            denied(70, item, item)
            denied(spec, item, item)
            -- Invalid/missing public first input cannot mask a secret second input.
            denied(false, item, item)
            denied(nil, item, item)
            denied('70', item, item)
            denied(spec, false, spec)
            denied(spec, nil, spec)
            -- A miss must not short-circuit second-input authentication.
            denied(901001, item, item)
            denied(70, secretMiss, secretMiss)
            for _, value in ipairs(malformed) do
                denied(value, 211995, value)
                denied(false, value, value)
            end
            ISList({453729, 453728, 453729}, ISQuery(70, 211995))
            ISList({}, ISQuery(66, 211995))
            ISMiss(65, 212000)
            ISNumberError(false, 211995)
            assert(debug.getstacktaint() == 'ItemSetDraft', 'public recovery retains taint')
        end
        debug.setobjecttaint(addon, 'ItemSetDraft')
        addon()
        assert(debug.getstacktaint() == nil, 'outer caller restored')
        assert(issecretvalue(spec) and issecretvalue(item))
        ISList({453729, 453728, 453729}, ISQuery(spec, item))
    "#,
    );
}
