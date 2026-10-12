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
