//! Default no-association behavior is simulator policy, not native bounty data
//! or ordering. Positive explicit associations need separate tests once modeled.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

const QUERY_ASSERTIONS: &str = r#"
    local get = C_Map.GetBountySetMaps

    local function empty(...)
        local result = ...
        assert(type(result) == 'table', 'bounty set query must return a non-nil table')
        assert(select('#', ...) == 1, 'bounty set query returns exactly one value')
        assert(next(result) == nil, 'absent association returns an empty table')
        assert(#result == 0, 'absent association has zero sequence length')
        local count = 0
        for _ in ipairs(result) do
            count = count + 1
        end
        assert(count == 0, 'bounty board consumer sees an empty sequence')
        return result
    end

    local function rejected(...)
        local ok, message = pcall(get, ...)
        assert(not ok, 'invalid bounty set input must raise an error')
        assert(type(message) == 'string' and #message > 0, 'error has context')
    end
"#;

fn query_probe(body: &str) {
    let env = WowLuaEnv::new().expect("bare map environment");
    env.exec(&format!("{QUERY_ASSERTIONS}\n{body}"))
        .expect("public bounty set map behavior");
}

#[test]
fn c_map_bounty_set_maps_default_associations_return_one_empty_sequence() {
    query_probe(
        r#"
        empty(get(90001))
        empty(get(90002))
        "#,
    );
}

#[test]
fn c_map_bounty_set_maps_empty_results_are_fresh_and_mutation_isolated() {
    query_probe(
        r#"
        local first = empty(get(90001))
        local second = empty(get(90001))
        local other = empty(get(90002))
        assert(not rawequal(first, second), 'same set returns fresh tables')
        assert(not rawequal(first, other), 'different sets return fresh tables')
        assert(not rawequal(second, other), 'no shared empty table')
        first[1], first.extra = 2248, 'caller mutation'
        empty(second)
        empty(other)
        local third = empty(get(90001))
        assert(not rawequal(first, third) and not rawequal(second, third),
            'subsequent query returns a fresh unmodified table')
        other[1] = 2339
        empty(get(90002))
        empty(get(90001))
        "#,
    );
}

#[test]
fn c_map_bounty_set_maps_required_number_rejects_missing_nil_and_wrong_types() {
    query_probe(
        r#"
        rejected()
        rejected(nil)
        for _, value in ipairs({'90001', '', {}, false, true, function() end}) do
            rejected(value)
        end
        empty(get(90001))
        "#,
    );
}

#[test]
fn c_map_bounty_set_maps_numeric_edges_return_empty_without_associations() {
    query_probe(
        r#"
        -- Without positive associations this does not prove non-aliasing.
        for _, value in ipairs({
            0, -1, 90001.5, 90002.5, 0/0,
            math.huge, -math.huge, 1e308, -1e308
        }) do
            empty(get(value))
        end
        "#,
    );
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn c_map_bounty_set_maps_untainted_secret_inputs_preserve_wrappers() {
    query_probe(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'untainted entry')
        local first, second = secretwrap(90001), secretwrap(90002)
        empty(get(first))
        empty(get(second))
        for _, value in ipairs({first, second}) do
            assert(issecretvalue(value), 'query preserves secret wrapper')
        end
        assert(secretunwrap(first) == 90001 and secretunwrap(second) == 90002,
            'query preserves wrapped values')
        assert(debug.getstacktaint() == nil, 'query preserves caller context')
        "#,
    );
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn c_map_bounty_set_maps_tainted_secret_inputs_are_denied_before_lookup() {
    query_probe(
        r#"
        debug.settaintmode(true)
        local first, second = secretwrap(90001), secretwrap(90002)
        local function addon()
            assert(debug.getstacktaint() == 'BountySetMapsProbe', 'tainted entry')
            for _, value in ipairs({first, second}) do
                local ok, message = pcall(get, value)
                assert(not ok and type(message) == 'string', 'secret query denied')
                assert(string.find(message, 'untainted caller', 1, true),
                    'authentication precedes absent association lookup: ' .. tostring(message))
                assert(debug.getstacktaint() == 'BountySetMapsProbe', 'taint preserved')
                assert(issecretvalue(value), 'denial preserves wrapper')
                assert(not pcall(secretunwrap, value), 'no secret declassification')
                assert(debug.getstacktaint() == 'BountySetMapsProbe',
                    'failed declassification preserves taint')
            end
            empty(get(90001))
            empty(get(90002))
        end
        debug.setobjecttaint(addon, 'BountySetMapsProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'untainted caller restored')
        empty(get(first))
        empty(get(second))
        "#,
    );
}

// Supplied relations are test fixtures, not production membership or native order.
// API metadata: numeric ID, non-nil numeric map sequence, AllowedWhenUntainted.
const RELATION_ASSERTIONS: &str = r#"
    local function sequence(expected, ...)
        local result = ...
        assert(type(result) == 'table', 'bounty set query must return a non-nil table')
        assert(select('#', ...) == 1, 'bounty set query returns exactly one value')
        assert(#result == #expected, 'sequence length matches supplied relation')
        local keys = 0
        for key, value in pairs(result) do
            assert(type(key) == 'number' and key % 1 == 0
                and key >= 1 and key <= #expected, 'only dense sequence keys')
            assert(type(value) == 'number', 'map sequence values are numeric')
            assert(value == expected[key], 'supplied order and duplicates preserved')
            keys = keys + 1
        end
        assert(keys == #expected, 'every supplied element is present')
        local consumed = 0
        for index, value in ipairs(result) do
            consumed = consumed + 1
            assert(index == consumed and value == expected[index],
                'ipairs consumes the supplied relation in order')
        end
        assert(consumed == #expected, 'ipairs consumes the entire dense sequence')
        return result
    end
"#;

fn relation_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("bare supplied-relation environment");
    env.state().borrow_mut().bounty_set_maps = [
        (90001, vec![71, 19, 71]),
        (90002, vec![42, 19]),
        (90003, vec![]),
        (i32::MAX, vec![42]),
    ]
    .into_iter()
    .collect();
    env
}

fn relation_probe(env: &WowLuaEnv, body: &str) {
    env.exec(&format!(
        "{QUERY_ASSERTIONS}\n{RELATION_ASSERTIONS}\n{body}"
    ))
    .expect("public supplied bounty set map behavior");
}

#[test]
fn c_map_bounty_set_maps_supplied_relations_preserve_dense_order_and_duplicates() {
    let env = relation_env();
    relation_probe(
        &env,
        r#"
        sequence({71, 19, 71}, get(90001))
        sequence({42, 19}, get(90002))
        sequence({}, get(90003))
        empty(get(90004))
        "#,
    );
}

#[test]
fn c_map_bounty_set_maps_owned_updates_and_removal_preserve_snapshots_and_other_sets() {
    let env = relation_env();
    relation_probe(
        &env,
        r#"
        original = sequence({71, 19, 71}, get(90001))
        unrelated = sequence({42, 19}, get(90002))
        "#,
    );
    env.state()
        .borrow_mut()
        .bounty_set_maps
        .insert(90001, vec![19, 88]);
    relation_probe(
        &env,
        r#"
        sequence({71, 19, 71}, original)
        sequence({42, 19}, unrelated)
        updated = sequence({19, 88}, get(90001))
        assert(not rawequal(original, updated), 'update produces a new snapshot')
        sequence({42, 19}, get(90002))
        "#,
    );
    env.state().borrow_mut().bounty_set_maps.remove(&90001);
    relation_probe(
        &env,
        r#"
        empty(get(90001))
        sequence({71, 19, 71}, original)
        sequence({19, 88}, updated)
        sequence({42, 19}, unrelated)
        sequence({42, 19}, get(90002))
        empty(get(90003))
        empty(get(90004))
        "#,
    );
}

#[test]
fn c_map_bounty_set_maps_positive_results_are_fresh_and_mutation_isolated() {
    let env = relation_env();
    relation_probe(
        &env,
        r#"
        local first = sequence({71, 19, 71}, get(90001))
        local second = sequence({71, 19, 71}, get(90001))
        local other = sequence({42, 19}, get(90002))
        assert(not rawequal(first, second) and not rawequal(first, other)
            and not rawequal(second, other), 'positive queries return fresh tables')
        first[1], first[2], first.extra = 999, nil, 'caller mutation'
        sequence({71, 19, 71}, second)
        sequence({42, 19}, other)
        local third = sequence({71, 19, 71}, get(90001))
        assert(not rawequal(first, third) and not rawequal(second, third),
            'later positive query returns an independent snapshot')
        other[1], other[3] = 888, 777
        sequence({42, 19}, get(90002))
        sequence({71, 19, 71}, get(90001))
        "#,
    );
    let state = env.state().borrow();
    assert_eq!(state.bounty_set_maps.get(&90001), Some(&vec![71, 19, 71]));
    assert_eq!(state.bounty_set_maps.get(&90002), Some(&vec![42, 19]));
}

#[test]
fn c_map_bounty_set_maps_fractional_and_extreme_inputs_do_not_alias_supplied_ids() {
    let env = relation_env();
    relation_probe(
        &env,
        r#"
        sequence({71, 19, 71}, get(90001))
        sequence({42, 19}, get(90002))
        sequence({42}, get(2147483647))
        for _, value in ipairs({
            90001.5, 90002.5, 2147483647.5, 2147483648,
            4294967296 + 90001, 90002 - 4294967296,
            -2147483649, 0, -1, 0/0,
            math.huge, -math.huge, 1e308, -1e308
        }) do
            empty(get(value))
        end
        sequence({71, 19, 71}, get(90001))
        sequence({42, 19}, get(90002))
        sequence({42}, get(2147483647))
        "#,
    );
}

#[test]
fn c_map_bounty_set_maps_actual_map_util_consumes_relations_and_preserves_cache() {
    let env = relation_env();
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir()
        .join("Blizzard_FrameXMLUtil/Mainline/MapUtil.lua");
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read actual MapUtil source {}: {error}", path.display()));
    env.exec(&source)
        .expect("execute entire unmodified cached MapUtil.lua");
    relation_probe(
        &env,
        r#"
        cached = sequence({71, 19, 71}, MapUtil.GetBountySetMaps(90001))
        sequence({42, 19}, MapUtil.GetBountySetMaps(90002))
        cachedEmpty = sequence({}, MapUtil.GetBountySetMaps(90003))
        assert(rawequal(cached, MapUtil.GetBountySetMaps(90001)),
            'actual consumer memoizes the returned table')
        assert(rawequal(cachedEmpty, MapUtil.GetBountySetMaps(90003)),
            'actual consumer also memoizes empty tables')
        "#,
    );
    env.state()
        .borrow_mut()
        .bounty_set_maps
        .insert(90001, vec![88]);
    relation_probe(
        &env,
        r#"
        sequence({88}, get(90001))
        sequence({71, 19, 71}, MapUtil.GetBountySetMaps(90001))
        assert(rawequal(cached, MapUtil.GetBountySetMaps(90001)),
            'owned updates do not invalidate the consumer cache')
        cached[1] = 55
        sequence({55, 19, 71}, MapUtil.GetBountySetMaps(90001))
        sequence({88}, get(90001))
        sequence({42, 19}, MapUtil.GetBountySetMaps(90002))
        "#,
    );
}
