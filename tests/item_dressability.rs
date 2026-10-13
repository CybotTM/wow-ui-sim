//! Approved empty host dressability policy; not native item classification proof.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn item_dressability_empty_input_returns_exactly_one_boolean_false() {
    let env = WowLuaEnv::new().expect("fresh dressability environment");
    env.exec(
        r#"
        local query = C_Item.IsDressableItemByID
        assert(type(query) == 'function', 'missing public C_Item.IsDressableItemByID')
        local function miss(...)
            assert(select('#', ...) == 1, 'dressability miss must return exactly one value')
            local result = ...
            assert(type(result) == 'boolean' and result == false,
                'empty supplied classification must return boolean false')
        end
        local _, link = C_Item.GetItemInfo(6948)
        assert(type(link) == 'string' and link:find('|Hitem:6948:', 1, true),
            'existing generated item hyperlink required')
        miss(query(6948))
        miss(query(901001))
        miss(query('item:6948'))
        miss(query(link))
        "#,
    )
    .expect("numeric and supported item-link misses use the approved empty default");
}

#[test]
fn item_dressability_requires_item_info_and_authenticates_vm_secrets() {
    let env = WowLuaEnv::new().expect("fresh dressability argument environment");
    env.exec(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'untainted setup required')
        local query = C_Item.IsDressableItemByID
        assert(type(query) == 'function', 'missing public C_Item.IsDressableItemByID')
        local function argumentError(...)
            local ok, message = pcall(query, ...)
            assert(not ok and type(message) == 'string', 'required ItemInfo must reject input')
            assert(not message:find('untainted caller', 1, true),
                'ordinary or untainted malformed argument must reach argument validation')
        end
        argumentError()
        argumentError(nil)
        argumentError(false)
        argumentError({})
        argumentError(function() end)

        local _, link = C_Item.GetItemInfo(6948)
        assert(type(link) == 'string' and link:find('|Hitem:6948:', 1, true),
            'existing generated item hyperlink required')
        local numeric = secretwrap(6948)
        local hyperlink = secretwrap(link)
        local malformed = secretwrap(false)
        local missing = secretwrap(nil)
        local function miss(...)
            assert(select('#', ...) == 1, 'exact secret/ordinary miss arity')
            assert(type((...)) == 'boolean' and (...) == false, 'approved false miss')
        end
        for _, value in ipairs({numeric, hyperlink, malformed, missing}) do
            assert(issecretvalue(value), 'authentic VM secret required')
        end
        miss(query(numeric))
        miss(query(hyperlink))
        argumentError(malformed)
        argumentError(missing)

        local function denied(value)
            assert(issecretvalue(value), 'original VM wrapper required before denial')
            local ok, message = pcall(query, value)
            assert(not ok and type(message) == 'string', 'tainted secret must be denied')
            assert(message:find('untainted caller', 1, true),
                'AllowedWhenUntainted denial must precede parsing/lookup: ' .. message)
            assert(issecretvalue(value), 'denial must retain original wrapper')
            assert(not pcall(secretunwrap, value), 'tainted caller cannot unwrap secret')
            assert(debug.getstacktaint() == 'DressabilityTest', 'denial preserves caller taint')
        end
        local function addon()
            assert(debug.getstacktaint() == 'DressabilityTest', 'real tainted closure required')
            denied(numeric)
            denied(hyperlink)
            denied(malformed)
            denied(missing)
            miss(query(6948))
            miss(query(link))
            argumentError(false)
            assert(debug.getstacktaint() == 'DressabilityTest', 'ordinary queries preserve taint')
        end
        debug.setobjecttaint(addon, 'DressabilityTest')
        addon()
        assert(debug.getstacktaint() == nil, 'outer untainted caller restored')
        assert(issecretvalue(numeric) and issecretvalue(hyperlink), 'wrappers survive round trip')
        miss(query(numeric))
        miss(query(hyperlink))
        "#,
    )
    .expect("required ItemInfo and AllowedWhenUntainted use real VM security");
}
