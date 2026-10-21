//! Public spell-link boundary with authentic numeric VM secrets.
#![cfg(all(
    feature = "retail-12-0-5",
    any(feature = "profile-retail", feature = "client-ptr")
))]

use rilua::{LuaApi, LuaApiMut, Val};
use wow_ui_sim::lua_api::WowLuaEnv;

const EXPECTED_LINK: &str = "|cff71d5ff|Hspell:19750|h[Flash of Light]|h|r";

fn fixture() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    for (name, number) in [("Selector", 19750.0), ("Missing", 900002.0)] {
        let value = rilua::table_security::wrap_host_secret_number(lua.state_mut(), number);
        lua.state_mut().push(value);
        lua.set_global_val(name, value).unwrap();
        lua.state_mut().pop();
    }
    drop(lua);
    env
}

fn invoke(env: &WowLuaEnv, code: &str, tainted: bool) {
    env.exec(&format!(
        r#"
        local function probe()
            local before = debug.getstacktaint()
            assert(before == {expected})
            {code}
            assert(issecretvalue(Selector), 'selector stays secret')
            if {tainted} then
                assert(not canaccessvalue(Selector))
                assert(not pcall(secretunwrap, Selector))
            end
            assert(debug.getstacktaint() == before, 'caller taint preserved')
        end
        if {tainted} then debug.setobjecttaint(probe, 'SpellLinkProbe') end
        probe()
    "#,
        expected = if tainted { "'SpellLinkProbe'" } else { "nil" }
    ))
    .unwrap_or_else(|error| panic!("{code}: {error}"));
}

#[test]
fn numeric_secret_selector_returns_whole_secret_spell_link() {
    let env = fixture();
    for tainted in [false, true] {
        invoke(
            &env,
            r#"
            assert(select('#', C_Spell.GetSpellLink(Selector)) == 1)
            Link = C_Spell.GetSpellLink(Selector)
            assert(issecretvalue(Link), 'whole link must be secret, not nil or public')
            if canaccessvalue(Link) then
                assert(secretunwrap(Link) == C_Spell.GetSpellLink(19750))
            else
                assert(not pcall(secretunwrap, Link))
                assert(not pcall(function() return Link .. 'suffix' end))
            end
        "#,
            tainted,
        );
        env.exec("collectgarbage('collect'); collectgarbage('collect')")
            .unwrap();
        let value: Val = env.eval("return Link").unwrap();
        let lua = env.lua();
        let payload = rilua::table_security::unwrap_secret(lua.state(), value).unwrap();
        let Val::Str(string) = payload else {
            panic!("whole secret payload must be a string");
        };
        let bytes = lua.state().gc.string_arena.get(string).unwrap().data();
        assert_eq!(bytes, EXPECTED_LINK.as_bytes());
    }
}

#[test]
fn public_selector_returns_ordinary_spell_link() {
    let env = fixture();
    for tainted in [false, true] {
        invoke(
            &env,
            &format!(
                r#"
                local link = C_Spell.GetSpellLink(19750)
                assert(select('#', C_Spell.GetSpellLink(19750)) == 1)
                assert(not issecretvalue(link) and type(link) == 'string')
                assert(link == '{EXPECTED_LINK}')
            "#
            ),
            tainted,
        );
    }
}

#[test]
fn unknown_spell_link_returns_one_ordinary_nil() {
    let env = fixture();
    for tainted in [false, true] {
        invoke(
            &env,
            r#"
            for _, selector in ipairs({900002, Missing}) do
                assert(select('#', C_Spell.GetSpellLink(selector)) == 1)
                local link = C_Spell.GetSpellLink(selector)
                assert(link == nil and not issecretvalue(link), 'one ordinary nil on miss')
            end
        "#,
            tainted,
        );
    }
}

#[test]
fn secret_spell_link_glyph_argument_still_rejected() {
    let env = fixture();
    for tainted in [false, true] {
        invoke(
            &env,
            r#"
            for _, selector in ipairs({19750, Selector, Missing}) do
                local ok, err = pcall(C_Spell.GetSpellLink, selector, Selector)
                assert(not ok and string.find(err, 'argument 2 rejects secret values under NeverSecret policy', 1, true))
            end
        "#,
            tainted,
        );
    }
}
