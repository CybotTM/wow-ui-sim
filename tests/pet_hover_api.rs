//! Pet-hover API contract; behavioral RED/GREEN not yet executed.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn pet_hover_requires_string_without_numeric_coercion() {
    let env = WowLuaEnv::new().expect("pet hover environment");
    env.exec(
        r#"
        local setter = C_PetJournal.SetHoveredBattlePet
        local function stringError(...)
            local ok, message = pcall(setter, ...)
            assert(not ok and type(message) == 'string', 'required string must error')
            assert(message:find('string', 1, true), 'expected string validation: ' .. message)
        end
        stringError()
        stringError(nil)
        for _, value in ipairs({0, 901001, false, true, {}, function() end,
            coroutine.create(function() end)}) do
            stringError(value)
        end
        "#,
    )
    .expect("required strict string without coercion");
}

#[test]
fn pet_hover_authenticates_real_secrets_before_string_validation() {
    let env = WowLuaEnv::new().expect("pet hover security environment");
    env.exec(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil, 'untainted setup')
        local setter = C_PetJournal.SetHoveredBattlePet
        local guid = secretwrap(' opaque Pet-901001 ')
        local invalid = {secretwrap(901001), secretwrap(false), secretwrap({}), secretwrap(nil)}
        assert(issecretvalue(guid), 'authentic VM string wrapper')
        assert(select('#', setter(guid)) == 0, 'setter is void')
        for _, value in ipairs(invalid) do
            assert(issecretvalue(value), 'authentic malformed wrapper')
            local ok, message = pcall(setter, value)
            assert(not ok and type(message) == 'string', 'secure malformed input must error')
            assert(message:find('string', 1, true), 'secure input reaches type validation')
            assert(not message:find('untainted caller', 1, true), 'secure input not denied')
        end
        local function denied(value)
            local ok, message = pcall(setter, value)
            assert(not ok and type(message) == 'string', 'tainted secret must error')
            assert(message:find('untainted caller', 1, true), 'authentication precedes type validation')
            assert(not message:find('string', 1, true), 'security error not a type error')
            assert(issecretvalue(value), 'original wrapper retained')
            assert(not pcall(secretunwrap, value), 'tainted caller cannot unwrap')
            assert(debug.getstacktaint() == 'PetHoverProbe', 'denial preserves taint')
        end
        local function addon()
            assert(debug.getstacktaint() == 'PetHoverProbe', 'real tainted closure')
            denied(guid)
            for _, value in ipairs(invalid) do denied(value) end
            assert(select('#', setter('ordinary-public-guid')) == 0, 'public string allowed when tainted')
            assert(select('#', C_PetJournal.ClearHoveredBattlePet()) == 0, 'clear is void')
            assert(debug.getstacktaint() == 'PetHoverProbe', 'public mutations preserve taint')
        end
        debug.setobjecttaint(addon, 'PetHoverProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'secure caller restored')
        assert(issecretvalue(guid), 'success and failure retain original wrapper')
        assert(select('#', C_PetJournal.ClearHoveredBattlePet()) == 0, 'secure clear is void')
        "#,
    )
    .expect("AllowedWhenUntainted authentication uses VM secrets and tainted closures");
}
